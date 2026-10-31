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
// package org.allbinary.business.user.commerce.money.payment.gateway
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

pub struct PaymentGatewayData {
    pub key: String,
}

thread_local! { pub static PaymentGatewayData_CRYPTNUM: i32 = 5; }
thread_local! { pub static PaymentGatewayData_ID: Option<Rc<RefCell<PaymentGatewayData>>> = PaymentGatewayData::new(String::from("PAYMENTGATEWAY_ID")); }
thread_local! { pub static PaymentGatewayData_NAME: Option<Rc<RefCell<PaymentGatewayData>>> = PaymentGatewayData::new(String::from("PAYMENTGATEWAY_NAME")); }
thread_local! { pub static PaymentGatewayData_VALUE: Option<Rc<RefCell<PaymentGatewayData>>> = PaymentGatewayData::new(String::from("PAYMENTGATEWAY_VALUE")); }
thread_local! { pub static PaymentGatewayData_PAYMENTTRANSACTIONINTERFACEFACTORYINTERFACE: Option<Rc<RefCell<PaymentGatewayData>>> = PaymentGatewayData::new(String::from("PAYMENTGATEWAY_PAYMENTTRANSACTIONINTERFACEFACTORYINTERFACE")); }
thread_local! { pub static PaymentGatewayData_PAYMENTPROCESSORINTERFACEFACTORYINTERFACE: Option<Rc<RefCell<PaymentGatewayData>>> = PaymentGatewayData::new(String::from("PAYMENTGATEWAY_PAYMENTPROCESSORINTERFACEFACTORYINTERFACE")); }
thread_local! { pub static PaymentGatewayData_MODE: Option<Rc<RefCell<PaymentGatewayData>>> = PaymentGatewayData::new(String::from("MODE")); }
thread_local! { pub static PaymentGatewayData_SERVERPROTOCOL: Option<Rc<RefCell<PaymentGatewayData>>> = PaymentGatewayData::new(String::from("SERVERPROTOCOL")); }
thread_local! { pub static PaymentGatewayData_SERVER: Option<Rc<RefCell<PaymentGatewayData>>> = PaymentGatewayData::new(String::from("SERVER")); }
thread_local! { pub static PaymentGatewayData_SERVERPORT: Option<Rc<RefCell<PaymentGatewayData>>> = PaymentGatewayData::new(String::from("SERVERPORT")); }
thread_local! { pub static PaymentGatewayData_SERVERPATH: Option<Rc<RefCell<PaymentGatewayData>>> = PaymentGatewayData::new(String::from("SERVERPATH")); }
thread_local! { pub static PaymentGatewayData_TESTPROTOCOL: Option<Rc<RefCell<PaymentGatewayData>>> = PaymentGatewayData::new(String::from("TESTSERVERPROTOCOL")); }
thread_local! { pub static PaymentGatewayData_TESTSERVER: Option<Rc<RefCell<PaymentGatewayData>>> = PaymentGatewayData::new(String::from("TESTSERVER")); }
thread_local! { pub static PaymentGatewayData_TESTPORT: Option<Rc<RefCell<PaymentGatewayData>>> = PaymentGatewayData::new(String::from("TESTPORT")); }
thread_local! { pub static PaymentGatewayData_TESTPATH: Option<Rc<RefCell<PaymentGatewayData>>> = PaymentGatewayData::new(String::from("TESTSERVERPATH")); }
thread_local! { pub static PaymentGatewayData_TIMEOUT: Option<Rc<RefCell<PaymentGatewayData>>> = PaymentGatewayData::new(String::from("TIMEOUT")); }
thread_local! { pub static PaymentGatewayData_PROXYPROTOCOL: Option<Rc<RefCell<PaymentGatewayData>>> = PaymentGatewayData::new(String::from("PROXYPROTOCOL")); }
thread_local! { pub static PaymentGatewayData_PROXYSERVER: Option<Rc<RefCell<PaymentGatewayData>>> = PaymentGatewayData::new(String::from("PROXYSERVER")); }
thread_local! { pub static PaymentGatewayData_PROXYPATH: Option<Rc<RefCell<PaymentGatewayData>>> = PaymentGatewayData::new(String::from("PROXYPATH")); }
thread_local! { pub static PaymentGatewayData_PROXYUSERNAME: Option<Rc<RefCell<PaymentGatewayData>>> = PaymentGatewayData::new(String::from("PROXYUSERNAME")); }
thread_local! { pub static PaymentGatewayData_PROXYPASSWORD: Option<Rc<RefCell<PaymentGatewayData>>> = PaymentGatewayData::new(String::from("PROXYPASSWORD")); }
thread_local! { pub static PaymentGatewayData_PROXYPORT: Option<Rc<RefCell<PaymentGatewayData>>> = PaymentGatewayData::new(String::from("PROXYPORT")); }
thread_local! { pub static PaymentGatewayData_PROXYTIMEOUT: Option<Rc<RefCell<PaymentGatewayData>>> = PaymentGatewayData::new(String::from("PROXYTIMEOUT")); }
thread_local! { pub static PaymentGatewayData_SPECIAL1: Option<Rc<RefCell<PaymentGatewayData>>> = PaymentGatewayData::new(String::from("SPECIAL1")); }
thread_local! { pub static PaymentGatewayData_SPECIAL2: Option<Rc<RefCell<PaymentGatewayData>>> = PaymentGatewayData::new(String::from("SPECIAL2")); }
thread_local! { pub static PaymentGatewayData_SPECIAL3: Option<Rc<RefCell<PaymentGatewayData>>> = PaymentGatewayData::new(String::from("SPECIAL3")); }
thread_local! { pub static PaymentGatewayData_SPECIAL4: Option<Rc<RefCell<PaymentGatewayData>>> = PaymentGatewayData::new(String::from("SPECIAL4")); }
thread_local! { pub static PaymentGatewayData_SPECIAL5: Option<Rc<RefCell<PaymentGatewayData>>> = PaymentGatewayData::new(String::from("SPECIAL5")); }
thread_local! { pub static PaymentGatewayData_SPECIAL6: Option<Rc<RefCell<PaymentGatewayData>>> = PaymentGatewayData::new(String::from("SPECIAL6")); }
thread_local! { pub static PaymentGatewayData_SPECIAL7: Option<Rc<RefCell<PaymentGatewayData>>> = PaymentGatewayData::new(String::from("SPECIAL7")); }
thread_local! { pub static PaymentGatewayData_SPECIAL8: Option<Rc<RefCell<PaymentGatewayData>>> = PaymentGatewayData::new(String::from("SPECIAL8")); }
thread_local! { pub static PaymentGatewayData_SPECIAL9: Option<Rc<RefCell<PaymentGatewayData>>> = PaymentGatewayData::new(String::from("SPECIAL9")); }

impl PaymentGatewayData {
    

    pub fn new_struct(mut value: String) -> Self {
        let mut this = Self {
            key: String::new(),
        };
        this.key = value.clone();
        this
    }

    pub fn new(mut value: String) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(value))))
    }
    

    pub fn toString(&mut self) -> String {
        return self.key.clone();
    }
}

