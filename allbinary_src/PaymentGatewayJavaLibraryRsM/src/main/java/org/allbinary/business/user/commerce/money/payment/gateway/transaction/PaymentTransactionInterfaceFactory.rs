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
// package org.allbinary.business.user.commerce.money.payment.gateway.transaction
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::business::user::commerce::inventory::order::OrderHistory::OrderHistory;
use crate::org::allbinary::business::user::commerce::money::payment::types::PaymentType::PaymentType;
use crate::org::allbinary::business::user::commerce::money::payment::types::PaymentTypeUtil::PaymentTypeUtil;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use super::PaymentTransactionInterfaceFactoryInterface::PaymentTransactionInterfaceFactoryInterface;
use super::PaymentTransactionInterface::PaymentTransactionInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct PaymentTransactionInterfaceFactory {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
}

thread_local! { pub static PaymentTransactionInterfaceFactory_instance: Option<Rc<RefCell<PaymentTransactionInterfaceFactory>>> = PaymentTransactionInterfaceFactory::new(); }

impl PaymentTransactionInterfaceFactory {
    

    pub fn getInstance() -> Option<Rc<RefCell<PaymentTransactionInterfaceFactory>>> {
        return PaymentTransactionInterfaceFactory_instance.with(Clone::clone).clone();
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            logUtil: None,
        };
        this.logUtil = LogUtil::getInstance();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    // throws Exception
    // Java overloaded method: Rust needs a different name
    pub fn getInstance1(&mut self, mut orderReview: Option<Rc<RefCell<OrderHistory>>>) -> Option<Rc<RefCell<dyn PaymentTransactionInterface>>> {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut gatewayName: String = jcall!(orderReview).getPaymentMethod();
            if true {
                let mut paymentType: Option<Rc<RefCell<PaymentType>>> = jcall!(PaymentTypeUtil::getInstance()).get(gatewayName.clone());
                let mut paymentTransactionInterfaceFactoryInterface: Option<Rc<RefCell<dyn PaymentTransactionInterfaceFactoryInterface>>> = jcall!(paymentType).getPaymentTransactionInterfaceFactoryInterface();
                let mut paymentTransactionInterface: Option<Rc<RefCell<dyn PaymentTransactionInterface>>> = jcall!(paymentTransactionInterfaceFactoryInterface).getInstance(orderReview.clone());
                return paymentTransactionInterface.clone();
            }
            panic!("{:?}", format!("{}: {:?}", "Exception", (format!("{}{}", format!("{}{}", String::from("Failed to create PaymentTransactionInterface: "), String::from("GatewayName is: ")), gatewayName))));
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).PAYMENTERROR.clone()) {
                let mut commonStrings: Option<Rc<RefCell<CommonStrings>>> = CommonStrings::getInstance();
                jcall!(self.logUtil).put(String::from("Failed to generate test data"), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(commonStrings).GET_INSTANCE.clone(), crate::java_into_object(e.clone()));
            }
            panic!("{:?}", e);
            }
        }
    }
}

