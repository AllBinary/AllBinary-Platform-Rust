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
// package org.allbinary.data.tables.user.commerce.inventory.order
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
use crate::org::allbinary::business::user::commerce::inventory::basket::BasketReview::BasketReview;
use crate::org::allbinary::business::user::commerce::inventory::order::OrderInterface::OrderInterface;
use crate::org::allbinary::data::tables::BasicDataTableInterface::BasicDataTableInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub trait OrderItemsEntityInterface: BasicDataTableInterface {
    

    fn setStatus(&mut self, orderId: String, groupId: String, status: String);
    

    fn isEverythingShipped(&mut self, orderId: String) -> bool;
    

    fn getBasketReview(&mut self, orderId: String) -> Option<Rc<RefCell<BasketReview>>>;
    

    fn insert2(&mut self, userName: String, order: Option<Rc<RefCell<dyn OrderInterface>>>);
    

    // Java overloaded method: Rust needs a different name
    fn insert(&mut self, values: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>);
}


