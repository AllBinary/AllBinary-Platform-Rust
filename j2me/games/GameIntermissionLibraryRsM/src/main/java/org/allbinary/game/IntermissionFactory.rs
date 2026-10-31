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
use crate::org::allbinary::game::state::GameState::GameState;
use crate::org::allbinary::game::state::GameStateFactory::GameStateFactory;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct IntermissionFactory {
    pub START_LEVEL_INTERMISSION_GAME_STATE: Option<Rc<RefCell<GameState>>>,
    pub WAIT_LEVEL_INTERMISSION_GAME_STATE: Option<Rc<RefCell<GameState>>>,
    pub SHOW_RESULTS_LEVEL_INTERMISSION_GAME_STATE: Option<Rc<RefCell<GameState>>>,
    pub SHOW_HIGH_SCORE_LEVEL_INTERMISSION_GAME_STATE: Option<Rc<RefCell<GameState>>>,
}

thread_local! { pub static IntermissionFactory_instance: Option<Rc<RefCell<IntermissionFactory>>> = IntermissionFactory::new(); }

impl IntermissionFactory {

    pub fn new_struct() -> Self {
        let mut this = Self {
            START_LEVEL_INTERMISSION_GAME_STATE: None,
            WAIT_LEVEL_INTERMISSION_GAME_STATE: None,
            SHOW_RESULTS_LEVEL_INTERMISSION_GAME_STATE: None,
            SHOW_HIGH_SCORE_LEVEL_INTERMISSION_GAME_STATE: None,
        };
        this.START_LEVEL_INTERMISSION_GAME_STATE = jcall!(GameStateFactory::getInstance()).createGameState(String::from("START_LEVEL_INTERMISSION_GAME_STATE"));
        this.WAIT_LEVEL_INTERMISSION_GAME_STATE = jcall!(GameStateFactory::getInstance()).createGameState(String::from("WAIT_LEVEL_INTERMISSION_GAME_STATE"));
        this.SHOW_RESULTS_LEVEL_INTERMISSION_GAME_STATE = jcall!(GameStateFactory::getInstance()).createGameState(String::from("SHOW_RESULTS_LEVEL_INTERMISSION_GAME_STATE"));
        this.SHOW_HIGH_SCORE_LEVEL_INTERMISSION_GAME_STATE = jcall!(GameStateFactory::getInstance()).createGameState(String::from("SHOW_HIGH_SCORE_LEVEL_INTERMISSION_GAME_STATE"));
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<IntermissionFactory>>> {
        return IntermissionFactory_instance.with(Clone::clone).clone();
    }
}

