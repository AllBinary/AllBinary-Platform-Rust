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
// package org.allbinary.logic.communication.log.config.type
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct LogConfigType {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub name: String,
    pub description: String,
}

pub static mut LogConfigType_availableLogConfigTypes: std::cell::LazyCell<Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>> = std::cell::LazyCell::new(|| BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>));

impl LogConfigType {
    

    pub fn new_struct(mut name: String, mut description: String) -> Self {
        let mut this = Self {
            logUtil: None,
            name: String::new(),
            description: String::new(),
        };
        this.logUtil = LogUtil::getInstance();
        this.name = name.clone();
        this.description = description.clone();
        jcall!(unsafe { LogConfigType_availableLogConfigTypes.clone() }).add((|java_value: Option<Rc<RefCell<_>>>| java_value.map(|reference| Rc::new(RefCell::new(Box::new(Some(reference)) as Box<dyn std::any::Any>))))(this.clone()));
        this
    }

    pub fn new(mut name: String, mut description: String) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(name, description))))
    }
    

    pub fn getName(&mut self) -> String {
        return self.name.clone();
    }
    

    pub fn getDescription(&mut self) -> String {
        return self.description.clone();
    }
    

    pub fn setName(&mut self, mut value: String) {
        self.name = value.clone();
    }
    

    pub fn setDescription(&mut self, mut value: String) {
        self.description = value.clone();
    }
}

