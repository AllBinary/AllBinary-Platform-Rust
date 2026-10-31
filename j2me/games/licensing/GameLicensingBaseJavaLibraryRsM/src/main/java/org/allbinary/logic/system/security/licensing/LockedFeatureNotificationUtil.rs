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
// package org.allbinary.logic.system.security.licensing
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::game::layer::hud::event::GameNotificationEvent::GameNotificationEvent;
use crate::org::allbinary::game::layer::hud::event::GameNotificationEventHandler::GameNotificationEventHandler;
use crate::org::allbinary::graphics::color::BasicColor::BasicColor;
use crate::org::allbinary::graphics::color::BasicColorFactory::BasicColorFactory;
use crate::org::allbinary::logic::java::bool::BooleanFactory::BooleanFactory;
use crate::org::allbinary::logic::math::SmallIntegerSingletonFactory::SmallIntegerSingletonFactory;
use crate::org::allbinary::media::audio::ErrorSound::ErrorSound;
use crate::org::allbinary::media::audio::PrimaryPlayerQueueFactory::PrimaryPlayerQueueFactory;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct LockedFeatureNotificationUtil {
    pub gameNotificationEvent: Option<Rc<RefCell<GameNotificationEvent>>>,
}

thread_local! { pub static LockedFeatureNotificationUtil_instance: Option<Rc<RefCell<LockedFeatureNotificationUtil>>> = LockedFeatureNotificationUtil::new(); }

impl LockedFeatureNotificationUtil {

    pub fn new_struct() -> Self {
        let mut this = Self {
            gameNotificationEvent: None,
        };
        this.gameNotificationEvent = GameNotificationEvent::new(this, jcall!(LicenseStrings::LicenseStrings::getInstance()).LOCKED.clone(), jcall!(SmallIntegerSingletonFactory::getInstance()).getAt(3), jcall!(BasicColorFactory::getInstance()).RED.clone(), jcall!(BooleanFactory::getInstance()).FALSE.clone());
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<LockedFeatureNotificationUtil>>> {
        return LockedFeatureNotificationUtil_instance.with(Clone::clone).clone();
    }
    

    // throws Exception
    pub fn fire(&mut self) {
        jcall!(PrimaryPlayerQueueFactory::getInstance()).add(ErrorSound::getInstance());
        jcall!(GameNotificationEventHandler::getInstance()).fireEvent(self.gameNotificationEvent.clone());
    }
    

    // throws Exception
    pub fn fireWithColor(&mut self, mut basicColor: Option<Rc<RefCell<BasicColor>>>) {
        jcall!(PrimaryPlayerQueueFactory::getInstance()).add(ErrorSound::getInstance());
        jcall!(self.gameNotificationEvent).setBasicColorP(basicColor.clone());
        jcall!(GameNotificationEventHandler::getInstance()).fireEvent(self.gameNotificationEvent.clone());
    }
}

