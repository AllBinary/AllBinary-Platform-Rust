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
// package org.allbinary.input.motion.gesture.observer
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::input::motion::gesture::configuration::MotionGestureConfiguration::MotionGestureConfiguration;
use crate::org::allbinary::input::motion::gesture::configuration::MotionGestureConfigurationFactory::MotionGestureConfigurationFactory;
use super::ResolveCompleteMotionGestureListener::ResolveCompleteMotionGestureListener;
use super::NullCompleteMotionGestureReceiver::NullCompleteMotionGestureReceiver;
use super::CompleteMotionGestureListenerInterface::CompleteMotionGestureListenerInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct MotionGestureReceiveInterfaceFactory {
}


impl MotionGestureReceiveInterfaceFactory {

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<dyn CompleteMotionGestureListenerInterface>>> {
        let mut configuration: Option<Rc<RefCell<MotionGestureConfiguration>>> = MotionGestureConfigurationFactory::getInstance();
        if !jcall!(configuration).isExecutingActionsAllowed() {
            return NullCompleteMotionGestureReceiver::new().map(|reference| reference as Rc<RefCell<dyn CompleteMotionGestureListenerInterface>>);
        } else {
            return ResolveCompleteMotionGestureListener::new().map(|reference| reference as Rc<RefCell<dyn CompleteMotionGestureListenerInterface>>);
        }
    }
}

