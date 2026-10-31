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
// package org.allbinary.game
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

pub struct GameInfoData {
    pub GAME_TYPE: String,
    pub GAME_MODE: String,
    pub PLAYER_TYPE: String,
    pub HIGHEST_LEVEL: String,
    pub CURRENT_LEVEL: String,
    pub SOFTWARE_INFORMATION: String,
}

thread_local! { pub static GameInfoData_instance: Option<Rc<RefCell<GameInfoData>>> = GameInfoData::new(); }

impl GameInfoData {
    

    pub fn getInstance() -> Option<Rc<RefCell<GameInfoData>>> {
        return GameInfoData_instance.with(Clone::clone).clone();
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            GAME_TYPE: String::new(),
            GAME_MODE: String::new(),
            PLAYER_TYPE: String::new(),
            HIGHEST_LEVEL: String::new(),
            CURRENT_LEVEL: String::new(),
            SOFTWARE_INFORMATION: String::new(),
        };
        this.GAME_TYPE = String::from("GAME_TYPE");
        this.GAME_MODE = String::from("GAME_MODE");
        this.PLAYER_TYPE = String::from("PLAYER_TYPE");
        this.HIGHEST_LEVEL = String::from("HIGHEST_LEVEL");
        this.CURRENT_LEVEL = String::from("CURRENT_LEVEL");
        this.SOFTWARE_INFORMATION = String::from("SOFTWARE_INFORMATION");
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
}

