//! Benchmarking setup for pallet-template

use super::*;

#[allow(unused)]
use crate::Pallet as Template;
use frame_benchmarking::{benchmarks, whitelisted_caller};
use frame_benchmarking::{impl_benchmark_test_suite};
use frame_system::RawOrigin;
use primitives::MemeReference;
use sp_std::vec::Vec;
use frame_system::Origin;

benchmarks! {
	insert_meme {
		let s = MemeReference{ owner_name: vec![], meme_name: vec![]};
		let caller: T::AccountId = T::AccountId::default();
	}: _(RawOrigin::Signed(caller.clone()), s)
	// verify {
	// 	assert_eq!(RecordBook::<T>::get(), Some(s));
	// }

}
impl_benchmark_test_suite!(
	Template,
	crate::mock::new_test_ext(),
	crate::mock::Test
);
