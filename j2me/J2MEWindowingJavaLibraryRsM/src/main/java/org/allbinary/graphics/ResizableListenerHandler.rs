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
// package org.allbinary.graphics
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use super::ResizableListenerInterface::ResizableListenerInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct ResizableListenerHandler {
}

thread_local! { pub static ResizableListenerHandler_instance: Option<Rc<RefCell<ResizableListenerHandler>>> = ResizableListenerHandler::new(); }

impl ResizableListenerHandler {

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<ResizableListenerHandler>>> {
        return ResizableListenerHandler_instance.with(Clone::clone).clone();
    }
    

    pub fn setListener(&mut self, mut resizableListenerInterface: Option<Rc<RefCell<dyn ResizableListenerInterface>>>) {
    }
    

    pub fn fire(&mut self, mut isFullScreen: bool) {
    }
    

    // throws Exception
    // synchronized
    pub fn fireEvent(&mut self, mut value: bool) {
    }
}

