// This file is part of Bifrost.

// Copyright (C) 2019-2021 Liebi Technologies (UK) Ltd.
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

//! Autogenerated weights for `bifrost_salp`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2021-09-26, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("bifrost-local"), DB CACHE: 128

// Executed Command:
// ./target/release/bifrost
// benchmark
// --chain=bifrost-local
// --steps=50
// --repeat=20
// --pallet=bifrost-salp
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --header=./HEADER-GPL3
// --output=./runtime/bifrost/src/weights/bifrost_salp.rs


#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for bifrost_salp.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> bifrost_salp::WeightInfo for WeightInfo<T> {
	// Storage: Salp Funds (r:1 w:0)
	// Storage: Tokens Accounts (r:1 w:1)
	// Storage: System Number (r:1 w:0)
	// Storage: System ExecutionPhase (r:1 w:0)
	// Storage: System EventCount (r:1 w:1)
	// Storage: System Events (r:1 w:1)
	// Storage: Salp CurrentNonce (r:1 w:1)
	// Storage: ParachainSystem HostConfiguration (r:1 w:0)
	// Storage: ParachainSystem PendingUpwardMessages (r:1 w:1)
	// Storage: unknown [0xd861ea1ebf4800d4b89f4ff787ad79ee96d9a708c85b57da7eb8f9ddeda61291] (r:1 w:1)
	fn contribute() -> Weight {
		(335_598_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(10 as Weight))
			.saturating_add(T::DbWeight::get().writes(6 as Weight))
	}
	// Storage: Salp Funds (r:1 w:0)
	// Storage: Tokens Accounts (r:4 w:4)
	// Storage: Tokens TotalIssuance (r:2 w:2)
	// Storage: System Account (r:1 w:1)
	// Storage: System Number (r:1 w:0)
	// Storage: System ExecutionPhase (r:1 w:0)
	// Storage: System EventCount (r:1 w:1)
	// Storage: System Events (r:1 w:1)
	// Storage: unknown [0xd861ea1ebf4800d4b89f4ff787ad79ee96d9a708c85b57da7eb8f9ddeda61291] (r:1 w:1)
	fn refund() -> Weight {
		(328_431_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(13 as Weight))
			.saturating_add(T::DbWeight::get().writes(10 as Weight))
	}
	// Storage: Salp Funds (r:1 w:0)
	// Storage: Tokens Accounts (r:2 w:2)
	// Storage: System Number (r:1 w:0)
	// Storage: System ExecutionPhase (r:1 w:0)
	// Storage: System EventCount (r:1 w:1)
	// Storage: System Events (r:1 w:1)
	// Storage: unknown [0xd861ea1ebf4800d4b89f4ff787ad79ee96d9a708c85b57da7eb8f9ddeda61291] (r:1 w:1)
	fn unlock() -> Weight {
		(252_844_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(8 as Weight))
			.saturating_add(T::DbWeight::get().writes(5 as Weight))
	}
	// Storage: Salp Funds (r:1 w:0)
	// Storage: Salp RedeemPool (r:1 w:1)
	// Storage: System Number (r:1 w:0)
	// Storage: Tokens Accounts (r:4 w:4)
	// Storage: Tokens TotalIssuance (r:2 w:2)
	// Storage: System ExecutionPhase (r:1 w:0)
	// Storage: System EventCount (r:1 w:1)
	// Storage: System Events (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: unknown [0xd861ea1ebf4800d4b89f4ff787ad79ee96d9a708c85b57da7eb8f9ddeda61291] (r:1 w:1)
	fn redeem() -> Weight {
		(395_212_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(14 as Weight))
			.saturating_add(T::DbWeight::get().writes(11 as Weight))
	}

	fn batch_unlock(k: u32) -> Weight {
		(0 as Weight)
			.saturating_add((45_890_000 as Weight).saturating_mul(k as Weight))
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().reads((2 as Weight).saturating_mul(k as Weight)))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
			.saturating_add(T::DbWeight::get().writes((2 as Weight).saturating_mul(k as Weight)))
	}
}
