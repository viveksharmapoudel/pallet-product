use core::{ str::Bytes};

use crate::{mock::{*, self}, Error, types::{ self, Product, ProductPositionEnum}};
use frame_support::{ assert_ok, BoundedVec, assert_err};
use crate as pallet_product;


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
        let p =  types::Product::<Test>::new(vec.clone(),4,1, types::ProductPositionEnum::Manufacture,);
        assert_ok!(ProductModule::add_product(RuntimeOrigin::signed(1), vec, 4 ));
        let savedP:Product<Test>=ProductModule::get_product_info(1).unwrap(); 
        assert_eq!(savedP, p);
        
	});
}

#[test]
fn it_works_product_position_update(){

    new_test_ext().execute_with(||{
        assert_ok!(ProductModule::add_authorized_user(RuntimeOrigin::root(), 1));
        let mut vec = BoundedVec::try_from(Vec::from("other")).unwrap();
        
        assert_ok!(ProductModule::add_product(RuntimeOrigin::signed(1), vec, 4));

        assert_ok!(ProductModule::update_position(RuntimeOrigin::signed(1),1, ProductPositionEnum::Distribution));
        let mut p: Product<Test> = ProductModule::get_product_info(1).unwrap();
        assert_eq!(p.get_position(), ProductPositionEnum::Retailer);
    })
}

#[test]
fn it_works_buy_product(){
    minimal_test_ext().execute_with(||{
        //balance transfer to owner account just for testing 
        //bacause of existential deposit value 
        assert_ok!(
            <Test as pallet_product::Config>::Currency::set_balance(
                mock::RuntimeOrigin::root(),
                TEST_OWNER_ACCOUNT,
                20000,
                0u32.into(),
            )
        );

        //adding authorized user
        assert_ok!(ProductModule::add_authorized_user(RuntimeOrigin::root(), 1));

        let buyer=2;
     
        //add a test product
        assert_ok!(ProductModule::add_product(
            RuntimeOrigin::signed(1), 
            BoundedVec::try_from(Vec::from("other")).unwrap(), 
            20u32.into())
        );

        //updating position
        //sell_product is not allowed without position distribution
        assert_ok!(
            ProductModule::update_position(RuntimeOrigin::signed(1),
            1, 
            ProductPositionEnum::Distribution)
        );
        
        //set balance of buyer for payment 
        assert_ok!(
            <Test as pallet_product::Config>::Currency::set_balance(
                mock::RuntimeOrigin::root(),
                buyer,
                2000000,
                0u32.into(),
            )
        );
            
        assert_ok!(ProductModule::buy_product(RuntimeOrigin::signed(2), 1));

        assert_eq!(
            ProductModule::get_product_info(1).unwrap().get_owner(), 
            buyer
        );
    })
}


#[test]
fn it_works_update_resale(){
    minimal_test_ext().execute_with(||{

        //balance transfer to owner account just for testing 
        //bacause of existential deposit value 
        assert_ok!(
            <Test as pallet_product::Config>::Currency::set_balance(
                mock::RuntimeOrigin::root(),
                TEST_OWNER_ACCOUNT,
                20000,
                0u32.into(),
            )
        );

        //adding authorized user
        assert_ok!(ProductModule::add_authorized_user(RuntimeOrigin::root(), 1));

        //add a test product
        assert_ok!(ProductModule::add_product(
            RuntimeOrigin::signed(1), 
            BoundedVec::try_from(Vec::from("other")).unwrap(), 
            20u32.into())
        );
       
        //updating position
        //sell_product is not allowed without position distribution
        assert_ok!(
            ProductModule::update_position(RuntimeOrigin::signed(1),
            1, 
            ProductPositionEnum::Retailer)
        );
        
        let buyer=2;
    
        //set balance of buyer for payment 
        assert_ok!(
            <Test as pallet_product::Config>::Currency::set_balance(
                mock::RuntimeOrigin::root(),
                buyer,
                2000000,
                0u32.into(),
            )
        );
            
        assert_ok!(ProductModule::buy_product(RuntimeOrigin::signed(2), 1));

        assert_ok!(ProductModule::enable_resale(RuntimeOrigin::signed(2), 1, 100));

        let mut p:Product<Test>= ProductModule::get_product_info(1).unwrap();
        assert_eq!(p.get_price(), 100);
        assert_eq!(p.get_resale(), true);    

    })
}