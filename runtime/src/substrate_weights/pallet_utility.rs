
//! Autogenerated weights for `pallet_utility`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2024-08-16, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `recrafter-Legion-5-16IRX9`, CPU: `Intel(R) Core(TM) i7-14650HX`
//! WASM-EXECUTION: `Compiled`, CHAIN: `Some("dev")`, DB CACHE: 1024

// Executed Command:
// ./target/release/node-template
// benchmark
// pallet
// --chain
// dev
// --wasm-execution=compiled
// --pallet
// pallet-utility
// --no-storage-info
// --no-median-slopes
// --no-min-squares
// --extrinsic=*
// --heap-pages=4096
// --steps
// 50
// --repeat
// 20
// --output=./runtime/src/substrate_weights/pallet_utility.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions for `pallet_utility`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_utility::WeightInfo for WeightInfo<T> {
	/// The range of component `c` is `[0, 1000]`.
	fn batch(c: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 7_951_000 picoseconds.
		Weight::from_parts(53_801_957, 0)
			.saturating_add(Weight::from_parts(0, 0))
			// Standard Error: 21_479
			.saturating_add(Weight::from_parts(4_668_337, 0).saturating_mul(c.into()))
	}
	fn as_derivative() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 6_277_000 picoseconds.
		Weight::from_parts(6_739_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
	}
	/// The range of component `c` is `[0, 1000]`.
	fn batch_all(c: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 6_753_000 picoseconds.
		Weight::from_parts(7_047_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
			// Standard Error: 12_383
			.saturating_add(Weight::from_parts(5_146_402, 0).saturating_mul(c.into()))
	}
	fn dispatch_as() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 9_260_000 picoseconds.
		Weight::from_parts(9_778_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
	}
	/// The range of component `c` is `[0, 1000]`.
	fn force_batch(c: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 6_823_000 picoseconds.
		Weight::from_parts(14_346_410, 0)
			.saturating_add(Weight::from_parts(0, 0))
			// Standard Error: 21_141
			.saturating_add(Weight::from_parts(4_545_462, 0).saturating_mul(c.into()))
	}
}
