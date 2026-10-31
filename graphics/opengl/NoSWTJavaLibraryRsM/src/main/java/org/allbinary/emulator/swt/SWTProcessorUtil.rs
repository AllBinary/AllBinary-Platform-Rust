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
// package org.allbinary.emulator.swt
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use super::SWTProcessor::SWTProcessor;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct SWTProcessorUtil {
    pub swtProcessor: Option<Rc<RefCell<dyn crate::org::allbinary::emulator::swt::SWTProcessor::SWTProcessorDyn>>>,
}

thread_local! { pub static SWTProcessorUtil_instance: Option<Rc<RefCell<SWTProcessorUtil>>> = SWTProcessorUtil::new(); }

impl SWTProcessorUtil {

    pub fn new_struct() -> Self {
        let mut this = Self {
            swtProcessor: None,
        };
        this.swtProcessor = SWTProcessor::getInstance();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<SWTProcessorUtil>>> {
        return SWTProcessorUtil_instance.with(Clone::clone).clone();
    }
}

