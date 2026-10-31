//
// AllBinary Open License Version 1
// Copyright (c) 2009 AllBinary
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
// package org.allbinary.android.input.motion
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::android::view::MotionEvent::MotionEvent;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct MotionEventProcessor {
}


impl MotionEventProcessor {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    // throws Exception
    pub fn process(&mut self, mut motionEvent: Option<Rc<RefCell<MotionEvent>>>) {
    }
}
pub trait MotionEventProcessorDyn {
    fn as_MotionEventProcessor(&self) -> &MotionEventProcessor;
    fn as_MotionEventProcessor_mut(&mut self) -> &mut MotionEventProcessor;
    fn process(&mut self, mut motionEvent: Option<Rc<RefCell<MotionEvent>>>) { MotionEventProcessor::process(self.as_MotionEventProcessor_mut(), motionEvent) }
}
impl MotionEventProcessorDyn for MotionEventProcessor {
    fn as_MotionEventProcessor(&self) -> &MotionEventProcessor { self }
    fn as_MotionEventProcessor_mut(&mut self) -> &mut MotionEventProcessor { self }
}


