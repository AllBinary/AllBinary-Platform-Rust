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

pub static PaymentGatewayData_CRYPTNUM: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 5);
pub static PaymentGatewayData_ID: crate::JavaStatic<Option<Rc<RefCell<PaymentGatewayData>>>> = crate::JavaStatic::new(|| PaymentGatewayData::new(String::from("PAYMENTGATEWAY_ID")));
pub static PaymentGatewayData_NAME: crate::JavaStatic<Option<Rc<RefCell<PaymentGatewayData>>>> = crate::JavaStatic::new(|| PaymentGatewayData::new(String::from("PAYMENTGATEWAY_NAME")));
pub static PaymentGatewayData_VALUE: crate::JavaStatic<Option<Rc<RefCell<PaymentGatewayData>>>> = crate::JavaStatic::new(|| PaymentGatewayData::new(String::from("PAYMENTGATEWAY_VALUE")));
pub static PaymentGatewayData_PAYMENTTRANSACTIONINTERFACEFACTORYINTERFACE: crate::JavaStatic<Option<Rc<RefCell<PaymentGatewayData>>>> = crate::JavaStatic::new(|| PaymentGatewayData::new(String::from("PAYMENTGATEWAY_PAYMENTTRANSACTIONINTERFACEFACTORYINTERFACE")));
pub static PaymentGatewayData_PAYMENTPROCESSORINTERFACEFACTORYINTERFACE: crate::JavaStatic<Option<Rc<RefCell<PaymentGatewayData>>>> = crate::JavaStatic::new(|| PaymentGatewayData::new(String::from("PAYMENTGATEWAY_PAYMENTPROCESSORINTERFACEFACTORYINTERFACE")));
pub static PaymentGatewayData_MODE: crate::JavaStatic<Option<Rc<RefCell<PaymentGatewayData>>>> = crate::JavaStatic::new(|| PaymentGatewayData::new(String::from("MODE")));
pub static PaymentGatewayData_SERVERPROTOCOL: crate::JavaStatic<Option<Rc<RefCell<PaymentGatewayData>>>> = crate::JavaStatic::new(|| PaymentGatewayData::new(String::from("SERVERPROTOCOL")));
pub static PaymentGatewayData_SERVER: crate::JavaStatic<Option<Rc<RefCell<PaymentGatewayData>>>> = crate::JavaStatic::new(|| PaymentGatewayData::new(String::from("SERVER")));
pub static PaymentGatewayData_SERVERPORT: crate::JavaStatic<Option<Rc<RefCell<PaymentGatewayData>>>> = crate::JavaStatic::new(|| PaymentGatewayData::new(String::from("SERVERPORT")));
pub static PaymentGatewayData_SERVERPATH: crate::JavaStatic<Option<Rc<RefCell<PaymentGatewayData>>>> = crate::JavaStatic::new(|| PaymentGatewayData::new(String::from("SERVERPATH")));
pub static PaymentGatewayData_TESTPROTOCOL: crate::JavaStatic<Option<Rc<RefCell<PaymentGatewayData>>>> = crate::JavaStatic::new(|| PaymentGatewayData::new(String::from("TESTSERVERPROTOCOL")));
pub static PaymentGatewayData_TESTSERVER: crate::JavaStatic<Option<Rc<RefCell<PaymentGatewayData>>>> = crate::JavaStatic::new(|| PaymentGatewayData::new(String::from("TESTSERVER")));
pub static PaymentGatewayData_TESTPORT: crate::JavaStatic<Option<Rc<RefCell<PaymentGatewayData>>>> = crate::JavaStatic::new(|| PaymentGatewayData::new(String::from("TESTPORT")));
pub static PaymentGatewayData_TESTPATH: crate::JavaStatic<Option<Rc<RefCell<PaymentGatewayData>>>> = crate::JavaStatic::new(|| PaymentGatewayData::new(String::from("TESTSERVERPATH")));
pub static PaymentGatewayData_TIMEOUT: crate::JavaStatic<Option<Rc<RefCell<PaymentGatewayData>>>> = crate::JavaStatic::new(|| PaymentGatewayData::new(String::from("TIMEOUT")));
pub static PaymentGatewayData_PROXYPROTOCOL: crate::JavaStatic<Option<Rc<RefCell<PaymentGatewayData>>>> = crate::JavaStatic::new(|| PaymentGatewayData::new(String::from("PROXYPROTOCOL")));
pub static PaymentGatewayData_PROXYSERVER: crate::JavaStatic<Option<Rc<RefCell<PaymentGatewayData>>>> = crate::JavaStatic::new(|| PaymentGatewayData::new(String::from("PROXYSERVER")));
pub static PaymentGatewayData_PROXYPATH: crate::JavaStatic<Option<Rc<RefCell<PaymentGatewayData>>>> = crate::JavaStatic::new(|| PaymentGatewayData::new(String::from("PROXYPATH")));
pub static PaymentGatewayData_PROXYUSERNAME: crate::JavaStatic<Option<Rc<RefCell<PaymentGatewayData>>>> = crate::JavaStatic::new(|| PaymentGatewayData::new(String::from("PROXYUSERNAME")));
pub static PaymentGatewayData_PROXYPASSWORD: crate::JavaStatic<Option<Rc<RefCell<PaymentGatewayData>>>> = crate::JavaStatic::new(|| PaymentGatewayData::new(String::from("PROXYPASSWORD")));
pub static PaymentGatewayData_PROXYPORT: crate::JavaStatic<Option<Rc<RefCell<PaymentGatewayData>>>> = crate::JavaStatic::new(|| PaymentGatewayData::new(String::from("PROXYPORT")));
pub static PaymentGatewayData_PROXYTIMEOUT: crate::JavaStatic<Option<Rc<RefCell<PaymentGatewayData>>>> = crate::JavaStatic::new(|| PaymentGatewayData::new(String::from("PROXYTIMEOUT")));
pub static PaymentGatewayData_SPECIAL1: crate::JavaStatic<Option<Rc<RefCell<PaymentGatewayData>>>> = crate::JavaStatic::new(|| PaymentGatewayData::new(String::from("SPECIAL1")));
pub static PaymentGatewayData_SPECIAL2: crate::JavaStatic<Option<Rc<RefCell<PaymentGatewayData>>>> = crate::JavaStatic::new(|| PaymentGatewayData::new(String::from("SPECIAL2")));
pub static PaymentGatewayData_SPECIAL3: crate::JavaStatic<Option<Rc<RefCell<PaymentGatewayData>>>> = crate::JavaStatic::new(|| PaymentGatewayData::new(String::from("SPECIAL3")));
pub static PaymentGatewayData_SPECIAL4: crate::JavaStatic<Option<Rc<RefCell<PaymentGatewayData>>>> = crate::JavaStatic::new(|| PaymentGatewayData::new(String::from("SPECIAL4")));
pub static PaymentGatewayData_SPECIAL5: crate::JavaStatic<Option<Rc<RefCell<PaymentGatewayData>>>> = crate::JavaStatic::new(|| PaymentGatewayData::new(String::from("SPECIAL5")));
pub static PaymentGatewayData_SPECIAL6: crate::JavaStatic<Option<Rc<RefCell<PaymentGatewayData>>>> = crate::JavaStatic::new(|| PaymentGatewayData::new(String::from("SPECIAL6")));
pub static PaymentGatewayData_SPECIAL7: crate::JavaStatic<Option<Rc<RefCell<PaymentGatewayData>>>> = crate::JavaStatic::new(|| PaymentGatewayData::new(String::from("SPECIAL7")));
pub static PaymentGatewayData_SPECIAL8: crate::JavaStatic<Option<Rc<RefCell<PaymentGatewayData>>>> = crate::JavaStatic::new(|| PaymentGatewayData::new(String::from("SPECIAL8")));
pub static PaymentGatewayData_SPECIAL9: crate::JavaStatic<Option<Rc<RefCell<PaymentGatewayData>>>> = crate::JavaStatic::new(|| PaymentGatewayData::new(String::from("SPECIAL9")));

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

