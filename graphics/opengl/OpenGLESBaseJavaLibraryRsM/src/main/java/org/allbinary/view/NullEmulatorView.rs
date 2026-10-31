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
// package org.allbinary.view
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::javax::microedition::lcdui::Displayable::Displayable;
// use crate::javax::microedition::midlet::MIDlet::MIDlet;
use super::EmulatorViewInterface::EmulatorViewInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct NullEmulatorView {
}

thread_local! { pub static NullEmulatorView_NULL_EMULATOR_VIEW: Option<Rc<RefCell<NullEmulatorView>>> = NullEmulatorView::new(); }
impl EmulatorViewInterface for NullEmulatorView {
    fn setMidlet(&mut self, mut midlet: Option<Rc<RefCell<MIDlet>>>) { Self::setMidlet(self, midlet) }
    fn onEmulatorInitComplete(&mut self, mut midletActivity: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) { Self::onEmulatorInitComplete(self, midletActivity) }
    fn onSetDisplayable(&mut self, mut displayable: Option<Rc<RefCell<Displayable>>>) { Self::onSetDisplayable(self, displayable) }
}

impl NullEmulatorView {

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn setMidlet(&mut self, mut midlet: Option<Rc<RefCell<MIDlet>>>) {
    }
    

    pub fn onEmulatorInitComplete(&mut self, mut midletActivity: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) {
    }
    

    pub fn onSetDisplayable(&mut self, mut displayable: Option<Rc<RefCell<Displayable>>>) {
    }
}

