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
// package org.allbinary.game.ag.layer.hud.basic.notification
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::game::graphics::hud::BasicHudFactory::BasicHudFactory;
use crate::org::allbinary::graphics::color::BasicColorFactory::BasicColorFactory;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use super::GameNotificationHud::GameNotificationHud;
use super::NoGameGameNotificationHud::NoGameGameNotificationHud;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct NoGameNotificationHudSingleton {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub gameNotificationHud: Option<Rc<RefCell<dyn crate::org::allbinary::game::ag::layer::hud::basic::notification::GameNotificationHud::GameNotificationHudDyn>>>,
}

thread_local! { pub static NoGameNotificationHudSingleton_instance: Option<Rc<RefCell<NoGameNotificationHudSingleton>>> = NoGameNotificationHudSingleton::new(); }

impl NoGameNotificationHudSingleton {
    

    pub fn getInstance() -> Option<Rc<RefCell<NoGameNotificationHudSingleton>>> {
        return NoGameNotificationHudSingleton_instance.with(Clone::clone).clone();
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            logUtil: None,
            gameNotificationHud: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.gameNotificationHud = GameNotificationHud_NULL_GAME_NOTIFICATION.with(Clone::clone).clone();
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut basicHudFactory: Option<Rc<RefCell<BasicHudFactory>>> = BasicHudFactory::getInstance();
            /* Java local class: move it out of the function
pub struct NoGameGameNotificationHud {
    pub base: GameNotificationHud,
}


impl NoGameGameNotificationHud {
                

                pub fn new_struct() -> Self {
                    let mut this = Self {
            base: GameNotificationHud::new_struct(jcall!(this.base.basicHudFactory).TOPCENTER.clone(), jcall!(this.base.basicHudFactory).HORIZONTAL.clone(), 2, jcall!(BasicColorFactory::getInstance()).RED.clone()),
                    };
                    // super(...) is in the struct literal above
                    this.updateMaxWidth = 40;
                    this.updateMaxHeight = 14;
                    this
                }

                pub fn new() -> Option<Rc<RefCell<Self>>> {
                    Some(Rc::new(RefCell::new(Self::new_struct())))
                }
}
impl std::ops::Deref for NoGameGameNotificationHud {
    type Target = GameNotificationHud;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for NoGameGameNotificationHud {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::game::ag::layer::hud::basic::notification::GameNotificationHud::GameNotificationHudDyn for NoGameGameNotificationHud {
    fn as_GameNotificationHud(&self) -> &crate::org::allbinary::game::ag::layer::hud::basic::notification::GameNotificationHud::GameNotificationHud { &self }
    fn as_GameNotificationHud_mut(&mut self) -> &mut crate::org::allbinary::game::ag::layer::hud::basic::notification::GameNotificationHud::GameNotificationHud { &mut self }
}
impl crate::org::allbinary::game::graphics::hud::BasicHud::BasicHudDyn for NoGameGameNotificationHud {
    fn as_BasicHud(&self) -> &crate::org::allbinary::game::graphics::hud::BasicHud::BasicHud { &self }
    fn as_BasicHud_mut(&mut self) -> &mut crate::org::allbinary::game::graphics::hud::BasicHud::BasicHud { &mut self }
    fn onEvent(&mut self, p1: Option<Rc<RefCell<dyn crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObjectDyn>>>) { crate::org::allbinary::game::ag::layer::hud::basic::notification::GameNotificationHud::GameNotificationHud::onEvent(crate::org::allbinary::game::ag::layer::hud::basic::notification::GameNotificationHud::GameNotificationHudDyn::as_GameNotificationHud_mut(self), p1) }
}

*/
            this.gameNotificationHud = NoGameGameNotificationHud::new();
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            let mut commonStrings: Option<Rc<RefCell<CommonStrings>>> = CommonStrings::getInstance();
            jcall!(this.logUtil).put(jcall!(commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(commonStrings).CONSTRUCTOR.clone(), crate::java_into_object(e.clone()));
            }
        }
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getGameNotificationHud(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::game::ag::layer::hud::basic::notification::GameNotificationHud::GameNotificationHudDyn>>> {
        return self.gameNotificationHud.clone();
    }
}

