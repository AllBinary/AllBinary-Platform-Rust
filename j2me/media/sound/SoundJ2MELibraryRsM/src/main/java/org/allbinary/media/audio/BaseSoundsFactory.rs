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
use super::SoundsFactoryInterface::SoundsFactoryInterface;
use super::Sound::Sound;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct BaseSoundsFactory {
    pub initialized: bool,
}

impl SoundsFactoryInterface for BaseSoundsFactory {
    fn init(&mut self) { Self::init(self) }
    fn getSoundInterfaceArray(&mut self) -> Vec<Option<Rc<RefCell<dyn crate::org::allbinary::media::audio::Sound::SoundDyn>>>> { Self::getSoundInterfaceArray(self) }
    fn setInitialized(&mut self, mut initialized: bool) { Self::setInitialized(self, initialized) }
    fn isInitialized(&mut self) -> bool { Self::isInitialized(self) }
}

impl BaseSoundsFactory {

    pub fn new_struct() -> Self {
        let mut this = Self {
            initialized: false,
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn init(&mut self) {
    }
    

    // throws Exception
    pub fn getSoundInterfaceArray(&mut self) -> Vec<Option<Rc<RefCell<dyn crate::org::allbinary::media::audio::Sound::SoundDyn>>>> {
        return vec![None; (0) as usize];
    }
    

    pub fn setInitialized(&mut self, mut initialized: bool) {
        self.initialized = initialized.clone();
    }
    

    pub fn isInitialized(&mut self) -> bool {
        return self.initialized.clone();
    }
}
pub trait BaseSoundsFactoryDyn {
    fn as_BaseSoundsFactory(&self) -> &BaseSoundsFactory;
    fn as_BaseSoundsFactory_mut(&mut self) -> &mut BaseSoundsFactory;
    fn init(&mut self) { BaseSoundsFactory::init(self.as_BaseSoundsFactory_mut()) }
    fn getSoundInterfaceArray(&mut self) -> Vec<Option<Rc<RefCell<dyn crate::org::allbinary::media::audio::Sound::SoundDyn>>>> { BaseSoundsFactory::getSoundInterfaceArray(self.as_BaseSoundsFactory_mut()) }
    fn setInitialized(&mut self, mut initialized: bool) { BaseSoundsFactory::setInitialized(self.as_BaseSoundsFactory_mut(), initialized) }
    fn isInitialized(&mut self) -> bool { BaseSoundsFactory::isInitialized(self.as_BaseSoundsFactory_mut()) }
}
impl BaseSoundsFactoryDyn for BaseSoundsFactory {
    fn as_BaseSoundsFactory(&self) -> &BaseSoundsFactory { self }
    fn as_BaseSoundsFactory_mut(&mut self) -> &mut BaseSoundsFactory { self }
}


