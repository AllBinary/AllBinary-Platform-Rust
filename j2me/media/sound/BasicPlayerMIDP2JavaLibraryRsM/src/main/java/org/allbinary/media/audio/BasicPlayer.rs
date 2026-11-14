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
use crate::javax::microedition::media::MediaException::MediaException;
use crate::javax::microedition::media::Player::Player;
use crate::javax::microedition::media::PlayerListener::PlayerListener;
use crate::javax::microedition::media::TimeBase::TimeBase;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
use super::TimeBaseInterface::TimeBaseInterface;
use super::Controllable2::Controllable2;
use super::NullControl::NullControl;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

//BasicPlayerMIDP2JavaLibrary
pub struct BasicPlayer {
    pub listenersList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>,
    pub timeBase: Option<Rc<RefCell<dyn TimeBase>>>,
    pub stateP: i32,
    pub loopCountP: i32,
}

pub static mut BasicPlayer_CONTROL_TYPE: std::cell::LazyCell<String> = std::cell::LazyCell::new(|| String::from("ToneControl"));
impl Player for BasicPlayer {
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
impl crate::javax::microedition::media::Controllable::Controllable for BasicPlayer {
    fn getControls(&mut self) -> Vec<Option<Rc<RefCell<dyn Control>>>> { Self::getControls(self) }
    fn getControl(&mut self, mut controlType: String) -> Option<Rc<RefCell<dyn Control>>> { Self::getControl(self, controlType) }
}
impl TimeBaseInterface for BasicPlayer {
    fn getTimeBase(&mut self) -> Option<Rc<RefCell<dyn TimeBase>>> { Self::getTimeBase(self) }
    fn setTimeBase(&mut self, mut timeBase: Option<Rc<RefCell<dyn TimeBase>>>) { Self::setTimeBase(self, timeBase) }
}
impl Controllable2 for BasicPlayer {
    fn setVolume(&mut self, mut leftVolume: i32, mut rightVolume: i32) { Self::setVolume(self, leftVolume, rightVolume) }
}

impl BasicPlayer {

    pub fn toString(&mut self) -> String {
        String::from(std::any::type_name::<Self>())
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            listenersList: None,
            timeBase: None,
            stateP: 0,
            loopCountP: 0,
        };
        this.listenersList = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        this.setLoopCount(0);
        { let java_arguments = (crate::javax::microedition::media::Player::Player_UNREALIZED.with(Clone::clone).clone(),); this.setState(java_arguments.0) };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getContentType(&mut self) -> String {
        return { let java_field = jcall!(StringUtil::getInstance()).EMPTY_STRING.clone(); java_field }.clone();
    }
    

    pub fn getControl(&mut self, mut controlType: String) -> Option<Rc<RefCell<dyn Control>>> {
        return NullControl::new().map(|reference| reference as Rc<RefCell<dyn Control>>).map(|reference| reference as Rc<RefCell<dyn Control>>);
    }
    

    pub fn getControls(&mut self) -> Vec<Option<Rc<RefCell<dyn Control>>>> {
        return vec![None; (0) as usize];
    }
    

    pub fn close(&mut self) {
    }
    

    // synchronized
    pub fn addPlayerListener(&mut self, mut playerListener: Option<Rc<RefCell<dyn PlayerListener>>>) {
        if !jcall!(self.listenersList).contains(crate::java_into_object(playerListener.clone())) {
            jcall!(self.listenersList).add(crate::java_into_object(playerListener.clone()));
        }
    }
    

    pub fn removePlayerListener(&mut self, mut playerListener: Option<Rc<RefCell<dyn PlayerListener>>>) {
        jcall!(self.listenersList).remove(crate::java_into_object(playerListener.clone()));
    }
    

    pub fn getState(&mut self) -> i32 {
        return self.stateP.clone();
    }
    

    // synchronized
    pub fn setState(&mut self, mut state: i32) {
        self.stateP = state.clone();
    }
    

    pub fn getDuration(&mut self) -> i64 {
        return 0_i64;
    }
    

    pub fn getMediaTime(&mut self) -> i64 {
        return 0_i64;
    }
    

    pub fn getTimeBase(&mut self) -> Option<Rc<RefCell<dyn TimeBase>>> {
        return self.timeBase.clone();
    }
    

    // synchronized
    pub fn setTimeBase(&mut self, mut timeBase: Option<Rc<RefCell<dyn TimeBase>>>) {
        self.timeBase = timeBase.clone();
    }
    

    pub fn deallocate(&mut self) {
    }
    

    // throws MediaException
    pub fn prefetch(&mut self) {
    }
    

    // throws MediaException
    pub fn realize(&mut self) {
    }
    

    // synchronized
    pub fn setLoopCount(&mut self, mut count: i32) {
        self.loopCountP = count.clone();
    }
    

    pub fn getLoopCount(&mut self) -> i32 {
        return self.loopCountP.clone();
    }
    

    // throws MediaException
    // synchronized
    pub fn setMediaTime(&mut self, mut now: i64) -> i64 {
        return 0_i64;
    }
    

    // throws MediaException
    // synchronized
    pub fn start(&mut self) {
        { let java_arguments = (crate::javax::microedition::media::Player::Player_STARTED.with(Clone::clone).clone(),); self.setState(java_arguments.0) };
    }
    

    // throws MediaException
    // synchronized
    pub fn stop(&mut self) {
        { let java_arguments = (crate::javax::microedition::media::Player::Player_PREFETCHED.with(Clone::clone).clone(),); self.setState(java_arguments.0) };
    }
    

    pub fn setVolume(&mut self, mut leftVolume: i32, mut rightVolume: i32) {
    }
}
pub trait BasicPlayerDyn: std::any::Any + crate::org::allbinary::media::audio::Controllable2::Controllable2 + crate::org::allbinary::media::audio::TimeBaseInterface::TimeBaseInterface {
    fn as_BasicPlayer(&self) -> &BasicPlayer;
    fn as_BasicPlayer_mut(&mut self) -> &mut BasicPlayer;
    fn getContentType(&mut self) -> String { BasicPlayer::getContentType(self.as_BasicPlayer_mut()) }
    fn getControl(&mut self, mut controlType: String) -> Option<Rc<RefCell<dyn Control>>> { BasicPlayer::getControl(self.as_BasicPlayer_mut(), controlType) }
    fn getControls(&mut self) -> Vec<Option<Rc<RefCell<dyn Control>>>> { BasicPlayer::getControls(self.as_BasicPlayer_mut()) }
    fn close(&mut self) { BasicPlayer::close(self.as_BasicPlayer_mut()) }
    fn addPlayerListener(&mut self, mut playerListener: Option<Rc<RefCell<dyn PlayerListener>>>) { BasicPlayer::addPlayerListener(self.as_BasicPlayer_mut(), playerListener) }
    fn removePlayerListener(&mut self, mut playerListener: Option<Rc<RefCell<dyn PlayerListener>>>) { BasicPlayer::removePlayerListener(self.as_BasicPlayer_mut(), playerListener) }
    fn getState(&mut self) -> i32 { BasicPlayer::getState(self.as_BasicPlayer_mut()) }
    fn setState(&mut self, mut state: i32) { BasicPlayer::setState(self.as_BasicPlayer_mut(), state) }
    fn getDuration(&mut self) -> i64 { BasicPlayer::getDuration(self.as_BasicPlayer_mut()) }
    fn getMediaTime(&mut self) -> i64 { BasicPlayer::getMediaTime(self.as_BasicPlayer_mut()) }
    fn deallocate(&mut self) { BasicPlayer::deallocate(self.as_BasicPlayer_mut()) }
    fn prefetch(&mut self) { BasicPlayer::prefetch(self.as_BasicPlayer_mut()) }
    fn realize(&mut self) { BasicPlayer::realize(self.as_BasicPlayer_mut()) }
    fn setLoopCount(&mut self, mut count: i32) { BasicPlayer::setLoopCount(self.as_BasicPlayer_mut(), count) }
    fn getLoopCount(&mut self) -> i32 { BasicPlayer::getLoopCount(self.as_BasicPlayer_mut()) }
    fn setMediaTime(&mut self, mut now: i64) -> i64 { BasicPlayer::setMediaTime(self.as_BasicPlayer_mut(), now) }
    fn start(&mut self) { BasicPlayer::start(self.as_BasicPlayer_mut()) }
    fn stop(&mut self) { BasicPlayer::stop(self.as_BasicPlayer_mut()) }
}
impl BasicPlayerDyn for BasicPlayer {
    fn as_BasicPlayer(&self) -> &BasicPlayer { self }
    fn as_BasicPlayer_mut(&mut self) -> &mut BasicPlayer { self }
}


