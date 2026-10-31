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
use super::UpGameKeyEventHandlerBase::UpGameKeyEventHandlerBase;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct UpGameKeyEventHandler {
    pub base: UpGameKeyEventHandlerBase,
    pub playerInputIdFactory: Option<Rc<RefCell<LocalPlayerInputIdFactory>>>,
}

thread_local! { pub static UpGameKeyEventHandler_instance: Option<Rc<RefCell<UpGameKeyEventHandler>>> = UpGameKeyEventHandler::new(); }
thread_local! { pub static UpGameKeyEventHandler_instanceArray: Vec<Option<Rc<RefCell<dyn crate::org::allbinary::game::input::event::UpGameKeyEventHandlerBase::UpGameKeyEventHandlerBaseDyn>>>> = vec![UpGameKeyEventHandlerBase::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::input::event::UpGameKeyEventHandlerBase::UpGameKeyEventHandlerBaseDyn>>), UpGameKeyEventHandlerBase::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::input::event::UpGameKeyEventHandlerBase::UpGameKeyEventHandlerBaseDyn>>), UpGameKeyEventHandlerBase::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::input::event::UpGameKeyEventHandlerBase::UpGameKeyEventHandlerBaseDyn>>), UpGameKeyEventHandlerBase::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::input::event::UpGameKeyEventHandlerBase::UpGameKeyEventHandlerBaseDyn>>), UpGameKeyEventHandlerBase::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::input::event::UpGameKeyEventHandlerBase::UpGameKeyEventHandlerBaseDyn>>), UpGameKeyEventHandlerBase::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::input::event::UpGameKeyEventHandlerBase::UpGameKeyEventHandlerBaseDyn>>)]; }

impl UpGameKeyEventHandler {
    

    pub fn getInstance() -> Option<Rc<RefCell<UpGameKeyEventHandler>>> {
        return UpGameKeyEventHandler_instance.with(Clone::clone).clone();
    }
    

    pub fn getInstanceForDevice(&mut self, mut deviceId: i32) -> Option<Rc<RefCell<dyn crate::org::allbinary::game::input::event::UpGameKeyEventHandlerBase::UpGameKeyEventHandlerBaseDyn>>> {
        let mut playerInputId: i32 = jcall!(self.playerInputIdFactory).getPlayerForDevice(deviceId.clone());
        return UpGameKeyEventHandler_instanceArray.with(Clone::clone)[(playerInputId) as usize].clone();
    }
    

    pub fn getInstanceForPlayer(&mut self, mut playerInputId: i32) -> Option<Rc<RefCell<dyn crate::org::allbinary::game::input::event::UpGameKeyEventHandlerBase::UpGameKeyEventHandlerBaseDyn>>> {
        return UpGameKeyEventHandler_instanceArray.with(Clone::clone)[(playerInputId) as usize].clone();
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: UpGameKeyEventHandlerBase::new_struct(),
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
        for index in (0..=(UpGameKeyEventHandler_instanceArray.with(Clone::clone).len() as i32) - 1).rev() {
            jcall!(UpGameKeyEventHandler_instanceArray.with(Clone::clone)[(index) as usize]).removeAllListeners();
        }
    }
    

    pub fn removeListenerSingleThreaded(&mut self, mut eventListenerInterface: Option<Rc<RefCell<dyn EventListenerInterface>>>) {
        self.base.removeListenerSingleThreaded(eventListenerInterface.clone());
        for index in (0..=(UpGameKeyEventHandler_instanceArray.with(Clone::clone).len() as i32) - 1).rev() {
            jcall!(UpGameKeyEventHandler_instanceArray.with(Clone::clone)[(index) as usize]).removeListenerSingleThreaded(eventListenerInterface.clone());
        }
    }
    

    pub fn removeListener(&mut self, mut eventListenerInterface: Option<Rc<RefCell<dyn EventListenerInterface>>>) {
        self.base.removeListener(eventListenerInterface.clone());
        for index in (0..=(UpGameKeyEventHandler_instanceArray.with(Clone::clone).len() as i32) - 1).rev() {
            jcall!(UpGameKeyEventHandler_instanceArray.with(Clone::clone)[(index) as usize]).removeListener(eventListenerInterface.clone());
        }
    }
}
impl std::ops::Deref for UpGameKeyEventHandler {
    type Target = UpGameKeyEventHandlerBase;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for UpGameKeyEventHandler {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::game::input::event::UpGameKeyEventHandlerBase::UpGameKeyEventHandlerBaseDyn for UpGameKeyEventHandler {
    fn as_UpGameKeyEventHandlerBase(&self) -> &crate::org::allbinary::game::input::event::UpGameKeyEventHandlerBase::UpGameKeyEventHandlerBase { &self.base }
    fn as_UpGameKeyEventHandlerBase_mut(&mut self) -> &mut crate::org::allbinary::game::input::event::UpGameKeyEventHandlerBase::UpGameKeyEventHandlerBase { &mut self.base }
}
impl crate::org::allbinary::logic::util::event::handler::BasicEventHandler::BasicEventHandlerDyn for UpGameKeyEventHandler {
    fn as_BasicEventHandler(&self) -> &crate::org::allbinary::logic::util::event::handler::BasicEventHandler::BasicEventHandler { &self.base.base }
    fn as_BasicEventHandler_mut(&mut self) -> &mut crate::org::allbinary::logic::util::event::handler::BasicEventHandler::BasicEventHandler { &mut self.base.base }
    fn removeListenerSingleThreaded(&mut self, mut eventListenerInterface: Option<Rc<RefCell<dyn EventListenerInterface>>>) { Self::removeListenerSingleThreaded(self, eventListenerInterface) }
    fn process(&mut self, p1: Option<Rc<RefCell<dyn crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObjectDyn>>>, p2: Option<Rc<RefCell<dyn crate::org::allbinary::logic::util::event::EventListenerInterface::EventListenerInterface>>>) { crate::org::allbinary::game::input::event::UpGameKeyEventHandlerBase::UpGameKeyEventHandlerBase::process(crate::org::allbinary::game::input::event::UpGameKeyEventHandlerBase::UpGameKeyEventHandlerBaseDyn::as_UpGameKeyEventHandlerBase_mut(self), p1, p2) }
}
impl crate::org::allbinary::logic::util::event::handler::BasicEventHandlerInterface::BasicEventHandlerInterface for UpGameKeyEventHandler {
    fn addListenerInterface(&mut self, p1: Option<Rc<RefCell<dyn crate::org::allbinary::logic::util::event::EventListenerInterface::EventListenerInterface>>>) { crate::org::allbinary::logic::util::event::handler::BasicEventHandler::BasicEventHandler::addListenerInterface(crate::org::allbinary::logic::util::event::handler::BasicEventHandler::BasicEventHandlerDyn::as_BasicEventHandler_mut(self), p1) }
    fn addListeners(&mut self, p1: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) { crate::org::allbinary::logic::util::event::handler::BasicEventHandler::BasicEventHandler::addListeners(crate::org::allbinary::logic::util::event::handler::BasicEventHandler::BasicEventHandlerDyn::as_BasicEventHandler_mut(self), p1) }
    fn fireEvent(&mut self, p1: Option<Rc<RefCell<dyn crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObjectDyn>>>) { crate::org::allbinary::game::input::event::UpGameKeyEventHandlerBase::UpGameKeyEventHandlerBase::fireEvent(crate::org::allbinary::game::input::event::UpGameKeyEventHandlerBase::UpGameKeyEventHandlerBaseDyn::as_UpGameKeyEventHandlerBase_mut(self), p1) }
    fn removeAllListeners(&mut self) { Self::removeAllListeners(self) }
    fn removeListener(&mut self, mut eventListenerInterface: Option<Rc<RefCell<dyn EventListenerInterface>>>) { Self::removeListener(self, eventListenerInterface) }
}


