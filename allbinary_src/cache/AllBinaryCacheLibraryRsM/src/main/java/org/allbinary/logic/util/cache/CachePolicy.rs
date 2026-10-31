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
// package org.allbinary.logic.util.cache
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

pub struct CachePolicy {
    pub name: String,
    pub maxTime: i64,
    pub maxSize: i64,
}


impl CachePolicy {
    

    pub fn new_struct(mut name: String, mut maxTime: i64, mut maxSize: i64) -> Self {
        let mut this = Self {
            name: String::new(),
            maxTime: 0_i64,
            maxSize: 0_i64,
        };
        this.name = name.clone();
        this.setMaxTime(maxTime.clone());
        this.setMaxSize(maxSize.clone());
        this
    }

    pub fn new(mut name: String, mut maxTime: i64, mut maxSize: i64) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(name, maxTime, maxSize))))
    }
    

    pub fn getMaxTime(&mut self) -> i64 {
        return self.maxTime.clone();
    }
    

    pub fn setMaxTime(&mut self, mut maxTime: i64) {
        self.maxTime = maxTime.clone();
    }
    

    pub fn getMaxSize(&mut self) -> i64 {
        return self.maxSize.clone();
    }
    

    pub fn setMaxSize(&mut self, mut maxSize: i64) {
        self.maxSize = maxSize.clone();
    }
    

    pub fn getName(&mut self) -> String {
        return self.name.clone();
    }
}

