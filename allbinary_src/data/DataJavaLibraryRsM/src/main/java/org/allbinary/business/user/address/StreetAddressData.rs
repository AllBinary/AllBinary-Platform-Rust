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

thread_local! { pub static StreetAddressData_MAX: i32 = 255; }
thread_local! { pub static StreetAddressData_MIN: i32 = 0; }
thread_local! { pub static StreetAddressData_FORM: String = String::from("FORM"); }
thread_local! { pub static StreetAddressData_STATES: String = String::from("STATES"); }
thread_local! { pub static StreetAddressData_NUMBEROFADDRESSES: String = String::from("STREETADDRESS_NUMBEROFADDRESSES"); }
thread_local! { pub static StreetAddressData_ID: String = String::from("STREETADDRESS_ID"); }
thread_local! { pub static StreetAddressData_DEFAULT: String = String::from("STREETADDRESS_DEFAULT_ADDRESS"); }
thread_local! { pub static StreetAddressData_ADDRESS: String = String::from("STREETADDRESS_ADDRESS"); }
thread_local! { pub static StreetAddressData_NAME: String = String::from("STREETADDRESS_NAME"); }
thread_local! { pub static StreetAddressData_STREET: String = String::from("STREETADDRESS_STREET"); }
thread_local! { pub static StreetAddressData_CITY: String = String::from("STREETADDRESS_CITY"); }
thread_local! { pub static StreetAddressData_STATE: String = String::from("STREETADDRESS_STATE"); }
thread_local! { pub static StreetAddressData_CODE: String = String::from("STREETADDRESS_CODE"); }
thread_local! { pub static StreetAddressData_COUNTRY: String = String::from("STREETADDRESS_COUNTRY"); }

impl StreetAddressData {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
}

