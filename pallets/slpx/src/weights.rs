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

//! Autogenerated weights for bifrost_slpx
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 32.0.0
//! DATE: 2024-11-20, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `bifrost-jenkins`, CPU: `Intel(R) Xeon(R) CPU E5-26xx v4`
//! WASM-EXECUTION: Compiled, CHAIN: Some("bifrost-kusama-local"), DB CACHE: 1024

// Executed Command:
// ./target/release/bifrost
// benchmark
// pallet
// --chain=bifrost-kusama-local
// --steps=50
// --repeat=20
// --pallet=bifrost-slpx
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --header=./HEADER-GPL3
// --output=./weight.rs
// --template
// ./weight-template/pallet-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for bifrost_slpx.
pub trait WeightInfo {
	fn add_whitelist() -> Weight;
	fn remove_whitelist() -> Weight;
	fn set_execution_fee() -> Weight;
	fn set_transfer_to_fee() -> Weight;
	fn mint() -> Weight;
	fn mint_with_channel_id() -> Weight;
	fn redeem() -> Weight;
	fn evm_create_order() -> Weight;
}

// For backwards compatibility and tests
impl WeightInfo for () {
	/// Storage: `Slpx::WhitelistAccountId` (r:1 w:1)
	/// Proof: `Slpx::WhitelistAccountId` (`max_values`: None, `max_size`: Some(338), added: 2813, mode: `MaxEncodedLen`)
	fn add_whitelist() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `4`
		//  Estimated: `3803`
		// Minimum execution time: 25_376_000 picoseconds.
		Weight::from_parts(26_123_000, 3803)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `Slpx::WhitelistAccountId` (r:1 w:1)
	/// Proof: `Slpx::WhitelistAccountId` (`max_values`: None, `max_size`: Some(338), added: 2813, mode: `MaxEncodedLen`)
	fn remove_whitelist() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `81`
		//  Estimated: `3803`
		// Minimum execution time: 27_460_000 picoseconds.
		Weight::from_parts(28_054_000, 3803)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `Slpx::ExecutionFee` (r:0 w:1)
	/// Proof: `Slpx::ExecutionFee` (`max_values`: None, `max_size`: Some(46), added: 2521, mode: `MaxEncodedLen`)
	fn set_execution_fee() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 15_854_000 picoseconds.
		Weight::from_parts(16_280_000, 0)
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `Slpx::TransferToFee` (r:0 w:1)
	/// Proof: `Slpx::TransferToFee` (`max_values`: None, `max_size`: Some(33), added: 2508, mode: `MaxEncodedLen`)
	fn set_transfer_to_fee() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 15_482_000 picoseconds.
		Weight::from_parts(15_979_000, 0)
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `Slpx::WhitelistAccountId` (r:1 w:0)
	/// Proof: `Slpx::WhitelistAccountId` (`max_values`: None, `max_size`: Some(338), added: 2813, mode: `MaxEncodedLen`)
	/// Storage: `Slpx::OrderQueue` (r:1 w:1)
	/// Proof: `Slpx::OrderQueue` (`max_values`: Some(1), `max_size`: Some(113502), added: 113997, mode: `MaxEncodedLen`)
	fn mint() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `81`
		//  Estimated: `114987`
		// Minimum execution time: 33_529_000 picoseconds.
		Weight::from_parts(34_511_000, 114987)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `Slpx::WhitelistAccountId` (r:1 w:0)
	/// Proof: `Slpx::WhitelistAccountId` (`max_values`: None, `max_size`: Some(338), added: 2813, mode: `MaxEncodedLen`)
	/// Storage: `Slpx::OrderQueue` (r:1 w:1)
	/// Proof: `Slpx::OrderQueue` (`max_values`: Some(1), `max_size`: Some(113502), added: 113997, mode: `MaxEncodedLen`)
	fn mint_with_channel_id() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `81`
		//  Estimated: `114987`
		// Minimum execution time: 32_901_000 picoseconds.
		Weight::from_parts(33_851_000, 114987)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `Slpx::WhitelistAccountId` (r:1 w:0)
	/// Proof: `Slpx::WhitelistAccountId` (`max_values`: None, `max_size`: Some(338), added: 2813, mode: `MaxEncodedLen`)
	/// Storage: `Slpx::OrderQueue` (r:1 w:1)
	/// Proof: `Slpx::OrderQueue` (`max_values`: Some(1), `max_size`: Some(113502), added: 113997, mode: `MaxEncodedLen`)
	fn redeem() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `81`
		//  Estimated: `114987`
		// Minimum execution time: 32_912_000 picoseconds.
		Weight::from_parts(33_600_000, 114987)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `Slpx::WhitelistAccountId` (r:1 w:0)
	/// Proof: `Slpx::WhitelistAccountId` (`max_values`: None, `max_size`: Some(338), added: 2813, mode: `MaxEncodedLen`)
	/// Storage: `Slpx::OrderQueue` (r:1 w:1)
	/// Proof: `Slpx::OrderQueue` (`max_values`: Some(1), `max_size`: Some(113502), added: 113997, mode: `MaxEncodedLen`)
	fn evm_create_order() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `81`
		//  Estimated: `114987`
		// Minimum execution time: 32_556_000 picoseconds.
		Weight::from_parts(33_884_000, 114987)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
}
