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
// package org.allbinary.logic.system.security.licensing
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use super::LockableFeature::LockableFeature;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct InApplicationPurchaseFactory {
}

thread_local! { pub static InApplicationPurchaseFactory_instance: Option<Rc<RefCell<InApplicationPurchaseFactory>>> = InApplicationPurchaseFactory::new(); }

impl InApplicationPurchaseFactory {
    

    pub fn getInstance() -> Option<Rc<RefCell<InApplicationPurchaseFactory>>> {
        return InApplicationPurchaseFactory_instance.with(Clone::clone).clone();
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn init(&mut self, mut anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>>, mut stateObject: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) {
    }
    

    pub fn onCreate(&mut self, mut anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>>, mut stateObject: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) {
    }
    

    pub fn onStart(&mut self) {
    }
    

    pub fn onResult(&mut self, mut requestCode: i32, mut resultCode: i32, mut data: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) {
    }
    

    pub fn onSaveState(&mut self, mut anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) {
    }
    

    pub fn onStop(&mut self) {
    }
    

    pub fn onDestroy(&mut self) {
    }
    

    pub fn purchase(&mut self, mut lockableFeature: Option<Rc<RefCell<LockableFeature>>>) {
    }
    

    pub fn isPurchased(&mut self, mut lockableFeature: Option<Rc<RefCell<LockableFeature>>>) -> bool {
        return false;
    }
    

    pub fn add(&mut self, mut lockableFeature: Option<Rc<RefCell<LockableFeature>>>) {
    }
    

    pub fn getUserName(&mut self) -> String {
        return jcall!(StringUtil::getInstance()).EMPTY_STRING.clone();
    }
    

    pub fn isEnabled(&mut self) -> bool {
        return false;
    }
}

