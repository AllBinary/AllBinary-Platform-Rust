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
// package org.allbinary.thread
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use crate::Runnable;
use super::*;
use super::PriorityRunnable::PriorityRunnable;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct NullPriorityRunnable {
}

impl PriorityRunnable for NullPriorityRunnable {
    fn getPriority(&mut self) -> i32 { Self::getPriority(self) }
    fn isDone(&mut self) -> bool { Self::isDone(self) }
    fn reset(&mut self) { Self::reset(self) }
}
impl crate::Runnable for NullPriorityRunnable {
    fn run(&mut self) { Self::run(self) }
}

impl NullPriorityRunnable {

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getPriority(&mut self) -> i32 {
        return 0;
    }
    

    pub fn isDone(&mut self) -> bool {
        return true;
    }
    

    pub fn reset(&mut self) {
    }
    

    pub fn run(&mut self) {
    }
}

