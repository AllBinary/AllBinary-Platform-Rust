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
// package org.allbinary.business.context.modules.storefront.statistics.orders.history
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::business::user::commerce::money::Money::Money;
use crate::org::allbinary::data::tables::TableMappingInterface::TableMappingInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub trait StoreFrontOrdersHistoryStatisticsInterface: TableMappingInterface {
    

    fn getNumberOfOrders(&mut self) -> Option<i64>;
    

    fn getSubTotal(&mut self) -> Option<Rc<RefCell<Money>>>;
    

    fn getShippingCost(&mut self) -> Option<Rc<RefCell<Money>>>;
    

    fn getTaxes(&mut self) -> Option<Rc<RefCell<Money>>>;
    

    fn getTotal(&mut self) -> Option<Rc<RefCell<Money>>>;
}


