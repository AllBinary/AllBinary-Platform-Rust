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
use crate::org::allbinary::game::input::event::DownGameKeyEventHandler::DownGameKeyEventHandler;
use crate::org::allbinary::game::input::event::GameKeyEvent::GameKeyEvent;
use crate::org::allbinary::game::input::event::UpGameKeyEventHandler::UpGameKeyEventHandler;
use crate::org::allbinary::logic::communication::log::ForcedLogUtil::ForcedLogUtil;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObject;
use crate::org::allbinary::logic::util::event::EventStrings::EventStrings;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use super::GameKeyCompleteMotionGestureInputEvent::GameKeyCompleteMotionGestureInputEvent;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct GameKeyFromMotionGestureEventListener {
    pub base: CompleteMotionGestureInputEventListener,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub previousGameKeyEvent: Option<Rc<RefCell<GameKeyEvent>>>,
}

impl crate::org::allbinary::game::input::CompleteMotionGestureInputEventListenerInterface::CompleteMotionGestureInputEventListenerInterface for GameKeyFromMotionGestureEventListener {
    fn onCompleteMotionGestureInputEvent(&mut self, mut completeMotionGestureInputEvent: Option<Rc<RefCell<dyn crate::org::allbinary::game::input::CompleteMotionGestureInputEvent::CompleteMotionGestureInputEventDyn>>>) { Self::onCompleteMotionGestureInputEvent(self, completeMotionGestureInputEvent) }
}
impl crate::org::allbinary::logic::util::event::EventListenerInterface::EventListenerInterface for GameKeyFromMotionGestureEventListener {
    fn onEvent(&mut self, mut eventObject: Option<Rc<RefCell<dyn crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObjectDyn>>>) { Self::onEvent(self, eventObject) }
}

impl GameKeyFromMotionGestureEventListener {

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: CompleteMotionGestureInputEventListener::new_struct(),
            logUtil: None,
            previousGameKeyEvent: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.previousGameKeyEvent = crate::org::allbinary::game::input::event::GameKeyEvent::GameKeyEvent_NONE.with(Clone::clone).clone();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn onEvent(&mut self, mut eventObject: Option<Rc<RefCell<dyn crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObjectDyn>>>) {
        ForcedLogUtil::log(jcall!(EventStrings::getInstance()).PERFORMANCE_MESSAGE.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())));
    }
    

    pub fn onCompleteMotionGestureInputEvent(&mut self, mut completeMotionGestureInputEvent: Option<Rc<RefCell<dyn crate::org::allbinary::game::input::CompleteMotionGestureInputEvent::CompleteMotionGestureInputEventDyn>>>) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut gameKeyCompleteMotionGestureInputEvent: Option<Rc<RefCell<dyn crate::org::allbinary::game::input::motion::action::GameKeyCompleteMotionGestureInputEvent::GameKeyCompleteMotionGestureInputEventDyn>>> = (completeMotionGestureInputEvent).clone() /* as GameKeyCompleteMotionGestureInputEvent */;
            let mut gameKeyEvent: Option<Rc<RefCell<GameKeyEvent>>> = jcall!(gameKeyCompleteMotionGestureInputEvent).getGameKeyEvent();
            if !match (&gameKeyEvent, &self.previousGameKeyEvent) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                if !match (&self.previousGameKeyEvent, &crate::org::allbinary::game::input::event::GameKeyEvent::GameKeyEvent_NONE.with(Clone::clone)) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                    jcall!(UpGameKeyEventHandler::getInstance()).fireEvent(self.previousGameKeyEvent.clone());
                }
                self.previousGameKeyEvent = gameKeyEvent.clone();
                if gameKeyEvent.is_some() && !match (&gameKeyEvent, &crate::org::allbinary::game::input::event::GameKeyEvent::GameKeyEvent_NONE.with(Clone::clone)) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                    jcall!(DownGameKeyEventHandler::getInstance()).fireEvent(gameKeyEvent.clone());
                }
            }
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            let mut commonStrings: Option<Rc<RefCell<CommonStrings>>> = CommonStrings::getInstance();
            jcall!(self.logUtil).put(jcall!(commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("onCompleteMotionGestureInputEvent"), crate::java_into_object(e.clone()));
            }
        }
    }
}
impl std::ops::Deref for GameKeyFromMotionGestureEventListener {
    type Target = CompleteMotionGestureInputEventListener;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for GameKeyFromMotionGestureEventListener {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::game::input::CompleteMotionGestureInputEventListener::CompleteMotionGestureInputEventListenerDyn for GameKeyFromMotionGestureEventListener {
    fn as_CompleteMotionGestureInputEventListener(&self) -> &crate::org::allbinary::game::input::CompleteMotionGestureInputEventListener::CompleteMotionGestureInputEventListener { &self.base }
    fn as_CompleteMotionGestureInputEventListener_mut(&mut self) -> &mut crate::org::allbinary::game::input::CompleteMotionGestureInputEventListener::CompleteMotionGestureInputEventListener { &mut self.base }
    fn onEvent(&mut self, mut eventObject: Option<Rc<RefCell<dyn crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObjectDyn>>>) { Self::onEvent(self, eventObject) }
    fn onCompleteMotionGestureInputEvent(&mut self, mut completeMotionGestureInputEvent: Option<Rc<RefCell<dyn crate::org::allbinary::game::input::CompleteMotionGestureInputEvent::CompleteMotionGestureInputEventDyn>>>) { Self::onCompleteMotionGestureInputEvent(self, completeMotionGestureInputEvent) }
}


