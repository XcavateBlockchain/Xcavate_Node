
//! Autogenerated weights for `pallet_nft_marketplace`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-12-11, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `LAPTOP-DFFNONK6`, CPU: `11th Gen Intel(R) Core(TM) i7-1165G7 @ 2.80GHz`
//! WASM-EXECUTION: `Compiled`, CHAIN: `Some("dev")`, DB CACHE: 1024

// Executed Command:
// ./target/release/node-template
// benchmark
// pallet
// --chain
// dev
// --pallet
// pallet_nft_marketplace
// --extrinsic
// *
// --steps
// 50
// --repeat
// 20
// --output
// pallets/nft-marketplace/src/weights.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

pub trait WeightInfo {
	fn list_object() -> Weight;
	fn list_nft() -> Weight;
	fn buy_nft() -> Weight;
	fn buy_single_nft() -> Weight;
	fn upgrade_listing() -> Weight;
	fn upgrade_object() -> Weight;
	fn delist_nft() -> Weight;
}

/// Weight functions for `pallet_nft_marketplace`.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	/// Storage: `Nfts::NextCollectionId` (r:1 w:1)
	/// Proof: `Nfts::NextCollectionId` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `Nfts::Collection` (r:1 w:1)
	/// Proof: `Nfts::Collection` (`max_values`: None, `max_size`: Some(84), added: 2559, mode: `MaxEncodedLen`)
	/// Storage: `Nfts::CollectionMetadataOf` (r:1 w:1)
	/// Proof: `Nfts::CollectionMetadataOf` (`max_values`: None, `max_size`: Some(5038), added: 7513, mode: `MaxEncodedLen`)
	/// Storage: `Nfts::Item` (r:100 w:100)
	/// Proof: `Nfts::Item` (`max_values`: None, `max_size`: Some(861), added: 3336, mode: `MaxEncodedLen`)
	/// Storage: `Nfts::ItemConfigOf` (r:100 w:100)
	/// Proof: `Nfts::ItemConfigOf` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	/// Storage: `Nfts::ItemMetadataOf` (r:100 w:100)
	/// Proof: `Nfts::ItemMetadataOf` (`max_values`: None, `max_size`: Some(5091), added: 7566, mode: `MaxEncodedLen`)
	/// Storage: `NftMarketplace::ListedNftsOfCollection` (r:1 w:1)
	/// Proof: `NftMarketplace::ListedNftsOfCollection` (`max_values`: None, `max_size`: Some(422), added: 2897, mode: `MaxEncodedLen`)
	/// Storage: `NftMarketplace::ListedNfts` (r:1 w:1)
	/// Proof: `NftMarketplace::ListedNfts` (`max_values`: Some(1), `max_size`: Some(2400004), added: 2400499, mode: `MaxEncodedLen`)
	/// Storage: `NftMarketplace::SellerListings` (r:1 w:1)
	/// Proof: `NftMarketplace::SellerListings` (`max_values`: None, `max_size`: Some(2400052), added: 2402527, mode: `MaxEncodedLen`)
	/// Storage: `Nfts::CollectionRoleOf` (r:0 w:1)
	/// Proof: `Nfts::CollectionRoleOf` (`max_values`: None, `max_size`: Some(69), added: 2544, mode: `MaxEncodedLen`)
	/// Storage: `Nfts::CollectionConfigOf` (r:0 w:1)
	/// Proof: `Nfts::CollectionConfigOf` (`max_values`: None, `max_size`: Some(73), added: 2548, mode: `MaxEncodedLen`)
	/// Storage: `Nfts::Account` (r:0 w:100)
	/// Proof: `Nfts::Account` (`max_values`: None, `max_size`: Some(88), added: 2563, mode: `MaxEncodedLen`)
	/// Storage: `Nfts::CollectionAccount` (r:0 w:1)
	/// Proof: `Nfts::CollectionAccount` (`max_values`: None, `max_size`: Some(68), added: 2543, mode: `MaxEncodedLen`)
	/// Storage: `NftMarketplace::OngoingNftDetails` (r:0 w:100)
	/// Proof: `NftMarketplace::OngoingNftDetails` (`max_values`: None, `max_size`: Some(129), added: 2604, mode: `MaxEncodedLen`)
	/// Storage: `NftMarketplace::ListedCollectionDetails` (r:0 w:1)
	/// Proof: `NftMarketplace::ListedCollectionDetails` (`max_values`: None, `max_size`: Some(21), added: 2496, mode: `MaxEncodedLen`)
	fn list_object() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `79`
		//  Estimated: `2403517`
		// Minimum execution time: 7_383_782_000 picoseconds.
		Weight::from_parts(7_766_911_000, 0)
			.saturating_add(Weight::from_parts(0, 2403517))
			.saturating_add(T::DbWeight::get().reads(306))
			.saturating_add(T::DbWeight::get().writes(510))
	}
	/// Storage: `NftMarketplace::ListedCollectionDetails` (r:1 w:0)
	/// Proof: `NftMarketplace::ListedCollectionDetails` (`max_values`: None, `max_size`: Some(21), added: 2496, mode: `MaxEncodedLen`)
	/// Storage: `Nfts::Collection` (r:1 w:0)
	/// Proof: `Nfts::Collection` (`max_values`: None, `max_size`: Some(84), added: 2559, mode: `MaxEncodedLen`)
	/// Storage: `Nfts::Attribute` (r:1 w:0)
	/// Proof: `Nfts::Attribute` (`max_values`: None, `max_size`: Some(446), added: 2921, mode: `MaxEncodedLen`)
	/// Storage: `Nfts::CollectionConfigOf` (r:1 w:0)
	/// Proof: `Nfts::CollectionConfigOf` (`max_values`: None, `max_size`: Some(73), added: 2548, mode: `MaxEncodedLen`)
	/// Storage: `Nfts::ItemConfigOf` (r:1 w:0)
	/// Proof: `Nfts::ItemConfigOf` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	/// Storage: `Nfts::Item` (r:1 w:1)
	/// Proof: `Nfts::Item` (`max_values`: None, `max_size`: Some(861), added: 3336, mode: `MaxEncodedLen`)
	/// Storage: `NftMarketplace::ListedNftsOfCollection` (r:1 w:1)
	/// Proof: `NftMarketplace::ListedNftsOfCollection` (`max_values`: None, `max_size`: Some(422), added: 2897, mode: `MaxEncodedLen`)
	/// Storage: `NftMarketplace::ListedNfts` (r:1 w:1)
	/// Proof: `NftMarketplace::ListedNfts` (`max_values`: Some(1), `max_size`: Some(2400004), added: 2400499, mode: `MaxEncodedLen`)
	/// Storage: `NftMarketplace::SellerListings` (r:1 w:1)
	/// Proof: `NftMarketplace::SellerListings` (`max_values`: None, `max_size`: Some(2400052), added: 2402527, mode: `MaxEncodedLen`)
	/// Storage: `Nfts::Account` (r:0 w:2)
	/// Proof: `Nfts::Account` (`max_values`: None, `max_size`: Some(88), added: 2563, mode: `MaxEncodedLen`)
	/// Storage: `Nfts::ItemPriceOf` (r:0 w:1)
	/// Proof: `Nfts::ItemPriceOf` (`max_values`: None, `max_size`: Some(89), added: 2564, mode: `MaxEncodedLen`)
	/// Storage: `Nfts::PendingSwapOf` (r:0 w:1)
	/// Proof: `Nfts::PendingSwapOf` (`max_values`: None, `max_size`: Some(71), added: 2546, mode: `MaxEncodedLen`)
	/// Storage: `NftMarketplace::OngoingNftDetails` (r:0 w:1)
	/// Proof: `NftMarketplace::OngoingNftDetails` (`max_values`: None, `max_size`: Some(129), added: 2604, mode: `MaxEncodedLen`)
	fn list_nft() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2148`
		//  Estimated: `2403517`
		// Minimum execution time: 100_008_000 picoseconds.
		Weight::from_parts(115_253_000, 0)
			.saturating_add(Weight::from_parts(0, 2403517))
			.saturating_add(T::DbWeight::get().reads(9))
			.saturating_add(T::DbWeight::get().writes(9))
	}
	/// Storage: `NftMarketplace::ListedNftsOfCollection` (r:1 w:1)
	/// Proof: `NftMarketplace::ListedNftsOfCollection` (`max_values`: None, `max_size`: Some(422), added: 2897, mode: `MaxEncodedLen`)
	/// Storage: `NftMarketplace::SoldNftsCollection` (r:1 w:1)
	/// Proof: `NftMarketplace::SoldNftsCollection` (`max_values`: None, `max_size`: Some(422), added: 2897, mode: `MaxEncodedLen`)
	/// Storage: `NftMarketplace::ListedCollectionDetails` (r:1 w:1)
	/// Proof: `NftMarketplace::ListedCollectionDetails` (`max_values`: None, `max_size`: Some(21), added: 2496, mode: `MaxEncodedLen`)
	/// Storage: `NftMarketplace::OngoingNftDetails` (r:100 w:100)
	/// Proof: `NftMarketplace::OngoingNftDetails` (`max_values`: None, `max_size`: Some(129), added: 2604, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `NftMarketplace::ListedNfts` (r:1 w:1)
	/// Proof: `NftMarketplace::ListedNfts` (`max_values`: Some(1), `max_size`: Some(2400004), added: 2400499, mode: `MaxEncodedLen`)
	/// Storage: `NftMarketplace::SellerListings` (r:1 w:1)
	/// Proof: `NftMarketplace::SellerListings` (`max_values`: None, `max_size`: Some(2400052), added: 2402527, mode: `MaxEncodedLen`)
	/// Storage: `Nfts::Collection` (r:1 w:0)
	/// Proof: `Nfts::Collection` (`max_values`: None, `max_size`: Some(84), added: 2559, mode: `MaxEncodedLen`)
	/// Storage: `Nfts::Attribute` (r:100 w:0)
	/// Proof: `Nfts::Attribute` (`max_values`: None, `max_size`: Some(446), added: 2921, mode: `MaxEncodedLen`)
	/// Storage: `Nfts::CollectionConfigOf` (r:1 w:0)
	/// Proof: `Nfts::CollectionConfigOf` (`max_values`: None, `max_size`: Some(73), added: 2548, mode: `MaxEncodedLen`)
	/// Storage: `Nfts::ItemConfigOf` (r:100 w:0)
	/// Proof: `Nfts::ItemConfigOf` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	/// Storage: `Nfts::Item` (r:100 w:100)
	/// Proof: `Nfts::Item` (`max_values`: None, `max_size`: Some(861), added: 3336, mode: `MaxEncodedLen`)
	/// Storage: `Nfts::Account` (r:0 w:200)
	/// Proof: `Nfts::Account` (`max_values`: None, `max_size`: Some(88), added: 2563, mode: `MaxEncodedLen`)
	/// Storage: `Nfts::ItemPriceOf` (r:0 w:100)
	/// Proof: `Nfts::ItemPriceOf` (`max_values`: None, `max_size`: Some(89), added: 2564, mode: `MaxEncodedLen`)
	/// Storage: `Nfts::PendingSwapOf` (r:0 w:100)
	/// Proof: `Nfts::PendingSwapOf` (`max_values`: None, `max_size`: Some(71), added: 2546, mode: `MaxEncodedLen`)
	fn buy_nft() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `28529`
		//  Estimated: `2403517`
		// Minimum execution time: 9_415_806_000 picoseconds.
		Weight::from_parts(9_936_742_000, 0)
			.saturating_add(Weight::from_parts(0, 2403517))
			.saturating_add(T::DbWeight::get().reads(408))
			.saturating_add(T::DbWeight::get().writes(606))
	}
	/// Storage: `NftMarketplace::ListedNftsOfCollection` (r:1 w:1)
	/// Proof: `NftMarketplace::ListedNftsOfCollection` (`max_values`: None, `max_size`: Some(422), added: 2897, mode: `MaxEncodedLen`)
	/// Storage: `NftMarketplace::SoldNftsCollection` (r:1 w:0)
	/// Proof: `NftMarketplace::SoldNftsCollection` (`max_values`: None, `max_size`: Some(422), added: 2897, mode: `MaxEncodedLen`)
	/// Storage: `NftMarketplace::ListedCollectionDetails` (r:1 w:0)
	/// Proof: `NftMarketplace::ListedCollectionDetails` (`max_values`: None, `max_size`: Some(21), added: 2496, mode: `MaxEncodedLen`)
	/// Storage: `NftMarketplace::OngoingNftDetails` (r:1 w:1)
	/// Proof: `NftMarketplace::OngoingNftDetails` (`max_values`: None, `max_size`: Some(129), added: 2604, mode: `MaxEncodedLen`)
	/// Storage: `Nfts::Collection` (r:1 w:0)
	/// Proof: `Nfts::Collection` (`max_values`: None, `max_size`: Some(84), added: 2559, mode: `MaxEncodedLen`)
	/// Storage: `Nfts::Attribute` (r:1 w:0)
	/// Proof: `Nfts::Attribute` (`max_values`: None, `max_size`: Some(446), added: 2921, mode: `MaxEncodedLen`)
	/// Storage: `Nfts::CollectionConfigOf` (r:1 w:0)
	/// Proof: `Nfts::CollectionConfigOf` (`max_values`: None, `max_size`: Some(73), added: 2548, mode: `MaxEncodedLen`)
	/// Storage: `Nfts::ItemConfigOf` (r:1 w:0)
	/// Proof: `Nfts::ItemConfigOf` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	/// Storage: `Nfts::Item` (r:1 w:1)
	/// Proof: `Nfts::Item` (`max_values`: None, `max_size`: Some(861), added: 3336, mode: `MaxEncodedLen`)
	/// Storage: `NftMarketplace::ListedNfts` (r:1 w:1)
	/// Proof: `NftMarketplace::ListedNfts` (`max_values`: Some(1), `max_size`: Some(2400004), added: 2400499, mode: `MaxEncodedLen`)
	/// Storage: `NftMarketplace::SellerListings` (r:1 w:1)
	/// Proof: `NftMarketplace::SellerListings` (`max_values`: None, `max_size`: Some(2400052), added: 2402527, mode: `MaxEncodedLen`)
	/// Storage: `Nfts::Account` (r:0 w:2)
	/// Proof: `Nfts::Account` (`max_values`: None, `max_size`: Some(88), added: 2563, mode: `MaxEncodedLen`)
	/// Storage: `Nfts::ItemPriceOf` (r:0 w:1)
	/// Proof: `Nfts::ItemPriceOf` (`max_values`: None, `max_size`: Some(89), added: 2564, mode: `MaxEncodedLen`)
	/// Storage: `Nfts::PendingSwapOf` (r:0 w:1)
	/// Proof: `Nfts::PendingSwapOf` (`max_values`: None, `max_size`: Some(71), added: 2546, mode: `MaxEncodedLen`)
	fn buy_single_nft() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2322`
		//  Estimated: `2403517`
		// Minimum execution time: 101_327_000 picoseconds.
		Weight::from_parts(118_964_000, 0)
			.saturating_add(Weight::from_parts(0, 2403517))
			.saturating_add(T::DbWeight::get().reads(11))
			.saturating_add(T::DbWeight::get().writes(9))
	}
	/// Storage: `NftMarketplace::OngoingNftDetails` (r:1 w:1)
	/// Proof: `NftMarketplace::OngoingNftDetails` (`max_values`: None, `max_size`: Some(129), added: 2604, mode: `MaxEncodedLen`)
	fn upgrade_listing() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `328`
		//  Estimated: `3594`
		// Minimum execution time: 27_884_000 picoseconds.
		Weight::from_parts(32_187_000, 0)
			.saturating_add(Weight::from_parts(0, 3594))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `NftMarketplace::ListedNftsOfCollection` (r:1 w:0)
	/// Proof: `NftMarketplace::ListedNftsOfCollection` (`max_values`: None, `max_size`: Some(422), added: 2897, mode: `MaxEncodedLen`)
	/// Storage: `NftMarketplace::SoldNftsCollection` (r:1 w:0)
	/// Proof: `NftMarketplace::SoldNftsCollection` (`max_values`: None, `max_size`: Some(422), added: 2897, mode: `MaxEncodedLen`)
	/// Storage: `NftMarketplace::ListedCollectionDetails` (r:1 w:0)
	/// Proof: `NftMarketplace::ListedCollectionDetails` (`max_values`: None, `max_size`: Some(21), added: 2496, mode: `MaxEncodedLen`)
	/// Storage: `NftMarketplace::OngoingNftDetails` (r:100 w:100)
	/// Proof: `NftMarketplace::OngoingNftDetails` (`max_values`: None, `max_size`: Some(129), added: 2604, mode: `MaxEncodedLen`)
	fn upgrade_object() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `12339`
		//  Estimated: `261390`
		// Minimum execution time: 586_204_000 picoseconds.
		Weight::from_parts(625_466_000, 0)
			.saturating_add(Weight::from_parts(0, 261390))
			.saturating_add(T::DbWeight::get().reads(103))
			.saturating_add(T::DbWeight::get().writes(100))
	}
	/// Storage: `NftMarketplace::OngoingNftDetails` (r:1 w:1)
	/// Proof: `NftMarketplace::OngoingNftDetails` (`max_values`: None, `max_size`: Some(129), added: 2604, mode: `MaxEncodedLen`)
	/// Storage: `Nfts::Collection` (r:1 w:0)
	/// Proof: `Nfts::Collection` (`max_values`: None, `max_size`: Some(84), added: 2559, mode: `MaxEncodedLen`)
	/// Storage: `Nfts::Attribute` (r:1 w:0)
	/// Proof: `Nfts::Attribute` (`max_values`: None, `max_size`: Some(446), added: 2921, mode: `MaxEncodedLen`)
	/// Storage: `Nfts::CollectionConfigOf` (r:1 w:0)
	/// Proof: `Nfts::CollectionConfigOf` (`max_values`: None, `max_size`: Some(73), added: 2548, mode: `MaxEncodedLen`)
	/// Storage: `Nfts::ItemConfigOf` (r:1 w:0)
	/// Proof: `Nfts::ItemConfigOf` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	/// Storage: `Nfts::Item` (r:1 w:1)
	/// Proof: `Nfts::Item` (`max_values`: None, `max_size`: Some(861), added: 3336, mode: `MaxEncodedLen`)
	/// Storage: `NftMarketplace::ListedNfts` (r:1 w:1)
	/// Proof: `NftMarketplace::ListedNfts` (`max_values`: Some(1), `max_size`: Some(2400004), added: 2400499, mode: `MaxEncodedLen`)
	/// Storage: `NftMarketplace::ListedNftsOfCollection` (r:1 w:1)
	/// Proof: `NftMarketplace::ListedNftsOfCollection` (`max_values`: None, `max_size`: Some(422), added: 2897, mode: `MaxEncodedLen`)
	/// Storage: `NftMarketplace::SellerListings` (r:1 w:1)
	/// Proof: `NftMarketplace::SellerListings` (`max_values`: None, `max_size`: Some(2400052), added: 2402527, mode: `MaxEncodedLen`)
	/// Storage: `Nfts::Account` (r:0 w:2)
	/// Proof: `Nfts::Account` (`max_values`: None, `max_size`: Some(88), added: 2563, mode: `MaxEncodedLen`)
	/// Storage: `Nfts::ItemPriceOf` (r:0 w:1)
	/// Proof: `Nfts::ItemPriceOf` (`max_values`: None, `max_size`: Some(89), added: 2564, mode: `MaxEncodedLen`)
	/// Storage: `Nfts::PendingSwapOf` (r:0 w:1)
	/// Proof: `Nfts::PendingSwapOf` (`max_values`: None, `max_size`: Some(71), added: 2546, mode: `MaxEncodedLen`)
	fn delist_nft() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2314`
		//  Estimated: `2403517`
		// Minimum execution time: 94_506_000 picoseconds.
		Weight::from_parts(113_502_000, 0)
			.saturating_add(Weight::from_parts(0, 2403517))
			.saturating_add(T::DbWeight::get().reads(9))
			.saturating_add(T::DbWeight::get().writes(9))
	}
}
