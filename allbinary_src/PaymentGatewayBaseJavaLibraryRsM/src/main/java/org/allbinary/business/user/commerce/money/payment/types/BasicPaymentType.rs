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
use crate::org::allbinary::business::user::commerce::money::payment::gateway::PaymentGatewayInterfaceFactoryInterface::PaymentGatewayInterfaceFactoryInterface;
use crate::org::allbinary::business::user::commerce::money::payment::gateway::modules::gateway::PaymentGatewayDomNodeFactoryInterface::PaymentGatewayDomNodeFactoryInterface;
use crate::org::allbinary::logic::StdUtil::StdUtil;
use crate::org::allbinary::logic::io::path::AbPathData::AbPathData;
use crate::org::allbinary::logic::string::regex::replace::Replace::Replace;
use crate::org::allbinary::string::CommonSeps::CommonSeps;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct BasicPaymentType {
    pub name: String,
    pub value: String,
    pub paymentGatewayInterfaceFactoryInterface: Option<Rc<RefCell<dyn PaymentGatewayInterfaceFactoryInterface>>>,
    pub paymentGatewayDomNodeFactoryInterface: Option<Rc<RefCell<dyn PaymentGatewayDomNodeFactoryInterface>>>,
}


impl BasicPaymentType {
    

    pub fn new_struct1(mut paymentMethod: String) -> Self {
        let mut this = Self {
            name: String::new(),
            value: String::new(),
            paymentGatewayInterfaceFactoryInterface: None,
            paymentGatewayDomNodeFactoryInterface: None,
        };
        this.init(paymentMethod.clone());
        this
    }

    pub fn new1(mut paymentMethod: String) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct1(paymentMethod))))
    }
    

    pub fn new_struct3(mut paymentMethod: String, mut paymentGatewayInterfaceFactoryInterface: Option<Rc<RefCell<dyn PaymentGatewayInterfaceFactoryInterface>>>, mut paymentGatewayDomNodeFactoryInterface: Option<Rc<RefCell<dyn PaymentGatewayDomNodeFactoryInterface>>>) -> Self {
        let mut this = Self {
            name: String::new(),
            value: String::new(),
            paymentGatewayInterfaceFactoryInterface: None,
            paymentGatewayDomNodeFactoryInterface: None,
        };
        this.init(paymentMethod.clone());
        this.paymentGatewayInterfaceFactoryInterface = paymentGatewayInterfaceFactoryInterface.clone();
        this.paymentGatewayDomNodeFactoryInterface = paymentGatewayDomNodeFactoryInterface.clone();
        this
    }

    pub fn new3(mut paymentMethod: String, mut paymentGatewayInterfaceFactoryInterface: Option<Rc<RefCell<dyn PaymentGatewayInterfaceFactoryInterface>>>, mut paymentGatewayDomNodeFactoryInterface: Option<Rc<RefCell<dyn PaymentGatewayDomNodeFactoryInterface>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct3(paymentMethod, paymentGatewayInterfaceFactoryInterface, paymentGatewayDomNodeFactoryInterface))))
    }
    

    pub fn init(&mut self, mut paymentMethod: String) {
        let mut hashMap: Option<Rc<RefCell<HashMap>>> = jcall!(StdUtil::getInstance()).createHashMap();
        let mut UNDERSCORE: String = String::from("_");
        jcall!(hashMap).put(jcall!(CommonSeps::getInstance()).SPACE.clone(), UNDERSCORE.clone());
        jcall!(hashMap).put(jcall!(AbPathData::getInstance()).EXTENSION_SEP.clone(), UNDERSCORE.clone());
        jcall!(hashMap).put(String::from("-"), UNDERSCORE.clone());
        let mut replace: Option<Rc<RefCell<Replace>>> = Replace::new1(hashMap.clone());
        self.name = jcall!(replace).all(paymentMethod.clone());
        self.value = paymentMethod.clone();
        jcall!(BasicPaymentTypeUtil::BasicPaymentTypeUtil::getInstance()).add(None /* Java this: a &mut self method has no Rc of itself */);
    }
    

    pub fn getName(&mut self) -> String {
        return self.name.clone();
    }
    

    pub fn getValue(&mut self) -> String {
        return self.value.clone();
    }
    

    // throws Exception
    pub fn getPaymentGatewayInterfaceFactoryInterface(&mut self) -> Option<Rc<RefCell<dyn PaymentGatewayInterfaceFactoryInterface>>> {
        if self.paymentGatewayInterfaceFactoryInterface.is_none() {
            panic!("{:?}", format!("{}: {:?}", "Exception", (String::from("PaymentGatewayInterfaceFactoryInterface is Null"))));
        }
        return self.paymentGatewayInterfaceFactoryInterface.clone();
    }
    

    // throws Exception
    pub fn getPaymentGatewayDomNodeFactoryInterface(&mut self) -> Option<Rc<RefCell<dyn PaymentGatewayDomNodeFactoryInterface>>> {
        if self.paymentGatewayDomNodeFactoryInterface.is_none() {
            panic!("{:?}", format!("{}: {:?}", "Exception", (String::from("PaymentGatewayDomNodeFactoryInterface is Null"))));
        }
        return self.paymentGatewayDomNodeFactoryInterface.clone();
    }
    

    pub fn getKey(&mut self) -> Option<Rc<RefCell<Box<dyn std::any::Any>>>> {
        return self.getName();
    }
    

    pub fn toVector(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        let mut vector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        jcall!(vector).add(crate::java_into_object(self.name.clone()));
        jcall!(vector).add(crate::java_into_object(self.value.clone()));
        return vector.clone();
    }
    

    pub fn toHashMap(&mut self) -> Option<Rc<RefCell<HashMap>>> {
        let mut hashMap: Option<Rc<RefCell<HashMap>>> = jcall!(StdUtil::getInstance()).createHashMap();
        jcall!(hashMap).put(jcall!(crate::org::allbinary::business::user::commerce::money::payment::gateway::PaymentGatewayData::PaymentGatewayData_NAME.with(Clone::clone)).toString(), self.name.clone());
        jcall!(hashMap).put(jcall!(crate::org::allbinary::business::user::commerce::money::payment::gateway::PaymentGatewayData::PaymentGatewayData_VALUE.with(Clone::clone)).toString(), self.value.clone());
        return hashMap.clone();
    }
    

    pub fn toString(&mut self) -> String {
        return self.name.clone();
    }
}

