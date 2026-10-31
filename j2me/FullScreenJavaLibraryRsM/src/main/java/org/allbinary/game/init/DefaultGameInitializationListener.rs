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
// package org.allbinary.game.init
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::org::allbinary::logic::communication::log::ForcedLogUtil::ForcedLogUtil;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::game::configuration::event::GameInitializedEvent::GameInitializedEvent;
use crate::org::allbinary::game::configuration::event::GameInitializedEventHandler::GameInitializedEventHandler;
use crate::org::allbinary::game::configuration::event::GameInitializedListenerInterface::GameInitializedListenerInterface;
use crate::org::allbinary::game::resource::FeatureResourceInitializationUtil::FeatureResourceInitializationUtil;
use crate::org::allbinary::graphics::threed::SWTJOGLProcessor::SWTJOGLProcessor;
use crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObject;
use crate::org::allbinary::logic::util::event::EventStrings::EventStrings;
use crate::java::lang::Thread::*;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct DefaultGameInitializationListener {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub swtJOGLProcessor: Option<Rc<RefCell<SWTJOGLProcessor>>>,
    pub firstTime: bool,
}

impl GameInitializedListenerInterface for DefaultGameInitializationListener {
    fn onGameInitialized(&mut self, mut gameInitializedEvent: Option<Rc<RefCell<GameInitializedEvent>>>) { Self::onGameInitialized(self, gameInitializedEvent) }
}
impl crate::org::allbinary::logic::util::event::EventListenerInterface::EventListenerInterface for DefaultGameInitializationListener {
    fn onEvent(&mut self, mut eventObject: Option<Rc<RefCell<dyn crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObjectDyn>>>) { Self::onEvent(self, eventObject) }
}

impl DefaultGameInitializationListener {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            logUtil: None,
            swtJOGLProcessor: None,
            firstTime: false,
        };
        this.logUtil = LogUtil::getInstance();
        this.swtJOGLProcessor = SWTJOGLProcessor::getInstance();
        this.firstTime = true;
        let mut gameInitializedEventHandler: Option<Rc<RefCell<GameInitializedEventHandler>>> = GameInitializedEventHandler::getInstance();
        jcall!(gameInitializedEventHandler).removeAllListeners();
        jcall!(gameInitializedEventHandler).addListenerInterface((this).clone() /* as GameInitializedListenerInterface */);
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn onEvent(&mut self, mut eventObject: Option<Rc<RefCell<dyn crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObjectDyn>>>) {
        ForcedLogUtil::log(jcall!(EventStrings::getInstance()).PERFORMANCE_MESSAGE.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())));
    }
    

    pub fn onGameInitialized(&mut self, mut gameInitializedEvent: Option<Rc<RefCell<GameInitializedEvent>>>) {
        let mut ON_GAME_INITIALIZED: String = String::from("onGameInitialized");
        let mut commonStrings: Option<Rc<RefCell<CommonStrings>>> = CommonStrings::getInstance();
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            jcall!(self.logUtil).putF(jcall!(commonStrings).START.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), ON_GAME_INITIALIZED.clone());
            while !jcall!(self.swtJOGLProcessor).isHolderCreated() {
                jcall!(self.logUtil).putF(jcall!(commonStrings).UPDATE.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), ON_GAME_INITIALIZED.clone());
                std::thread::sleep(std::time::Duration::from_millis((20) as u64));
            }
            jcall!(FeatureResourceInitializationUtil::getInstance()).init(jcall!(gameInitializedEvent).getLevel());
            if self.firstTime {
                self.firstTime = false;
            } else {
                jcall!(self.swtJOGLProcessor).onSurfaceChanged();
            }
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            jcall!(self.logUtil).put(jcall!(commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), ON_GAME_INITIALIZED.clone(), crate::java_into_object(e.clone()));
            }
        }
    }
}

