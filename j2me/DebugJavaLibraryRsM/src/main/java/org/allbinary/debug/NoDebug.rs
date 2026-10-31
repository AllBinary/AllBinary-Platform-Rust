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
// package org.allbinary.debug
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use super::DebugInterface::DebugInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct NoDebug {
}

pub static mut NoDebug_SINGLETON: std::cell::LazyCell<Option<Rc<RefCell<dyn DebugInterface>>>> = std::cell::LazyCell::new(|| NoDebug::new().map(|reference| reference as Rc<RefCell<dyn DebugInterface>>));
impl DebugInterface for NoDebug {
    fn start(&mut self) { Self::start(self) }
    fn stop(&mut self) { Self::stop(self) }
    fn getStartTime(&mut self) -> i64 { Self::getStartTime(self) }
    fn getMaxTime(&mut self) -> i64 { Self::getMaxTime(self) }
    fn isRunning(&mut self) -> bool { Self::isRunning(self) }
}

impl NoDebug {

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<dyn DebugInterface>>> {
        return unsafe { NoDebug_SINGLETON.clone() }.clone();
    }
    

    pub fn start(&mut self) {
    }
    

    pub fn stop(&mut self) {
    }
    

    pub fn getMaxTime(&mut self) -> i64 {
        return 10000_i64;
    }
    

    pub fn getStartTime(&mut self) -> i64 {
        return 0_i64;
    }
    

    pub fn isRunning(&mut self) -> bool {
        return false;
    }
}

