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
// use crate::javax::microedition::media::Control::Control;
// use crate::javax::microedition::media::Controllable::Controllable;
// use crate::javax::microedition::media::Player::Player;
// use crate::javax::microedition::media::PlayerListener::PlayerListener;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::org::allbinary::time::GameTickTimeDelayHelper::GameTickTimeDelayHelper;
use crate::org::allbinary::time::GameTickTimeDelayHelperFactory::GameTickTimeDelayHelperFactory;
use crate::org::allbinary::time::TimeDelayHelper::TimeDelayHelper;
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
    pub player: Option<Rc<RefCell<Player>>>,
    pub timeElapsedHelper: Option<Rc<RefCell<dyn crate::org::allbinary::time::TimeDelayHelper::TimeDelayHelperDyn>>>,
    pub gameTickTimeDelayHelper: Option<Rc<RefCell<GameTickTimeDelayHelper>>>,
}

// impl Controllable for PlayerComposite with the methods of the trait
// impl Player for PlayerComposite with the methods of the trait

impl PlayerComposite {
    

    pub fn new_struct(mut player: Option<Rc<RefCell<Player>>>, mut repeatTime: i32) -> Self {
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

    pub fn new(mut player: Option<Rc<RefCell<Player>>>, mut repeatTime: i32) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(player, repeatTime))))
    }
    

    // synchronized
    pub fn addPlayerListener(&mut self, mut playerListener: Option<Rc<RefCell<PlayerListener>>>) {
        jcall!(self.player).addPlayerListener(playerListener.clone());
    }
    

    pub fn removePlayerListener(&mut self, mut playerListener: Option<Rc<RefCell<PlayerListener>>>) {
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
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("prefetch"), crate::java_into_object(e.clone()));
            }
        }
    }
    

    pub fn realize(&mut self) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            jcall!(self.player).realize();
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("realize"), crate::java_into_object(e.clone()));
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
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            return jcall!(self.player).setMediaTime(now.clone());
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("setMediaTime"), crate::java_into_object(e.clone()));
            return ((-1) as i64);
            }
        }
    }
    

    pub fn start(&mut self) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            if jcall!(self.timeElapsedHelper).isTime(jcall!(self.gameTickTimeDelayHelper).startTime.clone()) {
                jcall!(self.player).start();
            }
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).START_METHOD_NAME.clone(), crate::java_into_object(e.clone()));
            }
        }
    }
    

    pub fn stop(&mut self) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            jcall!(self.player).stop();
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("stop"), crate::java_into_object(e.clone()));
            }
        }
    }
    

    pub fn getControl(&mut self, mut controlType: String) -> Option<Rc<RefCell<Control>>> {
        return jcall!(self.player).getControl(controlType.clone());
    }
    

    pub fn getControls(&mut self) -> Vec<Option<Rc<RefCell<Control>>>> {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    pub fn setVolume(&mut self, mut leftVolume: i32, mut rightVolume: i32) {
        let mut controllable2: Option<Rc<RefCell<dyn Controllable2>>> = ((self.player).clone() /* as Controllable2 */);
        jcall!(controllable2).setVolume(leftVolume.clone(), rightVolume.clone());
    }
    

    pub fn getPlayerP(&mut self) -> Option<Rc<RefCell<Player>>> {
        return self.player.clone();
    }
}

