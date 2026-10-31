// 
// AllBinary Open License Version 1 
// Copyright (c) 2025 AllBinary 
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

pub struct RatioProcessor {
}


impl RatioProcessor {

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn setCustom(&mut self, mut width: i32, mut height: i32, mut ratio: f32) {
    }
}
pub trait RatioProcessorDyn {
    fn as_RatioProcessor(&self) -> &RatioProcessor;
    fn as_RatioProcessor_mut(&mut self) -> &mut RatioProcessor;
    fn setCustom(&mut self, mut width: i32, mut height: i32, mut ratio: f32) { RatioProcessor::setCustom(self.as_RatioProcessor_mut(), width, height, ratio) }
    fn setCustom(&mut self, mut width: i32, mut height: i32, mut ratio: f32) { RatioProcessor::setCustom(self.as_RatioProcessor_mut(), width, height, ratio) }
}
impl RatioProcessorDyn for RatioProcessor {
    fn as_RatioProcessor(&self) -> &RatioProcessor { self }
    fn as_RatioProcessor_mut(&mut self) -> &mut RatioProcessor { self }
}


