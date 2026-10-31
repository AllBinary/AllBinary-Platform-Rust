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
// package org.allbinary.graphics.displayable.screen
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::javax::microedition::lcdui::Displayable::Displayable;
use crate::org::allbinary::canvas::Processor::Processor;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct ScreenRepaintProcessor {
    pub base: Processor,
    pub displayable: Option<Rc<RefCell<Displayable>>>,
}


impl ScreenRepaintProcessor {
    

    pub fn new_struct(mut displayable: Option<Rc<RefCell<Displayable>>>) -> Self {
        let mut this = Self {
            base: Processor::new_struct(),
            displayable: None,
        };
        this.displayable = displayable.clone();
        this
    }

    pub fn new(mut displayable: Option<Rc<RefCell<Displayable>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(displayable))))
    }
    

    // throws Exception
    pub fn process(&mut self) {
        ScreenRepaintUtil::ScreenRepaintUtil::repaint(self.displayable.clone());
    }
}
impl std::ops::Deref for ScreenRepaintProcessor {
    type Target = Processor;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for ScreenRepaintProcessor {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::canvas::Processor::ProcessorDyn for ScreenRepaintProcessor {
    fn as_Processor(&self) -> &crate::org::allbinary::canvas::Processor::Processor { &self.base }
    fn as_Processor_mut(&mut self) -> &mut crate::org::allbinary::canvas::Processor::Processor { &mut self.base }
    fn process(&mut self) { Self::process(self) }
}


