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
use super::GameInfo::GameInfo;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct BasicHighScoresGameInfoFactory {
    pub MULTI_PLAYER_SERVER: Option<Rc<RefCell<GameInfo>>>,
    pub SINGLE_PLAYER_SERVER: Option<Rc<RefCell<GameInfo>>>,
}

thread_local! { pub static BasicHighScoresGameInfoFactory_SINGLETON: Option<Rc<RefCell<BasicHighScoresGameInfoFactory>>> = BasicHighScoresGameInfoFactory::new(); }

impl BasicHighScoresGameInfoFactory {

    pub fn new_struct() -> Self {
        let mut this = Self {
            MULTI_PLAYER_SERVER: None,
            SINGLE_PLAYER_SERVER: None,
        };
        this.MULTI_PLAYER_SERVER = GameInfo::new(jcall!(GameTypeFactory::GameTypeFactory::getInstance()).MULTI_PLAYER.clone(), unsafe { GameMode_SERVER.clone() }.clone(), jcall!(PlayerTypesFactory::PlayerTypesFactory::getInstance()).PLAYER_TYPE_ONE.clone(), 0, 0);
        this.SINGLE_PLAYER_SERVER = GameInfo::new(jcall!(GameTypeFactory::GameTypeFactory::getInstance()).SINGLE_PLAYER.clone(), unsafe { GameMode_SERVER.clone() }.clone(), jcall!(PlayerTypesFactory::PlayerTypesFactory::getInstance()).PLAYER_TYPE_ONE.clone(), 0, 0);
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<BasicHighScoresGameInfoFactory>>> {
        return BasicHighScoresGameInfoFactory_SINGLETON.with(Clone::clone).clone();
    }
}

