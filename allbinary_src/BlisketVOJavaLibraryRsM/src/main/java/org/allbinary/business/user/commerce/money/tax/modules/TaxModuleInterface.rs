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
// package org.allbinary.business.user.commerce.money.tax.modules
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::business::context::modules::storefront::StoreFrontInterface::StoreFrontInterface;
use crate::org::allbinary::business::user::address::StreetAddress::StreetAddress;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub trait TaxModuleInterface {
    

    // throws Exception
    fn getTaxRate(&mut self, streetAddress: Option<Rc<RefCell<StreetAddress>>>, storeFrontInterface: Option<Rc<RefCell<dyn StoreFrontInterface>>>) -> Option<f32>;
    

    // throws Exception
    fn isValid(&mut self, streetAddress: Option<Rc<RefCell<StreetAddress>>>, storeFrontInterface: Option<Rc<RefCell<dyn StoreFrontInterface>>>) -> Option<bool>;

    fn toString(&mut self) -> String { String::from(std::any::type_name::<Self>()) }
}


