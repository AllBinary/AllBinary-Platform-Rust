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
// package org.allbinary.game.input
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::input::motion::gesture::MotionGestureInput::MotionGestureInput;
use crate::org::allbinary::input::motion::gesture::TouchMotionGestureFactory::TouchMotionGestureFactory;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObject;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use super::CompleteMotionGestureInputInterface::CompleteMotionGestureInputInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct CompleteMotionGestureInputEvent {
    pub base: AllBinaryEventObject,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub name: String,
    pub motionGestureInput: Option<Rc<RefCell<MotionGestureInput>>>,
}

impl CompleteMotionGestureInputInterface for CompleteMotionGestureInputEvent {
}
impl crate::org::allbinary::layer::NamedInterface::NamedInterface for CompleteMotionGestureInputEvent {
    fn getName(&mut self) -> String { Self::getName(self) }
}

impl CompleteMotionGestureInputEvent {
    

    pub fn new_struct(mut name: String, mut motionGestureInput: Option<Rc<RefCell<MotionGestureInput>>>) -> Self {
        let mut this = Self {
            base: AllBinaryEventObject::new_struct(motionGestureInput.clone()),
            logUtil: None,
            name: String::new(),
            motionGestureInput: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.name = jcall!(StringUtil::getInstance()).EMPTY_STRING.clone();
        this.motionGestureInput = jcall!(TouchMotionGestureFactory::getInstance()).NO_MOTION.clone();
        // super(...) is in the struct literal above
        this.setName(name.clone());
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            this.setMotionGestureInput(motionGestureInput.clone());
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            let mut commonStrings: Option<Rc<RefCell<CommonStrings>>> = CommonStrings::getInstance();
            jcall!(this.logUtil).put(jcall!(commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("InputToGameKeyEventAction"), crate::java_into_object(e.clone()));
            }
        }
        this
    }

    pub fn new(mut name: String, mut motionGestureInput: Option<Rc<RefCell<MotionGestureInput>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(name, motionGestureInput))))
    }
    

    pub fn setName(&mut self, mut name: String) {
        self.name = name.clone();
    }
    

    pub fn getName(&mut self) -> String {
        return self.name.clone();
    }
    

    pub fn setMotionGestureInput(&mut self, mut motionGestureInput: Option<Rc<RefCell<MotionGestureInput>>>) {
        self.motionGestureInput = motionGestureInput.clone();
    }
    

    pub fn getMotionGestureInput(&mut self) -> Option<Rc<RefCell<MotionGestureInput>>> {
        return self.motionGestureInput.clone();
    }
}
impl std::ops::Deref for CompleteMotionGestureInputEvent {
    type Target = AllBinaryEventObject;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for CompleteMotionGestureInputEvent {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

pub trait CompleteMotionGestureInputEventDyn: crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObjectDyn {
    fn as_CompleteMotionGestureInputEvent(&self) -> &CompleteMotionGestureInputEvent;
    fn as_CompleteMotionGestureInputEvent_mut(&mut self) -> &mut CompleteMotionGestureInputEvent;
    fn setName(&mut self, mut name: String) { CompleteMotionGestureInputEvent::setName(self.as_CompleteMotionGestureInputEvent_mut(), name) }
    fn getName(&mut self) -> String { CompleteMotionGestureInputEvent::getName(self.as_CompleteMotionGestureInputEvent_mut()) }
    fn setMotionGestureInput(&mut self, mut motionGestureInput: Option<Rc<RefCell<MotionGestureInput>>>) { CompleteMotionGestureInputEvent::setMotionGestureInput(self.as_CompleteMotionGestureInputEvent_mut(), motionGestureInput) }
    fn getMotionGestureInput(&mut self) -> Option<Rc<RefCell<MotionGestureInput>>> { CompleteMotionGestureInputEvent::getMotionGestureInput(self.as_CompleteMotionGestureInputEvent_mut()) }
}
impl CompleteMotionGestureInputEventDyn for CompleteMotionGestureInputEvent {
    fn as_CompleteMotionGestureInputEvent(&self) -> &CompleteMotionGestureInputEvent { self }
    fn as_CompleteMotionGestureInputEvent_mut(&mut self) -> &mut CompleteMotionGestureInputEvent { self }
}

impl crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObjectDyn for CompleteMotionGestureInputEvent {
    fn as_AllBinaryEventObject(&self) -> &crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObject { &self.base }
    fn as_AllBinaryEventObject_mut(&mut self) -> &mut crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObject { &mut self.base }
}


