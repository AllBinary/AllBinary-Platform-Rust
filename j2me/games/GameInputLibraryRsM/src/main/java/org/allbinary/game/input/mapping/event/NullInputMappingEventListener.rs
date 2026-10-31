// 
// AllBinary Open License Version 1 
// Copyright (c) 2025 AllBinary 
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
// package org.allbinary.game.input.mapping.event
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObject;
use super::InputMappingEventListenerInterface::InputMappingEventListenerInterface;
use super::InputMappingEvent::InputMappingEvent;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct NullInputMappingEventListener {
}

thread_local! { pub static NullInputMappingEventListener_NULL_INPUT_MAPPING_EVENT_LISTENER: Option<Rc<RefCell<NullInputMappingEventListener>>> = NullInputMappingEventListener::new(); }
impl InputMappingEventListenerInterface for NullInputMappingEventListener {
    fn onInputMappingEvent(&mut self, mut inputMappingEvent: Option<Rc<RefCell<InputMappingEvent>>>) { Self::onInputMappingEvent(self, inputMappingEvent) }
}
impl crate::org::allbinary::logic::util::event::EventListenerInterface::EventListenerInterface for NullInputMappingEventListener {
    fn onEvent(&mut self, mut eventObject: Option<Rc<RefCell<dyn crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObjectDyn>>>) { Self::onEvent(self, eventObject) }
}

impl NullInputMappingEventListener {

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    // throws Exception
    pub fn onInputMappingEvent(&mut self, mut inputMappingEvent: Option<Rc<RefCell<InputMappingEvent>>>) {
    }
    

    pub fn onEvent(&mut self, mut eventObject: Option<Rc<RefCell<dyn crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObjectDyn>>>) {
    }
}

