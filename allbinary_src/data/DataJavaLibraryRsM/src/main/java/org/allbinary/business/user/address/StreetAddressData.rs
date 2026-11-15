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

pub struct StreetAddressData {
}

pub static StreetAddressData_MAX: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 255);
pub static StreetAddressData_MIN: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 0);
pub static StreetAddressData_FORM: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("FORM"));
pub static StreetAddressData_STATES: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("STATES"));
pub static StreetAddressData_NUMBEROFADDRESSES: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("STREETADDRESS_NUMBEROFADDRESSES"));
pub static StreetAddressData_ID: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("STREETADDRESS_ID"));
pub static StreetAddressData_DEFAULT: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("STREETADDRESS_DEFAULT_ADDRESS"));
pub static StreetAddressData_ADDRESS: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("STREETADDRESS_ADDRESS"));
pub static StreetAddressData_NAME: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("STREETADDRESS_NAME"));
pub static StreetAddressData_STREET: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("STREETADDRESS_STREET"));
pub static StreetAddressData_CITY: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("STREETADDRESS_CITY"));
pub static StreetAddressData_STATE: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("STREETADDRESS_STATE"));
pub static StreetAddressData_CODE: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("STREETADDRESS_CODE"));
pub static StreetAddressData_COUNTRY: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("STREETADDRESS_COUNTRY"));

impl StreetAddressData {

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

