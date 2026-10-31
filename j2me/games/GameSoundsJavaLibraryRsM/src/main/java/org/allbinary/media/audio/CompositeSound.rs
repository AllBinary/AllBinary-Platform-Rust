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
use super::Sound::Sound;
use super::PlayerComposite::PlayerComposite;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct CompositeSound {
    pub base: Sound,
}


impl CompositeSound {
    

    pub fn new_struct(mut resource: String) -> Self {
        let mut this = Self {
            base: Sound::new_struct(resource.clone()),
        };
        // super(...) is in the struct literal above
        this
    }

    pub fn new(mut resource: String) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(resource))))
    }
    

    // throws Exception
    pub fn init(&mut self) {
        { let java_arguments = (PlayerComposite::new(AllBinaryMediaManager::AllBinaryMediaManager::createPlayer(self.getResource()), 570),); self.setPlayerP(java_arguments.0) };
    }
}
impl std::ops::Deref for CompositeSound {
    type Target = Sound;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for CompositeSound {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

pub trait CompositeSoundDyn: crate::org::allbinary::media::audio::Sound::SoundDyn {
    fn as_CompositeSound(&self) -> &CompositeSound;
    fn as_CompositeSound_mut(&mut self) -> &mut CompositeSound;
}
impl CompositeSoundDyn for CompositeSound {
    fn as_CompositeSound(&self) -> &CompositeSound { self }
    fn as_CompositeSound_mut(&mut self) -> &mut CompositeSound { self }
}

impl crate::org::allbinary::media::audio::Sound::SoundDyn for CompositeSound {
    fn as_Sound(&self) -> &crate::org::allbinary::media::audio::Sound::Sound { &self.base }
    fn as_Sound_mut(&mut self) -> &mut crate::org::allbinary::media::audio::Sound::Sound { &mut self.base }
    fn init(&mut self) { Self::init(self) }
}
impl crate::Runnable for CompositeSound {
    fn run(&mut self) { crate::org::allbinary::media::audio::Sound::Sound::run(crate::org::allbinary::media::audio::Sound::SoundDyn::as_Sound_mut(self)) }
}


