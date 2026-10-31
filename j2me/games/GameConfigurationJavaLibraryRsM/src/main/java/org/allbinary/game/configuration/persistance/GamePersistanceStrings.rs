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
// package org.allbinary.game.configuration.persistance
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

pub struct GamePersistanceStrings {
    pub SAVED_GAME_RECORD_ID: String,
    pub SAVED_GAME_CONFIGURATION_RECORD_ID: String,
    pub DEFAULT_INPUT_MAPPING_RECORD_ID: String,
    pub SAVED_INPUT_CONFIGURATION_RECORD_ID: String,
}

thread_local! { pub static GamePersistanceStrings_instance: Option<Rc<RefCell<GamePersistanceStrings>>> = GamePersistanceStrings::new(); }

impl GamePersistanceStrings {

    pub fn new_struct() -> Self {
        let mut this = Self {
            SAVED_GAME_RECORD_ID: String::new(),
            SAVED_GAME_CONFIGURATION_RECORD_ID: String::new(),
            DEFAULT_INPUT_MAPPING_RECORD_ID: String::new(),
            SAVED_INPUT_CONFIGURATION_RECORD_ID: String::new(),
        };
        this.SAVED_GAME_RECORD_ID = String::from("_SG");
        this.SAVED_GAME_CONFIGURATION_RECORD_ID = String::from("_SGC");
        this.DEFAULT_INPUT_MAPPING_RECORD_ID = String::from("_DIM");
        this.SAVED_INPUT_CONFIGURATION_RECORD_ID = String::from("_SIC");
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<GamePersistanceStrings>>> {
        return GamePersistanceStrings_instance.with(Clone::clone).clone();
    }
}

