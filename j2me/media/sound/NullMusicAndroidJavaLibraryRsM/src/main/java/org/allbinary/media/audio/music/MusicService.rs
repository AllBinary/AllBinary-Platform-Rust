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
// package org.allbinary.media.audio.music
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::media::audio::music::BaseMusicService::BaseMusicService;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct MusicService {
    pub base: BaseMusicService,
}


impl MusicService {

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: BaseMusicService::new_struct(),
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
}
impl std::ops::Deref for MusicService {
    type Target = BaseMusicService;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for MusicService {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::media::audio::music::BaseMusicService::BaseMusicServiceDyn for MusicService {
    fn as_BaseMusicService(&self) -> &crate::org::allbinary::media::audio::music::BaseMusicService::BaseMusicService { &self.base }
    fn as_BaseMusicService_mut(&mut self) -> &mut crate::org::allbinary::media::audio::music::BaseMusicService::BaseMusicService { &mut self.base }
}


