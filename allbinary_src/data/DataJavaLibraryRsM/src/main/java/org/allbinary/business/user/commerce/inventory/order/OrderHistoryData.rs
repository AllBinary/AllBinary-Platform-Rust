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
pub static OrderHistoryData_DATEDELTA: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("DATE_DELTA"));
pub static OrderHistoryData_DATETYPE: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("DATE_TYPE"));
pub static OrderHistoryData_TYPELONG: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("TYPE_LONG"));
pub static OrderHistoryData_TYPECAESAR: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("TYPE_CAESAR"));
pub static OrderHistoryData_TODATE: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("TO_DATE"));
pub static OrderHistoryData_FROMHOUR: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("FROM_HOUR"));
pub static OrderHistoryData_FROMDAY: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("FROM_DAY"));
pub static OrderHistoryData_FROMMONTH: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("FROM_MONTH"));
pub static OrderHistoryData_FROMYEAR: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("FROM_YEAR"));
pub static OrderHistoryData_TOHOUR: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("TO_HOUR"));
pub static OrderHistoryData_TODAY: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("TO_DAY"));
pub static OrderHistoryData_TOMONTH: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("TO_MONTH"));
pub static OrderHistoryData_TOYEAR: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("TO_YEAR"));
pub static OrderHistoryData_ORDERHISTORY: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("ORDERHISTORY"));
pub static OrderHistoryData_SHIPPEDDATE: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("ORDERHISTORY_SHIPPED_DATE"));
pub static OrderHistoryData_ORDERDATE: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("ORDERHISTORY_ORDER_DATE"));
pub static OrderHistoryData_TRANSDATE: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("ORDERHISTORY_TRANS_DATE"));
pub static OrderHistoryData_CANCELDATE: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("ORDERHISTORY_CANCEL_DATE"));
pub static OrderHistoryData_SHIPPEDDATEFORMATTED: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("ORDERHISTORY_SHIPPED_DATE_FORMATTED"));
pub static OrderHistoryData_ORDERDATEFORMATTED: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("ORDERHISTORY_ORDER_DATE_FORMATTED"));
pub static OrderHistoryData_TRANSDATEFORMATTED: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("ORDERHISTORY_TRANS_DATE_FORMATTED"));
pub static OrderHistoryData_CANCELDATEFORMATTED: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("ORDERHISTORY_CANCEL_DATE_FORMATTED"));
pub static OrderHistoryData_SUBTOTAL: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("ORDERHISTORY_SUB_TOTAL"));
pub static OrderHistoryData_SHIPPINGCOST: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("ORDERHISTORY_SHIPPING_COST"));
pub static OrderHistoryData_TAX: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("ORDERHISTORY_TAX"));
pub static OrderHistoryData_TOTAL: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("ORDERHISTORY_TOTAL"));
pub static OrderHistoryData_CANCELINFO: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("ORDERHISTORY_CANCEL_INFO"));
pub static OrderHistoryData_CANCELTYPE: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("ORDERHISTORY_CANCEL_TYPE"));
pub static OrderHistoryData_STATUS: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("ORDERHISTORY_STATUS"));
pub static OrderHistoryData_PREPROCESSINGNAME: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("PREPROCESSING"));
pub static OrderHistoryData_PROCESSINGNAME: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("PROCESSING"));
pub static OrderHistoryData_CANCELLEDNAME: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("CANCELLED"));
pub static OrderHistoryData_PARTIALLYSHIPPEDNAME: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("PARTIALLYSHIPPED"));
pub static OrderHistoryData_SHIPPEDNAME: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("SHIPPED"));
pub static OrderHistoryData_PREPROCESSING: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("Order Preprocessing"));
pub static OrderHistoryData_PROCESSING: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("Order Processing"));
pub static OrderHistoryData_CANCELLED: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("Order Cancelled"));
pub static OrderHistoryData_PARTIALLYSHIPPED: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("Order Partially Shipped"));
pub static OrderHistoryData_SHIPPED: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("Order Shipped"));
pub static OrderHistoryData_SETSTATUS: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("Set Status"));

impl OrderHistoryData {

    pub fn toString(&mut self) -> String {
        String::from(std::any::type_name::<Self>())
    }

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
}

