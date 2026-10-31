// 
// AllBinary Open License Version 1 
// Copyright (c) 2022 AllBinary 
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
// package org.allbinary.graphics.threed
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

pub struct GLHolder {
    pub isCreated: bool,
}


impl GLHolder {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            isCreated: false,
        };
        this.isCreated = true;
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn runOnSurfaceCreated(&mut self, mut renderer: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) {
    }
    

    pub fn runOnSurfaceChanged(&mut self, mut renderer: Option<Rc<RefCell<Box<dyn std::any::Any>>>>, mut width: i32, mut height: i32) {
    }
    

    pub fn run(&mut self, mut renderer: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) {
    }
}

