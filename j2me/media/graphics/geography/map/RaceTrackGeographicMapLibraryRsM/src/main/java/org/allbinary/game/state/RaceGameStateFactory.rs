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

pub struct RaceGameStateFactory {
    pub START_GAME_STATE: Option<Rc<RefCell<GameState>>>,
    pub YELLOW_LIGHT_GAME_STATE: Option<Rc<RefCell<GameState>>>,
    pub RED_LIGHT_GAME_STATE: Option<Rc<RefCell<GameState>>>,
}

pub static mut RaceGameStateFactory_instance: std::cell::LazyCell<Option<Rc<RefCell<Box<dyn std::any::Any>>>>> = std::cell::LazyCell::new(|| jcall!(NullUtil::getInstance()).NULL_OBJECT.clone());

impl RaceGameStateFactory {

    pub fn new_struct() -> Self {
        let mut this = Self {
            START_GAME_STATE: None,
            YELLOW_LIGHT_GAME_STATE: None,
            RED_LIGHT_GAME_STATE: None,
        };
        this.START_GAME_STATE = jcall!(GameStateFactory::GameStateFactory::getInstance()).createGameState(String::from("START_GAME_STATE"));
        this.YELLOW_LIGHT_GAME_STATE = jcall!(GameStateFactory::GameStateFactory::getInstance()).createGameState(String::from("YELLOW_LIGHT_GAME_STATE"));
        this.RED_LIGHT_GAME_STATE = jcall!(GameStateFactory::GameStateFactory::getInstance()).createGameState(String::from("RED_LIGHT_GAME_STATE"));
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<RaceGameStateFactory>>> {
        if match (&unsafe { RaceGameStateFactory_instance.clone() }, &jcall!(NullUtil::getInstance()).NULL_OBJECT) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            unsafe { *RaceGameStateFactory_instance = RaceGameStateFactory::new() };
        }
        return (unsafe { RaceGameStateFactory_instance.clone() }).clone() /* as RaceGameStateFactory */;
    }
}

