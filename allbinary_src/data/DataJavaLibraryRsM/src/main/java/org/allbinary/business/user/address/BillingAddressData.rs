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

pub struct BillingAddressData {
}

thread_local! { pub static BillingAddressData_MAX: i32 = 255; }
thread_local! { pub static BillingAddressData_MIN: i32 = 0; }
thread_local! { pub static BillingAddressData_MULTIPLE: String = String::from("BILLINGADDRESSES"); }
thread_local! { pub static BillingAddressData_STATES: String = String::from("BILLINGADDRESS_STATES"); }
thread_local! { pub static BillingAddressData_SETTOSHIPPINGADDRESS: String = String::from("Set To Shipping Address"); }
thread_local! { pub static BillingAddressData_SELECT: String = String::from("Select Billing Address"); }
thread_local! { pub static BillingAddressData_SETTOSHIPPINGADDRESSNAME: String = String::from("SETTOSHIPPINGADDRESS"); }
thread_local! { pub static BillingAddressData_BILLINGADDRESSES: String = String::from("BILLINGADDRESSES"); }
thread_local! { pub static BillingAddressData_BILLINGADDRESS: String = String::from("BILLINGADDRESS"); }
thread_local! { pub static BillingAddressData_ID: String = String::from("BILLINGADDRESS_ID"); }
thread_local! { pub static BillingAddressData_ADDRESS: String = String::from("BILLINGADDRESS_ADDRESS"); }
thread_local! { pub static BillingAddressData_NAME: String = String::from("BILLINGADDRESS_NAME"); }
thread_local! { pub static BillingAddressData_STREET: String = String::from("BILLINGADDRESS_STREET"); }
thread_local! { pub static BillingAddressData_CITY: String = String::from("BILLINGADDRESS_CITY"); }
thread_local! { pub static BillingAddressData_STATE: String = String::from("BILLINGADDRESS_STATE"); }
thread_local! { pub static BillingAddressData_CODE: String = String::from("BILLINGADDRESS_CODE"); }
thread_local! { pub static BillingAddressData_COUNTRY: String = String::from("BILLINGADDRESS_COUNTRY"); }

impl BillingAddressData {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
}

