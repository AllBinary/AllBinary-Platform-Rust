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

pub static BillingAddressData_MAX: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 255);
pub static BillingAddressData_MIN: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 0);
pub static BillingAddressData_MULTIPLE: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("BILLINGADDRESSES"));
pub static BillingAddressData_STATES: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("BILLINGADDRESS_STATES"));
pub static BillingAddressData_SETTOSHIPPINGADDRESS: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("Set To Shipping Address"));
pub static BillingAddressData_SELECT: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("Select Billing Address"));
pub static BillingAddressData_SETTOSHIPPINGADDRESSNAME: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("SETTOSHIPPINGADDRESS"));
pub static BillingAddressData_BILLINGADDRESSES: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("BILLINGADDRESSES"));
pub static BillingAddressData_BILLINGADDRESS: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("BILLINGADDRESS"));
pub static BillingAddressData_ID: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("BILLINGADDRESS_ID"));
pub static BillingAddressData_ADDRESS: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("BILLINGADDRESS_ADDRESS"));
pub static BillingAddressData_NAME: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("BILLINGADDRESS_NAME"));
pub static BillingAddressData_STREET: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("BILLINGADDRESS_STREET"));
pub static BillingAddressData_CITY: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("BILLINGADDRESS_CITY"));
pub static BillingAddressData_STATE: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("BILLINGADDRESS_STATE"));
pub static BillingAddressData_CODE: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("BILLINGADDRESS_CODE"));
pub static BillingAddressData_COUNTRY: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("BILLINGADDRESS_COUNTRY"));

impl BillingAddressData {

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

