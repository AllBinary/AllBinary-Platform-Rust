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

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct OrderHistoryData {
}

pub static mut OrderHistoryData_MAXIDLEN: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| 19);
thread_local! { pub static OrderHistoryData_DATEDELTA: String = String::from("DATE_DELTA"); }
thread_local! { pub static OrderHistoryData_DATETYPE: String = String::from("DATE_TYPE"); }
thread_local! { pub static OrderHistoryData_TYPELONG: String = String::from("TYPE_LONG"); }
thread_local! { pub static OrderHistoryData_TYPECAESAR: String = String::from("TYPE_CAESAR"); }
thread_local! { pub static OrderHistoryData_TODATE: String = String::from("TO_DATE"); }
thread_local! { pub static OrderHistoryData_FROMHOUR: String = String::from("FROM_HOUR"); }
thread_local! { pub static OrderHistoryData_FROMDAY: String = String::from("FROM_DAY"); }
thread_local! { pub static OrderHistoryData_FROMMONTH: String = String::from("FROM_MONTH"); }
thread_local! { pub static OrderHistoryData_FROMYEAR: String = String::from("FROM_YEAR"); }
thread_local! { pub static OrderHistoryData_TOHOUR: String = String::from("TO_HOUR"); }
thread_local! { pub static OrderHistoryData_TODAY: String = String::from("TO_DAY"); }
thread_local! { pub static OrderHistoryData_TOMONTH: String = String::from("TO_MONTH"); }
thread_local! { pub static OrderHistoryData_TOYEAR: String = String::from("TO_YEAR"); }
thread_local! { pub static OrderHistoryData_ORDERHISTORY: String = String::from("ORDERHISTORY"); }
thread_local! { pub static OrderHistoryData_SHIPPEDDATE: String = String::from("ORDERHISTORY_SHIPPED_DATE"); }
thread_local! { pub static OrderHistoryData_ORDERDATE: String = String::from("ORDERHISTORY_ORDER_DATE"); }
thread_local! { pub static OrderHistoryData_TRANSDATE: String = String::from("ORDERHISTORY_TRANS_DATE"); }
thread_local! { pub static OrderHistoryData_CANCELDATE: String = String::from("ORDERHISTORY_CANCEL_DATE"); }
thread_local! { pub static OrderHistoryData_SHIPPEDDATEFORMATTED: String = String::from("ORDERHISTORY_SHIPPED_DATE_FORMATTED"); }
thread_local! { pub static OrderHistoryData_ORDERDATEFORMATTED: String = String::from("ORDERHISTORY_ORDER_DATE_FORMATTED"); }
thread_local! { pub static OrderHistoryData_TRANSDATEFORMATTED: String = String::from("ORDERHISTORY_TRANS_DATE_FORMATTED"); }
thread_local! { pub static OrderHistoryData_CANCELDATEFORMATTED: String = String::from("ORDERHISTORY_CANCEL_DATE_FORMATTED"); }
thread_local! { pub static OrderHistoryData_SUBTOTAL: String = String::from("ORDERHISTORY_SUB_TOTAL"); }
thread_local! { pub static OrderHistoryData_SHIPPINGCOST: String = String::from("ORDERHISTORY_SHIPPING_COST"); }
thread_local! { pub static OrderHistoryData_TAX: String = String::from("ORDERHISTORY_TAX"); }
thread_local! { pub static OrderHistoryData_TOTAL: String = String::from("ORDERHISTORY_TOTAL"); }
thread_local! { pub static OrderHistoryData_CANCELINFO: String = String::from("ORDERHISTORY_CANCEL_INFO"); }
thread_local! { pub static OrderHistoryData_CANCELTYPE: String = String::from("ORDERHISTORY_CANCEL_TYPE"); }
thread_local! { pub static OrderHistoryData_STATUS: String = String::from("ORDERHISTORY_STATUS"); }
thread_local! { pub static OrderHistoryData_PREPROCESSINGNAME: String = String::from("PREPROCESSING"); }
thread_local! { pub static OrderHistoryData_PROCESSINGNAME: String = String::from("PROCESSING"); }
thread_local! { pub static OrderHistoryData_CANCELLEDNAME: String = String::from("CANCELLED"); }
thread_local! { pub static OrderHistoryData_PARTIALLYSHIPPEDNAME: String = String::from("PARTIALLYSHIPPED"); }
thread_local! { pub static OrderHistoryData_SHIPPEDNAME: String = String::from("SHIPPED"); }
thread_local! { pub static OrderHistoryData_PREPROCESSING: String = String::from("Order Preprocessing"); }
thread_local! { pub static OrderHistoryData_PROCESSING: String = String::from("Order Processing"); }
thread_local! { pub static OrderHistoryData_CANCELLED: String = String::from("Order Cancelled"); }
thread_local! { pub static OrderHistoryData_PARTIALLYSHIPPED: String = String::from("Order Partially Shipped"); }
thread_local! { pub static OrderHistoryData_SHIPPED: String = String::from("Order Shipped"); }
thread_local! { pub static OrderHistoryData_SETSTATUS: String = String::from("Set Status"); }

impl OrderHistoryData {

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
}

