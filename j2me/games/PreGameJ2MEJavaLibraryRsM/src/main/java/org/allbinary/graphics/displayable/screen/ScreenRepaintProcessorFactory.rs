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
use crate::org::allbinary::J2MEUtil::J2MEUtil;
use crate::org::allbinary::canvas::Processor::Processor;
use super::ScreenRepaintProcessor::ScreenRepaintProcessor;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct ScreenRepaintProcessorFactory {
}

thread_local! { pub static ScreenRepaintProcessorFactory_instance: Option<Rc<RefCell<ScreenRepaintProcessorFactory>>> = ScreenRepaintProcessorFactory::new(); }

impl ScreenRepaintProcessorFactory {
    

    pub fn getInstance() -> Option<Rc<RefCell<ScreenRepaintProcessorFactory>>> {
        return ScreenRepaintProcessorFactory_instance.with(Clone::clone).clone();
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn create(&mut self, mut displayable: Option<Rc<RefCell<Displayable>>>) -> Option<Rc<RefCell<dyn crate::org::allbinary::canvas::Processor::ProcessorDyn>>> {
        if !J2MEUtil::isJ2ME() {
            return ScreenRepaintProcessor::new(displayable.clone()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::canvas::Processor::ProcessorDyn>>);
        } else {
            return Processor::getInstance();
        }
    }
}

