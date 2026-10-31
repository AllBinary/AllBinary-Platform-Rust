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
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use super::BasicPlayer::BasicPlayer;
use super::NullControl::NullControl;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct NoPlayer {
    pub base: BasicPlayer,
}

thread_local! { pub static NoPlayer_NO_PLAYER: Option<Rc<RefCell<NoPlayer>>> = NoPlayer::new(); }

impl NoPlayer {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: BasicPlayer::new_struct(),
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn close(&mut self) {
    }
    

    pub fn getContentType(&mut self) -> String {
        return jcall!(StringUtil::getInstance()).NULL_STRING.clone();
    }
    

    pub fn getControl(&mut self, mut controlType: String) -> Option<Rc<RefCell<Control>>> {
        return NullControl::new();
    }
    

    pub fn getControls(&mut self) -> Vec<Option<Rc<RefCell<Control>>>> {
        return vec![None; (0) as usize];
    }
}
impl std::ops::Deref for NoPlayer {
    type Target = BasicPlayer;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for NoPlayer {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::media::audio::BasicPlayer::BasicPlayerDyn for NoPlayer {
    fn as_BasicPlayer(&self) -> &crate::org::allbinary::media::audio::BasicPlayer::BasicPlayer { &self.base }
    fn as_BasicPlayer_mut(&mut self) -> &mut crate::org::allbinary::media::audio::BasicPlayer::BasicPlayer { &mut self.base }
    fn getContentType(&mut self) -> String { Self::getContentType(self) }
    fn getControl(&mut self, mut controlType: String) -> Option<Rc<RefCell<Control>>> { Self::getControl(self, controlType) }
    fn getControls(&mut self) -> Vec<Option<Rc<RefCell<Control>>>> { Self::getControls(self) }
    fn close(&mut self) { Self::close(self) }
}
impl crate::org::allbinary::media::audio::Controllable2::Controllable2 for NoPlayer {
    fn setVolume(&mut self, p1: i32, p2: i32) { crate::org::allbinary::media::audio::BasicPlayer::BasicPlayer::setVolume(crate::org::allbinary::media::audio::BasicPlayer::BasicPlayerDyn::as_BasicPlayer_mut(self), p1, p2) }
}


