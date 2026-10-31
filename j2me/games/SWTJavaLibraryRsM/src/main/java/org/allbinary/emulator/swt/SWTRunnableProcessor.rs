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
use crate::org::eclipse::swt::widgets::Display::Display;
use super::SWTProcessor::SWTProcessor;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct SWTRunnableProcessor {
    pub base: SWTProcessor,
}

thread_local! { pub static SWTRunnableProcessor_instanceR: Option<Rc<RefCell<SWTRunnableProcessor>>> = SWTRunnableProcessor::new(); }

impl SWTRunnableProcessor {
    

    pub fn getInstance() -> Option<Rc<RefCell<SWTRunnableProcessor>>> {
        return SWTRunnableProcessor_instanceR.with(Clone::clone).clone();
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: SWTProcessor::new_struct(),
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn process(&mut self, mut display: Option<Rc<RefCell<Display>>>) {
        jcall!(self.runnable).run();
    }
}
impl std::ops::Deref for SWTRunnableProcessor {
    type Target = SWTProcessor;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for SWTRunnableProcessor {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::emulator::swt::SWTProcessor::SWTProcessorDyn for SWTRunnableProcessor {
    fn as_SWTProcessor(&self) -> &crate::org::allbinary::emulator::swt::SWTProcessor::SWTProcessor { &self.base }
    fn as_SWTProcessor_mut(&mut self) -> &mut crate::org::allbinary::emulator::swt::SWTProcessor::SWTProcessor { &mut self.base }
    fn process(&mut self, mut display: Option<Rc<RefCell<Display>>>) { Self::process(self, display) }
}


