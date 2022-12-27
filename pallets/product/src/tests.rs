use core::{ str::Bytes};

use crate::{mock::{*, self}, Error, types::{ProductName, self, Product, ProductPositionEnum}};
use frame_support::{assert_noop, assert_ok, BoundedVec, assert_err};
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
fn it_works_product_add() {
	new_test_ext().execute_with(  || {
		
        // Ensure the expected error is thrown when no value is present.
		assert_ok!(ProductModule::add_authorized_user(RuntimeOrigin::root(), 1));

        let mut vec = BoundedVec::try_from(Vec::from("other")).unwrap();
        let p =  types::Product::<Test>::new(vec.clone(),4,1,3, types::ProductPositionEnum::Manufacture);
        assert_ok!(ProductModule::add_product(RuntimeOrigin::signed(1), vec, 4, 3 ));
        let savedP:Product<Test>=ProductModule::get_product_info(1).unwrap(); 
        assert_eq!(savedP, p);
        
	});
}

#[test]
fn it_works_product_position_update(){

    new_test_ext().execute_with(||{
        assert_ok!(ProductModule::add_authorized_user(RuntimeOrigin::root(), 1));
        let mut vec = BoundedVec::try_from(Vec::from("other")).unwrap();
        
        assert_ok!(ProductModule::add_product(RuntimeOrigin::signed(1), vec, 4, 3 ));
        
        assert_ok!(ProductModule::update_position(RuntimeOrigin::signed(1),1, ProductPositionEnum::Distribution));
        let mut p: Product<Test> = ProductModule::get_product_info(1).unwrap();
        assert_eq!(p.get_position(), ProductPositionEnum::Distribution);
    })
}

#[test]
fn it_works_product_buy(){
    new_test_ext().execute_with(||{
        assert_ok!(ProductModule::add_authorized_user(RuntimeOrigin::root(), 1));
        let mut vec = BoundedVec::try_from(Vec::from("other")).unwrap();
        
        assert_ok!(ProductModule::add_product(RuntimeOrigin::signed(1), vec, 4, 1 ));

        assert_ok!(ProductModule::buy_product(RuntimeOrigin::signed(2), 1));
        let mut p: Product<Test> = ProductModule::get_product_info(1).unwrap();
        assert_eq!(p.get_count(), 0);
        assert_err!(ProductModule::buy_product(RuntimeOrigin::signed(2), 1), Error::<Test>::ProductFinished);
    })
}
