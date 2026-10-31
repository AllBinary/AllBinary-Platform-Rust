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
// package org.allbinary.game.score.remote
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

pub struct RemoteHighScoresData {
    pub HIGH_SCORES: String,
    pub CUSTOMER_USER_NAME: String,
    pub DISPLAY_NAME: String,
    pub SCORE: String,
    pub GAME_INFO: String,
    pub GAME_CONFIGURATION: String,
}

thread_local! { pub static RemoteHighScoresData_SINGLETON: Option<Rc<RefCell<RemoteHighScoresData>>> = RemoteHighScoresData::new(); }

impl RemoteHighScoresData {

    pub fn new_struct() -> Self {
        let mut this = Self {
            HIGH_SCORES: String::new(),
            CUSTOMER_USER_NAME: String::new(),
            DISPLAY_NAME: String::new(),
            SCORE: String::new(),
            GAME_INFO: String::new(),
            GAME_CONFIGURATION: String::new(),
        };
        this.HIGH_SCORES = String::from("HIGH_SCORES");
        this.CUSTOMER_USER_NAME = String::from("CUSTOMER_USER_NAME");
        this.DISPLAY_NAME = String::from("DISPLAY_NAME");
        this.SCORE = String::from("SCORE");
        this.GAME_INFO = String::from("GAME_INFO");
        this.GAME_CONFIGURATION = String::from("GAME_CONFIGURATION");
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<RemoteHighScoresData>>> {
        return RemoteHighScoresData_SINGLETON.with(Clone::clone).clone();
    }
}

