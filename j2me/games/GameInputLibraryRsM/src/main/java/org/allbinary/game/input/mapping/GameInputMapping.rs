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
// package org.allbinary.game.input.mapping
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::game::input::GameKey::GameKey;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct GameInputMapping {
    pub name: String,
    pub gameKey: Option<Rc<RefCell<GameKey>>>,
}


impl GameInputMapping {
    

    pub fn new_struct(mut name: String, mut gameKey: Option<Rc<RefCell<GameKey>>>) -> Self {
        let mut this = Self {
            name: String::new(),
            gameKey: None,
        };
        this.name = jcall!(StringUtil::getInstance()).EMPTY_STRING.clone();
        this.gameKey = crate::org::allbinary::game::input::GameKey::GameKey_NULL_GAME_KEY.with(Clone::clone).clone();
        this.setName(name.clone());
        this.setGameKey(gameKey.clone());
        this
    }

    pub fn new(mut name: String, mut gameKey: Option<Rc<RefCell<GameKey>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(name, gameKey))))
    }
    

    pub fn setGameKey(&mut self, mut gameKey: Option<Rc<RefCell<GameKey>>>) {
        self.gameKey = gameKey.clone();
    }
    

    pub fn getGameKey(&mut self) -> Option<Rc<RefCell<GameKey>>> {
        return self.gameKey.clone();
    }
    

    pub fn setName(&mut self, mut name: String) {
        self.name = name.clone();
    }
    

    pub fn getName(&mut self) -> String {
        return self.name.clone();
    }
}

