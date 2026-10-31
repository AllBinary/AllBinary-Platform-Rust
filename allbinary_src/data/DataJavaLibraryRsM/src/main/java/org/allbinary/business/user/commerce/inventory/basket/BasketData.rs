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
// package org.allbinary.business.user.commerce.inventory.basket
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct BasketData {
}

thread_local! { pub static BasketData_BASKET: String = String::from("BASKET"); }
thread_local! { pub static BasketData_NAME: String = String::from("NAME"); }
thread_local! { pub static BasketData_BASKETREVIEW: String = String::from("BASKET_REVIEW"); }
thread_local! { pub static BasketData_TOTALWEIGHT: String = String::from("BASKET_TOTALWEIGTH"); }
thread_local! { pub static BasketData_ITEMTOTAL: String = String::from("BASKET_ITEMTOTAL"); }
thread_local! { pub static BasketData_ITEMTOTALINBASKET: String = String::from("BASKET_ITEMTOTALINBASKET"); }
thread_local! { pub static BasketData_SUBTOTAL: String = String::from("BASKET_SUBTOTAL"); }
thread_local! { pub static BasketData_ISEMPTY: String = String::from("BASKET_ISEMPTY"); }
thread_local! { pub static BasketData_INSERT: String = String::from("BASKET_INSERT"); }
thread_local! { pub static BasketData_DELETE: String = String::from("BASKET_DELETE"); }
thread_local! { pub static BasketData_ADJUST: String = String::from("BASKET_ADJUST"); }

impl BasketData {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
}

