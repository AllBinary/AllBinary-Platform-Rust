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
// package org.allbinary.canvas
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct Processor {
}

thread_local! { pub static Processor_SINGLETON: Option<Rc<RefCell<dyn ProcessorDyn>>> = Processor::new().map(|reference| reference as Rc<RefCell<dyn ProcessorDyn>>); }

impl Processor {

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<dyn ProcessorDyn>>> {
        return Processor_SINGLETON.with(Clone::clone).clone();
    }
    

    // throws Exception
    pub fn process(&mut self) {
    }
    

    // throws Exception
    pub fn processt(&mut self, mut timeDelta: i64) {
    }
}
pub trait ProcessorDyn {
    fn as_Processor(&self) -> &Processor;
    fn as_Processor_mut(&mut self) -> &mut Processor;
    fn process(&mut self) { Processor::process(self.as_Processor_mut()) }
    fn processt(&mut self, mut timeDelta: i64) { Processor::processt(self.as_Processor_mut(), timeDelta) }
}
impl ProcessorDyn for Processor {
    fn as_Processor(&self) -> &Processor { self }
    fn as_Processor_mut(&mut self) -> &mut Processor { self }
}


