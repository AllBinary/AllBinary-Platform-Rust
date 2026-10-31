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
use crate::Runnable;
use super::*;
// use crate::javax::microedition::media::Player::Player;
use crate::org::allbinary::logic::communication::log::PreLogUtil::PreLogUtil;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use super::SoundInterface::SoundInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct Sound {
    pub resource: String,
    pub player: Option<Rc<RefCell<Player>>>,
}

impl SoundInterface for Sound {
    fn getResource(&mut self) -> String { Self::getResource(self) }
    fn getPlayerP(&mut self) -> Option<Rc<RefCell<Player>>> { Self::getPlayerP(self) }
    fn init(&mut self) { Self::init(self) }
}
impl Runnable for Sound {
    fn run(&mut self) { Self::run(self) }
}

impl Sound {
    

    pub fn new_struct(mut resource: String) -> Self {
        let mut this = Self {
            resource: String::new(),
            player: None,
        };
        this.player = NoPlayer_NO_PLAYER.with(Clone::clone).clone();
        this.resource = resource.clone();
        this
    }

    pub fn new(mut resource: String) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(resource))))
    }
    

    pub fn getResource(&mut self) -> String {
        return self.resource.clone();
    }
    

    pub fn getPlayerP(&mut self) -> Option<Rc<RefCell<Player>>> {
        return self.player.clone();
    }
    

    pub fn setPlayerP(&mut self, mut player: Option<Rc<RefCell<Player>>>) {
        self.player = player.clone();
    }
    

    // throws Exception
    pub fn init(&mut self) {
    }
    

    pub fn getDuration(&mut self) -> i32 {
        return -1;
    }
    

    pub fn run(&mut self) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            jcall!(self.player).start();
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            let mut commonStrings: Option<Rc<RefCell<CommonStrings>>> = CommonStrings::getInstance();
            PreLogUtil::put(jcall!(commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(commonStrings).RUN.clone());
            }
        }
    }
}
pub trait SoundDyn: crate::Runnable {
    fn as_Sound(&self) -> &Sound;
    fn as_Sound_mut(&mut self) -> &mut Sound;
    fn getResource(&mut self) -> String { Sound::getResource(self.as_Sound_mut()) }
    fn getPlayerP(&mut self) -> Option<Rc<RefCell<Player>>> { Sound::getPlayerP(self.as_Sound_mut()) }
    fn setPlayerP(&mut self, mut player: Option<Rc<RefCell<Player>>>) { Sound::setPlayerP(self.as_Sound_mut(), player) }
    fn init(&mut self) { Sound::init(self.as_Sound_mut()) }
    fn getDuration(&mut self) -> i32 { Sound::getDuration(self.as_Sound_mut()) }
}
impl SoundDyn for Sound {
    fn as_Sound(&self) -> &Sound { self }
    fn as_Sound_mut(&mut self) -> &mut Sound { self }
}


