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
// package org.allbinary.media.audio
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::javax::microedition::media::Control::Control;
use crate::javax::microedition::media::Controllable::Controllable;
use crate::javax::microedition::media::Player::Player;
use crate::javax::microedition::media::PlayerListener::PlayerListener;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::org::allbinary::time::GameTickTimeDelayHelper::GameTickTimeDelayHelper;
use crate::org::allbinary::time::GameTickTimeDelayHelperFactory::GameTickTimeDelayHelperFactory;
use crate::org::allbinary::time::TimeDelayHelper::TimeDelayHelper;
use crate::java::lang::Exception::*;
use crate::java::lang::RuntimeException::*;
use super::Controllable2::Controllable2;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

//BasicPlayer
pub struct PlayerComposite {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub commonStrings: Option<Rc<RefCell<CommonStrings>>>,
    pub player: Option<Rc<RefCell<dyn Player>>>,
    pub timeElapsedHelper: Option<Rc<RefCell<dyn crate::org::allbinary::time::TimeDelayHelper::TimeDelayHelperDyn>>>,
    pub gameTickTimeDelayHelper: Option<Rc<RefCell<GameTickTimeDelayHelper>>>,
}

impl Controllable for PlayerComposite {
    fn getControls(&mut self) -> Vec<Option<Rc<RefCell<dyn Control>>>> { Self::getControls(self) }
    fn getControl(&mut self, mut controlType: String) -> Option<Rc<RefCell<dyn Control>>> { Self::getControl(self, controlType) }
}
impl Player for PlayerComposite {
    fn realize(&mut self) { Self::realize(self) }
    fn prefetch(&mut self) { Self::prefetch(self) }
    fn start(&mut self) { Self::start(self) }
    fn stop(&mut self) { Self::stop(self) }
    fn deallocate(&mut self) { Self::deallocate(self) }
    fn close(&mut self) { Self::close(self) }
    fn setMediaTime(&mut self, mut now: i64) -> i64 { Self::setMediaTime(self, now) }
    fn getMediaTime(&mut self) -> i64 { Self::getMediaTime(self) }
    fn getState(&mut self) -> i32 { Self::getState(self) }
    fn getDuration(&mut self) -> i64 { Self::getDuration(self) }
    fn getContentType(&mut self) -> String { Self::getContentType(self) }
    fn setLoopCount(&mut self, mut count: i32) { Self::setLoopCount(self, count) }
    fn addPlayerListener(&mut self, mut playerListener: Option<Rc<RefCell<dyn PlayerListener>>>) { Self::addPlayerListener(self, playerListener) }
    fn removePlayerListener(&mut self, mut playerListener: Option<Rc<RefCell<dyn PlayerListener>>>) { Self::removePlayerListener(self, playerListener) }
}

impl PlayerComposite {

    pub fn toString(&mut self) -> String {
        String::from(std::any::type_name::<Self>())
    }
    

    pub fn new_struct(mut player: Option<Rc<RefCell<dyn Player>>>, mut repeatTime: i32) -> Self {
        let mut this = Self {
            logUtil: None,
            commonStrings: None,
            player: None,
            timeElapsedHelper: None,
            gameTickTimeDelayHelper: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.commonStrings = CommonStrings::getInstance();
        this.timeElapsedHelper = TimeDelayHelper::new(0).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::time::TimeDelayHelper::TimeDelayHelperDyn>>);
        this.gameTickTimeDelayHelper = GameTickTimeDelayHelperFactory::getInstance();
        this.player = player.clone();
        jcall!(this.timeElapsedHelper).as_TimeDelayHelper_mut().delay = repeatTime.clone();
        this
    }

    pub fn new(mut player: Option<Rc<RefCell<dyn Player>>>, mut repeatTime: i32) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(player, repeatTime))))
    }
    

    // synchronized
    pub fn addPlayerListener(&mut self, mut playerListener: Option<Rc<RefCell<dyn PlayerListener>>>) {
        jcall!(self.player).addPlayerListener(playerListener.clone());
    }
    

    pub fn removePlayerListener(&mut self, mut playerListener: Option<Rc<RefCell<dyn PlayerListener>>>) {
        jcall!(self.player).removePlayerListener(playerListener.clone());
    }
    

    pub fn close(&mut self) {
        jcall!(self.player).close();
    }
    

    pub fn deallocate(&mut self) {
        jcall!(self.player).deallocate();
    }
    

    pub fn getContentType(&mut self) -> String {
        return jcall!(self.player).getContentType();
    }
    

    pub fn getDuration(&mut self) -> i64 {
        return jcall!(self.player).getDuration();
    }
    

    pub fn getMediaTime(&mut self) -> i64 {
        return jcall!(self.player).getMediaTime();
    }
    

    pub fn prefetch(&mut self) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            jcall!(self.player).prefetch();
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            jcall!(self.logUtil).put({ let java_field = jcall!(self.commonStrings).EXCEPTION.clone(); java_field }.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("prefetch"), crate::java_into_object(e.clone()));
            }
        }
    }
    

    pub fn realize(&mut self) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            jcall!(self.player).realize();
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            jcall!(self.logUtil).put({ let java_field = jcall!(self.commonStrings).EXCEPTION.clone(); java_field }.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("realize"), crate::java_into_object(e.clone()));
            }
        }
    }
    

    pub fn getState(&mut self) -> i32 {
        return jcall!(self.player).getState();
    }
    

    pub fn setLoopCount(&mut self, mut count: i32) {
        jcall!(self.player).setLoopCount(count.clone());
    }
    

    pub fn setMediaTime(&mut self, mut now: i64) -> i64 {
        let java_try_result = match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            return Some(jcall!(self.player).setMediaTime(now.clone()));
        })) {
            Ok(java_return) => java_return,
            Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            jcall!(self.logUtil).put({ let java_field = jcall!(self.commonStrings).EXCEPTION.clone(); java_field }.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("setMediaTime"), crate::java_into_object(e.clone()));
            return ((-1) as i64);
            None
            }
        };if let Some(java_return) = java_try_result { return java_return; }unreachable!("try statement")
    }
    

    pub fn start(&mut self) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            if jcall!(self.timeElapsedHelper).isTime({ let java_field = jcall!(self.gameTickTimeDelayHelper).startTime.clone(); java_field }.clone()) {
                jcall!(self.player).start();
            }
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            jcall!(self.logUtil).put({ let java_field = jcall!(self.commonStrings).EXCEPTION.clone(); java_field }.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(self.commonStrings).START_METHOD_NAME.clone(); java_field }.clone(), crate::java_into_object(e.clone()));
            }
        }
    }
    

    pub fn stop(&mut self) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            jcall!(self.player).stop();
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            jcall!(self.logUtil).put({ let java_field = jcall!(self.commonStrings).EXCEPTION.clone(); java_field }.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("stop"), crate::java_into_object(e.clone()));
            }
        }
    }
    

    pub fn getControl(&mut self, mut controlType: String) -> Option<Rc<RefCell<dyn Control>>> {
        return jcall!(self.player).getControl(controlType.clone());
    }
    

    pub fn getControls(&mut self) -> Vec<Option<Rc<RefCell<dyn Control>>>> {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    pub fn setVolume(&mut self, mut leftVolume: i32, mut rightVolume: i32) {
        let mut controllable2: Option<Rc<RefCell<dyn Controllable2>>> = ((|| -> Option<Rc<RefCell<dyn Controllable2>>> { panic!("ClassCastException: a Rust dyn Player cannot be cast to Controllable2") })());
        jcall!(controllable2).setVolume(leftVolume.clone(), rightVolume.clone());
    }
    

    pub fn getPlayerP(&mut self) -> Option<Rc<RefCell<dyn Player>>> {
        return self.player.clone();
    }
}

