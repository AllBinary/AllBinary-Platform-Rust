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
// package org.allbinary.business.user.commerce.inventory.item
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::business::user::address::StreetAddress::StreetAddress;
use super::ItemInterface::ItemInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub trait OrderItemInterface: ItemInterface {
    

    fn setGroup(&mut self, value: String);
    

    fn setTax(&mut self, value: String);
    

    fn setShippingType(&mut self, value: String);
    

    fn setShippingCost(&mut self, value: String);
    

    fn setShippingAddress(&mut self, value: Option<Rc<RefCell<StreetAddress>>>);
    

    fn setSpecial(&mut self, value: String);
    

    fn getGroup(&mut self) -> String;
    

    fn getTax(&mut self) -> String;
    

    fn getShippingType(&mut self) -> String;
    

    fn getShippingCost(&mut self) -> String;
    

    fn getShippingAddress(&mut self) -> Option<Rc<RefCell<StreetAddress>>>;
    

    fn getSpecial(&mut self) -> String;
}


