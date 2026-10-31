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
// package org.allbinary.business.user.address
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

pub struct ShippingAddressData {
}

thread_local! { pub static ShippingAddressData_SETTOBILLINGADDRESS: String = String::from("Set To Billing Address"); }
thread_local! { pub static ShippingAddressData_SELECT: String = String::from("Select Shipping Address"); }
thread_local! { pub static ShippingAddressData_SELECTNAME: String = String::from("SELECTSHIPPINGADDRESS"); }
thread_local! { pub static ShippingAddressData_MULTIPLE: String = String::from("SHIPPINGADDRESSES"); }
thread_local! { pub static ShippingAddressData_SHIPPINGADDRESSES: String = String::from("SHIPPINGADDRESSES"); }
thread_local! { pub static ShippingAddressData_SHIPPINGADDRESS: String = String::from("SHIPPINGADDRESS"); }
thread_local! { pub static ShippingAddressData_ID: String = String::from("SHIPPINGADDRESS_ID"); }
thread_local! { pub static ShippingAddressData_ADDRESS: String = String::from("SHIPPINGADDRESS_ADDRESS"); }
thread_local! { pub static ShippingAddressData_NAME: String = String::from("SHIPPINGADDRESS_NAME"); }
thread_local! { pub static ShippingAddressData_STREET: String = String::from("SHIPPINGADDRESS_STREET"); }
thread_local! { pub static ShippingAddressData_CITY: String = String::from("SHIPPINGADDRESS_CITY"); }
thread_local! { pub static ShippingAddressData_STATE: String = String::from("SHIPPINGADDRESS_STATE"); }
thread_local! { pub static ShippingAddressData_CODE: String = String::from("SHIPPINGADDRESS_CODE"); }
thread_local! { pub static ShippingAddressData_COUNTRY: String = String::from("SHIPPINGADDRESS_COUNTRY"); }
thread_local! { pub static ShippingAddressData_TAX: String = String::from("SHIPPINGADDRESS_TAX"); }

impl ShippingAddressData {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
}

