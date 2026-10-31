// 
// AllBinary Open License Version 1 
// Copyright (c) 2022 AllBinary 
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
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use super::Sound::Sound;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct NoSound {
    pub base: Sound,
}

thread_local! { pub static NoSound_instance: Option<Rc<RefCell<NoSound>>> = NoSound::new(); }

impl NoSound {
    

    pub fn getInstance() -> Option<Rc<RefCell<NoSound>>> {
        return NoSound_instance.with(Clone::clone).clone();
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: Sound::new_struct(jcall!(StringUtil::getInstance()).EMPTY_STRING.clone()),
        };
        // super(...) is in the struct literal above
        this.setPlayerP(NoPlayer_NO_PLAYER.with(Clone::clone).clone());
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
}
impl std::ops::Deref for NoSound {
    type Target = Sound;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for NoSound {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::media::audio::Sound::SoundDyn for NoSound {
    fn as_Sound(&self) -> &crate::org::allbinary::media::audio::Sound::Sound { &self.base }
    fn as_Sound_mut(&mut self) -> &mut crate::org::allbinary::media::audio::Sound::Sound { &mut self.base }
}
impl crate::Runnable for NoSound {
    fn run(&mut self) { crate::org::allbinary::media::audio::Sound::Sound::run(crate::org::allbinary::media::audio::Sound::SoundDyn::as_Sound_mut(self)) }
}


