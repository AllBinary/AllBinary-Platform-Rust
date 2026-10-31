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
// use crate::javax::microedition::media::Player::Player;
use crate::org::allbinary::string::CommonStrings::CommonStrings;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct PlayerStateUtil {
    pub commonStrings: Option<Rc<RefCell<CommonStrings>>>,
    pub UNREALIZED: String,
    pub REALIZED: String,
    pub PREFETCHED: String,
    pub STARTED: String,
    pub CLOSED: String,
}

thread_local! { pub static PlayerStateUtil_instance: Option<Rc<RefCell<PlayerStateUtil>>> = PlayerStateUtil::new(); }

impl PlayerStateUtil {

    pub fn new_struct() -> Self {
        let mut this = Self {
            commonStrings: None,
            UNREALIZED: String::new(),
            REALIZED: String::new(),
            PREFETCHED: String::new(),
            STARTED: String::new(),
            CLOSED: String::new(),
        };
        this.commonStrings = CommonStrings::getInstance();
        this.UNREALIZED = String::from("UNREALIZED");
        this.REALIZED = String::from("REALIZED");
        this.PREFETCHED = String::from("PREFETCHED");
        this.STARTED = String::from("STARTED");
        this.CLOSED = String::from("CLOSED");
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<PlayerStateUtil>>> {
        return PlayerStateUtil_instance.with(Clone::clone).clone();
    }
    

    pub fn convert(&mut self, mut state: i32) -> String {
        if state == unsafe { crate::javax::microedition::media::Player::Player_UNREALIZED.clone() } {
            return self.UNREALIZED.clone();
        } else if state == unsafe { crate::javax::microedition::media::Player::Player_REALIZED.clone() } {
            return self.REALIZED.clone();
        } else if state == unsafe { crate::javax::microedition::media::Player::Player_PREFETCHED.clone() } {
            return self.PREFETCHED.clone();
        } else if state == unsafe { crate::javax::microedition::media::Player::Player_STARTED.clone() } {
            return self.STARTED.clone();
        } else if state == unsafe { crate::javax::microedition::media::Player::Player_CLOSED.clone() } {
            return self.CLOSED.clone();
        }
        return jcall!(self.commonStrings).UNKNOWN.clone();
    }
}

