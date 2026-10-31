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
// package org.allbinary.business.user.commerce.shipping.modules
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::business::user::commerce::inventory::basket::BasketInterface::BasketInterface;
use crate::org::allbinary::business::user::commerce::inventory::order::OrderInterface::OrderInterface;
use crate::org::allbinary::business::user::commerce::money::Money::Money;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use super::ShippingInterface::ShippingInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct BasicWeightShippingModule {
    pub commonStrings: Option<Rc<RefCell<CommonStrings>>>,
    pub name: String,
    pub description: String,
}

impl ShippingInterface for BasicWeightShippingModule {
    fn getCost(&mut self, mut order: Option<Rc<RefCell<dyn OrderInterface>>>) -> Option<Rc<RefCell<Money>>> { Self::getCost(self, order) }
    fn getName(&mut self) -> String { Self::getName(self) }
    fn getDescription(&mut self) -> String { Self::getDescription(self) }
}

impl BasicWeightShippingModule {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            commonStrings: None,
            name: String::new(),
            description: String::new(),
        };
        this.commonStrings = CommonStrings::getInstance();
        this.name = String::from("Basic Shipping (5 to 7 working days)");
        this.description = String::from("We will soon provide a variety of shipping methods.");
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getCost(&mut self, mut order: Option<Rc<RefCell<dyn OrderInterface>>>) -> Option<Rc<RefCell<Money>>> {
        let mut basket: Option<Rc<RefCell<dyn BasketInterface>>> = jcall!(order).getBasket();
        let mut money: Option<Rc<RefCell<Money>>> = Money::new();
        jcall!(money).add(jcall!(basket).getTotalWeight());
        jcall!(money).multiply(1);
        jcall!(money).add(String::from("3.50"));
        return money.clone();
    }
    

    pub fn getDescription(&mut self) -> String {
        return self.description.clone();
    }
    

    pub fn getName(&mut self) -> String {
        return self.name.clone();
    }
}
pub trait BasicWeightShippingModuleDyn: crate::org::allbinary::business::user::commerce::shipping::modules::ShippingInterface::ShippingInterface {
    fn as_BasicWeightShippingModule(&self) -> &BasicWeightShippingModule;
    fn as_BasicWeightShippingModule_mut(&mut self) -> &mut BasicWeightShippingModule;
}
impl BasicWeightShippingModuleDyn for BasicWeightShippingModule {
    fn as_BasicWeightShippingModule(&self) -> &BasicWeightShippingModule { self }
    fn as_BasicWeightShippingModule_mut(&mut self) -> &mut BasicWeightShippingModule { self }
}


