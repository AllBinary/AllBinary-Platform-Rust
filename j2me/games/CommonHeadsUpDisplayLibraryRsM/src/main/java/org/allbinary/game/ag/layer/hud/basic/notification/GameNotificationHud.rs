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
// use crate::javax::microedition::lcdui::Graphics::Graphics;
use crate::org::allbinary::game::graphics::hud::BasicHud::BasicHud;
use crate::org::allbinary::game::graphics::hud::BasicHudFactory::BasicHudFactory;
use crate::org::allbinary::game::layer::hud::event::GameNotificationEvent::GameNotificationEvent;
use crate::org::allbinary::game::layer::hud::event::GameNotificationListenerInterface::GameNotificationListenerInterface;
use crate::org::allbinary::graphics::color::BasicColor::BasicColor;
use crate::org::allbinary::graphics::color::BasicColorFactory::BasicColorFactory;
use crate::org::allbinary::logic::communication::log::ForcedLogUtil::ForcedLogUtil;
use crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObject;
use crate::org::allbinary::logic::util::event::EventStrings::EventStrings;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct GameNotificationHud {
    pub base: BasicHud,
    pub METHOD_NAME: String,
    pub lastGameNotificationEvent: Option<Rc<RefCell<GameNotificationEvent>>>,
}

thread_local! { pub static GameNotificationHud_NULL_GAME_NOTIFICATION: Option<Rc<RefCell<dyn GameNotificationHudDyn>>> = GameNotificationHud::new(jcall!(BasicHudFactory::getInstance()).TOPCENTER.clone(), jcall!(BasicHudFactory::getInstance()).HORIZONTAL.clone(), 0, jcall!(BasicColorFactory::getInstance()).RED.clone()).map(|reference| reference as Rc<RefCell<dyn GameNotificationHudDyn>>); }
impl GameNotificationListenerInterface for GameNotificationHud {
    fn onGameNotificationEvent(&mut self, mut gameNotificationEvent: Option<Rc<RefCell<GameNotificationEvent>>>) { Self::onGameNotificationEvent(self, gameNotificationEvent) }
}
impl crate::org::allbinary::logic::util::event::EventListenerInterface::EventListenerInterface for GameNotificationHud {
    fn onEvent(&mut self, mut eventObject: Option<Rc<RefCell<dyn crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObjectDyn>>>) { Self::onEvent(self, eventObject) }
}

impl GameNotificationHud {
    

    pub fn new_struct(mut location: i32, mut direction: i32, mut bufferZone: i32, mut basicColor: Option<Rc<RefCell<BasicColor>>>) -> Self {
        let mut this = Self {
            base: BasicHud::new_struct(location.clone(), direction.clone(), bufferZone.clone(), basicColor.clone()),
            METHOD_NAME: String::new(),
            lastGameNotificationEvent: None,
        };
        this.METHOD_NAME = String::from("onGameNotificationEvent");
        this.lastGameNotificationEvent = GameNotification_NULL_GAME_NOTIFICATION_EVENT.with(Clone::clone).clone();
        // super(...) is in the struct literal above
        this
    }

    pub fn new(mut location: i32, mut direction: i32, mut bufferZone: i32, mut basicColor: Option<Rc<RefCell<BasicColor>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(location, direction, bufferZone, basicColor))))
    }
    

    pub fn onEvent(&mut self, mut eventObject: Option<Rc<RefCell<dyn crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObjectDyn>>>) {
        ForcedLogUtil::log(jcall!(EventStrings::getInstance()).PERFORMANCE_MESSAGE.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())));
    }
    

    // throws Exception
    pub fn onGameNotificationEvent(&mut self, mut gameNotificationEvent: Option<Rc<RefCell<GameNotificationEvent>>>) {
        if !match (&self.lastGameNotificationEvent, &gameNotificationEvent) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            self.lastGameNotificationEvent = gameNotificationEvent.clone();
            { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(gameNotificationEvent).getString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), self.METHOD_NAME.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        }
        { let java_arguments = (jcall!(gameNotificationEvent).getString(), jcall!(gameNotificationEvent).getSeconds(), jcall!(gameNotificationEvent).getBasicColorP(), jcall!(gameNotificationEvent).getPermanent(),); self.add(java_arguments.0, java_arguments.1, java_arguments.2, java_arguments.3) };
    }
    

    pub fn add(&mut self, mut string: String, mut seconds: Option<i32>, mut basicColor: Option<Rc<RefCell<BasicColor>>>, mut permanent: Option<bool>) {
    }
    

    // throws Exception
    pub fn processTick(&mut self) {
    }
    

    pub fn paint(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) {
    }
    

    pub fn clear(&mut self) {
    }
}
impl std::ops::Deref for GameNotificationHud {
    type Target = BasicHud;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for GameNotificationHud {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

pub trait GameNotificationHudDyn: crate::org::allbinary::game::graphics::hud::BasicHud::BasicHudDyn {
    fn as_GameNotificationHud(&self) -> &GameNotificationHud;
    fn as_GameNotificationHud_mut(&mut self) -> &mut GameNotificationHud;
    fn onGameNotificationEvent(&mut self, mut gameNotificationEvent: Option<Rc<RefCell<GameNotificationEvent>>>) { GameNotificationHud::onGameNotificationEvent(self.as_GameNotificationHud_mut(), gameNotificationEvent) }
    fn add(&mut self, mut string: String, mut seconds: Option<i32>, mut basicColor: Option<Rc<RefCell<BasicColor>>>, mut permanent: Option<bool>) { GameNotificationHud::add(self.as_GameNotificationHud_mut(), string, seconds, basicColor, permanent) }
    fn processTick(&mut self) { GameNotificationHud::processTick(self.as_GameNotificationHud_mut()) }
    fn paint(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) { GameNotificationHud::paint(self.as_GameNotificationHud_mut(), graphics) }
    fn clear(&mut self) { GameNotificationHud::clear(self.as_GameNotificationHud_mut()) }
}
impl GameNotificationHudDyn for GameNotificationHud {
    fn as_GameNotificationHud(&self) -> &GameNotificationHud { self }
    fn as_GameNotificationHud_mut(&mut self) -> &mut GameNotificationHud { self }
}

impl crate::org::allbinary::game::graphics::hud::BasicHud::BasicHudDyn for GameNotificationHud {
    fn as_BasicHud(&self) -> &crate::org::allbinary::game::graphics::hud::BasicHud::BasicHud { &self.base }
    fn as_BasicHud_mut(&mut self) -> &mut crate::org::allbinary::game::graphics::hud::BasicHud::BasicHud { &mut self.base }
    fn onEvent(&mut self, mut eventObject: Option<Rc<RefCell<dyn crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObjectDyn>>>) { Self::onEvent(self, eventObject) }
}


