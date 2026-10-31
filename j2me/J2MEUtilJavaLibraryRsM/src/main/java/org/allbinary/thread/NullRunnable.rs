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
// package org.allbinary.thread
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::jsinterop::annotations::JsType;
use crate::jsinterop::annotations::JsMethod;
use crate::jsinterop::annotations::JsConstructor;
use super::ARunnable::ARunnable;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct NullRunnable {
    pub base: ARunnable,
}

thread_local! { pub static NullRunnable_instance: Option<Rc<RefCell<NullRunnable>>> = NullRunnable::new(); }

impl NullRunnable {
    

    pub fn getInstance() -> Option<Rc<RefCell<NullRunnable>>> {
        return NullRunnable_instance.with(Clone::clone).clone();
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: ARunnable::new_struct(),
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
}
impl std::ops::Deref for NullRunnable {
    type Target = ARunnable;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for NullRunnable {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::thread::ARunnable::ARunnableDyn for NullRunnable {
    fn as_ARunnable(&self) -> &crate::org::allbinary::thread::ARunnable::ARunnable { &self.base }
    fn as_ARunnable_mut(&mut self) -> &mut crate::org::allbinary::thread::ARunnable::ARunnable { &mut self.base }
}
impl crate::Runnable for NullRunnable {
    fn run(&mut self) { crate::org::allbinary::thread::ARunnable::ARunnable::run(crate::org::allbinary::thread::ARunnable::ARunnableDyn::as_ARunnable_mut(self)) }
}


