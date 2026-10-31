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
use std::collections::HashMap;
use super::*;
// use crate::java::util::HashMap::HashMap;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
use crate::org::allbinary::data::tables::TableMappingInterface::TableMappingInterface;
use super::PaymentGatewayInterface::PaymentGatewayInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct PaymentGatewayMapping {
    pub paymentGatewayInterface: Option<Rc<RefCell<dyn PaymentGatewayInterface>>>,
}

impl TableMappingInterface for PaymentGatewayMapping {
    fn getKey(&mut self) -> Option<Rc<RefCell<Box<dyn std::any::Any>>>> { Self::getKey(self) }
    fn toVector(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> { Self::toVector(self) }
    fn toHashMap(&mut self) -> Option<Rc<RefCell<HashMap>>> { Self::toHashMap(self) }
}

impl PaymentGatewayMapping {
    

    pub fn new_struct(mut paymentGatewayInterface: Option<Rc<RefCell<dyn PaymentGatewayInterface>>>) -> Self {
        let mut this = Self {
            paymentGatewayInterface: None,
        };
        this.paymentGatewayInterface = paymentGatewayInterface.clone();
        this
    }

    pub fn new(mut paymentGatewayInterface: Option<Rc<RefCell<dyn PaymentGatewayInterface>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(paymentGatewayInterface))))
    }
    

    // throws Exception
    pub fn toHashMap(&mut self) -> Option<Rc<RefCell<HashMap>>> {
        return jcall!(self.paymentGatewayInterface).toHashMap(false);
    }
    

    pub fn toVector(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        return jcall!(self.paymentGatewayInterface).toVector(false);
    }
    

    // throws Exception
    pub fn getKey(&mut self) -> Option<Rc<RefCell<Box<dyn std::any::Any>>>> {
        return (jcall!(self.paymentGatewayInterface).getKey()).clone() /* as Any */;
    }
}

