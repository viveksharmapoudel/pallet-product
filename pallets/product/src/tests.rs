use core::{ str::Bytes};

use crate::{mock::{*, self}, Error, types::{ProductName, self, Product}};
use frame_support::{assert_noop, assert_ok, BoundedVec};
use frame_system::Config;
use sp_core::ConstU32;


#[test]
fn it_works_for_add_authorized_user() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.

        //Add one account_id using root_user
		assert_ok!(ProductModule::add_authorized_user(RuntimeOrigin::root(), 1));
        assert_eq!(ProductModule::get_authorized_user(1),true);

        // add onother account_id with existing user
		assert_ok!(ProductModule::add_authorized_user(RuntimeOrigin::signed(1), 2));
        assert_eq!(ProductModule::get_authorized_user(2),true);
        
        //fail case
        assert_eq!(ProductModule::get_authorized_user(3),false);
	});
}

#[test]
fn correct_error_for_none_value() {
	new_test_ext().execute_with(  || {
		
        // Ensure the expected error is thrown when no value is present.
		assert_ok!(ProductModule::add_authorized_user(RuntimeOrigin::root(), 1));

        let mut vec = BoundedVec::try_from(Vec::from("other")).unwrap();
        let p =  types::Product::<Test>::new(vec.clone(),4,1,);
        assert_ok!(ProductModule::add_product(RuntimeOrigin::signed(1), vec, 4));
        let savedP:Product<Test>=ProductModule::get_product_info(1).unwrap(); 
        
        
        assert_eq!(savedP, p);
        
	});
}
