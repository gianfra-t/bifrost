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

//! Autogenerated weights for bifrost_stable_pool
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 32.0.0
//! DATE: 2024-11-21, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `mjl-legion`, CPU: `12th Gen Intel(R) Core(TM) i9-12900H`
//! WASM-EXECUTION: Compiled, CHAIN: Some("bifrost-kusama-local"), DB CACHE: 1024

// Executed Command:
// target/release/bifrost
// benchmark
// pallet
// --chain=bifrost-kusama-local
// --steps=50
// --repeat=20
// --pallet=bifrost_stable_pool
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./pallets/stable-pool/src/weights.rs
// --template=./weight-template/pallet-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for bifrost_stable_pool.
pub trait WeightInfo {
	fn create_pool() -> Weight;
	fn edit_token_rate() -> Weight;
	fn config_vtoken_auto_refresh() -> Weight;
	fn remove_vtoken_auto_refresh() -> Weight;
	fn add_liquidity() -> Weight;
	fn swap() -> Weight;
	fn redeem_proportion() -> Weight;
	fn redeem_single() -> Weight;
	fn redeem_multi() -> Weight;
	fn modify_a() -> Weight;
	fn modify_fees() -> Weight;
	fn modify_recipients() -> Weight;
}

// For backwards compatibility and tests
impl WeightInfo for () {
	/// Storage: `StableAsset::PoolCount` (r:1 w:1)
	/// Proof: `StableAsset::PoolCount` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `AssetRegistry::CurrencyMetadatas` (r:1 w:1)
	/// Proof: `AssetRegistry::CurrencyMetadatas` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `StableAsset::Pools` (r:1 w:1)
	/// Proof: `StableAsset::Pools` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	fn create_pool() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `413`
		//  Estimated: `3878`
		// Minimum execution time: 17_561_000 picoseconds.
		Weight::from_parts(18_680_000, 3878)
			.saturating_add(RocksDbWeight::get().reads(4_u64))
			.saturating_add(RocksDbWeight::get().writes(4_u64))
	}
	/// Storage: `StableAsset::PoolCount` (r:1 w:0)
	/// Proof: `StableAsset::PoolCount` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `StableAsset::TokenRateCaches` (r:3 w:2)
	/// Proof: `StableAsset::TokenRateCaches` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn edit_token_rate() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `258`
		//  Estimated: `8673`
		// Minimum execution time: 16_227_000 picoseconds.
		Weight::from_parts(16_831_000, 8673)
			.saturating_add(RocksDbWeight::get().reads(4_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
	/// Storage: `StableAsset::TokenRateHardcap` (r:0 w:1)
	/// Proof: `StableAsset::TokenRateHardcap` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn config_vtoken_auto_refresh() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 4_188_000 picoseconds.
		Weight::from_parts(4_410_000, 0)
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `StableAsset::TokenRateHardcap` (r:0 w:1)
	/// Proof: `StableAsset::TokenRateHardcap` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn remove_vtoken_auto_refresh() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 4_082_000 picoseconds.
		Weight::from_parts(4_439_000, 0)
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `StableAsset::Pools` (r:1 w:1)
	/// Proof: `StableAsset::Pools` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `StableAsset::TokenRateCaches` (r:2 w:0)
	/// Proof: `StableAsset::TokenRateCaches` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `System::Account` (r:2 w:2)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `Tokens::Accounts` (r:3 w:3)
	/// Proof: `Tokens::Accounts` (`max_values`: None, `max_size`: Some(118), added: 2593, mode: `MaxEncodedLen`)
	/// Storage: `AssetRegistry::CurrencyMetadatas` (r:2 w:0)
	/// Proof: `AssetRegistry::CurrencyMetadatas` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Tokens::TotalIssuance` (r:1 w:1)
	/// Proof: `Tokens::TotalIssuance` (`max_values`: None, `max_size`: Some(38), added: 2513, mode: `MaxEncodedLen`)
	fn add_liquidity() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1488`
		//  Estimated: `8769`
		// Minimum execution time: 124_856_000 picoseconds.
		Weight::from_parts(128_239_000, 8769)
			.saturating_add(RocksDbWeight::get().reads(11_u64))
			.saturating_add(RocksDbWeight::get().writes(7_u64))
	}
	/// Storage: `StableAsset::Pools` (r:1 w:1)
	/// Proof: `StableAsset::Pools` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `System::Account` (r:2 w:2)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `StableAsset::TokenRateCaches` (r:1 w:0)
	/// Proof: `StableAsset::TokenRateCaches` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Tokens::Accounts` (r:1 w:1)
	/// Proof: `Tokens::Accounts` (`max_values`: None, `max_size`: Some(118), added: 2593, mode: `MaxEncodedLen`)
	/// Storage: `AssetRegistry::CurrencyMetadatas` (r:1 w:0)
	/// Proof: `AssetRegistry::CurrencyMetadatas` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Tokens::TotalIssuance` (r:1 w:1)
	/// Proof: `Tokens::TotalIssuance` (`max_values`: None, `max_size`: Some(38), added: 2513, mode: `MaxEncodedLen`)
	/// Storage: `StableAsset::TokenRateHardcap` (r:1 w:0)
	/// Proof: `StableAsset::TokenRateHardcap` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn swap() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1399`
		//  Estimated: `6196`
		// Minimum execution time: 108_159_000 picoseconds.
		Weight::from_parts(117_868_000, 6196)
			.saturating_add(RocksDbWeight::get().reads(8_u64))
			.saturating_add(RocksDbWeight::get().writes(5_u64))
	}
	/// Storage: `StableAsset::Pools` (r:1 w:1)
	/// Proof: `StableAsset::Pools` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `System::Account` (r:2 w:2)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `StableAsset::TokenRateCaches` (r:1 w:0)
	/// Proof: `StableAsset::TokenRateCaches` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Tokens::Accounts` (r:1 w:1)
	/// Proof: `Tokens::Accounts` (`max_values`: None, `max_size`: Some(118), added: 2593, mode: `MaxEncodedLen`)
	/// Storage: `AssetRegistry::CurrencyMetadatas` (r:1 w:0)
	/// Proof: `AssetRegistry::CurrencyMetadatas` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Tokens::TotalIssuance` (r:1 w:1)
	/// Proof: `Tokens::TotalIssuance` (`max_values`: None, `max_size`: Some(38), added: 2513, mode: `MaxEncodedLen`)
	fn redeem_proportion() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1399`
		//  Estimated: `6196`
		// Minimum execution time: 100_787_000 picoseconds.
		Weight::from_parts(102_812_000, 6196)
			.saturating_add(RocksDbWeight::get().reads(7_u64))
			.saturating_add(RocksDbWeight::get().writes(5_u64))
	}
	/// Storage: `StableAsset::Pools` (r:1 w:1)
	/// Proof: `StableAsset::Pools` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `System::Account` (r:2 w:2)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `StableAsset::TokenRateCaches` (r:1 w:0)
	/// Proof: `StableAsset::TokenRateCaches` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Tokens::Accounts` (r:1 w:1)
	/// Proof: `Tokens::Accounts` (`max_values`: None, `max_size`: Some(118), added: 2593, mode: `MaxEncodedLen`)
	/// Storage: `AssetRegistry::CurrencyMetadatas` (r:1 w:0)
	/// Proof: `AssetRegistry::CurrencyMetadatas` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Tokens::TotalIssuance` (r:1 w:1)
	/// Proof: `Tokens::TotalIssuance` (`max_values`: None, `max_size`: Some(38), added: 2513, mode: `MaxEncodedLen`)
	fn redeem_single() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1399`
		//  Estimated: `6196`
		// Minimum execution time: 84_565_000 picoseconds.
		Weight::from_parts(91_907_000, 6196)
			.saturating_add(RocksDbWeight::get().reads(7_u64))
			.saturating_add(RocksDbWeight::get().writes(5_u64))
	}
	/// Storage: `StableAsset::Pools` (r:1 w:1)
	/// Proof: `StableAsset::Pools` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `System::Account` (r:2 w:2)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `StableAsset::TokenRateCaches` (r:1 w:0)
	/// Proof: `StableAsset::TokenRateCaches` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Tokens::Accounts` (r:1 w:1)
	/// Proof: `Tokens::Accounts` (`max_values`: None, `max_size`: Some(118), added: 2593, mode: `MaxEncodedLen`)
	/// Storage: `AssetRegistry::CurrencyMetadatas` (r:1 w:0)
	/// Proof: `AssetRegistry::CurrencyMetadatas` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Tokens::TotalIssuance` (r:1 w:1)
	/// Proof: `Tokens::TotalIssuance` (`max_values`: None, `max_size`: Some(38), added: 2513, mode: `MaxEncodedLen`)
	fn redeem_multi() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1399`
		//  Estimated: `6196`
		// Minimum execution time: 102_556_000 picoseconds.
		Weight::from_parts(104_555_000, 6196)
			.saturating_add(RocksDbWeight::get().reads(7_u64))
			.saturating_add(RocksDbWeight::get().writes(5_u64))
	}
	/// Storage: `StableAsset::Pools` (r:1 w:1)
	/// Proof: `StableAsset::Pools` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn modify_a() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `446`
		//  Estimated: `3911`
		// Minimum execution time: 8_645_000 picoseconds.
		Weight::from_parts(9_063_000, 3911)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `StableAsset::Pools` (r:1 w:1)
	/// Proof: `StableAsset::Pools` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn modify_fees() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `446`
		//  Estimated: `3911`
		// Minimum execution time: 8_319_000 picoseconds.
		Weight::from_parts(8_858_000, 3911)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `StableAsset::Pools` (r:1 w:1)
	/// Proof: `StableAsset::Pools` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn modify_recipients() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `446`
		//  Estimated: `3911`
		// Minimum execution time: 8_359_000 picoseconds.
		Weight::from_parts(8_570_000, 3911)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
}
