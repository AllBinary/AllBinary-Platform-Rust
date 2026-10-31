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
// package org.allbinary.game.resource
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

pub struct ResourceLoadingLevel {
    pub name: String,
    pub level: i32,
}


impl ResourceLoadingLevel {
    

    pub fn new_struct(mut name: String, mut level: i32) -> Self {
        let mut this = Self {
            name: String::new(),
            level: 0,
        };
        this.name = name.clone();
        this.level = level.clone();
        this
    }

    pub fn new(mut name: String, mut level: i32) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(name, level))))
    }
    

    pub fn getLevel(&mut self) -> i32 {
        return self.level.clone();
    }
    

    pub fn getName(&mut self) -> String {
        return self.name.clone();
    }
}

