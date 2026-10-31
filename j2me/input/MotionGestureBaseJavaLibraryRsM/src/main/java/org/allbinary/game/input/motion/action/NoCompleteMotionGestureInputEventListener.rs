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
// package org.allbinary.game.input.motion.action
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::game::input::CompleteMotionGestureInputEvent::CompleteMotionGestureInputEvent;
use crate::org::allbinary::game::input::CompleteMotionGestureInputEventListener::CompleteMotionGestureInputEventListener;
use crate::org::allbinary::logic::communication::log::ForcedLogUtil::ForcedLogUtil;
use crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObject;
use crate::org::allbinary::logic::util::event::EventStrings::EventStrings;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct NoCompleteMotionGestureInputEventListener {
    pub base: CompleteMotionGestureInputEventListener,
}

thread_local! { pub static NoCompleteMotionGestureInputEventListener_instance: Option<Rc<RefCell<NoCompleteMotionGestureInputEventListener>>> = NoCompleteMotionGestureInputEventListener::new(); }
impl crate::org::allbinary::game::input::CompleteMotionGestureInputEventListenerInterface::CompleteMotionGestureInputEventListenerInterface for NoCompleteMotionGestureInputEventListener {
    fn onCompleteMotionGestureInputEvent(&mut self, mut completeMotionGestureInputEvent: Option<Rc<RefCell<dyn crate::org::allbinary::game::input::CompleteMotionGestureInputEvent::CompleteMotionGestureInputEventDyn>>>) { Self::onCompleteMotionGestureInputEvent(self, completeMotionGestureInputEvent) }
}
impl crate::org::allbinary::logic::util::event::EventListenerInterface::EventListenerInterface for NoCompleteMotionGestureInputEventListener {
    fn onEvent(&mut self, mut eventObject: Option<Rc<RefCell<dyn crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObjectDyn>>>) { Self::onEvent(self, eventObject) }
}

impl NoCompleteMotionGestureInputEventListener {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: CompleteMotionGestureInputEventListener::new_struct(),
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn onEvent(&mut self, mut eventObject: Option<Rc<RefCell<dyn crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObjectDyn>>>) {
        ForcedLogUtil::log(jcall!(EventStrings::getInstance()).PERFORMANCE_MESSAGE.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())));
    }
    

    pub fn onCompleteMotionGestureInputEvent(&mut self, mut completeMotionGestureInputEvent: Option<Rc<RefCell<dyn crate::org::allbinary::game::input::CompleteMotionGestureInputEvent::CompleteMotionGestureInputEventDyn>>>) {
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<NoCompleteMotionGestureInputEventListener>>> {
        return NoCompleteMotionGestureInputEventListener_instance.with(Clone::clone).clone();
    }
}
impl std::ops::Deref for NoCompleteMotionGestureInputEventListener {
    type Target = CompleteMotionGestureInputEventListener;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for NoCompleteMotionGestureInputEventListener {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::game::input::CompleteMotionGestureInputEventListener::CompleteMotionGestureInputEventListenerDyn for NoCompleteMotionGestureInputEventListener {
    fn as_CompleteMotionGestureInputEventListener(&self) -> &crate::org::allbinary::game::input::CompleteMotionGestureInputEventListener::CompleteMotionGestureInputEventListener { &self.base }
    fn as_CompleteMotionGestureInputEventListener_mut(&mut self) -> &mut crate::org::allbinary::game::input::CompleteMotionGestureInputEventListener::CompleteMotionGestureInputEventListener { &mut self.base }
    fn onEvent(&mut self, mut eventObject: Option<Rc<RefCell<dyn crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObjectDyn>>>) { Self::onEvent(self, eventObject) }
    fn onCompleteMotionGestureInputEvent(&mut self, mut completeMotionGestureInputEvent: Option<Rc<RefCell<dyn crate::org::allbinary::game::input::CompleteMotionGestureInputEvent::CompleteMotionGestureInputEventDyn>>>) { Self::onCompleteMotionGestureInputEvent(self, completeMotionGestureInputEvent) }
}


