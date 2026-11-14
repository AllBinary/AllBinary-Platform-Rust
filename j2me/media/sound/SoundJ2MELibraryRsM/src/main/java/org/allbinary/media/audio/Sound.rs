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
use crate::javax::microedition::media::Player::Player;
use crate::org::allbinary::logic::communication::log::PreLogUtil::PreLogUtil;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::java::lang::Exception::*;
use super::SoundInterface::SoundInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct Sound {
    pub resource: String,
    pub player: Option<Rc<RefCell<dyn Player>>>,
}

impl SoundInterface for Sound {
    fn getResource(&mut self) -> String { Self::getResource(self) }
    fn getPlayerP(&mut self) -> Option<Rc<RefCell<dyn Player>>> { Self::getPlayerP(self) }
    fn init(&mut self) { Self::init(self) }
}
impl Runnable for Sound {
    fn run(&mut self) { Self::run(self) }
}

impl Sound {

    pub fn toString(&mut self) -> String {
        String::from(std::any::type_name::<Self>())
    }
    

    pub fn new_struct(mut resource: String) -> Self {
        let mut this = Self {
            resource: String::new(),
            player: None,
        };
        this.player = NoPlayer_NO_PLAYER.with(Clone::clone).clone().map(|reference| reference as Rc<RefCell<dyn Player>>);
        this.resource = resource.clone();
        this
    }

    pub fn new(mut resource: String) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(resource))))
    }
    

    pub fn getResource(&mut self) -> String {
        return self.resource.clone();
    }
    

    pub fn getPlayerP(&mut self) -> Option<Rc<RefCell<dyn Player>>> {
        return self.player.clone();
    }
    

    pub fn setPlayerP(&mut self, mut player: Option<Rc<RefCell<dyn Player>>>) {
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
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            let mut commonStrings: Option<Rc<RefCell<CommonStrings>>> = CommonStrings::getInstance();
            PreLogUtil::put({ let java_field = jcall!(commonStrings).EXCEPTION.clone(); java_field }.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(commonStrings).RUN.clone(); java_field }.clone());
            }
        }
    }
}
pub trait SoundDyn: std::any::Any + crate::org::allbinary::media::audio::SoundInterface::SoundInterface + crate::Runnable {
    fn as_Sound(&self) -> &Sound;
    fn as_Sound_mut(&mut self) -> &mut Sound;
    fn setPlayerP(&mut self, mut player: Option<Rc<RefCell<dyn Player>>>) { Sound::setPlayerP(self.as_Sound_mut(), player) }
    fn getDuration(&mut self) -> i32 { Sound::getDuration(self.as_Sound_mut()) }
}
impl SoundDyn for Sound {
    fn as_Sound(&self) -> &Sound { self }
    fn as_Sound_mut(&mut self) -> &mut Sound { self }
}


