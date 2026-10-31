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
// package org.allbinary.graphics.displayable.event
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObject;
use crate::org::allbinary::logic::util::event::EventListenerInterface::EventListenerInterface;
use crate::org::allbinary::logic::util::event::handler::BasicEventHandler::BasicEventHandler;
use super::DisplayChangeEventListener::DisplayChangeEventListener;
use super::DisplayChangeEvent::DisplayChangeEvent;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct DisplayChangeEventHandler {
    pub base: BasicEventHandler,
}

thread_local! { pub static DisplayChangeEventHandler_SINGLETON: Option<Rc<RefCell<DisplayChangeEventHandler>>> = DisplayChangeEventHandler::new(); }

impl DisplayChangeEventHandler {
    

    pub fn getInstance() -> Option<Rc<RefCell<DisplayChangeEventHandler>>> {
        return DisplayChangeEventHandler_SINGLETON.with(Clone::clone).clone();
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: BasicEventHandler::new_struct(),
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    // throws Exception
    pub fn process(&mut self, mut eventObject: Option<Rc<RefCell<dyn crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObjectDyn>>>, mut eventListenerInterface: Option<Rc<RefCell<dyn EventListenerInterface>>>) {
        let mut displayChangeEventListener: Option<Rc<RefCell<dyn DisplayChangeEventListener>>> = (panic!("ClassCastException: a Rust dyn EventListenerInterface cannot be cast to dyn DisplayChangeEventListener"));
        jcall!(displayChangeEventListener).onDisplayChangeEvent((eventObject).clone() /* as DisplayChangeEvent */);
    }
}
impl std::ops::Deref for DisplayChangeEventHandler {
    type Target = BasicEventHandler;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for DisplayChangeEventHandler {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::logic::util::event::handler::BasicEventHandler::BasicEventHandlerDyn for DisplayChangeEventHandler {
    fn as_BasicEventHandler(&self) -> &crate::org::allbinary::logic::util::event::handler::BasicEventHandler::BasicEventHandler { &self.base }
    fn as_BasicEventHandler_mut(&mut self) -> &mut crate::org::allbinary::logic::util::event::handler::BasicEventHandler::BasicEventHandler { &mut self.base }
    fn process(&mut self, mut eventObject: Option<Rc<RefCell<dyn crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObjectDyn>>>, mut eventListenerInterface: Option<Rc<RefCell<dyn EventListenerInterface>>>) { Self::process(self, eventObject, eventListenerInterface) }
}
impl crate::org::allbinary::logic::util::event::handler::BasicEventHandlerInterface::BasicEventHandlerInterface for DisplayChangeEventHandler {
    fn addListenerInterface(&mut self, p1: Option<Rc<RefCell<dyn crate::org::allbinary::logic::util::event::EventListenerInterface::EventListenerInterface>>>) { crate::org::allbinary::logic::util::event::handler::BasicEventHandler::BasicEventHandler::addListenerInterface(crate::org::allbinary::logic::util::event::handler::BasicEventHandler::BasicEventHandlerDyn::as_BasicEventHandler_mut(self), p1) }
    fn addListeners(&mut self, p1: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) { crate::org::allbinary::logic::util::event::handler::BasicEventHandler::BasicEventHandler::addListeners(crate::org::allbinary::logic::util::event::handler::BasicEventHandler::BasicEventHandlerDyn::as_BasicEventHandler_mut(self), p1) }
    fn fireEvent(&mut self, p1: Option<Rc<RefCell<dyn crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObjectDyn>>>) { crate::org::allbinary::logic::util::event::handler::BasicEventHandler::BasicEventHandler::fireEvent(crate::org::allbinary::logic::util::event::handler::BasicEventHandler::BasicEventHandlerDyn::as_BasicEventHandler_mut(self), p1) }
    fn removeAllListeners(&mut self) { crate::org::allbinary::logic::util::event::handler::BasicEventHandler::BasicEventHandler::removeAllListeners(crate::org::allbinary::logic::util::event::handler::BasicEventHandler::BasicEventHandlerDyn::as_BasicEventHandler_mut(self)) }
    fn removeListener(&mut self, p1: Option<Rc<RefCell<dyn crate::org::allbinary::logic::util::event::EventListenerInterface::EventListenerInterface>>>) { crate::org::allbinary::logic::util::event::handler::BasicEventHandler::BasicEventHandler::removeListener(crate::org::allbinary::logic::util::event::handler::BasicEventHandler::BasicEventHandlerDyn::as_BasicEventHandler_mut(self), p1) }
}


