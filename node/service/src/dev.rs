// This file is part of Bifrost.

// Copyright (C) Liebi Technologies PTE. LTD.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

use bifrost_kusama_runtime::{TransactionConverter, SLOT_DURATION};
use cumulus_primitives_core::{ParaId, PersistedValidationData};
use cumulus_primitives_parachain_inherent::{
	MockValidationDataInherentDataProvider, MockXcmConfig, ParachainInherentData,
};
use cumulus_test_relay_sproof_builder::RelayStateSproofBuilder;
use futures::StreamExt;
use jsonrpsee::core::async_trait;
use sc_client_api::Backend;
use sc_executor::NativeElseWasmExecutor;
use sc_service::{error::Error as ServiceError, Configuration, TaskManager};
use sc_transaction_pool_api::OffchainTransactionPoolFactory;
use sp_core::U256;
use std::{
	cell::RefCell,
	collections::BTreeMap,
	sync::{Arc, Mutex},
};

use crate::{
	collator_kusama::FullClient,
	eth::{spawn_frontier_tasks, EthConfiguration},
};

pub type Block = bifrost_primitives::Block;
pub type Executor = crate::collator_kusama::BifrostExecutor;
pub type RuntimeApi = crate::collator_kusama::bifrost_kusama_runtime::RuntimeApi;

pub type FullBackend = sc_service::TFullBackend<Block>;
pub type FullSelectChain = sc_consensus::LongestChain<FullBackend, Block>;

pub fn default_mock_parachain_inherent_data_provider() -> MockValidationDataInherentDataProvider {
	MockValidationDataInherentDataProvider {
		current_para_block: 0,
		relay_offset: 1000,
		relay_blocks_per_para_block: 2,
		xcm_config: Default::default(),
		raw_downward_messages: vec![],
		raw_horizontal_messages: vec![],
		para_blocks_per_relay_epoch: 0,
		relay_randomness_config: (),
	}
}

thread_local!(static TIMESTAMP: RefCell<u64> = const { RefCell::new(0) });

/// Provide a mock duration starting at 0 in millisecond for timestamp inherent.
/// Each call will increment timestamp by slot_duration making Aura think time has passed.
struct MockTimestampInherentDataProvider;

#[async_trait]
impl sp_inherents::InherentDataProvider for MockTimestampInherentDataProvider {
	async fn provide_inherent_data(
		&self,
		inherent_data: &mut sp_inherents::InherentData,
	) -> Result<(), sp_inherents::Error> {
		TIMESTAMP.with(|x| {
			*x.borrow_mut() += SLOT_DURATION;
			inherent_data.put_data(sp_timestamp::INHERENT_IDENTIFIER, &*x.borrow())
		})
	}

	async fn try_handle_error(
		&self,
		_identifier: &sp_inherents::InherentIdentifier,
		_error: &[u8],
	) -> Option<Result<(), sp_inherents::Error>> {
		// The pallet never reports error.
		None
	}
}

/// Builds a new development service. This service uses manual seal, and mocks
/// the parachain inherent.
/// Before calling this function, you must set OnTimestampSet in runtime to be ().
pub async fn start_node(
	config: Configuration,
	eth_config: EthConfiguration,
) -> sc_service::error::Result<(TaskManager, Arc<FullClient>)> {
	let params = crate::collator_kusama::new_partial(&config, &eth_config, true)?;
	let (block_import, mut telemetry, telemetry_worker_handle, frontier_backend) = params.other;

	let client = params.client.clone();
	let backend = params.backend.clone();
	let mut task_manager = params.task_manager;

	let validator = config.role.is_authority();
	let prometheus_registry = config.prometheus_registry().cloned();
	let transaction_pool = params.transaction_pool.clone();
	let net_config = sc_network::config::FullNetworkConfiguration::new(&config.network);

	let (network, system_rpc_tx, tx_handler_controller, start_network, sync_service) =
		sc_service::build_network(sc_service::BuildNetworkParams {
			config: &config,
			net_config,
			client: client.clone(),
			transaction_pool: transaction_pool.clone(),
			spawn_handle: task_manager.spawn_handle(),
			import_queue: params.import_queue,
			block_announce_validator_builder: None,
			warp_sync_params: None,
			block_relay: None,
		})?;

	let prometheus_registry = config.prometheus_registry().cloned();

	if config.offchain_worker.enabled {
		use futures::FutureExt;

		let backend_ofc = backend.clone();
		task_manager.spawn_handle().spawn(
			"offchain-workers-runner",
			"offchain-work",
			sc_offchain::OffchainWorkers::new(sc_offchain::OffchainWorkerOptions {
				runtime_api_provider: client.clone(),
				keystore: Some(params.keystore_container.keystore()),
				offchain_db: backend_ofc.offchain_storage(),
				transaction_pool: Some(OffchainTransactionPoolFactory::new(
					transaction_pool.clone(),
				)),
				network_provider: network.clone(),
				is_validator: config.role.is_authority(),
				enable_http_requests: false,
				custom_extensions: move |_| vec![],
			})
			.run(client.clone(), task_manager.spawn_handle())
			.boxed(),
		);
	}

	let role = config.role.clone();
	let force_authoring = config.force_authoring;
	let backoff_authoring_blocks: Option<()> = None;

	let select_chain = params
		.select_chain
		.expect("In `dev` mode, `new_partial` will return some `select_chain`; qed");

	let proposer_factory = sc_basic_authorship::ProposerFactory::new(
		task_manager.spawn_handle(),
		client.clone(),
		transaction_pool.clone(),
		None,
		None,
	);

	// Channel for the rpc handler to communicate with the authorship task.
	let (command_sink, commands_stream) = futures::channel::mpsc::channel(1024);

	let pool = transaction_pool.pool().clone();

	let client_for_cidp = client.clone();

	let (downward_xcm_sender, downward_xcm_receiver) = flume::bounded::<Vec<u8>>(100);
	let (hrmp_xcm_sender, hrmp_xcm_receiver) = flume::bounded::<(ParaId, Vec<u8>)>(100);

	let authorship_future =
		sc_consensus_manual_seal::run_manual_seal(sc_consensus_manual_seal::ManualSealParams {
			block_import: client.clone(),
			env: proposer_factory,
			client: client.clone(),
			pool: transaction_pool.clone(),
			commands_stream,
			select_chain,
			consensus_data_provider: None,
			create_inherent_data_providers: move |block, _| {
				let current_para_block = client_for_cidp
					.header(block)
					.ok()
					.flatten()
					.expect("Header lookup should succeed")
					.number;
				let downward_xcm_receiver = downward_xcm_receiver.clone();
				let hrmp_xcm_receiver = hrmp_xcm_receiver.clone();
				let client_for_xcm = client_for_cidp.clone();
				async move {
					let mocked_parachain = MockValidationDataInherentDataProvider {
						current_para_block,
						relay_offset: 1000,
						relay_blocks_per_para_block: 2,
						para_blocks_per_relay_epoch: 0,
						relay_randomness_config: (),
						xcm_config: MockXcmConfig::new(
							&*client_for_xcm,
							block,
							Default::default(),
							Default::default(),
						),
						raw_downward_messages: downward_xcm_receiver.drain().collect(),
						raw_horizontal_messages: hrmp_xcm_receiver.drain().collect(),
					};
					let timestamp = sp_timestamp::InherentDataProvider::from_system_time();
					Ok((timestamp, mocked_parachain))
				}
			},
		});
	// we spawn the future on a background thread managed by service.
	task_manager.spawn_essential_handle().spawn_blocking(
		"manual-seal",
		Some("block-authoring"),
		authorship_future,
	);

	let overrides = crate::rpc::overrides_handle(client.clone());

	// Sinks for pubsub notifications.
	// Everytime a new subscription is created, a new mpsc channel is added to the sink pool.
	// The MappingSyncWorker sends through the channel on block import and the subscription emits a
	// notification to the subscriber on receiving a message through this channel.
	// This way we avoid race conditions when using native substrate block import notification
	// stream.
	let pubsub_notification_sinks: fc_mapping_sync::EthereumBlockNotificationSinks<
		fc_mapping_sync::EthereumBlockNotification<Block>,
	> = Default::default();
	let pubsub_notification_sinks = Arc::new(pubsub_notification_sinks);

	let filter_pool = Some(Arc::new(Mutex::new(BTreeMap::new())));
	let fee_history_cache = Arc::new(Mutex::new(BTreeMap::new()));
	// let fee_history_cache_limit = parachain_config.fee_history_limit;
	let fee_history_cache_limit = 2048;

	let rpc_builder = {
		let client = client.clone();
		let pool = transaction_pool.clone();
		let network = network.clone();
		let sync_service = sync_service.clone();

		let is_authority = config.role.is_authority();
		let enable_dev_signer = eth_config.enable_dev_signer;
		let max_past_logs = eth_config.max_past_logs;
		let execute_gas_limit_multiplier = eth_config.execute_gas_limit_multiplier;
		let filter_pool = filter_pool.clone();
		let frontier_backend = frontier_backend.clone();
		let pubsub_notification_sinks = pubsub_notification_sinks.clone();
		let overrides = overrides.clone();
		let fee_history_cache = fee_history_cache.clone();
		let block_data_cache = Arc::new(fc_rpc::EthBlockDataCacheTask::new(
			task_manager.spawn_handle(),
			overrides.clone(),
			eth_config.eth_log_block_cache,
			eth_config.eth_statuses_cache,
			prometheus_registry.clone(),
		));

		let slot_duration = sc_consensus_aura::slot_duration(&*client)?;
		let target_gas_price = eth_config.target_gas_price;
		let pending_create_inherent_data_providers = move |_, ()| async move {
			let current = sp_timestamp::InherentDataProvider::from_system_time();
			let next_slot = current.timestamp().as_millis() + slot_duration.as_millis();
			let timestamp = sp_timestamp::InherentDataProvider::new(next_slot.into());
			let slot = sp_consensus_aura::inherents::InherentDataProvider::from_timestamp_and_slot_duration(
				*timestamp,
				slot_duration,
			);
			let dynamic_fee = fp_dynamic_fee::InherentDataProvider(U256::from(target_gas_price));
			// Create a dummy parachain inherent data provider which is required to pass
			// the checks by the para chain system. We use dummy values because in the 'pending
			// context' neither do we have access to the real values nor do we need them.
			let (relay_parent_storage_root, relay_chain_state) =
				RelayStateSproofBuilder::default().into_state_root_and_proof();
			let vfp = PersistedValidationData {
				// This is a hack to make
				// `cumulus_pallet_parachain_system::RelayNumberStrictlyIncreases` happy. Relay
				// parent number can't be bigger than u32::MAX.
				relay_parent_number: u32::MAX,
				relay_parent_storage_root,
				..Default::default()
			};
			let parachain_inherent_data = ParachainInherentData {
				validation_data: vfp,
				relay_chain_state,
				downward_messages: Default::default(),
				horizontal_messages: Default::default(),
			};
			Ok((slot, timestamp, dynamic_fee, parachain_inherent_data))
		};

		Box::new(move |deny_unsafe, subscription_task_executor| {
			let eth_deps = crate::rpc::EthDeps {
				client: client.clone(),
				pool: pool.clone(),
				graph: pool.pool().clone(),
				converter: Some(TransactionConverter),
				is_authority,
				enable_dev_signer,
				network: network.clone(),
				sync: sync_service.clone(),
				frontier_backend: match frontier_backend.clone() {
					fc_db::Backend::KeyValue(b) => Arc::new(b),
					fc_db::Backend::Sql(b) => Arc::new(b),
				},
				overrides: overrides.clone(),
				block_data_cache: block_data_cache.clone(),
				filter_pool: filter_pool.clone(),
				max_past_logs,
				fee_history_cache: fee_history_cache.clone(),
				fee_history_cache_limit,
				execute_gas_limit_multiplier,
				forced_parent_hashes: None,
				pending_create_inherent_data_providers,
			};
			let deps = crate::rpc::FullDeps {
				client: client.clone(),
				pool: pool.clone(),
				deny_unsafe,
				command_sink: Some(command_sink.clone()),
				eth: eth_deps,
			};
			crate::rpc::create_full(
				deps,
				subscription_task_executor,
				pubsub_notification_sinks.clone(),
			)
			.map_err(Into::into)
		})
	};

	sc_service::spawn_tasks(sc_service::SpawnTasksParams {
		rpc_builder,
		client: client.clone(),
		transaction_pool: transaction_pool.clone(),
		task_manager: &mut task_manager,
		config,
		keystore: params.keystore_container.keystore(),
		backend: backend.clone(),
		network: network.clone(),
		sync_service: sync_service.clone(),
		system_rpc_tx,
		tx_handler_controller,
		telemetry: telemetry.as_mut(),
	})?;
	start_network.start_network();

	spawn_frontier_tasks(
		&task_manager,
		client.clone(),
		backend,
		frontier_backend,
		filter_pool,
		overrides,
		fee_history_cache,
		fee_history_cache_limit,
		sync_service.clone(),
		pubsub_notification_sinks,
	)
	.await;

	Ok((task_manager, client))
}
