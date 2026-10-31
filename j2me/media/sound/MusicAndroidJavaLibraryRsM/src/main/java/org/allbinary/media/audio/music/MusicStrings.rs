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
// package org.allbinary.media.audio.music
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct MusicStrings {
    pub SONG_EXTRA: String,
    pub LEFT_VOLUME: String,
    pub RIGHT_VOLUME: String,
}

thread_local! { pub static MusicStrings_instance: Option<Rc<RefCell<MusicStrings>>> = MusicStrings::new(); }

impl MusicStrings {

    pub fn new_struct() -> Self {
        let mut this = Self {
            SONG_EXTRA: String::new(),
            LEFT_VOLUME: String::new(),
            RIGHT_VOLUME: String::new(),
        };
        this.SONG_EXTRA = String::from("SONG");
        this.LEFT_VOLUME = String::from("LEFT_VOLUME");
        this.RIGHT_VOLUME = String::from("RIGHT_VOLUME");
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<MusicStrings>>> {
        return MusicStrings_instance.with(Clone::clone).clone();
    }
}

