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
use super::ScreenEmptyListener::ScreenEmptyListener;
use super::ScreenListenerInterface::ScreenListenerInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct ScreenListenerHandler {
    pub screenListenerInterface: Option<Rc<RefCell<dyn ScreenListenerInterface>>>,
}

thread_local! { pub static ScreenListenerHandler_instance: Option<Rc<RefCell<ScreenListenerHandler>>> = ScreenListenerHandler::new(); }

impl ScreenListenerHandler {

    pub fn new_struct() -> Self {
        let mut this = Self {
            screenListenerInterface: None,
        };
        this.screenListenerInterface = ScreenEmptyListener::new().map(|reference| reference as Rc<RefCell<dyn ScreenListenerInterface>>);
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<ScreenListenerHandler>>> {
        return ScreenListenerHandler_instance.with(Clone::clone).clone();
    }
    

    pub fn setListener(&mut self, mut screenListenerInterface: Option<Rc<RefCell<dyn ScreenListenerInterface>>>) {
        self.screenListenerInterface = screenListenerInterface.clone();
    }
    

    pub fn fireWithState(&mut self, mut isFullScreen: bool) {
        jcall!(self.screenListenerInterface).onFullScreen(isFullScreen.clone());
    }
    

    pub fn fire(&mut self) {
        jcall!(self.screenListenerInterface).onFullScreenDisplay();
    }
}

