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
use crate::Runnable;
use super::*;
use crate::org::allbinary::thread::NullRunnable::NullRunnable;
use crate::org::eclipse::swt::widgets::Display::Display;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct SWTProcessor {
    pub runnable: Option<Rc<RefCell<dyn Runnable>>>,
}

thread_local! { pub static SWTProcessor_instance: Option<Rc<RefCell<dyn SWTProcessorDyn>>> = SWTProcessor::new().map(|reference| reference as Rc<RefCell<dyn SWTProcessorDyn>>); }

impl SWTProcessor {
    

    pub fn getInstance() -> Option<Rc<RefCell<dyn SWTProcessorDyn>>> {
        return SWTProcessor_instance.with(Clone::clone).clone();
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            runnable: None,
        };
        this.runnable = NullRunnable::getInstance();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn process(&mut self, mut display: Option<Rc<RefCell<Display>>>) {
        jcall!(display).sleep();
    }
}
pub trait SWTProcessorDyn {
    fn as_SWTProcessor(&self) -> &SWTProcessor;
    fn as_SWTProcessor_mut(&mut self) -> &mut SWTProcessor;
    fn process(&mut self, mut display: Option<Rc<RefCell<Display>>>) { SWTProcessor::process(self.as_SWTProcessor_mut(), display) }
}
impl SWTProcessorDyn for SWTProcessor {
    fn as_SWTProcessor(&self) -> &SWTProcessor { self }
    fn as_SWTProcessor_mut(&mut self) -> &mut SWTProcessor { self }
}


