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
use crate::android::os::Debug::Debug;
use super::DebugInterface::DebugInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct AndroidDebug {
    pub startTime: i64,
    pub running: bool,
    pub bufferSize: i32,
}

impl DebugInterface for AndroidDebug {
    fn start(&mut self) { Self::start(self) }
    fn stop(&mut self) { Self::stop(self) }
    fn getStartTime(&mut self) -> i64 { Self::getStartTime(self) }
    fn getMaxTime(&mut self) -> i64 { Self::getMaxTime(self) }
    fn isRunning(&mut self) -> bool { Self::isRunning(self) }
}

impl AndroidDebug {

    pub fn new_struct() -> Self {
        let mut this = Self {
            startTime: 0_i64,
            running: false,
            bufferSize: 0,
        };
        this.startTime = i64::MAX.clone();
        this.running = false;
        this.bufferSize = 32 * 1024 * 1024;
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn start(&mut self) {
        { let java_arguments = ((std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis() as i64),); self.setStartTime(java_arguments.0) };
        Debug::startMethodTracing(String::from("trace"), self.bufferSize.clone());
        self.setRunning(true);
    }
    

    pub fn stop(&mut self) {
        self.startTime = i64::MAX.clone();
        Debug::stopMethodTracing();
        self.setRunning(false);
    }
    

    pub fn getMaxTime(&mut self) -> i64 {
        return 120000_i64;
    }
    

    pub fn setStartTime(&mut self, mut startTime: i64) {
        self.startTime = startTime.clone();
    }
    

    pub fn getStartTime(&mut self) -> i64 {
        return self.startTime.clone();
    }
    

    pub fn setRunning(&mut self, mut running: bool) {
        self.running = running.clone();
    }
    

    pub fn isRunning(&mut self) -> bool {
        return self.running.clone();
    }
}

