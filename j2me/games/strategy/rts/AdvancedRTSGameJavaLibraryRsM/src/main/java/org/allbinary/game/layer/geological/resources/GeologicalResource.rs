// 
// AllBinary Open License Version 1 
// Copyright (c) 2006 AllBinary 
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
// package org.allbinary.game.layer.geological.resources
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

pub struct GeologicalResource {
    pub total: i32,
    pub initTotal: i32,
}


impl GeologicalResource {
    

    pub fn new_struct(mut initTotal: i32) -> Self {
        let mut this = Self {
            total: 0,
            initTotal: 0,
        };
        this.initTotal = initTotal.clone();
        this.init();
        this
    }

    pub fn new(mut initTotal: i32) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(initTotal))))
    }
    

    pub fn init(&mut self) {
        self.setTotal(self.initTotal.clone());
    }
    

    pub fn add(&mut self, mut value: i32) {
        let mut max: i32 = i32::MAX.clone();
        if self.getTotal() + value > max {
            self.setTotal(max.clone());
        } else {
            { let java_arguments = (self.getTotal() + value,); self.setTotal(java_arguments.0) };
        }
    }
    

    pub fn remove(&mut self, mut value: i32) {
        if self.getTotal() - value < 0 {
            self.setTotal(0);
        } else {
            { let java_arguments = (self.getTotal() - value,); self.setTotal(java_arguments.0) };
        }
    }
    

    pub fn getTotal(&mut self) -> i32 {
        return self.total.clone();
    }
    

    pub fn setTotal(&mut self, mut total: i32) {
        self.total = total.clone();
    }
}

