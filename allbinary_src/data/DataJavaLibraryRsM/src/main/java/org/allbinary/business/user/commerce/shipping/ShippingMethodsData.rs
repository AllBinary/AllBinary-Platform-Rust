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
// package org.allbinary.business.user.commerce.shipping
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

pub struct ShippingMethodsData {
}

thread_local! { pub static ShippingMethodsData_SHIPPINGMETHODSORDERSUMMARIES: String = String::from("SHIPPINGMETHODS_ORDERSUMMARIES"); }
thread_local! { pub static ShippingMethodsData_SHIPPINGMETHODORDERSUMMARY: String = String::from("SHIPPINGMETHOD_ORDERSUMMARY"); }
thread_local! { pub static ShippingMethodsData_SHIPPINGMETHODS: String = String::from("SHIPPINGMETHODS"); }
thread_local! { pub static ShippingMethodsData_NAME: String = String::from("SHIPPINGMETHODS_NAME"); }
thread_local! { pub static ShippingMethodsData_SHIPPINGCOST: String = String::from("SHIPPINGMETHOD_SHIPPINGCOST"); }
thread_local! { pub static ShippingMethodsData_TAXRATE: String = String::from("SHIPPINGMETHOD_TAXRATE"); }
thread_local! { pub static ShippingMethodsData_SUBTOTAL: String = String::from("SHIPPINGMETHOD_SUBTOTAL"); }
thread_local! { pub static ShippingMethodsData_TAX: String = String::from("SHIPPINGMETHOD_TAX"); }
thread_local! { pub static ShippingMethodsData_TOTAL: String = String::from("SHIPPINGMETHOD_TOTAL"); }

impl ShippingMethodsData {

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
}

