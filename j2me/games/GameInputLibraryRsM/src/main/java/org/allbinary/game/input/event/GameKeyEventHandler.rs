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
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::util::event::EventListenerInterface::EventListenerInterface;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use super::PressGameKeyEventHandler::PressGameKeyEventHandler;
use super::UpGameKeyEventHandler::UpGameKeyEventHandler;
use super::DownGameKeyEventHandler::DownGameKeyEventHandler;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct GameKeyEventHandler {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub commonStrings: Option<Rc<RefCell<CommonStrings>>>,
    pub pressGameKeyEventHandler: Option<Rc<RefCell<PressGameKeyEventHandler>>>,
    pub upGameKeyEventHandler: Option<Rc<RefCell<UpGameKeyEventHandler>>>,
    pub downGameKeyEventHandler: Option<Rc<RefCell<DownGameKeyEventHandler>>>,
}

thread_local! { pub static GameKeyEventHandler_instance: Option<Rc<RefCell<GameKeyEventHandler>>> = GameKeyEventHandler::new(); }

impl GameKeyEventHandler {

    pub fn new_struct() -> Self {
        let mut this = Self {
            logUtil: None,
            commonStrings: None,
            pressGameKeyEventHandler: None,
            upGameKeyEventHandler: None,
            downGameKeyEventHandler: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.commonStrings = CommonStrings::getInstance();
        this.pressGameKeyEventHandler = PressGameKeyEventHandler::getInstance();
        this.upGameKeyEventHandler = UpGameKeyEventHandler::getInstance();
        this.downGameKeyEventHandler = DownGameKeyEventHandler::getInstance();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<GameKeyEventHandler>>> {
        return GameKeyEventHandler_instance.with(Clone::clone).clone();
    }
    

    // synchronized
    pub fn removeAllListeners(&mut self) {
        jcall!(self.pressGameKeyEventHandler).removeAllListeners();
        jcall!(self.upGameKeyEventHandler).removeAllListeners();
        jcall!(self.downGameKeyEventHandler).removeAllListeners();
    }
    

    pub fn addListenerForPlayer(&mut self, mut eventListenerInterface: Option<Rc<RefCell<dyn EventListenerInterface>>>, mut playerInputId: i32) {
        { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(eventListenerInterface).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).ADD_LISTENER.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        jcall!(self.pressGameKeyEventHandler).addListenerSingleThreaded(eventListenerInterface.clone());
        jcall!(jcall!(self.upGameKeyEventHandler).getInstanceForPlayer(playerInputId.clone())).addListenerSingleThreaded(eventListenerInterface.clone());
        jcall!(jcall!(self.downGameKeyEventHandler).getInstanceForPlayer(playerInputId.clone())).addListenerSingleThreaded(eventListenerInterface.clone());
    }
    

    pub fn addListener(&mut self, mut eventListenerInterface: Option<Rc<RefCell<dyn EventListenerInterface>>>) {
        { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(eventListenerInterface).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).ADD_LISTENER.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        jcall!(self.pressGameKeyEventHandler).addListenerSingleThreaded(eventListenerInterface.clone());
        jcall!(self.upGameKeyEventHandler).addListenerSingleThreaded(eventListenerInterface.clone());
        jcall!(self.downGameKeyEventHandler).addListenerSingleThreaded(eventListenerInterface.clone());
    }
    

    pub fn removeListener(&mut self, mut eventListenerInterface: Option<Rc<RefCell<dyn EventListenerInterface>>>) {
        { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(eventListenerInterface).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).REMOVE_LISTENER.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        jcall!(self.pressGameKeyEventHandler).removeListenerSingleThreaded(eventListenerInterface.clone());
        jcall!(self.upGameKeyEventHandler).removeListenerSingleThreaded(eventListenerInterface.clone());
        jcall!(self.downGameKeyEventHandler).removeListenerSingleThreaded(eventListenerInterface.clone());
    }
}

