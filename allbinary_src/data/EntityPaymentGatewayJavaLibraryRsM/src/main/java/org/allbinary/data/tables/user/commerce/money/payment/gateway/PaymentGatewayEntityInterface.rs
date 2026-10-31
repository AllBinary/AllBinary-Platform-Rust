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
// package org.allbinary.data.tables.user.commerce.money.payment.gateway
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
use crate::org::allbinary::business::user::commerce::money::payment::gateway::PaymentGatewayInterface::PaymentGatewayInterface;
use crate::org::allbinary::business::user::commerce::money::payment::types::BasicPaymentType::BasicPaymentType;
use crate::org::allbinary::data::tables::BasicDataTableInterface::BasicDataTableInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub trait PaymentGatewayEntityInterface: BasicDataTableInterface {
    

    fn getPaymentGatewayInterface(&mut self, storeName: String, paymentType: Option<Rc<RefCell<BasicPaymentType>>>) -> Option<Rc<RefCell<dyn PaymentGatewayInterface>>>;
    

    fn findPaymentTypeVectorByStore(&mut self, storeName: String) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>;
    

    fn remove(&mut self, storeName: String, paymentType: Option<Rc<RefCell<BasicPaymentType>>>);
    

    fn add(&mut self, paymentGatewayInterface: Option<Rc<RefCell<dyn PaymentGatewayInterface>>>);
    

    fn update(&mut self, paymentGatewayInterface: Option<Rc<RefCell<dyn PaymentGatewayInterface>>>);
}


