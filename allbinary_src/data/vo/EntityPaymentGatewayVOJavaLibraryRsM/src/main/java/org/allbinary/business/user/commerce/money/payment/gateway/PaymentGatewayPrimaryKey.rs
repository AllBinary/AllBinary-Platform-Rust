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
use crate::org::allbinary::business::context::modules::storefront::StoreFront::StoreFront;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::string::CommonSeps::CommonSeps;
use super::Serializable::Serializable;
use super::io::io;
use super::java::java;
use super::PaymentGateway::PaymentGateway;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct PaymentGatewayPrimaryKey {
    pub storeName: String,
    pub name: String,
}

// impl Serializable for PaymentGatewayPrimaryKey with the methods of the trait

impl PaymentGatewayPrimaryKey {
    

    pub fn new_struct(mut storeName: String, mut gatewayName: String) -> Self {
        let mut this = Self {
            storeName: String::new(),
            name: String::new(),
        };
        this.setStoreName(storeName.clone());
        this.setName(gatewayName.clone());
        this
    }

    pub fn new(mut storeName: String, mut gatewayName: String) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(storeName, gatewayName))))
    }
    

    pub fn getStoreName(&mut self) -> String {
        return self.storeName.clone();
    }
    

    pub fn setStoreName(&mut self, mut storeName: String) {
        self.storeName = storeName.clone();
    }
    

    pub fn getName(&mut self) -> String {
        return self.name.clone();
    }
    

    pub fn setName(&mut self, mut name: String) {
        self.name = name.clone();
    }
    

    pub fn toString(&mut self) -> String {
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        jcall!(stringBuffer).append(jcall!(std::any::type_name::<PaymentGatewayPrimaryKey>()).getName());
        jcall!(stringBuffer).append(jcall!(std::any::type_name::<StoreFront>()).getName());
        jcall!(stringBuffer).append(String::from(" ="));
        jcall!(stringBuffer).append(self.getStoreName());
        jcall!(stringBuffer).append(jcall!(CommonSeps::getInstance()).SPACE.clone());
        jcall!(stringBuffer).append(jcall!(std::any::type_name::<PaymentGateway>()).getName());
        jcall!(stringBuffer).append(String::from("="));
        jcall!(stringBuffer).append(self.getName());
        return jcall!(stringBuffer).toString();
    }
}

