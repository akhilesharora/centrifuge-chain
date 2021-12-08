//! Autogenerated weights for pallet_crowdloan_claim
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2021-10-27, STEPS: `100`, REPEAT: 200, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("devel-local"), DB CACHE: 128

// Executed Command:
// target/release/centrifuge-chain
// benchmark
// --chain=devel-local
// --steps=100
// --repeat=200
// --pallet=pallet-crowdloan-claim
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=pallets/crowdloan-claim/src/weight.rs
// --template=./scripts/frame-weight-template.hbs

#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{
	traits::Get,
	weights::{constants::RocksDbWeight, Weight},
};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_crowdloan_claim.
pub trait WeightInfo {
	fn claim_reward_ed25519() -> Weight;
	fn claim_reward_sr25519() -> Weight;
	fn claim_reward_ecdsa() -> Weight;
	fn initialize() -> Weight;
	fn set_lease_start() -> Weight;
	fn set_lease_period() -> Weight;
	fn set_contributions_root() -> Weight;
	fn set_locked_at() -> Weight;
	fn set_crowdloan_trie_index() -> Weight;
}

/// Weights for pallet_crowdloan_claim using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	fn claim_reward_ed25519() -> Weight {
		(334_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(12 as Weight))
			.saturating_add(T::DbWeight::get().writes(5 as Weight))
	}
	fn claim_reward_sr25519() -> Weight {
		(345_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(12 as Weight))
			.saturating_add(T::DbWeight::get().writes(5 as Weight))
	}
	fn claim_reward_ecdsa() -> Weight {
		(462_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(12 as Weight))
			.saturating_add(T::DbWeight::get().writes(5 as Weight))
	}
	fn initialize() -> Weight {
		(50_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(6 as Weight))
	}
	fn set_lease_start() -> Weight {
		(28_000_000 as Weight).saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	fn set_lease_period() -> Weight {
		(24_000_000 as Weight).saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	fn set_contributions_root() -> Weight {
		(26_000_000 as Weight).saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	fn set_locked_at() -> Weight {
		(24_000_000 as Weight).saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	fn set_crowdloan_trie_index() -> Weight {
		(25_000_000 as Weight).saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	fn claim_reward_ed25519() -> Weight {
		(334_000_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(12 as Weight))
			.saturating_add(RocksDbWeight::get().writes(5 as Weight))
	}
	fn claim_reward_sr25519() -> Weight {
		(345_000_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(12 as Weight))
			.saturating_add(RocksDbWeight::get().writes(5 as Weight))
	}
	fn claim_reward_ecdsa() -> Weight {
		(462_000_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(12 as Weight))
			.saturating_add(RocksDbWeight::get().writes(5 as Weight))
	}
	fn initialize() -> Weight {
		(50_000_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(4 as Weight))
			.saturating_add(RocksDbWeight::get().writes(6 as Weight))
	}
	fn set_lease_start() -> Weight {
		(28_000_000 as Weight).saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	fn set_lease_period() -> Weight {
		(24_000_000 as Weight).saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	fn set_contributions_root() -> Weight {
		(26_000_000 as Weight).saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	fn set_locked_at() -> Weight {
		(24_000_000 as Weight).saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	fn set_crowdloan_trie_index() -> Weight {
		(25_000_000 as Weight).saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
}