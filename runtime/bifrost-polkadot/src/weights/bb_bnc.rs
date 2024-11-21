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

//! Autogenerated weights for bb_bnc
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
// --pallet=bb_bnc
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./runtime/bifrost-polkadot/src/weights/bb_bnc.rs
// --template=./weight-template/runtime-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions for bb_bnc.
pub struct BifrostWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> bb_bnc::WeightInfo for BifrostWeight<T> {
	// Storage: `BbBNC::BbConfigs` (r:1 w:1)
	// Proof: `BbBNC::BbConfigs` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `System::Number` (r:1 w:0)
	// Proof: `System::Number` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	// Storage: `System::ExecutionPhase` (r:1 w:0)
	// Proof: `System::ExecutionPhase` (`max_values`: Some(1), `max_size`: Some(5), added: 500, mode: `MaxEncodedLen`)
	// Storage: `System::EventCount` (r:1 w:1)
	// Proof: `System::EventCount` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	// Storage: `System::Events` (r:1 w:1)
	// Proof: `System::Events` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	fn set_config() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `134`
		//  Estimated: `1619`
		// Minimum execution time: 6_301 nanoseconds.
		Weight::from_parts(6_596_000, 1619)
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	// Storage: `BbBNC::Position` (r:1 w:1)
	// Proof: `BbBNC::Position` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `BbBNC::UserPositions` (r:1 w:1)
	// Proof: `BbBNC::UserPositions` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `BbBNC::BbConfigs` (r:1 w:0)
	// Proof: `BbBNC::BbConfigs` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `System::Number` (r:1 w:0)
	// Proof: `System::Number` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	// Storage: `BbBNC::Locked` (r:1 w:1)
	// Proof: `BbBNC::Locked` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `BbBNC::Supply` (r:1 w:1)
	// Proof: `BbBNC::Supply` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `Tokens::Accounts` (r:1 w:1)
	// Proof: `Tokens::Accounts` (`max_values`: None, `max_size`: Some(118), added: 2593, mode: `MaxEncodedLen`)
	// Storage: `BbBNC::UserLocked` (r:1 w:1)
	// Proof: `BbBNC::UserLocked` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `Tokens::Locks` (r:1 w:1)
	// Proof: `Tokens::Locks` (`max_values`: None, `max_size`: Some(1271), added: 3746, mode: `MaxEncodedLen`)
	// Storage: `AssetRegistry::CurrencyMetadatas` (r:1 w:0)
	// Proof: `AssetRegistry::CurrencyMetadatas` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `System::Account` (r:1 w:1)
	// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	// Storage: `System::ExecutionPhase` (r:1 w:0)
	// Proof: `System::ExecutionPhase` (`max_values`: Some(1), `max_size`: Some(5), added: 500, mode: `MaxEncodedLen`)
	// Storage: `System::EventCount` (r:1 w:1)
	// Proof: `System::EventCount` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	// Storage: `System::Events` (r:1 w:1)
	// Proof: `System::Events` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `BbBNC::UserMarkupInfos` (r:1 w:0)
	// Proof: `BbBNC::UserMarkupInfos` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `BbBNC::UserFarmingPool` (r:1 w:0)
	// Proof: `BbBNC::UserFarmingPool` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `BbBNC::IncentiveConfigs` (r:1 w:1)
	// Proof: `BbBNC::IncentiveConfigs` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `BbBNC::Epoch` (r:1 w:1)
	// Proof: `BbBNC::Epoch` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `BbBNC::PointHistory` (r:1 w:1)
	// Proof: `BbBNC::PointHistory` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `BbBNC::UserPointEpoch` (r:1 w:1)
	// Proof: `BbBNC::UserPointEpoch` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `BbBNC::Rewards` (r:1 w:0)
	// Proof: `BbBNC::Rewards` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `BbBNC::SlopeChanges` (r:2 w:1)
	// Proof: `BbBNC::SlopeChanges` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `BbBNC::UserPointHistory` (r:0 w:1)
	// Proof: `BbBNC::UserPointHistory` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `BbBNC::UserRewardPerTokenPaid` (r:0 w:1)
	// Proof: `BbBNC::UserRewardPerTokenPaid` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn create_lock() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1373`
		//  Estimated: `7313`
		// Minimum execution time: 70_062 nanoseconds.
		Weight::from_parts(72_073_000, 7313)
			.saturating_add(T::DbWeight::get().reads(23))
			.saturating_add(T::DbWeight::get().writes(17))
	}
	// Storage: `BbBNC::UserPositions` (r:1 w:0)
	// Proof: `BbBNC::UserPositions` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `BbBNC::BbConfigs` (r:1 w:0)
	// Proof: `BbBNC::BbConfigs` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `BbBNC::Locked` (r:1 w:1)
	// Proof: `BbBNC::Locked` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `System::Number` (r:1 w:0)
	// Proof: `System::Number` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	// Storage: `BbBNC::Supply` (r:1 w:1)
	// Proof: `BbBNC::Supply` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `Tokens::Accounts` (r:1 w:1)
	// Proof: `Tokens::Accounts` (`max_values`: None, `max_size`: Some(118), added: 2593, mode: `MaxEncodedLen`)
	// Storage: `BbBNC::UserLocked` (r:1 w:1)
	// Proof: `BbBNC::UserLocked` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `Tokens::Locks` (r:1 w:1)
	// Proof: `Tokens::Locks` (`max_values`: None, `max_size`: Some(1271), added: 3746, mode: `MaxEncodedLen`)
	// Storage: `AssetRegistry::CurrencyMetadatas` (r:1 w:0)
	// Proof: `AssetRegistry::CurrencyMetadatas` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `System::ExecutionPhase` (r:1 w:0)
	// Proof: `System::ExecutionPhase` (`max_values`: Some(1), `max_size`: Some(5), added: 500, mode: `MaxEncodedLen`)
	// Storage: `System::EventCount` (r:1 w:1)
	// Proof: `System::EventCount` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	// Storage: `System::Events` (r:1 w:1)
	// Proof: `System::Events` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `BbBNC::UserMarkupInfos` (r:1 w:0)
	// Proof: `BbBNC::UserMarkupInfos` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `BbBNC::UserFarmingPool` (r:1 w:0)
	// Proof: `BbBNC::UserFarmingPool` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `BbBNC::IncentiveConfigs` (r:1 w:1)
	// Proof: `BbBNC::IncentiveConfigs` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `BbBNC::Epoch` (r:1 w:1)
	// Proof: `BbBNC::Epoch` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `BbBNC::PointHistory` (r:1 w:1)
	// Proof: `BbBNC::PointHistory` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `BbBNC::UserPointEpoch` (r:1 w:1)
	// Proof: `BbBNC::UserPointEpoch` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `BbBNC::UserPointHistory` (r:1 w:1)
	// Proof: `BbBNC::UserPointHistory` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `BbBNC::Rewards` (r:1 w:1)
	// Proof: `BbBNC::Rewards` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `BbBNC::UserRewardPerTokenPaid` (r:1 w:1)
	// Proof: `BbBNC::UserRewardPerTokenPaid` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `BbBNC::SlopeChanges` (r:1 w:1)
	// Proof: `BbBNC::SlopeChanges` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn increase_amount() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2232`
		//  Estimated: `5697`
		// Minimum execution time: 89_772 nanoseconds.
		Weight::from_parts(92_789_000, 5697)
			.saturating_add(T::DbWeight::get().reads(22))
			.saturating_add(T::DbWeight::get().writes(15))
	}
	// Storage: `BbBNC::UserPositions` (r:1 w:0)
	// Proof: `BbBNC::UserPositions` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `BbBNC::BbConfigs` (r:1 w:0)
	// Proof: `BbBNC::BbConfigs` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `BbBNC::Locked` (r:1 w:1)
	// Proof: `BbBNC::Locked` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `System::Number` (r:1 w:0)
	// Proof: `System::Number` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	// Storage: `BbBNC::Supply` (r:1 w:1)
	// Proof: `BbBNC::Supply` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `Tokens::Accounts` (r:1 w:0)
	// Proof: `Tokens::Accounts` (`max_values`: None, `max_size`: Some(118), added: 2593, mode: `MaxEncodedLen`)
	// Storage: `BbBNC::UserMarkupInfos` (r:1 w:0)
	// Proof: `BbBNC::UserMarkupInfos` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `BbBNC::UserFarmingPool` (r:1 w:0)
	// Proof: `BbBNC::UserFarmingPool` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `BbBNC::IncentiveConfigs` (r:1 w:1)
	// Proof: `BbBNC::IncentiveConfigs` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `BbBNC::Epoch` (r:1 w:1)
	// Proof: `BbBNC::Epoch` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `BbBNC::PointHistory` (r:1 w:1)
	// Proof: `BbBNC::PointHistory` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `BbBNC::UserPointEpoch` (r:1 w:1)
	// Proof: `BbBNC::UserPointEpoch` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `BbBNC::UserPointHistory` (r:1 w:1)
	// Proof: `BbBNC::UserPointHistory` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `BbBNC::Rewards` (r:1 w:1)
	// Proof: `BbBNC::Rewards` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `BbBNC::UserRewardPerTokenPaid` (r:1 w:1)
	// Proof: `BbBNC::UserRewardPerTokenPaid` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `BbBNC::SlopeChanges` (r:2 w:2)
	// Proof: `BbBNC::SlopeChanges` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `System::ExecutionPhase` (r:1 w:0)
	// Proof: `System::ExecutionPhase` (`max_values`: Some(1), `max_size`: Some(5), added: 500, mode: `MaxEncodedLen`)
	// Storage: `System::EventCount` (r:1 w:1)
	// Proof: `System::EventCount` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	// Storage: `System::Events` (r:1 w:1)
	// Proof: `System::Events` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	fn increase_unlock_time() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1503`
		//  Estimated: `7443`
		// Minimum execution time: 72_025 nanoseconds.
		Weight::from_parts(74_574_000, 7443)
			.saturating_add(T::DbWeight::get().reads(20))
			.saturating_add(T::DbWeight::get().writes(13))
	}
	// Storage: `BbBNC::UserPositions` (r:1 w:1)
	// Proof: `BbBNC::UserPositions` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `BbBNC::Locked` (r:1 w:1)
	// Proof: `BbBNC::Locked` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `System::Number` (r:1 w:0)
	// Proof: `System::Number` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	// Storage: `BbBNC::Supply` (r:1 w:1)
	// Proof: `BbBNC::Supply` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `BbBNC::UserLocked` (r:1 w:1)
	// Proof: `BbBNC::UserLocked` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `Tokens::Locks` (r:1 w:1)
	// Proof: `Tokens::Locks` (`max_values`: None, `max_size`: Some(1271), added: 3746, mode: `MaxEncodedLen`)
	// Storage: `Tokens::Accounts` (r:1 w:1)
	// Proof: `Tokens::Accounts` (`max_values`: None, `max_size`: Some(118), added: 2593, mode: `MaxEncodedLen`)
	// Storage: `AssetRegistry::CurrencyMetadatas` (r:1 w:0)
	// Proof: `AssetRegistry::CurrencyMetadatas` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `System::Account` (r:1 w:1)
	// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	// Storage: `System::ExecutionPhase` (r:1 w:0)
	// Proof: `System::ExecutionPhase` (`max_values`: Some(1), `max_size`: Some(5), added: 500, mode: `MaxEncodedLen`)
	// Storage: `System::EventCount` (r:1 w:1)
	// Proof: `System::EventCount` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	// Storage: `System::Events` (r:1 w:1)
	// Proof: `System::Events` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `BbBNC::UserFarmingPool` (r:1 w:0)
	// Proof: `BbBNC::UserFarmingPool` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `BbBNC::IncentiveConfigs` (r:1 w:1)
	// Proof: `BbBNC::IncentiveConfigs` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `BbBNC::Epoch` (r:1 w:1)
	// Proof: `BbBNC::Epoch` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `BbBNC::PointHistory` (r:1 w:105)
	// Proof: `BbBNC::PointHistory` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `BbBNC::SlopeChanges` (r:104 w:0)
	// Proof: `BbBNC::SlopeChanges` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `BbBNC::Rewards` (r:1 w:0)
	// Proof: `BbBNC::Rewards` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `BbBNC::UserPointHistory` (r:0 w:1)
	// Proof: `BbBNC::UserPointHistory` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `BbBNC::UserRewardPerTokenPaid` (r:0 w:1)
	// Proof: `BbBNC::UserRewardPerTokenPaid` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `BbBNC::UserPointEpoch` (r:0 w:1)
	// Proof: `BbBNC::UserPointEpoch` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn withdraw() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2079`
		//  Estimated: `260469`
		// Minimum execution time: 423_406 nanoseconds.
		Weight::from_parts(430_674_000, 260469)
			.saturating_add(T::DbWeight::get().reads(121))
			.saturating_add(T::DbWeight::get().writes(119))
	}
	// Storage: `BbBNC::IncentiveConfigs` (r:1 w:1)
	// Proof: `BbBNC::IncentiveConfigs` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `System::Number` (r:1 w:0)
	// Proof: `System::Number` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	// Storage: `BbBNC::Epoch` (r:1 w:0)
	// Proof: `BbBNC::Epoch` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `BbBNC::PointHistory` (r:1 w:0)
	// Proof: `BbBNC::PointHistory` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `BbBNC::SlopeChanges` (r:104 w:0)
	// Proof: `BbBNC::SlopeChanges` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `BbBNC::UserPositions` (r:1 w:0)
	// Proof: `BbBNC::UserPositions` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `BbBNC::UserPointEpoch` (r:1 w:0)
	// Proof: `BbBNC::UserPointEpoch` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `BbBNC::UserPointHistory` (r:1 w:0)
	// Proof: `BbBNC::UserPointHistory` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `BbBNC::Rewards` (r:1 w:0)
	// Proof: `BbBNC::Rewards` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `BbBNC::UserRewardPerTokenPaid` (r:0 w:1)
	// Proof: `BbBNC::UserRewardPerTokenPaid` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn get_rewards() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `890`
		//  Estimated: `259280`
		// Minimum execution time: 196_816 nanoseconds.
		Weight::from_parts(198_309_000, 259280)
			.saturating_add(T::DbWeight::get().reads(112))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	// Storage: `BbBNC::IncentiveConfigs` (r:1 w:1)
	// Proof: `BbBNC::IncentiveConfigs` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `System::Number` (r:1 w:0)
	// Proof: `System::Number` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	// Storage: `System::ExecutionPhase` (r:1 w:0)
	// Proof: `System::ExecutionPhase` (`max_values`: Some(1), `max_size`: Some(5), added: 500, mode: `MaxEncodedLen`)
	// Storage: `System::EventCount` (r:1 w:1)
	// Proof: `System::EventCount` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	// Storage: `System::Events` (r:1 w:1)
	// Proof: `System::Events` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `BbBNC::Epoch` (r:1 w:0)
	// Proof: `BbBNC::Epoch` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `BbBNC::PointHistory` (r:1 w:0)
	// Proof: `BbBNC::PointHistory` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `System::Account` (r:2 w:2)
	// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	// Storage: `Balances::TotalIssuance` (r:1 w:0)
	// Proof: `Balances::TotalIssuance` (`max_values`: Some(1), `max_size`: Some(16), added: 511, mode: `MaxEncodedLen`)
	fn notify_rewards() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `598`
		//  Estimated: `6196`
		// Minimum execution time: 52_110 nanoseconds.
		Weight::from_parts(53_111_000, 6196)
			.saturating_add(T::DbWeight::get().reads(10))
			.saturating_add(T::DbWeight::get().writes(5))
	}
	// Storage: `BbBNC::TotalLock` (r:1 w:1)
	// Proof: `BbBNC::TotalLock` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `System::Number` (r:1 w:0)
	// Proof: `System::Number` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	// Storage: `BbBNC::MarkupCoefficient` (r:0 w:1)
	// Proof: `BbBNC::MarkupCoefficient` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn set_markup_coefficient() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `147`
		//  Estimated: `3612`
		// Minimum execution time: 5_494 nanoseconds.
		Weight::from_parts(5_813_000, 3612)
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	// Storage: `BbBNC::MarkupCoefficient` (r:1 w:0)
	// Proof: `BbBNC::MarkupCoefficient` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `BbBNC::TotalLock` (r:1 w:1)
	// Proof: `BbBNC::TotalLock` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `System::Number` (r:1 w:0)
	// Proof: `System::Number` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	// Storage: `BbBNC::UserMarkupInfos` (r:1 w:1)
	// Proof: `BbBNC::UserMarkupInfos` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `BbBNC::LockedTokens` (r:1 w:1)
	// Proof: `BbBNC::LockedTokens` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `Tokens::TotalIssuance` (r:1 w:0)
	// Proof: `Tokens::TotalIssuance` (`max_values`: None, `max_size`: Some(38), added: 2513, mode: `MaxEncodedLen`)
	// Storage: `Tokens::Locks` (r:1 w:1)
	// Proof: `Tokens::Locks` (`max_values`: None, `max_size`: Some(1271), added: 3746, mode: `MaxEncodedLen`)
	// Storage: `Tokens::Accounts` (r:1 w:1)
	// Proof: `Tokens::Accounts` (`max_values`: None, `max_size`: Some(118), added: 2593, mode: `MaxEncodedLen`)
	// Storage: `AssetRegistry::CurrencyMetadatas` (r:1 w:0)
	// Proof: `AssetRegistry::CurrencyMetadatas` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `System::ExecutionPhase` (r:1 w:0)
	// Proof: `System::ExecutionPhase` (`max_values`: Some(1), `max_size`: Some(5), added: 500, mode: `MaxEncodedLen`)
	// Storage: `System::EventCount` (r:1 w:1)
	// Proof: `System::EventCount` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	// Storage: `System::Events` (r:1 w:1)
	// Proof: `System::Events` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `BbBNC::UserPositions` (r:1 w:0)
	// Proof: `BbBNC::UserPositions` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `BbBNC::Locked` (r:1 w:0)
	// Proof: `BbBNC::Locked` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `BbBNC::UserFarmingPool` (r:1 w:0)
	// Proof: `BbBNC::UserFarmingPool` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `BbBNC::IncentiveConfigs` (r:1 w:1)
	// Proof: `BbBNC::IncentiveConfigs` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `BbBNC::Epoch` (r:1 w:1)
	// Proof: `BbBNC::Epoch` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `BbBNC::PointHistory` (r:1 w:105)
	// Proof: `BbBNC::PointHistory` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `BbBNC::SlopeChanges` (r:104 w:0)
	// Proof: `BbBNC::SlopeChanges` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `BbBNC::UserPointEpoch` (r:1 w:1)
	// Proof: `BbBNC::UserPointEpoch` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `BbBNC::UserPointHistory` (r:1 w:1)
	// Proof: `BbBNC::UserPointHistory` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `BbBNC::Rewards` (r:1 w:0)
	// Proof: `BbBNC::Rewards` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `BbBNC::Supply` (r:1 w:0)
	// Proof: `BbBNC::Supply` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `BbBNC::UserRewardPerTokenPaid` (r:0 w:1)
	// Proof: `BbBNC::UserRewardPerTokenPaid` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn deposit_markup() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2288`
		//  Estimated: `260678`
		// Minimum execution time: 431_675 nanoseconds.
		Weight::from_parts(438_644_000, 260678)
			.saturating_add(T::DbWeight::get().reads(126))
			.saturating_add(T::DbWeight::get().writes(117))
	}
	// Storage: `BbBNC::MarkupCoefficient` (r:1 w:0)
	// Proof: `BbBNC::MarkupCoefficient` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `BbBNC::UserMarkupInfos` (r:1 w:1)
	// Proof: `BbBNC::UserMarkupInfos` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `BbBNC::LockedTokens` (r:1 w:1)
	// Proof: `BbBNC::LockedTokens` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `BbBNC::TotalLock` (r:1 w:1)
	// Proof: `BbBNC::TotalLock` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `Tokens::Locks` (r:1 w:1)
	// Proof: `Tokens::Locks` (`max_values`: None, `max_size`: Some(1271), added: 3746, mode: `MaxEncodedLen`)
	// Storage: `Tokens::Accounts` (r:1 w:1)
	// Proof: `Tokens::Accounts` (`max_values`: None, `max_size`: Some(118), added: 2593, mode: `MaxEncodedLen`)
	// Storage: `AssetRegistry::CurrencyMetadatas` (r:1 w:0)
	// Proof: `AssetRegistry::CurrencyMetadatas` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `System::Number` (r:1 w:0)
	// Proof: `System::Number` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	// Storage: `System::ExecutionPhase` (r:1 w:0)
	// Proof: `System::ExecutionPhase` (`max_values`: Some(1), `max_size`: Some(5), added: 500, mode: `MaxEncodedLen`)
	// Storage: `System::EventCount` (r:1 w:1)
	// Proof: `System::EventCount` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	// Storage: `System::Events` (r:1 w:1)
	// Proof: `System::Events` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `BbBNC::UserPositions` (r:1 w:0)
	// Proof: `BbBNC::UserPositions` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `BbBNC::Locked` (r:1 w:0)
	// Proof: `BbBNC::Locked` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `BbBNC::UserFarmingPool` (r:1 w:0)
	// Proof: `BbBNC::UserFarmingPool` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `BbBNC::IncentiveConfigs` (r:1 w:1)
	// Proof: `BbBNC::IncentiveConfigs` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `BbBNC::Epoch` (r:1 w:1)
	// Proof: `BbBNC::Epoch` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `BbBNC::PointHistory` (r:1 w:105)
	// Proof: `BbBNC::PointHistory` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `BbBNC::SlopeChanges` (r:104 w:0)
	// Proof: `BbBNC::SlopeChanges` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `BbBNC::UserPointEpoch` (r:1 w:1)
	// Proof: `BbBNC::UserPointEpoch` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `BbBNC::UserPointHistory` (r:1 w:1)
	// Proof: `BbBNC::UserPointHistory` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `BbBNC::Rewards` (r:1 w:1)
	// Proof: `BbBNC::Rewards` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `BbBNC::UserRewardPerTokenPaid` (r:1 w:1)
	// Proof: `BbBNC::UserRewardPerTokenPaid` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `BbBNC::Supply` (r:1 w:0)
	// Proof: `BbBNC::Supply` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	fn withdraw_markup() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2715`
		//  Estimated: `261105`
		// Minimum execution time: 437_499 nanoseconds.
		Weight::from_parts(442_211_000, 261105)
			.saturating_add(T::DbWeight::get().reads(126))
			.saturating_add(T::DbWeight::get().writes(118))
	}
	// Storage: `BbBNC::Locked` (r:1 w:1)
	// Proof: `BbBNC::Locked` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `System::Number` (r:1 w:0)
	// Proof: `System::Number` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	// Storage: `BbBNC::Supply` (r:1 w:1)
	// Proof: `BbBNC::Supply` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `BbBNC::UserPositions` (r:1 w:1)
	// Proof: `BbBNC::UserPositions` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `BbBNC::UserLocked` (r:1 w:1)
	// Proof: `BbBNC::UserLocked` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `Tokens::Locks` (r:1 w:1)
	// Proof: `Tokens::Locks` (`max_values`: None, `max_size`: Some(1271), added: 3746, mode: `MaxEncodedLen`)
	// Storage: `Tokens::Accounts` (r:2 w:2)
	// Proof: `Tokens::Accounts` (`max_values`: None, `max_size`: Some(118), added: 2593, mode: `MaxEncodedLen`)
	// Storage: `AssetRegistry::CurrencyMetadatas` (r:1 w:0)
	// Proof: `AssetRegistry::CurrencyMetadatas` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `System::ExecutionPhase` (r:1 w:0)
	// Proof: `System::ExecutionPhase` (`max_values`: Some(1), `max_size`: Some(5), added: 500, mode: `MaxEncodedLen`)
	// Storage: `System::EventCount` (r:1 w:1)
	// Proof: `System::EventCount` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	// Storage: `System::Events` (r:1 w:1)
	// Proof: `System::Events` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `System::Account` (r:2 w:1)
	// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	// Storage: `BbBNC::UserFarmingPool` (r:1 w:0)
	// Proof: `BbBNC::UserFarmingPool` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `BbBNC::IncentiveConfigs` (r:1 w:1)
	// Proof: `BbBNC::IncentiveConfigs` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `BbBNC::Epoch` (r:1 w:1)
	// Proof: `BbBNC::Epoch` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `BbBNC::PointHistory` (r:1 w:1)
	// Proof: `BbBNC::PointHistory` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `BbBNC::Rewards` (r:1 w:1)
	// Proof: `BbBNC::Rewards` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `BbBNC::UserRewardPerTokenPaid` (r:1 w:1)
	// Proof: `BbBNC::UserRewardPerTokenPaid` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `BbBNC::SlopeChanges` (r:1 w:1)
	// Proof: `BbBNC::SlopeChanges` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `BbBNC::UserPointHistory` (r:0 w:1)
	// Proof: `BbBNC::UserPointHistory` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `BbBNC::UserPointEpoch` (r:0 w:1)
	// Proof: `BbBNC::UserPointEpoch` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn redeem_unlock() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2640`
		//  Estimated: `6196`
		// Minimum execution time: 104_953 nanoseconds.
		Weight::from_parts(109_096_000, 6196)
			.saturating_add(T::DbWeight::get().reads(21))
			.saturating_add(T::DbWeight::get().writes(18))
	}
	// Storage: `BbBNC::MarkupCoefficient` (r:1 w:0)
	// Proof: `BbBNC::MarkupCoefficient` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `System::Number` (r:1 w:0)
	// Proof: `System::Number` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	// Storage: `BbBNC::LockedTokens` (r:2 w:1)
	// Proof: `BbBNC::LockedTokens` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `BbBNC::TotalLock` (r:1 w:0)
	// Proof: `BbBNC::TotalLock` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `Tokens::TotalIssuance` (r:1 w:0)
	// Proof: `Tokens::TotalIssuance` (`max_values`: None, `max_size`: Some(38), added: 2513, mode: `MaxEncodedLen`)
	// Storage: `BbBNC::UserMarkupInfos` (r:1 w:1)
	// Proof: `BbBNC::UserMarkupInfos` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `BbBNC::UserPositions` (r:1 w:0)
	// Proof: `BbBNC::UserPositions` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `BbBNC::Locked` (r:1 w:0)
	// Proof: `BbBNC::Locked` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `BbBNC::UserFarmingPool` (r:1 w:0)
	// Proof: `BbBNC::UserFarmingPool` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `BbBNC::IncentiveConfigs` (r:1 w:1)
	// Proof: `BbBNC::IncentiveConfigs` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `BbBNC::Epoch` (r:1 w:1)
	// Proof: `BbBNC::Epoch` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `BbBNC::PointHistory` (r:1 w:1)
	// Proof: `BbBNC::PointHistory` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `BbBNC::UserPointEpoch` (r:1 w:1)
	// Proof: `BbBNC::UserPointEpoch` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `BbBNC::UserPointHistory` (r:1 w:1)
	// Proof: `BbBNC::UserPointHistory` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `BbBNC::Rewards` (r:1 w:1)
	// Proof: `BbBNC::Rewards` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `BbBNC::UserRewardPerTokenPaid` (r:1 w:1)
	// Proof: `BbBNC::UserRewardPerTokenPaid` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `BbBNC::SlopeChanges` (r:1 w:1)
	// Proof: `BbBNC::SlopeChanges` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `BbBNC::Supply` (r:1 w:0)
	// Proof: `BbBNC::Supply` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `System::ExecutionPhase` (r:1 w:0)
	// Proof: `System::ExecutionPhase` (`max_values`: Some(1), `max_size`: Some(5), added: 500, mode: `MaxEncodedLen`)
	// Storage: `System::EventCount` (r:1 w:1)
	// Proof: `System::EventCount` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	// Storage: `System::Events` (r:1 w:1)
	// Proof: `System::Events` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	fn refresh() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1788`
		//  Estimated: `7728`
		// Minimum execution time: 85_342 nanoseconds.
		Weight::from_parts(88_354_000, 7728)
			.saturating_add(T::DbWeight::get().reads(22))
			.saturating_add(T::DbWeight::get().writes(12))
	}
}
