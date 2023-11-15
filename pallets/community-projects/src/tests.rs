use crate::{mock::*, Error, Event};
use frame_support::{
	assert_noop, assert_ok,
	traits::{OnFinalize, OnInitialize},
};

use crate::Config;
use crate::{BalanceOf, BoundedNftDonationTypes, NftDonationTypes};

macro_rules! bvec {
	($( $x:tt )*) => {
		vec![$( $x )*].try_into().unwrap()
	}
}

fn get_project_nfts(mut n: u32) -> BoundedNftDonationTypes<Test> {
	let max = <Test as Config>::MaxNftTypes::get();
	if n > max {
		n = max
	}
	(1..=n)
		.map(|x| NftDonationTypes::<BalanceOf<Test>> { price: (100 * x).into(), amount: x })
		.collect::<Vec<NftDonationTypes<BalanceOf<Test>>>>()
		.try_into()
		.expect("bound is ensured; qed")
}

fn run_to_block(n: u64) {
	while System::block_number() < n {
		if System::block_number() > 0 {
			CommunityProjects::on_finalize(System::block_number());
			System::on_finalize(System::block_number());
		}
		System::reset_events();
		System::set_block_number(System::block_number() + 1);
		System::on_initialize(System::block_number());
		CommunityProjects::on_initialize(System::block_number());
	}
}

#[test]
fn list_works() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		assert_ok!(CommunityProjects::list_project(
			RuntimeOrigin::signed([0; 32].into()),
			get_project_nfts(3),
			5,
			900,
			bvec![22, 22]
		));
		assert_eq!(CommunityProjects::listed_nfts().len(), 6);
	});
}

#[test]
fn buy_works() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		assert_ok!(CommunityProjects::list_project(
			RuntimeOrigin::signed([0; 32].into()),
			get_project_nfts(3),
			5,
			900,
			bvec![22, 22]
		));
		assert_ok!(CommunityProjects::buy_nft(RuntimeOrigin::signed([1; 32].into()), 0, 1));
		assert_eq!(CommunityProjects::listed_nfts().len(), 5);
		assert_eq!(Assets::balance(1, &[1; 32].into()), 1300);
		assert_eq!(Assets::balance(1, &CommunityProjects::account_id()), 200);
	});
}

#[test]
fn launch_project_works() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		assert_ok!(CommunityProjects::list_project(
			RuntimeOrigin::signed([0; 32].into()),
			get_project_nfts(3),
			5,
			300,
			bvec![22, 22]
		));
		assert_ok!(CommunityProjects::buy_nft(RuntimeOrigin::signed([1; 32].into()), 0, 1));
		assert_ok!(CommunityProjects::buy_nft(RuntimeOrigin::signed([2; 32].into()), 0, 0));
		assert_eq!(CommunityProjects::listed_nfts().len(), 0);
	});
}

#[test]
fn voting_works() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		assert_ok!(CommunityProjects::list_project(
			RuntimeOrigin::signed([0; 32].into()),
			get_project_nfts(3),
			5,
			300,
			bvec![22, 22]
		));
		assert_ok!(CommunityProjects::buy_nft(RuntimeOrigin::signed([1; 32].into()), 0, 1));
		assert_ok!(CommunityProjects::buy_nft(RuntimeOrigin::signed([2; 32].into()), 0, 0));
		assert_eq!(CommunityProjects::listed_nfts().len(), 0);
		run_to_block(11);
		assert_ok!(CommunityProjects::vote_on_milestone(
			RuntimeOrigin::signed([2; 32].into()),
			0,
			crate::Vote::Yes
		));
		assert_eq!(CommunityProjects::ongoing_votes(0).unwrap().yes_votes, 100);
	});
}

#[test]
fn rejecting_vote_works() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		assert_ok!(CommunityProjects::list_project(
			RuntimeOrigin::signed([0; 32].into()),
			get_project_nfts(3),
			2,
			900,
			bvec![22, 22]
		));
		assert_ok!(CommunityProjects::buy_nft(RuntimeOrigin::signed([1; 32].into()), 0, 5));
		assert_ok!(CommunityProjects::buy_nft(RuntimeOrigin::signed([1; 32].into()), 0, 4));
		assert_ok!(CommunityProjects::buy_nft(RuntimeOrigin::signed([2; 32].into()), 0, 3));
		run_to_block(11);
		assert_ok!(CommunityProjects::vote_on_milestone(
			RuntimeOrigin::signed([2; 32].into()),
			0,
			crate::Vote::Yes
		));
		assert_ok!(CommunityProjects::vote_on_milestone(
			RuntimeOrigin::signed([1; 32].into()),
			0,
			crate::Vote::No
		));
		assert_eq!(CommunityProjects::ongoing_votes(0).unwrap().no_votes, 600);
		run_to_block(31);
		assert_ok!(CommunityProjects::vote_on_milestone(
			RuntimeOrigin::signed([1; 32].into()),
			0,
			crate::Vote::No
		));
		run_to_block(51);
		assert_ok!(CommunityProjects::vote_on_milestone(
			RuntimeOrigin::signed([1; 32].into()),
			0,
			crate::Vote::No
		));
		run_to_block(61);
		assert_eq!(CommunityProjects::ongoing_projects(0), None);
	})
}

#[test]
fn voting_fails_with_no_permission() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		assert_ok!(CommunityProjects::list_project(
			RuntimeOrigin::signed([0; 32].into()),
			get_project_nfts(3),
			3,
			300,
			bvec![22, 22]
		));
		assert_ok!(CommunityProjects::buy_nft(RuntimeOrigin::signed([1; 32].into()), 0, 5));
		run_to_block(11);
		assert_noop!(CommunityProjects::vote_on_milestone(
			RuntimeOrigin::signed([2; 32].into()),
			0,
			crate::Vote::Yes
		), Error::<Test>::InsufficientPermission);
	})
}

#[test]
fn voting_fails_with_double_voting() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		assert_ok!(CommunityProjects::list_project(
			RuntimeOrigin::signed([0; 32].into()),
			get_project_nfts(3),
			3,
			300,
			bvec![22, 22]
		));
		assert_ok!(CommunityProjects::buy_nft(RuntimeOrigin::signed([1; 32].into()), 0, 5));
		run_to_block(11);
		assert_ok!(CommunityProjects::vote_on_milestone(
			RuntimeOrigin::signed([1; 32].into()),
			0,
			crate::Vote::Yes
		));
		assert_noop!(CommunityProjects::vote_on_milestone(
			RuntimeOrigin::signed([1; 32].into()),
			0,
			crate::Vote::Yes
		), Error::<Test>::AlreadyVoted);
	})
}

#[test]
fn voting_fails_with_no_ongoing_voting() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		assert_ok!(CommunityProjects::list_project(
			RuntimeOrigin::signed([0; 32].into()),
			get_project_nfts(3),
			3,
			300,
			bvec![22, 22]
		));
		assert_ok!(CommunityProjects::buy_nft(RuntimeOrigin::signed([1; 32].into()), 0, 5));
		assert_noop!(CommunityProjects::vote_on_milestone(
			RuntimeOrigin::signed([1; 32].into()),
			0,
			crate::Vote::Yes
		), Error::<Test>::NoOngoingVotingPeriod);
	})
}

#[test]
fn set_strikes_works() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		assert_ok!(CommunityProjects::list_project(
			RuntimeOrigin::signed([0; 32].into()),
			get_project_nfts(3),
			3,
			300,
			bvec![22, 22]
		));
		assert_ok!(CommunityProjects::buy_nft(RuntimeOrigin::signed([1; 32].into()), 0, 5));
		run_to_block(11);
		assert_ok!(CommunityProjects::vote_on_milestone(
			RuntimeOrigin::signed([1; 32].into()),
			0,
			crate::Vote::No
		));
		run_to_block(31);
		assert_ok!(CommunityProjects::vote_on_milestone(
			RuntimeOrigin::signed([1; 32].into()),
			0,
			crate::Vote::No
		));
		run_to_block(41);
		assert_eq!(CommunityProjects::ongoing_projects(0).unwrap().strikes, 2);
		run_to_block(51);
		assert_ok!(CommunityProjects::vote_on_milestone(
			RuntimeOrigin::signed([1; 32].into()),
			0,
			crate::Vote::Yes
		));
		run_to_block(61);
		assert_eq!(CommunityProjects::ongoing_projects(0).unwrap().strikes, 0);
	})
}

#[test]
fn distributing_funds_works() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		assert_ok!(CommunityProjects::list_project(
			RuntimeOrigin::signed([0; 32].into()),
			get_project_nfts(3),
			3,
			300,
			bvec![22, 22]
		));
		assert_ok!(CommunityProjects::buy_nft(RuntimeOrigin::signed([1; 32].into()), 0, 1));
		assert_ok!(CommunityProjects::buy_nft(RuntimeOrigin::signed([2; 32].into()), 0, 0));
		assert_eq!(CommunityProjects::listed_nfts().len(), 0);
		run_to_block(11);
		assert_ok!(CommunityProjects::vote_on_milestone(
			RuntimeOrigin::signed([2; 32].into()),
			0,
			crate::Vote::Yes
		));
		run_to_block(22);
		assert_eq!(Assets::balance(1, &[0; 32].into()), 20_000_100);
		assert_eq!(Assets::balance(1, &CommunityProjects::account_id()), 200);
	});
}

#[test]
fn distributing_funds_works_for_2_rounds_works() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		assert_ok!(CommunityProjects::list_project(
			RuntimeOrigin::signed([0; 32].into()),
			get_project_nfts(3),
			3,
			300,
			bvec![22, 22]
		));
		assert_ok!(CommunityProjects::buy_nft(RuntimeOrigin::signed([1; 32].into()), 0, 1));
		assert_ok!(CommunityProjects::buy_nft(RuntimeOrigin::signed([2; 32].into()), 0, 0));
		assert_eq!(CommunityProjects::listed_nfts().len(), 0);
		run_to_block(11);
		assert_ok!(CommunityProjects::vote_on_milestone(
			RuntimeOrigin::signed([2; 32].into()),
			0,
			crate::Vote::Yes
		));
		run_to_block(22);
		assert_eq!(Assets::balance(1, &[0; 32].into()), 20_000_100);
		assert_eq!(Assets::balance(1, &CommunityProjects::account_id()), 200);
		run_to_block(32);
		assert_ok!(CommunityProjects::vote_on_milestone(
			RuntimeOrigin::signed([2; 32].into()),
			0,
			crate::Vote::Yes
		));
		run_to_block(42);
		assert_eq!(Assets::balance(1, &[0; 32].into()), 20_000_200);
		assert_eq!(Assets::balance(1, &CommunityProjects::account_id()), 100);
	});
}

#[test]
fn delete_project_works() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		assert_ok!(CommunityProjects::list_project(
			RuntimeOrigin::signed([0; 32].into()),
			get_project_nfts(3),
			1,
			300,
			bvec![22, 22]
		));
		assert_ok!(CommunityProjects::buy_nft(RuntimeOrigin::signed([1; 32].into()), 0, 1));
		assert_ok!(CommunityProjects::buy_nft(RuntimeOrigin::signed([2; 32].into()), 0, 0));
		run_to_block(11);
		assert_ok!(CommunityProjects::vote_on_milestone(
			RuntimeOrigin::signed([2; 32].into()),
			0,
			crate::Vote::Yes
		));
		run_to_block(22);
		assert_eq!(Assets::balance(1, &[0; 32].into()), 20_000_300);
		assert_eq!(Assets::balance(1, &CommunityProjects::account_id()), 0);
		assert_eq!(CommunityProjects::ongoing_projects(0), None);
	})
}
