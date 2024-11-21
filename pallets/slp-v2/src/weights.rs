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
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Autogenerated weights for bifrost_slp_v2
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 32.0.0
//! DATE: 2024-11-21, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `mjl-legion`, CPU: `12th Gen Intel(R) Core(TM) i9-12900H`
//! WASM-EXECUTION: Compiled, CHAIN: Some("bifrost-polkadot-local"), DB CACHE: 1024

// Executed Command:
// target/release/bifrost
// benchmark
// pallet
// --chain=bifrost-polkadot-local
// --steps=50
// --repeat=20
// --pallet=bifrost_slp_v2
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./pallets/slp-v2/src/weights.rs
// --template=./weight-template/pallet-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for bifrost_slp_v2.
pub trait WeightInfo {
	fn add_delegator() -> Weight;
	fn remove_delegator() -> Weight;
	fn add_validator() -> Weight;
	fn remove_validator() -> Weight;
	fn set_protocol_configuration() -> Weight;
	fn set_ledger() -> Weight;
	fn transfer_to() -> Weight;
	fn transfer_back() -> Weight;
	fn update_ongoing_time_unit() -> Weight;
	fn update_token_exchange_rate() -> Weight;
	fn astar_dapp_staking() -> Weight;
	fn notify_astar_dapp_staking() -> Weight;
}

// For backwards compatibility and tests
impl WeightInfo for () {
	/// Storage: `SlpV2::NextDelegatorIndexByStakingProtocol` (r:1 w:1)
	/// Proof: `SlpV2::NextDelegatorIndexByStakingProtocol` (`max_values`: None, `max_size`: Some(19), added: 2494, mode: `MaxEncodedLen`)
	/// Storage: `ParachainInfo::ParachainId` (r:1 w:0)
	/// Proof: `ParachainInfo::ParachainId` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `SlpV2::DelegatorByStakingProtocolAndDelegatorIndex` (r:1 w:1)
	/// Proof: `SlpV2::DelegatorByStakingProtocolAndDelegatorIndex` (`max_values`: None, `max_size`: Some(68), added: 2543, mode: `MaxEncodedLen`)
	/// Storage: `SlpV2::DelegatorIndexByStakingProtocolAndDelegator` (r:1 w:1)
	/// Proof: `SlpV2::DelegatorIndexByStakingProtocolAndDelegator` (`max_values`: None, `max_size`: Some(68), added: 2543, mode: `MaxEncodedLen`)
	/// Storage: `System::Number` (r:1 w:0)
	/// Proof: `System::Number` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `System::ExecutionPhase` (r:1 w:0)
	/// Proof: `System::ExecutionPhase` (`max_values`: Some(1), `max_size`: Some(5), added: 500, mode: `MaxEncodedLen`)
	/// Storage: `System::EventCount` (r:1 w:1)
	/// Proof: `System::EventCount` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `System::Events` (r:1 w:1)
	/// Proof: `System::Events` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `SlpV2::LedgerByStakingProtocolAndDelegator` (r:0 w:1)
	/// Proof: `SlpV2::LedgerByStakingProtocolAndDelegator` (`max_values`: None, `max_size`: Some(252), added: 2727, mode: `MaxEncodedLen`)
	fn add_delegator() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `341`
		//  Estimated: `3533`
		// Minimum execution time: 16_757_000 picoseconds.
		Weight::from_parts(17_662_000, 3533)
			.saturating_add(RocksDbWeight::get().reads(8_u64))
			.saturating_add(RocksDbWeight::get().writes(6_u64))
	}
	/// Storage: `SlpV2::DelegatorIndexByStakingProtocolAndDelegator` (r:1 w:1)
	/// Proof: `SlpV2::DelegatorIndexByStakingProtocolAndDelegator` (`max_values`: None, `max_size`: Some(68), added: 2543, mode: `MaxEncodedLen`)
	/// Storage: `System::Number` (r:1 w:0)
	/// Proof: `System::Number` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `System::ExecutionPhase` (r:1 w:0)
	/// Proof: `System::ExecutionPhase` (`max_values`: Some(1), `max_size`: Some(5), added: 500, mode: `MaxEncodedLen`)
	/// Storage: `System::EventCount` (r:1 w:1)
	/// Proof: `System::EventCount` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `System::Events` (r:1 w:1)
	/// Proof: `System::Events` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `SlpV2::LedgerByStakingProtocolAndDelegator` (r:0 w:1)
	/// Proof: `SlpV2::LedgerByStakingProtocolAndDelegator` (`max_values`: None, `max_size`: Some(252), added: 2727, mode: `MaxEncodedLen`)
	/// Storage: `SlpV2::DelegatorByStakingProtocolAndDelegatorIndex` (r:0 w:1)
	/// Proof: `SlpV2::DelegatorByStakingProtocolAndDelegatorIndex` (`max_values`: None, `max_size`: Some(68), added: 2543, mode: `MaxEncodedLen`)
	/// Storage: `SlpV2::ValidatorsByStakingProtocolAndDelegator` (r:0 w:1)
	/// Proof: `SlpV2::ValidatorsByStakingProtocolAndDelegator` (`max_values`: None, `max_size`: Some(8772), added: 11247, mode: `MaxEncodedLen`)
	fn remove_delegator() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `432`
		//  Estimated: `3533`
		// Minimum execution time: 14_029_000 picoseconds.
		Weight::from_parts(14_562_000, 3533)
			.saturating_add(RocksDbWeight::get().reads(5_u64))
			.saturating_add(RocksDbWeight::get().writes(6_u64))
	}
	/// Storage: `SlpV2::DelegatorIndexByStakingProtocolAndDelegator` (r:1 w:0)
	/// Proof: `SlpV2::DelegatorIndexByStakingProtocolAndDelegator` (`max_values`: None, `max_size`: Some(68), added: 2543, mode: `MaxEncodedLen`)
	/// Storage: `SlpV2::DelegatorByStakingProtocolAndDelegatorIndex` (r:1 w:0)
	/// Proof: `SlpV2::DelegatorByStakingProtocolAndDelegatorIndex` (`max_values`: None, `max_size`: Some(68), added: 2543, mode: `MaxEncodedLen`)
	/// Storage: `SlpV2::ValidatorsByStakingProtocolAndDelegator` (r:1 w:1)
	/// Proof: `SlpV2::ValidatorsByStakingProtocolAndDelegator` (`max_values`: None, `max_size`: Some(8772), added: 11247, mode: `MaxEncodedLen`)
	/// Storage: `System::Number` (r:1 w:0)
	/// Proof: `System::Number` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `System::ExecutionPhase` (r:1 w:0)
	/// Proof: `System::ExecutionPhase` (`max_values`: Some(1), `max_size`: Some(5), added: 500, mode: `MaxEncodedLen`)
	/// Storage: `System::EventCount` (r:1 w:1)
	/// Proof: `System::EventCount` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `System::Events` (r:1 w:1)
	/// Proof: `System::Events` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	fn add_validator() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `487`
		//  Estimated: `12237`
		// Minimum execution time: 14_360_000 picoseconds.
		Weight::from_parts(15_623_000, 12237)
			.saturating_add(RocksDbWeight::get().reads(7_u64))
			.saturating_add(RocksDbWeight::get().writes(3_u64))
	}
	/// Storage: `SlpV2::DelegatorIndexByStakingProtocolAndDelegator` (r:1 w:0)
	/// Proof: `SlpV2::DelegatorIndexByStakingProtocolAndDelegator` (`max_values`: None, `max_size`: Some(68), added: 2543, mode: `MaxEncodedLen`)
	/// Storage: `SlpV2::DelegatorByStakingProtocolAndDelegatorIndex` (r:1 w:0)
	/// Proof: `SlpV2::DelegatorByStakingProtocolAndDelegatorIndex` (`max_values`: None, `max_size`: Some(68), added: 2543, mode: `MaxEncodedLen`)
	/// Storage: `SlpV2::ValidatorsByStakingProtocolAndDelegator` (r:1 w:1)
	/// Proof: `SlpV2::ValidatorsByStakingProtocolAndDelegator` (`max_values`: None, `max_size`: Some(8772), added: 11247, mode: `MaxEncodedLen`)
	/// Storage: `System::Number` (r:1 w:0)
	/// Proof: `System::Number` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `System::ExecutionPhase` (r:1 w:0)
	/// Proof: `System::ExecutionPhase` (`max_values`: Some(1), `max_size`: Some(5), added: 500, mode: `MaxEncodedLen`)
	/// Storage: `System::EventCount` (r:1 w:1)
	/// Proof: `System::EventCount` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `System::Events` (r:1 w:1)
	/// Proof: `System::Events` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	fn remove_validator() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `598`
		//  Estimated: `12237`
		// Minimum execution time: 15_360_000 picoseconds.
		Weight::from_parts(15_760_000, 12237)
			.saturating_add(RocksDbWeight::get().reads(7_u64))
			.saturating_add(RocksDbWeight::get().writes(3_u64))
	}
	/// Storage: `SlpV2::ConfigurationByStakingProtocol` (r:1 w:1)
	/// Proof: `SlpV2::ConfigurationByStakingProtocol` (`max_values`: None, `max_size`: Some(102), added: 2577, mode: `MaxEncodedLen`)
	/// Storage: `System::Number` (r:1 w:0)
	/// Proof: `System::Number` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `System::ExecutionPhase` (r:1 w:0)
	/// Proof: `System::ExecutionPhase` (`max_values`: Some(1), `max_size`: Some(5), added: 500, mode: `MaxEncodedLen`)
	/// Storage: `System::EventCount` (r:1 w:1)
	/// Proof: `System::EventCount` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `System::Events` (r:1 w:1)
	/// Proof: `System::Events` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	fn set_protocol_configuration() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `238`
		//  Estimated: `3567`
		// Minimum execution time: 8_696_000 picoseconds.
		Weight::from_parts(9_011_000, 3567)
			.saturating_add(RocksDbWeight::get().reads(5_u64))
			.saturating_add(RocksDbWeight::get().writes(3_u64))
	}
	/// Storage: `SlpV2::DelegatorIndexByStakingProtocolAndDelegator` (r:1 w:0)
	/// Proof: `SlpV2::DelegatorIndexByStakingProtocolAndDelegator` (`max_values`: None, `max_size`: Some(68), added: 2543, mode: `MaxEncodedLen`)
	/// Storage: `SlpV2::DelegatorByStakingProtocolAndDelegatorIndex` (r:1 w:0)
	/// Proof: `SlpV2::DelegatorByStakingProtocolAndDelegatorIndex` (`max_values`: None, `max_size`: Some(68), added: 2543, mode: `MaxEncodedLen`)
	/// Storage: `SlpV2::LedgerByStakingProtocolAndDelegator` (r:1 w:1)
	/// Proof: `SlpV2::LedgerByStakingProtocolAndDelegator` (`max_values`: None, `max_size`: Some(252), added: 2727, mode: `MaxEncodedLen`)
	/// Storage: `System::Number` (r:1 w:0)
	/// Proof: `System::Number` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `System::ExecutionPhase` (r:1 w:0)
	/// Proof: `System::ExecutionPhase` (`max_values`: Some(1), `max_size`: Some(5), added: 500, mode: `MaxEncodedLen`)
	/// Storage: `System::EventCount` (r:1 w:1)
	/// Proof: `System::EventCount` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `System::Events` (r:1 w:1)
	/// Proof: `System::Events` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	fn set_ledger() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `545`
		//  Estimated: `3717`
		// Minimum execution time: 16_147_000 picoseconds.
		Weight::from_parts(16_723_000, 3717)
			.saturating_add(RocksDbWeight::get().reads(7_u64))
			.saturating_add(RocksDbWeight::get().writes(3_u64))
	}
	/// Storage: `SlpV2::DelegatorIndexByStakingProtocolAndDelegator` (r:1 w:0)
	/// Proof: `SlpV2::DelegatorIndexByStakingProtocolAndDelegator` (`max_values`: None, `max_size`: Some(68), added: 2543, mode: `MaxEncodedLen`)
	/// Storage: `SlpV2::DelegatorByStakingProtocolAndDelegatorIndex` (r:1 w:0)
	/// Proof: `SlpV2::DelegatorByStakingProtocolAndDelegatorIndex` (`max_values`: None, `max_size`: Some(68), added: 2543, mode: `MaxEncodedLen`)
	/// Storage: `Tokens::Accounts` (r:1 w:0)
	/// Proof: `Tokens::Accounts` (`max_values`: None, `max_size`: Some(118), added: 2593, mode: `MaxEncodedLen`)
	/// Storage: `System::Number` (r:1 w:0)
	/// Proof: `System::Number` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `System::ExecutionPhase` (r:1 w:0)
	/// Proof: `System::ExecutionPhase` (`max_values`: Some(1), `max_size`: Some(5), added: 500, mode: `MaxEncodedLen`)
	/// Storage: `System::EventCount` (r:1 w:1)
	/// Proof: `System::EventCount` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `System::Events` (r:1 w:1)
	/// Proof: `System::Events` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	fn transfer_to() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `942`
		//  Estimated: `3583`
		// Minimum execution time: 19_048_000 picoseconds.
		Weight::from_parts(19_731_000, 3583)
			.saturating_add(RocksDbWeight::get().reads(7_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
	/// Storage: `SlpV2::DelegatorIndexByStakingProtocolAndDelegator` (r:1 w:0)
	/// Proof: `SlpV2::DelegatorIndexByStakingProtocolAndDelegator` (`max_values`: None, `max_size`: Some(68), added: 2543, mode: `MaxEncodedLen`)
	/// Storage: `SlpV2::DelegatorByStakingProtocolAndDelegatorIndex` (r:1 w:0)
	/// Proof: `SlpV2::DelegatorByStakingProtocolAndDelegatorIndex` (`max_values`: None, `max_size`: Some(68), added: 2543, mode: `MaxEncodedLen`)
	/// Storage: `SlpV2::ConfigurationByStakingProtocol` (r:1 w:0)
	/// Proof: `SlpV2::ConfigurationByStakingProtocol` (`max_values`: None, `max_size`: Some(102), added: 2577, mode: `MaxEncodedLen`)
	/// Storage: `System::Number` (r:1 w:0)
	/// Proof: `System::Number` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `System::ExecutionPhase` (r:1 w:0)
	/// Proof: `System::ExecutionPhase` (`max_values`: Some(1), `max_size`: Some(5), added: 500, mode: `MaxEncodedLen`)
	/// Storage: `System::EventCount` (r:1 w:1)
	/// Proof: `System::EventCount` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `System::Events` (r:1 w:1)
	/// Proof: `System::Events` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	fn transfer_back() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `596`
		//  Estimated: `3567`
		// Minimum execution time: 21_718_000 picoseconds.
		Weight::from_parts(22_486_000, 3567)
			.saturating_add(RocksDbWeight::get().reads(7_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
	/// Storage: `ParachainSystem::ValidationData` (r:1 w:0)
	/// Proof: `ParachainSystem::ValidationData` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `ParachainSystem::LastRelayChainBlockNumber` (r:1 w:0)
	/// Proof: `ParachainSystem::LastRelayChainBlockNumber` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `SlpV2::ConfigurationByStakingProtocol` (r:1 w:0)
	/// Proof: `SlpV2::ConfigurationByStakingProtocol` (`max_values`: None, `max_size`: Some(102), added: 2577, mode: `MaxEncodedLen`)
	/// Storage: `SlpV2::LastUpdateOngoingTimeUnitBlockNumber` (r:1 w:1)
	/// Proof: `SlpV2::LastUpdateOngoingTimeUnitBlockNumber` (`max_values`: None, `max_size`: Some(21), added: 2496, mode: `MaxEncodedLen`)
	/// Storage: `VtokenMinting::OngoingTimeUnit` (r:1 w:1)
	/// Proof: `VtokenMinting::OngoingTimeUnit` (`max_values`: None, `max_size`: Some(27), added: 2502, mode: `MaxEncodedLen`)
	/// Storage: `System::Number` (r:1 w:0)
	/// Proof: `System::Number` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `System::ExecutionPhase` (r:1 w:0)
	/// Proof: `System::ExecutionPhase` (`max_values`: Some(1), `max_size`: Some(5), added: 500, mode: `MaxEncodedLen`)
	/// Storage: `System::EventCount` (r:1 w:1)
	/// Proof: `System::EventCount` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `System::Events` (r:1 w:1)
	/// Proof: `System::Events` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	fn update_ongoing_time_unit() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `560`
		//  Estimated: `3567`
		// Minimum execution time: 16_642_000 picoseconds.
		Weight::from_parts(17_177_000, 3567)
			.saturating_add(RocksDbWeight::get().reads(9_u64))
			.saturating_add(RocksDbWeight::get().writes(4_u64))
	}
	/// Storage: `SlpV2::ConfigurationByStakingProtocol` (r:1 w:0)
	/// Proof: `SlpV2::ConfigurationByStakingProtocol` (`max_values`: None, `max_size`: Some(102), added: 2577, mode: `MaxEncodedLen`)
	/// Storage: `ParachainSystem::ValidationData` (r:1 w:0)
	/// Proof: `ParachainSystem::ValidationData` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `ParachainSystem::LastRelayChainBlockNumber` (r:1 w:0)
	/// Proof: `ParachainSystem::LastRelayChainBlockNumber` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `SlpV2::LastUpdateTokenExchangeRateBlockNumber` (r:1 w:1)
	/// Proof: `SlpV2::LastUpdateTokenExchangeRateBlockNumber` (`max_values`: None, `max_size`: Some(70), added: 2545, mode: `MaxEncodedLen`)
	/// Storage: `VtokenMinting::TokenPool` (r:1 w:1)
	/// Proof: `VtokenMinting::TokenPool` (`max_values`: None, `max_size`: Some(38), added: 2513, mode: `MaxEncodedLen`)
	/// Storage: `SlpV2::LedgerByStakingProtocolAndDelegator` (r:1 w:1)
	/// Proof: `SlpV2::LedgerByStakingProtocolAndDelegator` (`max_values`: None, `max_size`: Some(252), added: 2727, mode: `MaxEncodedLen`)
	/// Storage: `System::Number` (r:1 w:0)
	/// Proof: `System::Number` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `System::ExecutionPhase` (r:1 w:0)
	/// Proof: `System::ExecutionPhase` (`max_values`: Some(1), `max_size`: Some(5), added: 500, mode: `MaxEncodedLen`)
	/// Storage: `System::EventCount` (r:1 w:1)
	/// Proof: `System::EventCount` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `System::Events` (r:1 w:1)
	/// Proof: `System::Events` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	fn update_token_exchange_rate() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `755`
		//  Estimated: `3717`
		// Minimum execution time: 23_611_000 picoseconds.
		Weight::from_parts(24_436_000, 3717)
			.saturating_add(RocksDbWeight::get().reads(10_u64))
			.saturating_add(RocksDbWeight::get().writes(5_u64))
	}
	/// Storage: `SlpV2::DelegatorIndexByStakingProtocolAndDelegator` (r:1 w:0)
	/// Proof: `SlpV2::DelegatorIndexByStakingProtocolAndDelegator` (`max_values`: None, `max_size`: Some(68), added: 2543, mode: `MaxEncodedLen`)
	/// Storage: `SlpV2::DelegatorByStakingProtocolAndDelegatorIndex` (r:1 w:0)
	/// Proof: `SlpV2::DelegatorByStakingProtocolAndDelegatorIndex` (`max_values`: None, `max_size`: Some(68), added: 2543, mode: `MaxEncodedLen`)
	/// Storage: `System::Number` (r:1 w:0)
	/// Proof: `System::Number` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `PolkadotXcm::QueryCounter` (r:1 w:1)
	/// Proof: `PolkadotXcm::QueryCounter` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `SlpV2::ConfigurationByStakingProtocol` (r:1 w:0)
	/// Proof: `SlpV2::ConfigurationByStakingProtocol` (`max_values`: None, `max_size`: Some(102), added: 2577, mode: `MaxEncodedLen`)
	/// Storage: `System::ExecutionPhase` (r:1 w:0)
	/// Proof: `System::ExecutionPhase` (`max_values`: Some(1), `max_size`: Some(5), added: 500, mode: `MaxEncodedLen`)
	/// Storage: `System::EventCount` (r:1 w:1)
	/// Proof: `System::EventCount` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `System::Events` (r:1 w:1)
	/// Proof: `System::Events` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `SlpV2::PendingStatusByQueryId` (r:0 w:1)
	/// Proof: `SlpV2::PendingStatusByQueryId` (`max_values`: None, `max_size`: Some(75), added: 2550, mode: `MaxEncodedLen`)
	/// Storage: `PolkadotXcm::Queries` (r:0 w:1)
	/// Proof: `PolkadotXcm::Queries` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn astar_dapp_staking() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `836`
		//  Estimated: `3567`
		// Minimum execution time: 26_759_000 picoseconds.
		Weight::from_parts(27_950_000, 3567)
			.saturating_add(RocksDbWeight::get().reads(8_u64))
			.saturating_add(RocksDbWeight::get().writes(5_u64))
	}
	/// Storage: `SlpV2::PendingStatusByQueryId` (r:1 w:0)
	/// Proof: `SlpV2::PendingStatusByQueryId` (`max_values`: None, `max_size`: Some(75), added: 2550, mode: `MaxEncodedLen`)
	/// Storage: `SlpV2::LedgerByStakingProtocolAndDelegator` (r:1 w:1)
	/// Proof: `SlpV2::LedgerByStakingProtocolAndDelegator` (`max_values`: None, `max_size`: Some(252), added: 2727, mode: `MaxEncodedLen`)
	/// Storage: `System::Number` (r:1 w:0)
	/// Proof: `System::Number` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `System::ExecutionPhase` (r:1 w:0)
	/// Proof: `System::ExecutionPhase` (`max_values`: Some(1), `max_size`: Some(5), added: 500, mode: `MaxEncodedLen`)
	/// Storage: `System::EventCount` (r:1 w:1)
	/// Proof: `System::EventCount` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `System::Events` (r:1 w:1)
	/// Proof: `System::Events` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	fn notify_astar_dapp_staking() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `567`
		//  Estimated: `3717`
		// Minimum execution time: 12_613_000 picoseconds.
		Weight::from_parts(13_981_000, 3717)
			.saturating_add(RocksDbWeight::get().reads(6_u64))
			.saturating_add(RocksDbWeight::get().writes(3_u64))
	}
}
