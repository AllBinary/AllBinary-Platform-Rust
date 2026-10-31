// 
// AllBinary Open License Version 1 
// Copyright (c) 2025 AllBinary 
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

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct SoundStrings {
    pub STOP_ALL: String,
    pub CLOSE_ALL: String,
    pub SOUND: String,
    pub INIT_SOUND: String,
    pub STOPPING_SOUND: String,
    pub CLOSING_SOUND: String,
}

thread_local! { pub static SoundStrings_instance: Option<Rc<RefCell<SoundStrings>>> = SoundStrings::new(); }

impl SoundStrings {

    pub fn new_struct() -> Self {
        let mut this = Self {
            STOP_ALL: String::new(),
            CLOSE_ALL: String::new(),
            SOUND: String::new(),
            INIT_SOUND: String::new(),
            STOPPING_SOUND: String::new(),
            CLOSING_SOUND: String::new(),
        };
        this.STOP_ALL = String::from("stopAll");
        this.CLOSE_ALL = String::from("closeAll");
        this.SOUND = String::from(" Sound: ");
        this.INIT_SOUND = String::from("Initializing Sound: ");
        this.STOPPING_SOUND = String::from("Stopping Sound: ");
        this.CLOSING_SOUND = String::from("Closing Sound: ");
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<SoundStrings>>> {
        return SoundStrings_instance.with(Clone::clone).clone();
    }
}

