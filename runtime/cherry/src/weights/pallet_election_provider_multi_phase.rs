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
//! Autogenerated weights for `pallet_election_provider_multi_phase`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-01-23, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `runner-b3zmxxc-project-163-concurrent-0`, CPU: `Intel(R) Xeon(R) CPU @ 2.60GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("polkadot-dev"), DB CACHE: 1024

// Executed Command:
// ./target/production/polkadot
// benchmark
// pallet
// --chain=polkadot-dev
// --steps=50
// --repeat=20
// --pallet=pallet_election_provider_multi_phase
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --header=./file_header.txt
// --output=./runtime/polkadot/src/weights/

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_election_provider_multi_phase`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_election_provider_multi_phase::WeightInfo for WeightInfo<T> {
	// Storage: Staking CurrentEra (r:1 w:0)
	// Storage: Staking CurrentPlannedSession (r:1 w:0)
	// Storage: Staking ErasStartSessionIndex (r:1 w:0)
	// Storage: Babe EpochIndex (r:1 w:0)
	// Storage: Babe GenesisSlot (r:1 w:0)
	// Storage: Babe CurrentSlot (r:1 w:0)
	// Storage: Staking ForceEra (r:1 w:0)
	// Storage: ElectionProviderMultiPhase CurrentPhase (r:1 w:0)
	fn on_initialize_nothing() -> Weight {
		// Minimum execution time: 18_492 nanoseconds.
		Weight::from_ref_time(19_069_000)
			.saturating_add(T::DbWeight::get().reads(8))
	}
	// Storage: ElectionProviderMultiPhase Round (r:1 w:0)
	// Storage: ElectionProviderMultiPhase CurrentPhase (r:1 w:1)
	fn on_initialize_open_signed() -> Weight {
		// Minimum execution time: 16_448 nanoseconds.
		Weight::from_ref_time(17_165_000)
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: ElectionProviderMultiPhase Round (r:1 w:0)
	// Storage: ElectionProviderMultiPhase CurrentPhase (r:1 w:1)
	fn on_initialize_open_unsigned() -> Weight {
		// Minimum execution time: 17_477 nanoseconds.
		Weight::from_ref_time(18_090_000)
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: System Account (r:1 w:1)
	// Storage: ElectionProviderMultiPhase QueuedSolution (r:0 w:1)
	fn finalize_signed_phase_accept_solution() -> Weight {
		// Minimum execution time: 31_576 nanoseconds.
		Weight::from_ref_time(32_501_000)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	// Storage: System Account (r:1 w:1)
	fn finalize_signed_phase_reject_solution() -> Weight {
		// Minimum execution time: 24_211 nanoseconds.
		Weight::from_ref_time(25_437_000)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: ElectionProviderMultiPhase SnapshotMetadata (r:0 w:1)
	// Storage: ElectionProviderMultiPhase DesiredTargets (r:0 w:1)
	// Storage: ElectionProviderMultiPhase Snapshot (r:0 w:1)
	/// The range of component `v` is `[1000, 2000]`.
	/// The range of component `t` is `[500, 1000]`.
	fn create_snapshot_internal(v: u32, _t: u32, ) -> Weight {
		// Minimum execution time: 557_265 nanoseconds.
		Weight::from_ref_time(584_626_000)
			// Standard Error: 3_005
			.saturating_add(Weight::from_ref_time(336_932).saturating_mul(v.into()))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	// Storage: ElectionProviderMultiPhase SignedSubmissionIndices (r:1 w:1)
	// Storage: ElectionProviderMultiPhase SignedSubmissionNextIndex (r:1 w:1)
	// Storage: ElectionProviderMultiPhase SnapshotMetadata (r:1 w:1)
	// Storage: ElectionProviderMultiPhase SignedSubmissionsMap (r:1 w:0)
	// Storage: System BlockWeight (r:1 w:1)
	// Storage: ElectionProviderMultiPhase QueuedSolution (r:1 w:1)
	// Storage: ElectionProviderMultiPhase Round (r:1 w:1)
	// Storage: ElectionProviderMultiPhase CurrentPhase (r:1 w:1)
	// Storage: ElectionProviderMultiPhase DesiredTargets (r:0 w:1)
	// Storage: ElectionProviderMultiPhase Snapshot (r:0 w:1)
	/// The range of component `a` is `[500, 800]`.
	/// The range of component `d` is `[200, 400]`.
	fn elect_queued(a: u32, d: u32, ) -> Weight {
		// Minimum execution time: 443_968 nanoseconds.
		Weight::from_ref_time(92_327_257)
			// Standard Error: 9_377
			.saturating_add(Weight::from_ref_time(634_434).saturating_mul(a.into()))
			// Standard Error: 14_056
			.saturating_add(Weight::from_ref_time(227_206).saturating_mul(d.into()))
			.saturating_add(T::DbWeight::get().reads(8))
			.saturating_add(T::DbWeight::get().writes(9))
	}
	// Storage: ElectionProviderMultiPhase CurrentPhase (r:1 w:0)
	// Storage: ElectionProviderMultiPhase SnapshotMetadata (r:1 w:0)
	// Storage: TransactionPayment NextFeeMultiplier (r:1 w:0)
	// Storage: ElectionProviderMultiPhase SignedSubmissionIndices (r:1 w:1)
	// Storage: ElectionProviderMultiPhase SignedSubmissionNextIndex (r:1 w:1)
	// Storage: ElectionProviderMultiPhase SignedSubmissionsMap (r:0 w:1)
	fn submit() -> Weight {
		// Minimum execution time: 53_600 nanoseconds.
		Weight::from_ref_time(55_308_000)
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	// Storage: ElectionProviderMultiPhase CurrentPhase (r:1 w:0)
	// Storage: ElectionProviderMultiPhase Round (r:1 w:0)
	// Storage: ElectionProviderMultiPhase DesiredTargets (r:1 w:0)
	// Storage: ElectionProviderMultiPhase QueuedSolution (r:1 w:1)
	// Storage: ElectionProviderMultiPhase SnapshotMetadata (r:1 w:0)
	// Storage: ElectionProviderMultiPhase MinimumUntrustedScore (r:1 w:0)
	// Storage: ElectionProviderMultiPhase Snapshot (r:1 w:0)
	/// The range of component `v` is `[1000, 2000]`.
	/// The range of component `t` is `[500, 1000]`.
	/// The range of component `a` is `[500, 800]`.
	/// The range of component `d` is `[200, 400]`.
	fn submit_unsigned(v: u32, _t: u32, a: u32, _d: u32, ) -> Weight {
		// Minimum execution time: 5_921_018 nanoseconds.
		Weight::from_ref_time(6_045_046_000)
			// Standard Error: 20_028
			.saturating_add(Weight::from_ref_time(139_664).saturating_mul(v.into()))
			// Standard Error: 59_351
			.saturating_add(Weight::from_ref_time(5_539_786).saturating_mul(a.into()))
			.saturating_add(T::DbWeight::get().reads(7))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: ElectionProviderMultiPhase Round (r:1 w:0)
	// Storage: ElectionProviderMultiPhase DesiredTargets (r:1 w:0)
	// Storage: ElectionProviderMultiPhase MinimumUntrustedScore (r:1 w:0)
	// Storage: ElectionProviderMultiPhase Snapshot (r:1 w:0)
	/// The range of component `v` is `[1000, 2000]`.
	/// The range of component `t` is `[500, 1000]`.
	/// The range of component `a` is `[500, 800]`.
	/// The range of component `d` is `[200, 400]`.
	fn feasibility_check(v: u32, _t: u32, a: u32, _d: u32, ) -> Weight {
		// Minimum execution time: 4_806_817 nanoseconds.
		Weight::from_ref_time(4_935_057_000)
			// Standard Error: 14_025
			.saturating_add(Weight::from_ref_time(324_592).saturating_mul(v.into()))
			// Standard Error: 41_563
			.saturating_add(Weight::from_ref_time(3_230_713).saturating_mul(a.into()))
			.saturating_add(T::DbWeight::get().reads(4))
	}
}