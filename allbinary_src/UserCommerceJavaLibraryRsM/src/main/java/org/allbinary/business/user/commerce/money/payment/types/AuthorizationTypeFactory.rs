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
use super::*;
use super::AuthorizationType::AuthorizationType;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct AuthorizationTypeFactory {
    pub NONE: Option<Rc<RefCell<AuthorizationType>>>,
    pub VOICE: Option<Rc<RefCell<AuthorizationType>>>,
    pub RETINAL: Option<Rc<RefCell<AuthorizationType>>>,
    pub IMAGE: Option<Rc<RefCell<AuthorizationType>>>,
    pub DNA: Option<Rc<RefCell<AuthorizationType>>>,
    pub PRINT: Option<Rc<RefCell<AuthorizationType>>>,
    pub CHIP: Option<Rc<RefCell<AuthorizationType>>>,
}

thread_local! { pub static AuthorizationTypeFactory_instance: Option<Rc<RefCell<AuthorizationTypeFactory>>> = AuthorizationTypeFactory::new(); }

impl AuthorizationTypeFactory {

    pub fn new_struct() -> Self {
        let mut this = Self {
            NONE: None,
            VOICE: None,
            RETINAL: None,
            IMAGE: None,
            DNA: None,
            PRINT: None,
            CHIP: None,
        };
        this.NONE = AuthorizationType::new(String::from("None"));
        this.VOICE = AuthorizationType::new(String::from("Voice"));
        this.RETINAL = AuthorizationType::new(String::from("Retinal"));
        this.IMAGE = AuthorizationType::new(String::from("Image"));
        this.DNA = AuthorizationType::new(String::from("DNA"));
        this.PRINT = AuthorizationType::new(String::from("Print"));
        this.CHIP = AuthorizationType::new(String::from("Chip"));
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<AuthorizationTypeFactory>>> {
        return AuthorizationTypeFactory_instance.with(Clone::clone).clone();
    }
}

