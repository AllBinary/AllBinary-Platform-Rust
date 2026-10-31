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
use super::MotionEventProcessor::MotionEventProcessor;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct NoMotionEventProcessor {
    pub base: MotionEventProcessor,
}

thread_local! { pub static NoMotionEventProcessor_SINGLETON: Option<Rc<RefCell<dyn crate::org::allbinary::android::input::motion::MotionEventProcessor::MotionEventProcessorDyn>>> = MotionEventProcessor::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::android::input::motion::MotionEventProcessor::MotionEventProcessorDyn>>); }

impl NoMotionEventProcessor {

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: MotionEventProcessor::new_struct(),
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<dyn crate::org::allbinary::android::input::motion::MotionEventProcessor::MotionEventProcessorDyn>>> {
        return NoMotionEventProcessor_SINGLETON.with(Clone::clone).clone();
    }
}
impl std::ops::Deref for NoMotionEventProcessor {
    type Target = MotionEventProcessor;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for NoMotionEventProcessor {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::android::input::motion::MotionEventProcessor::MotionEventProcessorDyn for NoMotionEventProcessor {
    fn as_MotionEventProcessor(&self) -> &crate::org::allbinary::android::input::motion::MotionEventProcessor::MotionEventProcessor { &self.base }
    fn as_MotionEventProcessor_mut(&mut self) -> &mut crate::org::allbinary::android::input::motion::MotionEventProcessor::MotionEventProcessor { &mut self.base }
}


