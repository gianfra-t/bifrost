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

//! Autogenerated weights for bifrost_asset_registry
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
// --pallet=bifrost_asset_registry
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./pallets/asset-registry/src/weights.rs
// --template=./weight-template/pallet-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for bifrost_asset_registry.
pub trait WeightInfo {
	fn register_token_metadata() -> Weight;
	fn register_vtoken_metadata() -> Weight;
	fn register_location() -> Weight;
	fn force_set_location() -> Weight;
	fn update_currency_metadata() -> Weight;
}

// For backwards compatibility and tests
impl WeightInfo for () {
	/// Storage: `AssetRegistry::NextTokenId` (r:1 w:1)
	/// Proof: `AssetRegistry::NextTokenId` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `AssetRegistry::CurrencyMetadatas` (r:1 w:1)
	/// Proof: `AssetRegistry::CurrencyMetadatas` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn register_token_metadata() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `337`
		//  Estimated: `3802`
		// Minimum execution time: 8_739_000 picoseconds.
		Weight::from_parts(9_337_000, 3802)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
	/// Storage: `AssetRegistry::CurrencyMetadatas` (r:2 w:1)
	/// Proof: `AssetRegistry::CurrencyMetadatas` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn register_vtoken_metadata() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `424`
		//  Estimated: `6364`
		// Minimum execution time: 10_913_000 picoseconds.
		Weight::from_parts(11_718_000, 6364)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `AssetRegistry::CurrencyMetadatas` (r:1 w:0)
	/// Proof: `AssetRegistry::CurrencyMetadatas` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `AssetRegistry::LocationToCurrencyIds` (r:1 w:1)
	/// Proof: `AssetRegistry::LocationToCurrencyIds` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `AssetRegistry::CurrencyIdToLocations` (r:1 w:1)
	/// Proof: `AssetRegistry::CurrencyIdToLocations` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `AssetRegistry::CurrencyIdToWeights` (r:0 w:1)
	/// Proof: `AssetRegistry::CurrencyIdToWeights` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn register_location() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `414`
		//  Estimated: `3879`
		// Minimum execution time: 12_050_000 picoseconds.
		Weight::from_parts(12_529_000, 3879)
			.saturating_add(RocksDbWeight::get().reads(3_u64))
			.saturating_add(RocksDbWeight::get().writes(3_u64))
	}
	/// Storage: `AssetRegistry::CurrencyMetadatas` (r:1 w:0)
	/// Proof: `AssetRegistry::CurrencyMetadatas` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `AssetRegistry::LocationToCurrencyIds` (r:0 w:1)
	/// Proof: `AssetRegistry::LocationToCurrencyIds` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `AssetRegistry::CurrencyIdToWeights` (r:0 w:1)
	/// Proof: `AssetRegistry::CurrencyIdToWeights` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `AssetRegistry::CurrencyIdToLocations` (r:0 w:1)
	/// Proof: `AssetRegistry::CurrencyIdToLocations` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn force_set_location() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `414`
		//  Estimated: `3879`
		// Minimum execution time: 11_367_000 picoseconds.
		Weight::from_parts(11_741_000, 3879)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(3_u64))
	}
	/// Storage: `AssetRegistry::CurrencyMetadatas` (r:1 w:1)
	/// Proof: `AssetRegistry::CurrencyMetadatas` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn update_currency_metadata() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `409`
		//  Estimated: `3874`
		// Minimum execution time: 9_045_000 picoseconds.
		Weight::from_parts(9_388_000, 3874)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
}
