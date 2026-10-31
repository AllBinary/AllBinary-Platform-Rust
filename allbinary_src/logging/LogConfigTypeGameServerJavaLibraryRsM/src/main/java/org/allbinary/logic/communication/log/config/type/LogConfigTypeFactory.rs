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
// package org.allbinary.logic.communication.log.config.type
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::logic::communication::log::config::r#type::LogConfigTypes::LogConfigTypes;
use crate::org::allbinary::logic::communication::log::config::r#type::LogConfigTypeFactory::LogConfigTypeFactory;
use super::LogConfigType::LogConfigType;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct LogConfigTypeFactory {
    pub NO_DESCRIPTION: String,
    pub INIT_SERVER: Option<Rc<RefCell<LogConfigType>>>,
    pub LOBBY_SERVER: Option<Rc<RefCell<LogConfigType>>>,
    pub GAME_SERVER: Option<Rc<RefCell<LogConfigType>>>,
    pub GAME_SIMULATOR: Option<Rc<RefCell<LogConfigType>>>,
    pub CUSTOM_TYPES: Option<Rc<RefCell<LogConfigType>>>,
}

thread_local! { pub static LogConfigTypeFactory_instance: Option<Rc<RefCell<LogConfigTypeFactory>>> = LogConfigTypeFactory::new(); }

impl LogConfigTypeFactory {
    

    pub fn getInstance() -> Option<Rc<RefCell<LogConfigTypeFactory>>> {
        return LogConfigTypeFactory_instance.with(Clone::clone).clone();
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            NO_DESCRIPTION: String::new(),
            INIT_SERVER: None,
            LOBBY_SERVER: None,
            GAME_SERVER: None,
            GAME_SIMULATOR: None,
            CUSTOM_TYPES: None,
        };
        this.NO_DESCRIPTION = String::from("No Description");
        this.INIT_SERVER = LogConfigType::new(String::from("Init Server"), this.NO_DESCRIPTION.clone());
        this.LOBBY_SERVER = LogConfigType::new(String::from("Lobby Server"), this.NO_DESCRIPTION.clone());
        this.GAME_SERVER = LogConfigType::new(String::from("Game Server"), this.NO_DESCRIPTION.clone());
        this.GAME_SIMULATOR = LogConfigType::new(String::from("Game Simulator"), this.NO_DESCRIPTION.clone());
        this.CUSTOM_TYPES = LogConfigType::new(String::from("Custom Types"), this.NO_DESCRIPTION.clone());
        jcall!(unsafe { crate::org::allbinary::logic::communication::log::config::r#type::LogConfigTypes::LogConfigTypes_LOGGING.clone() }).add(crate::java_into_object(this.INIT_SERVER.clone()));
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
}

