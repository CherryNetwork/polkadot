// Copyright 2017-2022 Parity Technologies (UK) Ltd.
// This file is part of Polkadot.

// Polkadot is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Polkadot is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Polkadot.  If not, see <http://www.gnu.org/licenses/>.
//! Autogenerated weights for `pallet_nis`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-12-15, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `bm4`, CPU: `Intel(R) Core(TM) i7-7700K CPU @ 4.20GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("kusama-dev"), DB CACHE: 1024

// Executed Command:
// ./target/production/polkadot
// benchmark
// pallet
// --chain=kusama-dev
// --steps=50
// --repeat=20
// --pallet=pallet_nis
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --header=./file_header.txt
// --output=./runtime/kusama/src/weights/

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_nis`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_nis::WeightInfo for WeightInfo<T> {
	// Storage: Nis Queues (r:1 w:1)
	// Storage: Nis QueueTotals (r:1 w:1)
	/// The range of component `l` is `[0, 999]`.
	fn place_bid(l: u32, ) -> Weight {
		// Minimum execution time: 34_088 nanoseconds.
		Weight::from_ref_time(36_343_273)
			// Standard Error: 733
			.saturating_add(Weight::from_ref_time(72_985).saturating_mul(l.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	// Storage: Nis Queues (r:1 w:1)
	// Storage: Nis QueueTotals (r:1 w:1)
	fn place_bid_max() -> Weight {
		// Minimum execution time: 105_579 nanoseconds.
		Weight::from_ref_time(107_541_000)
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	// Storage: Nis Queues (r:1 w:1)
	// Storage: Nis QueueTotals (r:1 w:1)
	/// The range of component `l` is `[1, 1000]`.
	fn retract_bid(l: u32, ) -> Weight {
		// Minimum execution time: 42_023 nanoseconds.
		Weight::from_ref_time(37_884_296)
			// Standard Error: 707
			.saturating_add(Weight::from_ref_time(61_455).saturating_mul(l.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	// Storage: Nis Summary (r:1 w:0)
	// Storage: System Account (r:1 w:1)
	fn fund_deficit() -> Weight {
		// Minimum execution time: 41_848 nanoseconds.
		Weight::from_ref_time(42_431_000)
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: Nis Receipts (r:1 w:1)
	// Storage: Nis Summary (r:1 w:1)
	// Storage: NisCounterpartBalances Account (r:1 w:1)
	// Storage: NisCounterpartBalances TotalIssuance (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	fn thaw() -> Weight {
		// Minimum execution time: 63_268 nanoseconds.
		Weight::from_ref_time(64_073_000)
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(5))
	}
	// Storage: Nis Summary (r:1 w:1)
	// Storage: System Account (r:1 w:0)
	// Storage: Nis QueueTotals (r:1 w:1)
	fn process_queues() -> Weight {
		// Minimum execution time: 36_969 nanoseconds.
		Weight::from_ref_time(38_199_000)
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	// Storage: Nis Queues (r:1 w:1)
	fn process_queue() -> Weight {
		// Minimum execution time: 4_219 nanoseconds.
		Weight::from_ref_time(4_402_000)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: System Account (r:1 w:0)
	// Storage: Nis Receipts (r:0 w:1)
	fn process_bid() -> Weight {
		// Minimum execution time: 13_211 nanoseconds.
		Weight::from_ref_time(13_655_000)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
}