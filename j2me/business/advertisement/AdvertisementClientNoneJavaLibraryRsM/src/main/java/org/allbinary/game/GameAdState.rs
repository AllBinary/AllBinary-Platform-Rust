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

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct GameAdState {
    pub base: GameAdStateBase,
}

// impl VirtualKeyboardEventListenerInterface for GameAdState with the methods of the trait

impl GameAdState {
    

    pub fn new_struct(mut adConfiguration: Option<Rc<RefCell<dyn crate::org::allbinary::business::advertisement::AdConfiguration::AdConfigurationDyn>>>) -> Self {
        let mut this = Self {
            base: GameAdStateBase::new_struct(adConfiguration.clone()),
        };
        // super(...) is in the struct literal above
        this
    }

    pub fn new(mut adConfiguration: Option<Rc<RefCell<dyn crate::org::allbinary::business::advertisement::AdConfiguration::AdConfigurationDyn>>>) -> Option<Rc<RefCell<Self>>> {
        let this = Some(Rc::new(RefCell::new(Self::new_struct(adConfiguration))));
            jcall!(VirtualKeyboardEventHandler::getInstance()).addListenerInterface((|java_value: Option<Rc<RefCell<_>>>| java_value.map(|reference| Rc::new(RefCell::new(Box::new(Some(reference)) as Box<dyn std::any::Any>))))(this.clone()));
        this
    }
    

    // throws Exception
    pub fn onVirtualKeyboardEvent(&mut self, mut virtualKeyboardEvent: Option<Rc<RefCell<VirtualKeyboardEvent>>>) {
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
}


