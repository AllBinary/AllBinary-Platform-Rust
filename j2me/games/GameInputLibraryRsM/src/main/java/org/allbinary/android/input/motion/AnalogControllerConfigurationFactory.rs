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
// package org.allbinary.android.input.motion
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::android::input::motion::event::AnalogControllerConfigurationEvent::AnalogControllerConfigurationEvent;
use crate::org::allbinary::android::input::motion::event::AnalogControllerConfigurationEventHandler::AnalogControllerConfigurationEventHandler;
use crate::org::allbinary::logic::math::ScaleFactorFactory::ScaleFactorFactory;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct AnalogControllerConfigurationFactory {
    pub SCALE_FACTOR: i32,
    pub SCALE_VALUE: i32,
    pub available: bool,
    pub analogControllerConfigurationEvent: Option<Rc<RefCell<AnalogControllerConfigurationEvent>>>,
}

thread_local! { pub static AnalogControllerConfigurationFactory_instance: Option<Rc<RefCell<AnalogControllerConfigurationFactory>>> = AnalogControllerConfigurationFactory::new(); }

impl AnalogControllerConfigurationFactory {
    

    pub fn getInstance() -> Option<Rc<RefCell<AnalogControllerConfigurationFactory>>> {
        return AnalogControllerConfigurationFactory_instance.with(Clone::clone).clone();
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            SCALE_FACTOR: 0,
            SCALE_VALUE: 0,
            available: false,
            analogControllerConfigurationEvent: None,
        };
        this.SCALE_FACTOR = jcall!(ScaleFactorFactory::getInstance()).DEFAULT_SCALE_FACTOR.clone();
        this.SCALE_VALUE = jcall!(ScaleFactorFactory::getInstance()).DEFAULT_SCALE_VALUE.clone();
        this.analogControllerConfigurationEvent = AnalogControllerConfigurationEvent::new(this);
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn isAvailable(&mut self) -> bool {
        return self.available.clone();
    }
    

    // throws Exception
    pub fn setAvailable(&mut self, mut available: bool) {
        self.available = available.clone();
        jcall!(AnalogControllerConfigurationEventHandler::getInstance()).fireEvent(self.analogControllerConfigurationEvent.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObjectDyn>>));
    }
}

