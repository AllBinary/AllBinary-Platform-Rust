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
// package org.allbinary.game.input
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::game::layer::hud::event::GameNotificationEvent::GameNotificationEvent;
use crate::org::allbinary::game::layer::hud::event::GameNotificationEventHandler::GameNotificationEventHandler;
use crate::org::allbinary::graphics::color::BasicColorFactory::BasicColorFactory;
use crate::org::allbinary::logic::java::bool::BooleanFactory::BooleanFactory;
use crate::org::allbinary::logic::math::SmallIntegerSingletonFactory::SmallIntegerSingletonFactory;
use crate::org::allbinary::media::audio::ErrorSound::ErrorSound;
use crate::org::allbinary::media::audio::PrimaryPlayerQueueFactory::PrimaryPlayerQueueFactory;
use crate::org::allbinary::media::audio::SelectSound::SelectSound;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct TextNotificationUtil {
    pub TWO: Option<i32>,
    pub FALSE: Option<bool>,
    pub gameNotificationEventHandler: Option<Rc<RefCell<GameNotificationEventHandler>>>,
}

thread_local! { pub static TextNotificationUtil_instance: Option<Rc<RefCell<TextNotificationUtil>>> = TextNotificationUtil::new(); }

impl TextNotificationUtil {

    pub fn new_struct() -> Self {
        let mut this = Self {
            TWO: None,
            FALSE: None,
            gameNotificationEventHandler: None,
        };
        this.TWO = jcall!(SmallIntegerSingletonFactory::getInstance()).getAt(2);
        this.FALSE = jcall!(BooleanFactory::getInstance()).FALSE.clone();
        this.gameNotificationEventHandler = GameNotificationEventHandler::getInstance();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<TextNotificationUtil>>> {
        return TextNotificationUtil_instance.with(Clone::clone).clone();
    }
    

    // throws Exception
    pub fn fireError(&mut self, mut message: String) {
        jcall!(PrimaryPlayerQueueFactory::getInstance()).add(ErrorSound::getInstance());
        self.fire(message.clone());
    }
    

    // throws Exception
    pub fn fireNew(&mut self, mut message: String) {
        jcall!(PrimaryPlayerQueueFactory::getInstance()).add(SelectSound::getInstance());
        self.fire(message.clone());
    }
    

    // throws Exception
    pub fn fireSuccess(&mut self, mut message: String) {
        jcall!(PrimaryPlayerQueueFactory::getInstance()).add(SelectSound::getInstance());
        self.fire(message.clone());
    }
    

    // throws Exception
    pub fn fire(&mut self, mut message: String) {
        let mut gameNotificationEvent: Option<Rc<RefCell<GameNotificationEvent>>> = GameNotificationEvent::new(None /* Java this: a &mut self method has no Rc of itself */, message.clone(), self.TWO.clone(), jcall!(BasicColorFactory::getInstance()).RED.clone(), self.FALSE.clone());
        jcall!(self.gameNotificationEventHandler).fireEvent(gameNotificationEvent.clone());
    }
}

