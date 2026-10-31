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
use crate::org::allbinary::input::motion::gesture::observer::BasicMotionGesturesHandler::BasicMotionGesturesHandler;
use crate::org::allbinary::input::motion::gesture::observer::GameMotionGestureListener::GameMotionGestureListener;
use crate::org::allbinary::input::motion::gesture::observer::MotionGestureReceiveInterfaceFactory::MotionGestureReceiveInterfaceFactory;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use super::MotionEventProcessor::MotionEventProcessor;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct BaseGameInputMotionEventProcessor {
    pub base: MotionEventProcessor,
    pub commonStrings: Option<Rc<RefCell<CommonStrings>>>,
    pub gameMotionGestureListener: Option<Rc<RefCell<GameMotionGestureListener>>>,
}


impl BaseGameInputMotionEventProcessor {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: MotionEventProcessor::new_struct(),
            commonStrings: None,
            gameMotionGestureListener: None,
        };
        this.commonStrings = CommonStrings::getInstance();
        this.gameMotionGestureListener = GameMotionGestureListener::new(MotionGestureReceiveInterfaceFactory::getInstance());
        this.addListener();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn process(&mut self, mut motionEvent: Option<Rc<RefCell<MotionEvent>>>) {
    }
    

    pub fn addListener(&mut self) {
        let mut motionGesturesHandler: Option<Rc<RefCell<BasicMotionGesturesHandler>>> = BasicMotionGesturesHandler::getInstance();
        jcall!(motionGesturesHandler).addListenerInterface(self.gameMotionGestureListener.clone());
    }
}
impl std::ops::Deref for BaseGameInputMotionEventProcessor {
    type Target = MotionEventProcessor;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for BaseGameInputMotionEventProcessor {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

pub trait BaseGameInputMotionEventProcessorDyn: crate::org::allbinary::android::input::motion::MotionEventProcessor::MotionEventProcessorDyn {
    fn as_BaseGameInputMotionEventProcessor(&self) -> &BaseGameInputMotionEventProcessor;
    fn as_BaseGameInputMotionEventProcessor_mut(&mut self) -> &mut BaseGameInputMotionEventProcessor;
    fn addListener(&mut self) { BaseGameInputMotionEventProcessor::addListener(self.as_BaseGameInputMotionEventProcessor_mut()) }
}
impl BaseGameInputMotionEventProcessorDyn for BaseGameInputMotionEventProcessor {
    fn as_BaseGameInputMotionEventProcessor(&self) -> &BaseGameInputMotionEventProcessor { self }
    fn as_BaseGameInputMotionEventProcessor_mut(&mut self) -> &mut BaseGameInputMotionEventProcessor { self }
}

impl crate::org::allbinary::android::input::motion::MotionEventProcessor::MotionEventProcessorDyn for BaseGameInputMotionEventProcessor {
    fn as_MotionEventProcessor(&self) -> &crate::org::allbinary::android::input::motion::MotionEventProcessor::MotionEventProcessor { &self.base }
    fn as_MotionEventProcessor_mut(&mut self) -> &mut crate::org::allbinary::android::input::motion::MotionEventProcessor::MotionEventProcessor { &mut self.base }
    fn process(&mut self, mut motionEvent: Option<Rc<RefCell<MotionEvent>>>) { Self::process(self, motionEvent) }
}


