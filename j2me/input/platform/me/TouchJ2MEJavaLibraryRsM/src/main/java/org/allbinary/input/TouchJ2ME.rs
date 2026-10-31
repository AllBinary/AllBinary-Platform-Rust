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
// package org.allbinary.input
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::input::motion::AllMotionRecognizer::AllMotionRecognizer;
use crate::org::allbinary::input::motion::gesture::observer::BasicMotionGesturesHandler::BasicMotionGesturesHandler;
use crate::org::allbinary::input::motion::gesture::observer::GameMotionGestureListener::GameMotionGestureListener;
use crate::org::allbinary::input::motion::gesture::observer::MotionGestureReceiveInterfaceFactory::MotionGestureReceiveInterfaceFactory;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::string::CommonStrings::CommonStrings;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

//TouchJ2ME
pub struct TouchJ2ME {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub DEVICE_ID: i32,
    pub motionRecognizer: Option<Rc<RefCell<AllMotionRecognizer>>>,
}


impl TouchJ2ME {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            logUtil: None,
            DEVICE_ID: 0,
            motionRecognizer: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.DEVICE_ID = 0;
        this.motionRecognizer = AllMotionRecognizer::new();
        let mut motionGesturesHandler: Option<Rc<RefCell<BasicMotionGesturesHandler>>> = jcall!(jcall!(this.motionRecognizer).getMotionGestureRecognizer()).getMotionGesturesHandler();
        jcall!(motionGesturesHandler).addListenerInterface(GameMotionGestureListener::new(MotionGestureReceiveInterfaceFactory::getInstance()));
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn pointerDragged(&mut self, mut x: i32, mut y: i32) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            jcall!(self.motionRecognizer).processDraggedMotionEvent(x.clone(), y.clone(), self.DEVICE_ID.clone(), 0);
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            let mut commonStrings: Option<Rc<RefCell<CommonStrings>>> = CommonStrings::getInstance();
            jcall!(self.logUtil).put(jcall!(commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("pointerDragged"), e.clone());
            }
        }
    }
    

    pub fn pointerPressed(&mut self, mut x: i32, mut y: i32) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            jcall!(self.motionRecognizer).processStartMotionEvent(x.clone(), y.clone(), self.DEVICE_ID.clone(), 0);
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            let mut commonStrings: Option<Rc<RefCell<CommonStrings>>> = CommonStrings::getInstance();
            jcall!(self.logUtil).put(jcall!(commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("pointerPressed"), e.clone());
            }
        }
    }
    

    pub fn pointerReleased(&mut self, mut x: i32, mut y: i32) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            jcall!(self.motionRecognizer).processEndMotionEvent(x.clone(), y.clone(), self.DEVICE_ID.clone(), 0);
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            let mut commonStrings: Option<Rc<RefCell<CommonStrings>>> = CommonStrings::getInstance();
            jcall!(self.logUtil).put(jcall!(commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("pointerReleased"), e.clone());
            }
        }
    }
}

