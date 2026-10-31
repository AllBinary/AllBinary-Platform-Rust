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
use super::*;
use super::ABRunnable::ABRunnable;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct NullABRunnable {
    pub base: ABRunnable,
}

thread_local! { pub static NullABRunnable_instance: Option<Rc<RefCell<NullABRunnable>>> = NullABRunnable::new(); }
impl crate::Runnable for NullABRunnable {
    fn run(&mut self) { self.base.run() }
}

impl NullABRunnable {

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: ABRunnable::new_struct(),
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<NullABRunnable>>> {
        return NullABRunnable_instance.with(Clone::clone).clone();
    }
    

    // synchronized
    pub fn isRunning(&mut self) -> bool {
        return false;
    }
}
impl std::ops::Deref for NullABRunnable {
    type Target = ABRunnable;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for NullABRunnable {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::thread::ABRunnable::ABRunnableDyn for NullABRunnable {
    fn as_ABRunnable(&self) -> &crate::org::allbinary::thread::ABRunnable::ABRunnable { &self.base }
    fn as_ABRunnable_mut(&mut self) -> &mut crate::org::allbinary::thread::ABRunnable::ABRunnable { &mut self.base }
    fn isRunning(&mut self) -> bool { Self::isRunning(self) }
}


