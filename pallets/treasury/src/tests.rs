use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};
use primitives::Proposal;

#[test]
fn donate_works() {
	new_test_ext().execute_with(|| {
		assert_ok!(Treasury::donate(Origin::signed(1), 10000));
	});
}

#[test]
fn propose_spend_works() {
	new_test_ext().execute_with(|| {
		assert_ok!(Treasury::propose_spend(Origin::signed(1), 10000, Origin::signed(BOB)));
	});
}


#[test]
fn allocate_works() {
	new_test_ext().execute_with(|| {
		assert_ok!(Treasury::propose_spend(Origin::signed(1), 10000, Origin::signed(BOB)));
		assert_ok!(Treasury::allocate((Origin::root(), ALICE), 0));
	});
}
