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
// package org.allbinary.business.user
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

pub struct UserData {
}

pub static mut UserData_MAXLEN: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| 250);
pub static UserData_NAME: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("USER_NAME"));
pub static UserData_USERNAME: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("CUSTOMER_USER_NAME"));
pub static UserData_PASSWORD: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("CUSTOMER_PASSWORD"));
pub static UserData_PREFIXNAME: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("CUSTOMER_PREFIX_NAME"));
pub static UserData_FIRSTNAME: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("CUSTOMER_FIRST_NAME"));
pub static UserData_LASTNAME: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("CUSTOMER_LAST_NAME"));
pub static UserData_MIDDLENAME: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("CUSTOMER_MIDDLE_NAME"));
pub static UserData_SUFFIXNAME: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("CUSTOMER_SUFFIX_NAME"));
pub static UserData_COMPANY: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("CUSTOMER_COMPANY"));
pub static UserData_POSITIONATCOMPANY: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("CUSTOMER_POSITION_AT_COMPANY"));
pub static UserData_MAINEMAIL: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("CUSTOMER_MAIN_EMAIL"));
pub static UserData_SECONDARYEMAIL: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("CUSTOMER_SECONDARY_EMAIL"));
pub static UserData_HOMEPHONE: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("CUSTOMER_HOME_PHONE"));
pub static UserData_CELLPHONE: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("CUSTOMER_CELL_PHONE"));
pub static UserData_WORKPHONE: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("CUSTOMER_WORK_PHONE"));
pub static UserData_OTHERCONTACT: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("CUSTOMER_OTHER_CONTACT"));
pub static UserData_ELECTRONICDEVICE: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("CUSTOMER_ELECTRONIC_DEVICE"));
pub static UserData_FAX: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("CUSTOMER_FAX"));
pub static UserData_SECRET: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("CUSTOMER_SECRET"));
pub static UserData_PERMISSIONS: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("CUSTOMER_PERMISSIONS"));
pub static UserData_CONFIGURATION: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("CUSTOMER_CONFIGURATION"));

impl UserData {

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

