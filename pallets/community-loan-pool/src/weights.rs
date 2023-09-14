
//! Autogenerated weights for `pallet_community_loan_pool`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-09-09, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `LAPTOP-DFFNONK6`, CPU: `11th Gen Intel(R) Core(TM) i7-1165G7 @ 2.80GHz`
//! EXECUTION: None, WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/node-template
// benchmark
// pallet
// --chain
// dev
// --pallet
// pallet_community_loan_pool
// --extrinsic
// *
// --steps
// 50
// --repeat
// 20
// --output
// pallets/community-loan-pool/src/weights.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_community_loan_pool.
pub trait WeightInfo {
	fn propose() -> Weight;
	fn reject_proposal() -> Weight;
}

/// Weight functions for `pallet_community_loan_pool`.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	/// Storage: CommunityLoanPool ProposalCount (r:1 w:1)
	/// Proof: CommunityLoanPool ProposalCount (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: CommunityLoanPool Proposals (r:0 w:1)
	/// Proof: CommunityLoanPool Proposals (max_values: None, max_size: Some(108), added: 2583, mode: MaxEncodedLen)
	fn propose() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `210`
		//  Estimated: `1489`
		// Minimum execution time: 11_812_000 picoseconds.
		Weight::from_parts(12_571_000, 0)
			.saturating_add(Weight::from_parts(0, 1489))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: CommunityLoanPool Proposals (r:1 w:1)
	/// Proof: CommunityLoanPool Proposals (max_values: None, max_size: Some(108), added: 2583, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn reject_proposal() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `368`
		//  Estimated: `7166`
		// Minimum execution time: 13_903_000 picoseconds.
		Weight::from_parts(14_567_000, 0)
			.saturating_add(Weight::from_parts(0, 7166))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
}
