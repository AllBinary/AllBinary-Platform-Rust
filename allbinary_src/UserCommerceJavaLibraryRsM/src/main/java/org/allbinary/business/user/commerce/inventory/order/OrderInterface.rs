//
// AllBinary Open License Version 1
// Copyright (c) 2011 AllBinary
// 
// By agreeing to this license you and any business entity you represent are
// legally bound to the AllBinary Open License Version 1 legal agreement.
// 
// You may obtain the AllBinary Open License Version 1 legal agreement from
// AllBinary or the root directory of AllBinary's AllBinary Platform repository.
// 
// Created By: Travis Berthelot
//  
// Generated Code Do Not Modify
// package org.allbinary.business.user.commerce.inventory.order
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::business::user::commerce::inventory::basket::BasketInterface::BasketInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub trait OrderInterface {
    

    fn setShippingMethod(&mut self, shippingType: String);
    

    fn setPaymentMethod(&mut self, value: String);
    

    fn setSpecial(&mut self, special: String);
    

    fn setStoreName(&mut self, value: String);
    

    fn setUserComments(&mut self, value: String);
    

    fn setUserCancelComments(&mut self, value: String);
    

    fn setStoreComments(&mut self, value: String);
    

    fn setStoreCancelComments(&mut self, value: String);
    

    fn getBasket(&mut self) -> Option<Rc<RefCell<dyn BasketInterface>>>;
    

    fn getShippingMethod(&mut self) -> String;
    

    fn getPaymentMethod(&mut self) -> String;
    

    fn getSpecial(&mut self) -> String;
    

    fn getId(&mut self) -> String;
    

    fn getStoreName(&mut self) -> String;
    

    fn getUserComments(&mut self) -> String;
    

    fn getUserCancelComments(&mut self) -> String;
    

    fn getStoreComments(&mut self) -> String;
    

    fn getStoreCancelComments(&mut self) -> String;

    fn toString(&mut self) -> String { String::from(std::any::type_name::<Self>()) }
}


