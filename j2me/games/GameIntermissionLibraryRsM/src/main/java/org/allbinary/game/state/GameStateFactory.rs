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
// package org.allbinary.game.state
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::logic::NullUtil::NullUtil;
use super::GameState::GameState;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct GameStateFactory {
    pub index: i32,
    pub NO_GAME_STATE: Option<Rc<RefCell<GameState>>>,
    pub PLAYING_GAME_STATE: Option<Rc<RefCell<GameState>>>,
    pub SHOW_END_RESULT_GAME_STATE: Option<Rc<RefCell<GameState>>>,
    pub SHOW_HIGH_SCORE_GAME_STATE: Option<Rc<RefCell<GameState>>>,
}

pub static mut GameStateFactory_instance: std::cell::LazyCell<Option<Rc<RefCell<Box<dyn std::any::Any>>>>> = std::cell::LazyCell::new(|| jcall!(NullUtil::getInstance()).NULL_OBJECT.clone());

impl GameStateFactory {
    

    pub fn getInstance() -> Option<Rc<RefCell<GameStateFactory>>> {
        if match (&unsafe { GameStateFactory_instance.clone() }, &jcall!(NullUtil::getInstance()).NULL_OBJECT) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            unsafe { *GameStateFactory_instance = GameStateFactory::new() };
        }
        return (unsafe { GameStateFactory_instance.clone() }).clone() /* as GameStateFactory */;
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            index: 0,
            NO_GAME_STATE: None,
            PLAYING_GAME_STATE: None,
            SHOW_END_RESULT_GAME_STATE: None,
            SHOW_HIGH_SCORE_GAME_STATE: None,
        };
        this.index = 0;
        this.NO_GAME_STATE = this.createGameState(String::from("NO_GAME_STATE"));
        this.PLAYING_GAME_STATE = this.createGameState(String::from("PLAYING_GAME_STATE"));
        this.SHOW_END_RESULT_GAME_STATE = this.createGameState(String::from("SHOW_END_RESULT_GAME_STATE"));
        this.SHOW_HIGH_SCORE_GAME_STATE = this.createGameState(String::from("SHOW_HIGH_SCORE_GAME_STATE"));
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn createGameState(&mut self, mut name: String) -> Option<Rc<RefCell<GameState>>> {
        return GameState::new(name.clone(), { let value = self.index; self.index += 1; value });
    }
}

