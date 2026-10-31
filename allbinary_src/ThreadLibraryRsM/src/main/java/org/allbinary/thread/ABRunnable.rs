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
// package org.allbinary.thread
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use crate::Runnable;
use super::*;
use crate::java::lang::Thread::*;
use super::RunnableInterface::RunnableInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct ABRunnable {
    pub running: bool,
}

impl RunnableInterface for ABRunnable {
    fn isRunning(&mut self) -> bool { Self::isRunning(self) }
    fn setRunning(&mut self, mut running: bool) { Self::setRunning(self, running) }
    fn setThread(&mut self, mut thread: Option<Rc<RefCell<Thread>>>) { Self::setThread(self, thread) }
}
impl crate::Runnable for ABRunnable {
    fn run(&mut self) { Self::run(self) }
}

impl ABRunnable {

    pub fn new_struct() -> Self {
        let mut this = Self {
            running: false,
        };
        this.running = false;
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn setThread(&mut self, mut thread: Option<Rc<RefCell<Thread>>>) {
    }
    

    // synchronized
    pub fn isRunning(&mut self) -> bool {
        return self.running.clone();
    }
    

    // synchronized
    pub fn setRunning(&mut self, mut running: bool) {
        self.running = running.clone();
    }
    

    pub fn run(&mut self) {
    }
    

    pub fn getType(&mut self) -> i32 {
        return -1;
    }
}
pub trait ABRunnableDyn {
    fn as_ABRunnable(&self) -> &ABRunnable;
    fn as_ABRunnable_mut(&mut self) -> &mut ABRunnable;
    fn setThread(&mut self, mut thread: Option<Rc<RefCell<Thread>>>) { ABRunnable::setThread(self.as_ABRunnable_mut(), thread) }
    fn isRunning(&mut self) -> bool { ABRunnable::isRunning(self.as_ABRunnable_mut()) }
    fn setRunning(&mut self, mut running: bool) { ABRunnable::setRunning(self.as_ABRunnable_mut(), running) }
    fn run(&mut self) { ABRunnable::run(self.as_ABRunnable_mut()) }
    fn getType(&mut self) -> i32 { ABRunnable::getType(self.as_ABRunnable_mut()) }
}
impl ABRunnableDyn for ABRunnable {
    fn as_ABRunnable(&self) -> &ABRunnable { self }
    fn as_ABRunnable_mut(&mut self) -> &mut ABRunnable { self }
}


