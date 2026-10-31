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

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct LogConfigType {
    pub name: String,
    pub description: String,
}


impl LogConfigType {
    

    pub fn new_struct(mut name: String, mut description: String) -> Self {
        let mut this = Self {
            name: String::new(),
            description: String::new(),
        };
        this.name = name.clone();
        this.description = description.clone();
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

