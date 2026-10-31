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
// package org.allbinary.logic.system.security.licensing.registration
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct RegistrationConfiguration {
    pub registrationCode: String,
    pub NAME: String,
}

thread_local! { pub static RegistrationConfiguration_SINGLETON: Option<Rc<RefCell<RegistrationConfiguration>>> = RegistrationConfiguration::new(); }

impl RegistrationConfiguration {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            registrationCode: String::new(),
            NAME: String::new(),
        };
        this.registrationCode = String::from("No Registration Code");
        this.NAME = String::from("registrationid");
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<RegistrationConfiguration>>> {
        return RegistrationConfiguration_SINGLETON.with(Clone::clone).clone();
    }
    

    // throws Exception
    pub fn read(&mut self) {
    }
    

    // throws Exception
    pub fn write(&mut self) {
    }
    

    pub fn toString(&mut self) -> String {
        return jcall!(StringMaker::new()).append(String::from("Registration Code: ")).append(self.getRegistrationCode()).toString();
    }
    

    pub fn setRegistrationCode(&mut self, mut registrationCode: String) {
        self.registrationCode = registrationCode.clone();
    }
    

    pub fn getRegistrationCode(&mut self) -> String {
        return self.registrationCode.clone();
    }
}

