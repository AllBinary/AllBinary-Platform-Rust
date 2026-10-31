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
use std::collections::HashMap;
use super::*;
// use crate::java::util::HashMap::HashMap;
// use crate::java::util::Set::Set;
use crate::org::allbinary::business::user::commerce::money::Money::Money;
use crate::org::allbinary::business::user::commerce::money::MoneyException::MoneyException;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub trait BasketInterface {
    

    fn addItem(&mut self, id: String, num: String);
    

    fn removeItem(&mut self, id: String);
    

    fn adjustItem(&mut self, id: String, num: String);
    

    fn getTotalWeight(&mut self) -> String;
    

    fn getNumberOfItems(&mut self) -> Option<i32>;
    

    fn getIds(&mut self) -> Option<Rc<RefCell<dyn Set>>>;
    

    fn getItems(&mut self) -> Option<Rc<RefCell<HashMap>>>;
    

    fn getNumberOf(&mut self, product: String) -> Option<i32>;
    

    // throws MoneyException
    fn getSubTotal(&mut self) -> Option<Rc<RefCell<Money>>>;

    fn toString(&mut self) -> String { String::from(std::any::type_name::<Self>()) }
}


