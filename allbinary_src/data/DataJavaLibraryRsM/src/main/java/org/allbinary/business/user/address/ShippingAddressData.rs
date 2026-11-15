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

pub static ShippingAddressData_SETTOBILLINGADDRESS: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("Set To Billing Address"));
pub static ShippingAddressData_SELECT: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("Select Shipping Address"));
pub static ShippingAddressData_SELECTNAME: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("SELECTSHIPPINGADDRESS"));
pub static ShippingAddressData_MULTIPLE: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("SHIPPINGADDRESSES"));
pub static ShippingAddressData_SHIPPINGADDRESSES: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("SHIPPINGADDRESSES"));
pub static ShippingAddressData_SHIPPINGADDRESS: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("SHIPPINGADDRESS"));
pub static ShippingAddressData_ID: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("SHIPPINGADDRESS_ID"));
pub static ShippingAddressData_ADDRESS: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("SHIPPINGADDRESS_ADDRESS"));
pub static ShippingAddressData_NAME: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("SHIPPINGADDRESS_NAME"));
pub static ShippingAddressData_STREET: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("SHIPPINGADDRESS_STREET"));
pub static ShippingAddressData_CITY: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("SHIPPINGADDRESS_CITY"));
pub static ShippingAddressData_STATE: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("SHIPPINGADDRESS_STATE"));
pub static ShippingAddressData_CODE: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("SHIPPINGADDRESS_CODE"));
pub static ShippingAddressData_COUNTRY: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("SHIPPINGADDRESS_COUNTRY"));
pub static ShippingAddressData_TAX: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("SHIPPINGADDRESS_TAX"));

impl ShippingAddressData {

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

