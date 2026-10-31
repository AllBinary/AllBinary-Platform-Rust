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
// package org.allbinary.business.user.commerce.money.payment.gateway.modules.processor.authorizenet.aim
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

pub struct AuthorizeNetAimData {
}

thread_local! { pub static AuthorizeNetAimData_X_DELIM_DATA: String = String::from("x_delim_data"); }
thread_local! { pub static AuthorizeNetAimData_X_DELIM_CHAR: String = String::from("x_delim_char"); }
thread_local! { pub static AuthorizeNetAimData_X_ENCAP_CHAR: String = String::from("x_encap_char"); }
thread_local! { pub static AuthorizeNetAimData_X_AUTHENTICATION_INDICATOR: String = String::from("x_authentication_indicator"); }
thread_local! { pub static AuthorizeNetAimData_X_CARDHOLDER_AUTHENTICATION_VALUE: String = String::from("x_cardholder_authentication_value"); }
thread_local! { pub static AuthorizeNetAimData_X_DRIVERS_LICENSE_NUM: String = String::from("x_drivers_license_num"); }
thread_local! { pub static AuthorizeNetAimData_X_DRIVERS_LICENSE_STATE: String = String::from("x_drivers_license_state"); }
thread_local! { pub static AuthorizeNetAimData_X_DRIVERS_LICENSE_DOB: String = String::from("x_drivers_license_dob"); }

impl AuthorizeNetAimData {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
}

