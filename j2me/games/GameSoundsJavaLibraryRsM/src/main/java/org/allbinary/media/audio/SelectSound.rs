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
use super::CompositeSound::CompositeSound;
use super::Sound::Sound;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

//Use 8hz16bitmono for most J2ME
pub struct SelectSound {
    pub base: CompositeSound,
}

pub static mut SelectSound_soundInterface: std::cell::LazyCell<Option<Rc<RefCell<dyn crate::org::allbinary::media::audio::Sound::SoundDyn>>>> = std::cell::LazyCell::new(|| SelectSound::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::media::audio::Sound::SoundDyn>>));

impl SelectSound {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: CompositeSound::new_struct(String::from("resource:/wav/select.wav")),
        };
        // super(...) is in the struct literal above
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<dyn crate::org::allbinary::media::audio::Sound::SoundDyn>>> {
        return unsafe { SelectSound_soundInterface.clone() }.clone();
    }
}
impl std::ops::Deref for SelectSound {
    type Target = CompositeSound;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for SelectSound {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::media::audio::CompositeSound::CompositeSoundDyn for SelectSound {
    fn as_CompositeSound(&self) -> &crate::org::allbinary::media::audio::CompositeSound::CompositeSound { &self.base }
    fn as_CompositeSound_mut(&mut self) -> &mut crate::org::allbinary::media::audio::CompositeSound::CompositeSound { &mut self.base }
}
impl crate::org::allbinary::media::audio::Sound::SoundDyn for SelectSound {
    fn as_Sound(&self) -> &crate::org::allbinary::media::audio::Sound::Sound { &self.base.base }
    fn as_Sound_mut(&mut self) -> &mut crate::org::allbinary::media::audio::Sound::Sound { &mut self.base.base }
    fn init(&mut self) { crate::org::allbinary::media::audio::CompositeSound::CompositeSound::init(crate::org::allbinary::media::audio::CompositeSound::CompositeSoundDyn::as_CompositeSound_mut(self)) }
}
impl crate::Runnable for SelectSound {
    fn run(&mut self) { crate::org::allbinary::media::audio::Sound::Sound::run(crate::org::allbinary::media::audio::Sound::SoundDyn::as_Sound_mut(self)) }
}


