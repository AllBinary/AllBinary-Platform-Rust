// Generated Code Do Not Modify
// package org.allbinary.game.input
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObject;
use super::CompleteMotionGestureInputEventListenerInterface::CompleteMotionGestureInputEventListenerInterface;
use super::CompleteMotionGestureInputEvent::CompleteMotionGestureInputEvent;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct CompleteMotionGestureInputEventListener {
}

impl CompleteMotionGestureInputEventListenerInterface for CompleteMotionGestureInputEventListener {
    fn onCompleteMotionGestureInputEvent(&mut self, mut completeMotionGestureInputEvent: Option<Rc<RefCell<dyn crate::org::allbinary::game::input::CompleteMotionGestureInputEvent::CompleteMotionGestureInputEventDyn>>>) { Self::onCompleteMotionGestureInputEvent(self, completeMotionGestureInputEvent) }
}
impl crate::org::allbinary::logic::util::event::EventListenerInterface::EventListenerInterface for CompleteMotionGestureInputEventListener {
    fn onEvent(&mut self, mut eventObject: Option<Rc<RefCell<dyn crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObjectDyn>>>) { Self::onEvent(self, eventObject) }
}

impl CompleteMotionGestureInputEventListener {

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn onEvent(&mut self, mut eventObject: Option<Rc<RefCell<dyn crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObjectDyn>>>) {
    }
    

    pub fn onCompleteMotionGestureInputEvent(&mut self, mut completeMotionGestureInputEvent: Option<Rc<RefCell<dyn crate::org::allbinary::game::input::CompleteMotionGestureInputEvent::CompleteMotionGestureInputEventDyn>>>) {
    }
}
pub trait CompleteMotionGestureInputEventListenerDyn {
    fn as_CompleteMotionGestureInputEventListener(&self) -> &CompleteMotionGestureInputEventListener;
    fn as_CompleteMotionGestureInputEventListener_mut(&mut self) -> &mut CompleteMotionGestureInputEventListener;
    fn onEvent(&mut self, mut eventObject: Option<Rc<RefCell<dyn crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObjectDyn>>>) { CompleteMotionGestureInputEventListener::onEvent(self.as_CompleteMotionGestureInputEventListener_mut(), eventObject) }
    fn onCompleteMotionGestureInputEvent(&mut self, mut completeMotionGestureInputEvent: Option<Rc<RefCell<dyn crate::org::allbinary::game::input::CompleteMotionGestureInputEvent::CompleteMotionGestureInputEventDyn>>>) { CompleteMotionGestureInputEventListener::onCompleteMotionGestureInputEvent(self.as_CompleteMotionGestureInputEventListener_mut(), completeMotionGestureInputEvent) }
}
impl CompleteMotionGestureInputEventListenerDyn for CompleteMotionGestureInputEventListener {
    fn as_CompleteMotionGestureInputEventListener(&self) -> &CompleteMotionGestureInputEventListener { self }
    fn as_CompleteMotionGestureInputEventListener_mut(&mut self) -> &mut CompleteMotionGestureInputEventListener { self }
}


