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
use super::PlayerType::PlayerType;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct PlayerTypesFactory {
    pub PLAYER_TYPE_ONE: Option<Rc<RefCell<PlayerType>>>,
    pub PLAYER_TYPE_TWO: Option<Rc<RefCell<PlayerType>>>,
    pub PLAYER_TYPE_THREE: Option<Rc<RefCell<PlayerType>>>,
}

thread_local! { pub static PlayerTypesFactory_SINGLETON: Option<Rc<RefCell<PlayerTypesFactory>>> = PlayerTypesFactory::new(); }

impl PlayerTypesFactory {

    pub fn new_struct() -> Self {
        let mut this = Self {
            PLAYER_TYPE_ONE: None,
            PLAYER_TYPE_TWO: None,
            PLAYER_TYPE_THREE: None,
        };
        this.PLAYER_TYPE_ONE = PlayerType::new(String::from("Player Type One"), 0);
        this.PLAYER_TYPE_TWO = PlayerType::new(String::from("Player Type Two"), 1);
        this.PLAYER_TYPE_THREE = PlayerType::new(String::from("Player Type Three"), 2);
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<PlayerTypesFactory>>> {
        return PlayerTypesFactory_SINGLETON.with(Clone::clone).clone();
    }
}

