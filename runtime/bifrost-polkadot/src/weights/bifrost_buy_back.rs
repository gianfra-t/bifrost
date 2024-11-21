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

//! Autogenerated weights for bifrost_buy_back
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
// --pallet=bifrost_buy_back
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./runtime/bifrost-polkadot/src/weights/bifrost_buy_back.rs
// --template=./weight-template/runtime-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions for bifrost_buy_back.
pub struct BifrostWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> bifrost_buy_back::WeightInfo for BifrostWeight<T> {
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
	// Storage: `BuyBack::Infos` (r:0 w:1)
	// Proof: `BuyBack::Infos` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn set_vtoken() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `667`
		//  Estimated: `4132`
		// Minimum execution time: 12_348 nanoseconds.
		Weight::from_parts(12_776_000, 4132)
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	// Storage: `AssetRegistry::CurrencyMetadatas` (r:1 w:0)
	// Proof: `AssetRegistry::CurrencyMetadatas` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `Tokens::Accounts` (r:2 w:2)
	// Proof: `Tokens::Accounts` (`max_values`: None, `max_size`: Some(118), added: 2593, mode: `MaxEncodedLen`)
	// Storage: `System::Account` (r:2 w:1)
	// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	// Storage: `System::Number` (r:1 w:0)
	// Proof: `System::Number` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	// Storage: `System::ExecutionPhase` (r:1 w:0)
	// Proof: `System::ExecutionPhase` (`max_values`: Some(1), `max_size`: Some(5), added: 500, mode: `MaxEncodedLen`)
	// Storage: `System::EventCount` (r:1 w:1)
	// Proof: `System::EventCount` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	// Storage: `System::Events` (r:1 w:1)
	// Proof: `System::Events` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	fn charge() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1441`
		//  Estimated: `6196`
		// Minimum execution time: 36_603 nanoseconds.
		Weight::from_parts(37_893_000, 6196)
			.saturating_add(T::DbWeight::get().reads(9))
			.saturating_add(T::DbWeight::get().writes(5))
	}
	// Storage: `BuyBack::Infos` (r:1 w:1)
	// Proof: `BuyBack::Infos` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `System::Number` (r:1 w:0)
	// Proof: `System::Number` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	// Storage: `System::ExecutionPhase` (r:1 w:0)
	// Proof: `System::ExecutionPhase` (`max_values`: Some(1), `max_size`: Some(5), added: 500, mode: `MaxEncodedLen`)
	// Storage: `System::EventCount` (r:1 w:1)
	// Proof: `System::EventCount` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	// Storage: `System::Events` (r:1 w:1)
	// Proof: `System::Events` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	fn remove_vtoken() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `266`
		//  Estimated: `3731`
		// Minimum execution time: 9_050 nanoseconds.
		Weight::from_parts(9_568_000, 3731)
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	// Storage: `BuyBack::Infos` (r:2 w:0)
	// Proof: `BuyBack::Infos` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn on_initialize() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `303`
		//  Estimated: `6243`
		// Minimum execution time: 6_686 nanoseconds.
		Weight::from_parts(7_136_000, 6243)
			.saturating_add(T::DbWeight::get().reads(2))
	}
}
