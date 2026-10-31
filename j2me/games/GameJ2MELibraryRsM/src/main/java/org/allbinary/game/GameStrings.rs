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

pub struct GameStrings {
    pub PROCESS_TICK: String,
    pub ON_GAME_INITIALIZED: String,
    pub UPDATESURFACE: String,
    pub ON_SET_DISPLAYABLE: String,
    pub UNPAUSE: String,
    pub TOGGLE_MENU: String,
    pub SET_GAME_OVER: String,
    pub GAME_STATE: String,
    pub SET_GAME_STATE: String,
    pub STOP_GAME_CANVAS_RUNNABLE_INTERFACE: String,
}

thread_local! { pub static GameStrings_instance: Option<Rc<RefCell<GameStrings>>> = GameStrings::new(); }

impl GameStrings {

    pub fn new_struct() -> Self {
        let mut this = Self {
            PROCESS_TICK: String::new(),
            ON_GAME_INITIALIZED: String::new(),
            UPDATESURFACE: String::new(),
            ON_SET_DISPLAYABLE: String::new(),
            UNPAUSE: String::new(),
            TOGGLE_MENU: String::new(),
            SET_GAME_OVER: String::new(),
            GAME_STATE: String::new(),
            SET_GAME_STATE: String::new(),
            STOP_GAME_CANVAS_RUNNABLE_INTERFACE: String::new(),
        };
        this.PROCESS_TICK = String::from("processTick");
        this.ON_GAME_INITIALIZED = String::from("onGameInitialized");
        this.UPDATESURFACE = String::from("updateSurface");
        this.ON_SET_DISPLAYABLE = String::from("onSetDisplayable");
        this.UNPAUSE = String::from("unPause");
        this.TOGGLE_MENU = String::from("toggleMenu");
        this.SET_GAME_OVER = String::from("setGameOver");
        this.GAME_STATE = String::from("Game State: ");
        this.SET_GAME_STATE = String::from("setGameState");
        this.STOP_GAME_CANVAS_RUNNABLE_INTERFACE = String::from("stopGameCanvasRunnableInterface");
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<GameStrings>>> {
        return GameStrings_instance.with(Clone::clone).clone();
    }
}

