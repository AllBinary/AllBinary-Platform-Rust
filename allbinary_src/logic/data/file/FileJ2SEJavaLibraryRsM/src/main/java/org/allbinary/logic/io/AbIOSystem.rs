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
// package org.allbinary.logic.io
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

pub struct AbIOSystem {
    pub r#type: String,
}

thread_local! { pub static AbIOSystem_instance: Option<Rc<RefCell<AbIOSystem>>> = AbIOSystem::new(); }

impl AbIOSystem {

    pub fn new_struct() -> Self {
        let mut this = Self {
            r#type: String::new(),
        };
        this.r#type = String::from("java.io");
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<AbIOSystem>>> {
        return AbIOSystem_instance.with(Clone::clone).clone();
    }
    

    pub fn getType(&mut self) -> String {
        return self.r#type.clone();
    }
    

    pub fn isType(&mut self, mut r#type: String) -> bool {
        if (self.r#type.cmp(&r#type.clone()) as i32) == 0 {
            return true;
        } else {
            return false;
        }
    }
}

