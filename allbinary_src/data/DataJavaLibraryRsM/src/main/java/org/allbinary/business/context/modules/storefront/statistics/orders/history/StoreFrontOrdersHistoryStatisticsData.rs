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

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct StoreFrontOrdersHistoryStatisticsData {
    pub NAME: String,
    pub NUMBEROFORDERS: String,
    pub SUBTOTAL: String,
    pub SHIPPINGCOST: String,
    pub TAXES: String,
    pub TOTAL: String,
}

thread_local! { pub static StoreFrontOrdersHistoryStatisticsData_instance: Option<Rc<RefCell<StoreFrontOrdersHistoryStatisticsData>>> = StoreFrontOrdersHistoryStatisticsData::new(); }

impl StoreFrontOrdersHistoryStatisticsData {
    

    pub fn getInstance() -> Option<Rc<RefCell<StoreFrontOrdersHistoryStatisticsData>>> {
        return StoreFrontOrdersHistoryStatisticsData_instance.with(Clone::clone).clone();
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            NAME: String::new(),
            NUMBEROFORDERS: String::new(),
            SUBTOTAL: String::new(),
            SHIPPINGCOST: String::new(),
            TAXES: String::new(),
            TOTAL: String::new(),
        };
        this.NAME = String::from("STOREFRONT_ORDERS_HISTORY_STATISTICS_NAME");
        this.NUMBEROFORDERS = String::from("STOREFRONT_ORDERS_HISTORY_STATISTICS_NUMBER_OF_ORDERS");
        this.SUBTOTAL = String::from("STOREFRONT_ORDERS_HISTORY_STATISTICS_SUBTOTAL");
        this.SHIPPINGCOST = String::from("STOREFRONT_ORDERS_HISTORY_STATISTICS_SHIPPINGCOST");
        this.TAXES = String::from("STOREFRONT_ORDERS_HISTORY_STATISTICS_TAXES");
        this.TOTAL = String::from("STOREFRONT_ORDERS_HISTORY_STATISTICS_TOTAL");
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
}

