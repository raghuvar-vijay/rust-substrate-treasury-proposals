use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};
use primitives::MemeReference;

#[test]
fn insert_order_works() {
	new_test_ext().execute_with(|| {
		assert_ok!(MemePallet::insert_meme(Origin::signed(1), MemeReference{ owner_name: vec![], meme_name: vec![] }));
	});
}

#[test]
fn error_insert_order() {
	new_test_ext().execute_with(|| {
		assert_ok!(MemePallet::insert_meme(Origin::signed(1), MemeReference{ owner_name: vec![], meme_name: vec![] }));
		assert_noop!(MemePallet::insert_meme(Origin::signed(1), MemeReference{ owner_name: vec![], meme_name: vec![] }), Error::<Test>::MemeAlreadyExists);
	});
}

#[test]
fn get_meme_reference_hash_works() {
	new_test_ext().execute_with(|| {
		// Ensure the expected error is thrown when no value is present.
		// assert_noop!(TemplateModule::cause_error(Origin::signed(1)), Error::<Test>::NoneValue);
		assert_ok!(MemePallet::insert_meme(Origin::signed(1), MemeReference{ owner_name: vec![], meme_name: vec![] }));
		assert_ok!(MemePallet::get_meme_reference_hash(Origin::signed(1)));
	});
}
