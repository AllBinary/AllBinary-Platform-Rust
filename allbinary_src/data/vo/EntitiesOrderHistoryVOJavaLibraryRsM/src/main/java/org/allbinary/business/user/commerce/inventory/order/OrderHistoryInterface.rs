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
// package org.allbinary.business.user.commerce.inventory.order
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::business::user::address::StreetAddress::StreetAddress;
use crate::org::allbinary::business::user::commerce::money::Money::Money;
use crate::org::allbinary::business::user::commerce::money::payment::Payment::Payment;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub trait OrderHistoryInterface {
    

    fn setUserName(&mut self, value: String);
    

    fn setDefaultShippingAddress(&mut self, shippingAddress: Option<Rc<RefCell<StreetAddress>>>);
    

    fn setBillingAddress(&mut self, billingAddress: Option<Rc<RefCell<StreetAddress>>>);
    

    fn setPaymentInfo(&mut self, payment: Option<Rc<RefCell<Payment>>>);
    

    fn setShipDate(&mut self, value: String);
    

    fn setOrderDate(&mut self, value: String);
    

    fn setTransDate(&mut self, value: String);
    

    fn setCancelDate(&mut self, value: String);
    

    fn setStatus(&mut self, value: String);
    

    fn setSubTotal(&mut self, value: Option<Rc<RefCell<Money>>>);
    

    fn setShippingCost(&mut self, value: Option<Rc<RefCell<Money>>>);
    

    fn setTaxes(&mut self, value: Option<Rc<RefCell<Money>>>);
    

    fn setTotal(&mut self, value: Option<Rc<RefCell<Money>>>);
    

    fn getUserName(&mut self) -> String;
    

    fn getShippingAddress(&mut self) -> Option<Rc<RefCell<StreetAddress>>>;
    

    fn getBillingAddress(&mut self) -> Option<Rc<RefCell<StreetAddress>>>;
    

    fn getPaymentInfo(&mut self) -> Option<Rc<RefCell<Payment>>>;
    

    fn getShipDate(&mut self) -> String;
    

    fn getOrderDate(&mut self) -> String;
    

    fn getTransDate(&mut self) -> String;
    

    fn getCancelDate(&mut self) -> String;
    

    fn getStatus(&mut self) -> String;
    

    fn getSubTotal(&mut self) -> Option<Rc<RefCell<Money>>>;
    

    fn getShippingCost(&mut self) -> Option<Rc<RefCell<Money>>>;
    

    fn getTaxes(&mut self) -> Option<Rc<RefCell<Money>>>;
    

    fn getTotal(&mut self) -> Option<Rc<RefCell<Money>>>;
    

    fn getPaymentMethod(&mut self) -> String;

    fn toString(&mut self) -> String { String::from(std::any::type_name::<Self>()) }
}


