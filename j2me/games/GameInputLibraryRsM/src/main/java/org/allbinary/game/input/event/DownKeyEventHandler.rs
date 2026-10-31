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
// package org.allbinary.game.input.event
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::game::input::LocalPlayerInputIdFactory::LocalPlayerInputIdFactory;
use crate::org::allbinary::logic::util::event::EventListenerInterface::EventListenerInterface;
use super::DownKeyEventHandlerBase::DownKeyEventHandlerBase;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct DownKeyEventHandler {
    pub base: DownKeyEventHandlerBase,
    pub playerInputIdFactory: Option<Rc<RefCell<LocalPlayerInputIdFactory>>>,
}

thread_local! { pub static DownKeyEventHandler_instance: Option<Rc<RefCell<DownKeyEventHandler>>> = DownKeyEventHandler::new(); }
thread_local! { pub static DownKeyEventHandler_instanceArray: Vec<Option<Rc<RefCell<dyn crate::org::allbinary::game::input::event::DownKeyEventHandlerBase::DownKeyEventHandlerBaseDyn>>>> = vec![DownKeyEventHandlerBase::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::input::event::DownKeyEventHandlerBase::DownKeyEventHandlerBaseDyn>>), DownKeyEventHandlerBase::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::input::event::DownKeyEventHandlerBase::DownKeyEventHandlerBaseDyn>>), DownKeyEventHandlerBase::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::input::event::DownKeyEventHandlerBase::DownKeyEventHandlerBaseDyn>>), DownKeyEventHandlerBase::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::input::event::DownKeyEventHandlerBase::DownKeyEventHandlerBaseDyn>>), DownKeyEventHandlerBase::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::input::event::DownKeyEventHandlerBase::DownKeyEventHandlerBaseDyn>>), DownKeyEventHandlerBase::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::input::event::DownKeyEventHandlerBase::DownKeyEventHandlerBaseDyn>>)]; }

impl DownKeyEventHandler {
    

    pub fn getInstance() -> Option<Rc<RefCell<DownKeyEventHandler>>> {
        return DownKeyEventHandler_instance.with(Clone::clone).clone();
    }
    

    pub fn getInstanceForDevice(&mut self, mut deviceId: i32) -> Option<Rc<RefCell<dyn crate::org::allbinary::game::input::event::DownKeyEventHandlerBase::DownKeyEventHandlerBaseDyn>>> {
        let mut playerInputId: i32 = jcall!(self.playerInputIdFactory).getPlayerForDevice(deviceId.clone());
        return DownKeyEventHandler_instanceArray.with(Clone::clone)[(playerInputId) as usize].clone();
    }
    

    pub fn getInstanceForPlayer(&mut self, mut playerInputId: i32) -> Option<Rc<RefCell<dyn crate::org::allbinary::game::input::event::DownKeyEventHandlerBase::DownKeyEventHandlerBaseDyn>>> {
        return DownKeyEventHandler_instanceArray.with(Clone::clone)[(playerInputId) as usize].clone();
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: DownKeyEventHandlerBase::new_struct(),
            playerInputIdFactory: None,
        };
        this.playerInputIdFactory = LocalPlayerInputIdFactory::getInstance();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn removeAllListeners(&mut self) {
        self.base.removeAllListeners();
        for index in (0..=(DownKeyEventHandler_instanceArray.with(Clone::clone).len() as i32) - 1).rev() {
            jcall!(DownKeyEventHandler_instanceArray.with(Clone::clone)[(index) as usize]).removeAllListeners();
        }
    }
    

    pub fn removeListenerSingleThreaded(&mut self, mut eventListenerInterface: Option<Rc<RefCell<dyn EventListenerInterface>>>) {
        self.base.removeListenerSingleThreaded(eventListenerInterface.clone());
        for index in (0..=(DownKeyEventHandler_instanceArray.with(Clone::clone).len() as i32) - 1).rev() {
            jcall!(DownKeyEventHandler_instanceArray.with(Clone::clone)[(index) as usize]).removeListenerSingleThreaded(eventListenerInterface.clone());
        }
    }
    

    // synchronized
    pub fn removeListener(&mut self, mut eventListenerInterface: Option<Rc<RefCell<dyn EventListenerInterface>>>) {
        self.base.removeListener(eventListenerInterface.clone());
        for index in (0..=(DownKeyEventHandler_instanceArray.with(Clone::clone).len() as i32) - 1).rev() {
            jcall!(DownKeyEventHandler_instanceArray.with(Clone::clone)[(index) as usize]).removeListener(eventListenerInterface.clone());
        }
    }
}
impl std::ops::Deref for DownKeyEventHandler {
    type Target = DownKeyEventHandlerBase;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for DownKeyEventHandler {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::game::input::event::DownKeyEventHandlerBase::DownKeyEventHandlerBaseDyn for DownKeyEventHandler {
    fn as_DownKeyEventHandlerBase(&self) -> &crate::org::allbinary::game::input::event::DownKeyEventHandlerBase::DownKeyEventHandlerBase { &self.base }
    fn as_DownKeyEventHandlerBase_mut(&mut self) -> &mut crate::org::allbinary::game::input::event::DownKeyEventHandlerBase::DownKeyEventHandlerBase { &mut self.base }
}
impl crate::org::allbinary::logic::util::event::handler::BasicEventHandler::BasicEventHandlerDyn for DownKeyEventHandler {
    fn as_BasicEventHandler(&self) -> &crate::org::allbinary::logic::util::event::handler::BasicEventHandler::BasicEventHandler { &self.base.base }
    fn as_BasicEventHandler_mut(&mut self) -> &mut crate::org::allbinary::logic::util::event::handler::BasicEventHandler::BasicEventHandler { &mut self.base.base }
    fn removeListenerSingleThreaded(&mut self, mut eventListenerInterface: Option<Rc<RefCell<dyn EventListenerInterface>>>) { Self::removeListenerSingleThreaded(self, eventListenerInterface) }
}
impl crate::org::allbinary::logic::util::event::handler::BasicEventHandlerInterface::BasicEventHandlerInterface for DownKeyEventHandler {
    fn addListenerInterface(&mut self, p1: Option<Rc<RefCell<dyn crate::org::allbinary::logic::util::event::EventListenerInterface::EventListenerInterface>>>) { crate::org::allbinary::logic::util::event::handler::BasicEventHandler::BasicEventHandler::addListenerInterface(crate::org::allbinary::logic::util::event::handler::BasicEventHandler::BasicEventHandlerDyn::as_BasicEventHandler_mut(self), p1) }
    fn addListeners(&mut self, p1: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) { crate::org::allbinary::logic::util::event::handler::BasicEventHandler::BasicEventHandler::addListeners(crate::org::allbinary::logic::util::event::handler::BasicEventHandler::BasicEventHandlerDyn::as_BasicEventHandler_mut(self), p1) }
    fn fireEvent(&mut self, p1: Option<Rc<RefCell<dyn crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObjectDyn>>>) { crate::org::allbinary::logic::util::event::handler::BasicEventHandler::BasicEventHandler::fireEvent(crate::org::allbinary::logic::util::event::handler::BasicEventHandler::BasicEventHandlerDyn::as_BasicEventHandler_mut(self), p1) }
    fn removeAllListeners(&mut self) { Self::removeAllListeners(self) }
    fn removeListener(&mut self, mut eventListenerInterface: Option<Rc<RefCell<dyn EventListenerInterface>>>) { Self::removeListener(self, eventListenerInterface) }
}


