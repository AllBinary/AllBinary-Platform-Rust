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
// package org.allbinary.game
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::business::advertisement::GameAdStateBase::GameAdStateBase;
use crate::org::allbinary::business::advertisement::AdConfiguration::AdConfiguration;
use crate::org::allbinary::input::event::VirtualKeyboardEvent::VirtualKeyboardEvent;
use crate::org::allbinary::input::event::VirtualKeyboardEventHandler::VirtualKeyboardEventHandler;
use crate::org::allbinary::input::event::VirtualKeyboardEventListenerInterface::VirtualKeyboardEventListenerInterface;
use crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObject;
use crate::java::lang::Exception::*;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct GameAdState {
    pub base: GameAdStateBase,
}

impl VirtualKeyboardEventListenerInterface for GameAdState {
    fn onVirtualKeyboardEvent(&mut self, mut virtualKeyboardEvent: Option<Rc<RefCell<VirtualKeyboardEvent>>>) { Self::onVirtualKeyboardEvent(self, virtualKeyboardEvent) }
}
impl crate::org::allbinary::logic::util::event::EventListenerInterface::EventListenerInterface for GameAdState {
    fn onEvent(&mut self, mut eventObject: Option<Rc<RefCell<dyn crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObjectDyn>>>) { Self::onEvent(self, eventObject) }
}

impl GameAdState {
    

    pub fn new_struct(mut adConfiguration: Option<Rc<RefCell<dyn crate::org::allbinary::business::advertisement::AdConfiguration::AdConfigurationDyn>>>) -> Self {
        let mut this = Self {
            base: GameAdStateBase::new_struct(adConfiguration.clone()),
        };
        // super(...) is in the struct literal above
        this
    }

    pub fn new(mut adConfiguration: Option<Rc<RefCell<dyn crate::org::allbinary::business::advertisement::AdConfiguration::AdConfigurationDyn>>>) -> Option<Rc<RefCell<Self>>> {
        let this = Some(Rc::new(RefCell::new(Self::new_struct(adConfiguration.clone()))));
            jcall!(VirtualKeyboardEventHandler::getInstance()).addListenerInterface(this.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::util::event::EventListenerInterface::EventListenerInterface>>));
        this
    }
    

    // throws Exception
    pub fn onVirtualKeyboardEvent(&mut self, mut virtualKeyboardEvent: Option<Rc<RefCell<VirtualKeyboardEvent>>>) {
    }
    

    pub fn onEvent(&mut self, mut eventObject: Option<Rc<RefCell<dyn crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObjectDyn>>>) {
        { let java_arguments = (eventObject.clone(),); self.base.onEvent(java_arguments.0) };
    }
}
impl std::ops::Deref for GameAdState {
    type Target = GameAdStateBase;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for GameAdState {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::business::advertisement::GameAdStateBase::GameAdStateBaseDyn for GameAdState {
    fn as_GameAdStateBase(&self) -> &crate::org::allbinary::business::advertisement::GameAdStateBase::GameAdStateBase { &self.base }
    fn as_GameAdStateBase_mut(&mut self) -> &mut crate::org::allbinary::business::advertisement::GameAdStateBase::GameAdStateBase { &mut self.base }
    fn onEvent(&mut self, mut eventObject: Option<Rc<RefCell<dyn crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObjectDyn>>>) { Self::onEvent(self, eventObject) }
}


