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
use super::BaseSoundsFactory::BaseSoundsFactory;
use super::Sound::Sound;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct EarlySoundsFactory {
    pub base: BaseSoundsFactory,
}

thread_local! { pub static EarlySoundsFactory_instance: Option<Rc<RefCell<EarlySoundsFactory>>> = EarlySoundsFactory::new(); }

impl EarlySoundsFactory {

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: BaseSoundsFactory::new_struct(),
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    // throws Exception
    pub fn getSoundInterfaceArray(&mut self) -> Vec<Option<Rc<RefCell<dyn crate::org::allbinary::media::audio::Sound::SoundDyn>>>> {
        let mut soundInterfaceArray: Vec<Option<Rc<RefCell<dyn crate::org::allbinary::media::audio::Sound::SoundDyn>>>> = vec![None; (2) as usize];
        let mut index: i32 = 0;
        soundInterfaceArray[({ let value = index; index += 1; value }) as usize] = SelectSound::SelectSound::getInstance().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::media::audio::Sound::SoundDyn>>);
        soundInterfaceArray[({ let value = index; index += 1; value }) as usize] = ErrorSound::ErrorSound::getInstance().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::media::audio::Sound::SoundDyn>>);
        return soundInterfaceArray.clone();
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<EarlySoundsFactory>>> {
        return EarlySoundsFactory_instance.with(Clone::clone).clone();
    }
}
impl std::ops::Deref for EarlySoundsFactory {
    type Target = BaseSoundsFactory;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for EarlySoundsFactory {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::media::audio::BaseSoundsFactory::BaseSoundsFactoryDyn for EarlySoundsFactory {
    fn as_BaseSoundsFactory(&self) -> &crate::org::allbinary::media::audio::BaseSoundsFactory::BaseSoundsFactory { &self.base }
    fn as_BaseSoundsFactory_mut(&mut self) -> &mut crate::org::allbinary::media::audio::BaseSoundsFactory::BaseSoundsFactory { &mut self.base }
    fn getSoundInterfaceArray(&mut self) -> Vec<Option<Rc<RefCell<dyn crate::org::allbinary::media::audio::Sound::SoundDyn>>>> { Self::getSoundInterfaceArray(self) }
}


