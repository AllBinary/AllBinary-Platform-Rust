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
use crate::org::allbinary::business::user::commerce::inventory::order::OrderInterface::OrderInterface;
use crate::org::allbinary::business::user::commerce::money::Money::Money;
use super::ShippingInterface::ShippingInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct UpsShippingModule {
    pub name: String,
    pub description: String,
}

impl ShippingInterface for UpsShippingModule {
    fn getCost(&mut self, mut order: Option<Rc<RefCell<dyn OrderInterface>>>) -> Option<Rc<RefCell<Money>>> { Self::getCost(self, order) }
    fn getName(&mut self) -> String { Self::getName(self) }
    fn getDescription(&mut self) -> String { Self::getDescription(self) }
}

impl UpsShippingModule {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            name: String::new(),
            description: String::new(),
        };
        this.name = String::from("UPS");
        this.description = String::from("Brown Mail");
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getCost(&mut self, mut order: Option<Rc<RefCell<dyn OrderInterface>>>) -> Option<Rc<RefCell<Money>>> {
        return Money::new();
    }
    

    pub fn getDescription(&mut self) -> String {
        return self.description.clone();
    }
    

    pub fn getName(&mut self) -> String {
        return self.name.clone();
    }
}

