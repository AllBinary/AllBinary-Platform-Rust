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

pub struct GameDifficultyFactory {
    pub level: i32,
}

thread_local! { pub static GameDifficultyFactory_instance: Option<Rc<RefCell<GameDifficultyFactory>>> = GameDifficultyFactory::new(); }

impl GameDifficultyFactory {

    pub fn new_struct() -> Self {
        let mut this = Self {
            level: 0,
        };
        this.level = 0;
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<GameDifficultyFactory>>> {
        return GameDifficultyFactory_instance.with(Clone::clone).clone();
    }
    

    pub fn setLevel(&mut self, mut level: i32) {
        self.level = level.clone();
    }
    

    pub fn getLevel(&mut self) -> i32 {
        return self.level.clone();
    }
}

