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
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use super::GameType::GameType;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct GameTypeFactory {
    pub NULL_GAME_TYPE_ARRAY: Vec<Option<Rc<RefCell<GameType>>>>,
    pub NONE: Option<Rc<RefCell<GameType>>>,
    pub SINGLE_PLAYER: Option<Rc<RefCell<GameType>>>,
    pub MULTI_PLAYER: Option<Rc<RefCell<GameType>>>,
    pub BOT: Option<Rc<RefCell<GameType>>>,
}

thread_local! { pub static GameTypeFactory_instance: Option<Rc<RefCell<GameTypeFactory>>> = GameTypeFactory::new(); }

impl GameTypeFactory {

    pub fn new_struct() -> Self {
        let mut this = Self {
            NULL_GAME_TYPE_ARRAY: Vec::new(),
            NONE: None,
            SINGLE_PLAYER: None,
            MULTI_PLAYER: None,
            BOT: None,
        };
        this.NULL_GAME_TYPE_ARRAY = vec![None; (0) as usize];
        this.NONE = GameType::new(jcall!(StringUtil::getInstance()).NULL_STRING.clone());
        this.SINGLE_PLAYER = GameType::new(String::from("Single Player"));
        this.MULTI_PLAYER = GameType::new(String::from("Multi Player"));
        this.BOT = GameType::new(String::from("Artificial Player"));
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<GameTypeFactory>>> {
        return GameTypeFactory_instance.with(Clone::clone).clone();
    }
}

