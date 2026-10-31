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
// package org.allbinary.business.user.commerce.money.payment.types
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;
use super::*;
// use crate::java::util::HashMap::HashMap;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
use crate::org::allbinary::business::user::commerce::money::payment::gateway::PaymentGatewayData::PaymentGatewayData;
use crate::org::allbinary::business::user::commerce::money::payment::gateway::processor::PaymentProcessorInterfaceFactoryInterface::PaymentProcessorInterfaceFactoryInterface;
use crate::org::allbinary::business::user::commerce::money::payment::gateway::transaction::PaymentTransactionInterfaceFactoryInterface::PaymentTransactionInterfaceFactoryInterface;
use crate::org::allbinary::logic::StdUtil::StdUtil;
use super::BasicPaymentType::BasicPaymentType;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct PaymentType {
    pub basicPaymentType: Option<Rc<RefCell<BasicPaymentType>>>,
    pub paymentTransactionInterfaceFactoryInterface: Option<Rc<RefCell<dyn PaymentTransactionInterfaceFactoryInterface>>>,
    pub paymentProcessorInterfaceFactoryInterface: Option<Rc<RefCell<dyn PaymentProcessorInterfaceFactoryInterface>>>,
}


impl PaymentType {
    

    pub fn new_struct1(mut basicPaymentType: Option<Rc<RefCell<BasicPaymentType>>>) -> Self {
        let mut this = Self {
            basicPaymentType: None,
            paymentTransactionInterfaceFactoryInterface: None,
            paymentProcessorInterfaceFactoryInterface: None,
        };
        this.basicPaymentType = basicPaymentType.clone();
        this.paymentTransactionInterfaceFactoryInterface = None;
        this.paymentProcessorInterfaceFactoryInterface = None;
        this
    }

    pub fn new1(mut basicPaymentType: Option<Rc<RefCell<BasicPaymentType>>>) -> Option<Rc<RefCell<Self>>> {
        let this = Some(Rc::new(RefCell::new(Self::new_struct1(basicPaymentType))));
            jcall!(PaymentTypeUtil::PaymentTypeUtil::getInstance()).add((|java_value: Option<Rc<RefCell<_>>>| java_value.map(|reference| Rc::new(RefCell::new(Box::new(Some(reference)) as Box<dyn std::any::Any>))))(this.clone()));
        this
    }
    

    pub fn new_struct3(mut basicPaymentType: Option<Rc<RefCell<BasicPaymentType>>>, mut paymentTransactionInterfaceFactoryInterface: Option<Rc<RefCell<dyn PaymentTransactionInterfaceFactoryInterface>>>, mut paymentProcessorInterfaceFactoryInterface: Option<Rc<RefCell<dyn PaymentProcessorInterfaceFactoryInterface>>>) -> Self {
        let mut this = Self {
            basicPaymentType: None,
            paymentTransactionInterfaceFactoryInterface: None,
            paymentProcessorInterfaceFactoryInterface: None,
        };
        this.basicPaymentType = basicPaymentType.clone();
        this.paymentTransactionInterfaceFactoryInterface = paymentTransactionInterfaceFactoryInterface.clone();
        this.paymentProcessorInterfaceFactoryInterface = paymentProcessorInterfaceFactoryInterface.clone();
        this
    }

    pub fn new3(mut basicPaymentType: Option<Rc<RefCell<BasicPaymentType>>>, mut paymentTransactionInterfaceFactoryInterface: Option<Rc<RefCell<dyn PaymentTransactionInterfaceFactoryInterface>>>, mut paymentProcessorInterfaceFactoryInterface: Option<Rc<RefCell<dyn PaymentProcessorInterfaceFactoryInterface>>>) -> Option<Rc<RefCell<Self>>> {
        let this = Some(Rc::new(RefCell::new(Self::new_struct3(basicPaymentType, paymentTransactionInterfaceFactoryInterface, paymentProcessorInterfaceFactoryInterface))));
            jcall!(PaymentTypeUtil::PaymentTypeUtil::getInstance()).add((|java_value: Option<Rc<RefCell<_>>>| java_value.map(|reference| Rc::new(RefCell::new(Box::new(Some(reference)) as Box<dyn std::any::Any>))))(this.clone()));
        this
    }
    

    // throws Exception
    pub fn getPaymentTransactionInterfaceFactoryInterface(&mut self) -> Option<Rc<RefCell<dyn PaymentTransactionInterfaceFactoryInterface>>> {
        if self.paymentTransactionInterfaceFactoryInterface.is_none() {
            panic!("{:?}", format!("{}: {:?}", "Exception", (String::from("PaymentTransactionInterfaceFactoryInterface is Null"))));
        }
        return self.paymentTransactionInterfaceFactoryInterface.clone();
    }
    

    // throws Exception
    pub fn getPaymentProcessorInterfaceFactoryInterface(&mut self) -> Option<Rc<RefCell<dyn PaymentProcessorInterfaceFactoryInterface>>> {
        if self.paymentProcessorInterfaceFactoryInterface.is_none() {
            panic!("{:?}", format!("{}: {:?}", "Exception", (String::from("PaymentProcessorInterfaceFactoryInterface is Null"))));
        }
        return self.paymentProcessorInterfaceFactoryInterface.clone();
    }
    

    pub fn toVector(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        let mut vector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        jcall!(vector).addAllList(jcall!(self.getBasicPaymentType()).toVector());
        if self.paymentTransactionInterfaceFactoryInterface.is_some() {
            jcall!(vector).add(crate::java_into_object(std::any::type_name_of_val(&self.paymentTransactionInterfaceFactoryInterface).to_string()));
        }
        if self.paymentProcessorInterfaceFactoryInterface.is_some() {
            jcall!(vector).add(crate::java_into_object(std::any::type_name_of_val(&self.paymentProcessorInterfaceFactoryInterface).to_string()));
        }
        return vector.clone();
    }
    

    pub fn toHashMap(&mut self) -> Option<Rc<RefCell<HashMap>>> {
        let mut hashMap: Option<Rc<RefCell<HashMap>>> = jcall!(StdUtil::getInstance()).createHashMap();
        jcall!(hashMap).putAll(jcall!(self.getBasicPaymentType()).toHashMap());
        if self.paymentTransactionInterfaceFactoryInterface.is_some() {
            jcall!(hashMap).put(jcall!(crate::org::allbinary::business::user::commerce::money::payment::gateway::PaymentGatewayData::PaymentGatewayData_PAYMENTTRANSACTIONINTERFACEFACTORYINTERFACE.with(Clone::clone)).toString(), std::any::type_name_of_val(&self.paymentTransactionInterfaceFactoryInterface).to_string());
        }
        if self.paymentProcessorInterfaceFactoryInterface.is_some() {
            jcall!(hashMap).put(jcall!(crate::org::allbinary::business::user::commerce::money::payment::gateway::PaymentGatewayData::PaymentGatewayData_PAYMENTPROCESSORINTERFACEFACTORYINTERFACE.with(Clone::clone)).toString(), std::any::type_name_of_val(&self.paymentProcessorInterfaceFactoryInterface).to_string());
        }
        return hashMap.clone();
    }
    

    pub fn getBasicPaymentType(&mut self) -> Option<Rc<RefCell<BasicPaymentType>>> {
        return self.basicPaymentType.clone();
    }
}

