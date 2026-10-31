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
// package org.allbinary.business.user.commerce.money.payment
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

pub struct PaymentData {
}

thread_local! { pub static PaymentData_MIN: i32 = 4; }
thread_local! { pub static PaymentData_MAX: i32 = 19; }
thread_local! { pub static PaymentData_ADDNEW: String = String::from("Add New Payment"); }
thread_local! { pub static PaymentData_VIEW: String = String::from("View Payment"); }
thread_local! { pub static PaymentData_SELECT: String = String::from("Select Payment"); }
thread_local! { pub static PaymentData_CHANGE: String = String::from("Change Payment"); }
thread_local! { pub static PaymentData_PAYMENT: String = String::from("PAYMENT"); }
thread_local! { pub static PaymentData_METHOD: String = String::from("PAYMENT_METHOD"); }
thread_local! { pub static PaymentData_ID: String = String::from("PAYMENT_ID"); }
thread_local! { pub static PaymentData_NUMBER: String = String::from("PAYMENT_NUMBER"); }
thread_local! { pub static PaymentData_NAME: String = String::from("PAYMENT_NAME"); }
thread_local! { pub static PaymentData_TYPE: String = String::from("PAYMENT_TYPE"); }
thread_local! { pub static PaymentData_EXPIRATION: String = String::from("PAYMENT_EXPIRATION"); }
thread_local! { pub static PaymentData_TENDERTYPE: String = String::from("PAYMENT_TENDERTYPE"); }
thread_local! { pub static PaymentData_TRANSACTIONTYPE: String = String::from("PAYMENT_TRANSACTIONTYPE"); }
thread_local! { pub static PaymentData_ABA: String = String::from("PAYMENT_ABA"); }
thread_local! { pub static PaymentData_ACCOUNT: String = String::from("PAYMENT_ACCOUNT"); }
thread_local! { pub static PaymentData_ACCOUNTTYPE: String = String::from("PAYMENT_ACCOUNTTYPE"); }
thread_local! { pub static PaymentData_AUTHORIZATIONCODE: String = String::from("PAYMENT_AUTHORIZATIONCODE"); }
thread_local! { pub static PaymentData_CHECKNUMBER: String = String::from("PAYMENT_CHECKNUMBER"); }
thread_local! { pub static PaymentData_CHECKTYPE: String = String::from("PAYMENT_CHECKTYPE"); }
thread_local! { pub static PaymentData_DRIVERSLICENSE: String = String::from("PAYMENT_DRIVERSLICENSE"); }
thread_local! { pub static PaymentData_MAGNETICINKCHECKREADER: String = String::from("PAYMENT_MAGNETICINKCHECKREADER"); }

impl PaymentData {

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
}

