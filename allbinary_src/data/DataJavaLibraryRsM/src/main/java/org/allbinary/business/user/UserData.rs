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
thread_local! { pub static UserData_NAME: String = String::from("USER_NAME"); }
thread_local! { pub static UserData_USERNAME: String = String::from("CUSTOMER_USER_NAME"); }
thread_local! { pub static UserData_PASSWORD: String = String::from("CUSTOMER_PASSWORD"); }
thread_local! { pub static UserData_PREFIXNAME: String = String::from("CUSTOMER_PREFIX_NAME"); }
thread_local! { pub static UserData_FIRSTNAME: String = String::from("CUSTOMER_FIRST_NAME"); }
thread_local! { pub static UserData_LASTNAME: String = String::from("CUSTOMER_LAST_NAME"); }
thread_local! { pub static UserData_MIDDLENAME: String = String::from("CUSTOMER_MIDDLE_NAME"); }
thread_local! { pub static UserData_SUFFIXNAME: String = String::from("CUSTOMER_SUFFIX_NAME"); }
thread_local! { pub static UserData_COMPANY: String = String::from("CUSTOMER_COMPANY"); }
thread_local! { pub static UserData_POSITIONATCOMPANY: String = String::from("CUSTOMER_POSITION_AT_COMPANY"); }
thread_local! { pub static UserData_MAINEMAIL: String = String::from("CUSTOMER_MAIN_EMAIL"); }
thread_local! { pub static UserData_SECONDARYEMAIL: String = String::from("CUSTOMER_SECONDARY_EMAIL"); }
thread_local! { pub static UserData_HOMEPHONE: String = String::from("CUSTOMER_HOME_PHONE"); }
thread_local! { pub static UserData_CELLPHONE: String = String::from("CUSTOMER_CELL_PHONE"); }
thread_local! { pub static UserData_WORKPHONE: String = String::from("CUSTOMER_WORK_PHONE"); }
thread_local! { pub static UserData_OTHERCONTACT: String = String::from("CUSTOMER_OTHER_CONTACT"); }
thread_local! { pub static UserData_ELECTRONICDEVICE: String = String::from("CUSTOMER_ELECTRONIC_DEVICE"); }
thread_local! { pub static UserData_FAX: String = String::from("CUSTOMER_FAX"); }
thread_local! { pub static UserData_SECRET: String = String::from("CUSTOMER_SECRET"); }
thread_local! { pub static UserData_PERMISSIONS: String = String::from("CUSTOMER_PERMISSIONS"); }
thread_local! { pub static UserData_CONFIGURATION: String = String::from("CUSTOMER_CONFIGURATION"); }

impl UserData {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
}

