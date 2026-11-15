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
// package org.allbinary.business.user.commerce.money.payment.gateway.processor
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::business::user::commerce::money::payment::gateway::transaction::PaymentTransactionInterface::PaymentTransactionInterface;
use crate::org::allbinary::business::user::commerce::money::payment::gateway::transaction::PaymentTransactionInterfaceRequestFactory::PaymentTransactionInterfaceRequestFactory;
use crate::org::allbinary::business::user::commerce::money::payment::types::PaymentType::PaymentType;
use crate::org::allbinary::business::user::commerce::money::payment::types::PaymentTypeUtil::PaymentTypeUtil;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::string::StringValidationUtil::StringValidationUtil;
use crate::org::allbinary::logic::visual::transform::info::TransformInfoInterface::TransformInfoInterface;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::java::lang::Exception::*;
use super::PaymentProcessorInterfaceFactoryInterface::PaymentProcessorInterfaceFactoryInterface;
use super::PaymentProcessorInterface::PaymentProcessorInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct PaymentProcessorInterfaceFactory {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
}

pub static PaymentProcessorInterfaceFactory_instance: crate::JavaStatic<Option<Rc<RefCell<PaymentProcessorInterfaceFactory>>>> = crate::JavaStatic::new(|| PaymentProcessorInterfaceFactory::new());

impl PaymentProcessorInterfaceFactory {

    pub fn toString(&mut self) -> String {
        String::from(std::any::type_name::<Self>())
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<PaymentProcessorInterfaceFactory>>> {
        return PaymentProcessorInterfaceFactory_instance.with(Clone::clone).clone();
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
    // synchronized
    pub fn getInstance1(&mut self, mut transformInfoInterface: Option<Rc<RefCell<dyn TransformInfoInterface>>>) -> Option<Rc<RefCell<dyn PaymentProcessorInterface>>> {
        let java_try_result = match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut paymentTransactionInterface: Option<Rc<RefCell<dyn PaymentTransactionInterface>>> = jcall!(PaymentTransactionInterfaceRequestFactory::getInstance()).getInstance1(transformInfoInterface.clone());
            let mut gatewayName: String = jcall!(jcall!(paymentTransactionInterface).getOrderHistory()).getPaymentMethod();
            if !jcall!(StringValidationUtil::getInstance()).isEmpty(gatewayName.clone()) {
                let mut paymentType: Option<Rc<RefCell<PaymentType>>> = jcall!(PaymentTypeUtil::getInstance()).get(gatewayName.clone());
                let mut paymentProcessorInterfaceFactoryInterface: Option<Rc<RefCell<dyn PaymentProcessorInterfaceFactoryInterface>>> = jcall!(paymentType).getPaymentProcessorInterfaceFactoryInterface();
                return Some((jcall!(paymentProcessorInterfaceFactoryInterface).getInstance(transformInfoInterface.clone(), paymentTransactionInterface.clone())).clone());
            }
            panic!("{:?}", format!("{}: {:?}", "Exception", (String::from("Error Getting PaymentProcessorInterface"))));
        })) {
            Ok(java_return) => java_return,
            Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            if jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypes.clone(); java_field }).LOGGING.clone(); java_field }).contains({ let java_field = jcall!(jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypeFactory.clone(); java_field }).getInstance()).PAYMENTERROR.clone(); java_field }.clone()) {
                let mut commonStrings: Option<Rc<RefCell<CommonStrings>>> = CommonStrings::getInstance();
                jcall!(self.logUtil).put({ let java_field = jcall!(commonStrings).EXCEPTION.clone(); java_field }.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(commonStrings).GET_INSTANCE.clone(); java_field }.clone(), crate::java_into_object(e.clone()));
            }
            panic!("{:?}", e);
            None
            }
        };if let Some(java_return) = java_try_result { return java_return; }unreachable!("try statement")
    }
}

