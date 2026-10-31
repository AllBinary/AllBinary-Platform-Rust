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
// package org.allbinary.midlet
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::javax::microedition::midlet::MIDlet::MIDlet;
use crate::org::allbinary::AppletUtil::AppletUtil;
use crate::org::allbinary::J2MEUtil::J2MEUtil;
use crate::org::allbinary::canvas::Processor::Processor;
use super::MidletExitProcessor::MidletExitProcessor;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct MidletExitProcessorFactory {
}

thread_local! { pub static MidletExitProcessorFactory_instance: Option<Rc<RefCell<MidletExitProcessorFactory>>> = MidletExitProcessorFactory::new(); }

impl MidletExitProcessorFactory {

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<MidletExitProcessorFactory>>> {
        return MidletExitProcessorFactory_instance.with(Clone::clone).clone();
    }
    

    pub fn getExitInstance(&mut self, mut midlet: Option<Rc<RefCell<MIDlet>>>) -> Option<Rc<RefCell<dyn crate::org::allbinary::canvas::Processor::ProcessorDyn>>> {
        if AppletUtil::isAppletLoader(midlet.clone()) || J2MEUtil::isJ2ME() {
            return Processor::getInstance();
        } else {
            return MidletExitProcessor::new(midlet.clone()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::canvas::Processor::ProcessorDyn>>);
        }
    }
}

