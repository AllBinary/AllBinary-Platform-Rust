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

pub static AuthorizeNetAimData_X_DELIM_DATA: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("x_delim_data"));
pub static AuthorizeNetAimData_X_DELIM_CHAR: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("x_delim_char"));
pub static AuthorizeNetAimData_X_ENCAP_CHAR: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("x_encap_char"));
pub static AuthorizeNetAimData_X_AUTHENTICATION_INDICATOR: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("x_authentication_indicator"));
pub static AuthorizeNetAimData_X_CARDHOLDER_AUTHENTICATION_VALUE: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("x_cardholder_authentication_value"));
pub static AuthorizeNetAimData_X_DRIVERS_LICENSE_NUM: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("x_drivers_license_num"));
pub static AuthorizeNetAimData_X_DRIVERS_LICENSE_STATE: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("x_drivers_license_state"));
pub static AuthorizeNetAimData_X_DRIVERS_LICENSE_DOB: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("x_drivers_license_dob"));

impl AuthorizeNetAimData {

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

