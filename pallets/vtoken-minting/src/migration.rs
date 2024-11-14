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

use crate::{BalanceOf, Config, TokenUnlockLedger};
use bifrost_primitives::{RedeemType, TimeUnit};
use frame_support::{pallet_prelude::*, storage_alias, traits::OnRuntimeUpgrade};
use sp_std::marker::PhantomData;

/// Migrate TokenUnlockLedger
/// (T::AccountId, BalanceOf<T>, TimeUnit) to (T::AccountId, BalanceOf<T>,TimeUnit, RedeemType)
pub struct MigrateTokenUnlockLedger<T>(PhantomData<T>);
impl<T: Config> OnRuntimeUpgrade for MigrateTokenUnlockLedger<T> {
	fn on_runtime_upgrade() -> Weight {
		log::info!("MigrateTokenUnlockLedger::on_runtime_upgrade execute",);

		let mut weight: Weight = Weight::zero();

		// migrate the value type of TokenUnlockLedger
		TokenUnlockLedger::<T>::translate(
			|_key1, _key2, old_value: (T::AccountId, BalanceOf<T>, TimeUnit)| {
				weight.saturating_accrue(T::DbWeight::get().reads_writes(1, 1));
				let new_value = (old_value.0, old_value.1, old_value.2, RedeemType::Native);
				Some(new_value)
			},
		);

		weight
	}
}

pub mod v1 {
	use super::*;
	use crate::{
		types::{ActiveState, CurrencyConfiguration, CurrencyIdOf},
		ActiveStateByCurrency, ConfigurationByCurrency, Pallet,
	};
	use sp_runtime::Permill;

	#[storage_alias]
	pub type Fees<T: Config> = StorageValue<Pallet<T>, (Permill, Permill), ValueQuery>;

	#[storage_alias]
	pub type TokenPool<T: Config> =
		StorageMap<Pallet<T>, Twox64Concat, CurrencyIdOf<T>, BalanceOf<T>, ValueQuery>;

	#[storage_alias]
	pub type UnlockDuration<T: Config> =
		StorageMap<Pallet<T>, Twox64Concat, CurrencyIdOf<T>, TimeUnit>;

	#[storage_alias]
	pub type OngoingTimeUnit<T: Config> =
		StorageMap<Pallet<T>, Twox64Concat, CurrencyIdOf<T>, TimeUnit>;

	#[storage_alias]
	pub type MinimumMint<T: Config> =
		StorageMap<Pallet<T>, Twox64Concat, CurrencyIdOf<T>, BalanceOf<T>, ValueQuery>;

	#[storage_alias]
	pub type MinimumRedeem<T: Config> =
		StorageMap<Pallet<T>, Twox64Concat, CurrencyIdOf<T>, BalanceOf<T>, ValueQuery>;

	#[storage_alias]
	pub type TokenUnlockNextId<T: Config> =
		StorageMap<Pallet<T>, Twox64Concat, CurrencyIdOf<T>, u32, ValueQuery>;

	#[storage_alias]
	pub type TokenToRebond<T: Config> =
		StorageMap<Pallet<T>, Twox64Concat, CurrencyIdOf<T>, BalanceOf<T>>;

	#[storage_alias]
	pub type UnlockingTotal<T: Config> =
		StorageMap<Pallet<T>, Twox64Concat, CurrencyIdOf<T>, BalanceOf<T>, ValueQuery>;

	pub struct MigrateToV1<T>(PhantomData<T>);
	impl<T: Config> OnRuntimeUpgrade for MigrateToV1<T> {
		fn on_runtime_upgrade() -> Weight {
			// Check the storage version
			let in_code_version = Pallet::<T>::in_code_storage_version();
			let on_chain_version = Pallet::<T>::on_chain_storage_version();
			if on_chain_version == 0 && in_code_version == 1 {
				let (mint_fee_rate, redeem_fee_rate) = v1::Fees::<T>::take();
				v1::OngoingTimeUnit::<T>::drain().for_each(|(currency_id, ongoing_time_unit)| {
					let unlock_duration = v1::UnlockDuration::<T>::take(currency_id)
						.expect("unlock_duration should exist");
					let min_mint_amount = v1::MinimumMint::<T>::take(currency_id);
					let min_redeem_amount = v1::MinimumRedeem::<T>::take(currency_id);
					let total_restake_amount = v1::TokenToRebond::<T>::take(currency_id);

					ConfigurationByCurrency::<T>::insert(
						currency_id,
						CurrencyConfiguration {
							mint_fee_rate,
							redeem_fee_rate,
							unlock_duration,
							min_mint_amount,
							min_redeem_amount,
							is_supported_restake: total_restake_amount.is_some(),
							is_supported_fast_redeem: true,
						},
					);

					let total_stake_amount = v1::TokenPool::<T>::take(currency_id);
					let total_unstake_amount = v1::UnlockingTotal::<T>::take(currency_id);
					let total_restake_amount = match total_restake_amount {
						Some(amount) => amount,
						None => Default::default(),
					};
					let next_redeem_id = v1::TokenUnlockNextId::<T>::take(currency_id);
					ActiveStateByCurrency::<T>::insert(
						currency_id,
						ActiveState {
							total_stake_amount,
							total_unstake_amount,
							total_restake_amount,
							ongoing_time_unit,
							next_redeem_id,
						},
					);
				});
				Weight::zero()
			} else {
				// We don't do anything here.
				Weight::zero()
			}
		}

		#[cfg(feature = "try-runtime")]
		fn pre_upgrade() -> Result<Vec<u8>, sp_runtime::DispatchError> {
			log::info!(
				"vtoken-voting before migration: version: {:?}",
				StorageVersion::get::<Pallet<T>>(),
			);

			Ok(Vec::new())
		}

		#[cfg(feature = "try-runtime")]
		fn post_upgrade(_: Vec<u8>) -> Result<(), sp_runtime::DispatchError> {
			log::info!(
				"vtoken-voting after migration: version: {:?}",
				StorageVersion::get::<Pallet<T>>(),
			);

			Ok(())
		}
	}
}
