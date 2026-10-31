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
// package org.allbinary.game.input
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

pub struct GameInputStrings {
    pub PROCESS_INPUT: String,
    pub KEY_PRESSED: String,
    pub KEY_RELEASED: String,
    pub KEY_REPEATED: String,
    pub ON_PRESS_GAME_KEY: String,
    pub ON_DOWN_GAME_KEY: String,
    pub ON_UP_GAME_KEY: String,
    pub NO_KEY: String,
    pub ADD_KEY_EVENT: String,
    pub REMOVE_KEY_EVENT: String,
    pub STRAFE_LEFT: String,
    pub STRAFE_RIGHT: String,
    pub PROCESS_MOTION_INPUT: String,
    pub UDPATE_CURRENT_TOUCH_INPUT_FACTORY: String,
    pub ENABLE_PLAYER_GAME_INPUTS: String,
    pub ENABLE_PLAYER_GAME_INPUT: String,
}

thread_local! { pub static GameInputStrings_instance: Option<Rc<RefCell<GameInputStrings>>> = GameInputStrings::new(); }

impl GameInputStrings {

    pub fn new_struct() -> Self {
        let mut this = Self {
            PROCESS_INPUT: String::new(),
            KEY_PRESSED: String::new(),
            KEY_RELEASED: String::new(),
            KEY_REPEATED: String::new(),
            ON_PRESS_GAME_KEY: String::new(),
            ON_DOWN_GAME_KEY: String::new(),
            ON_UP_GAME_KEY: String::new(),
            NO_KEY: String::new(),
            ADD_KEY_EVENT: String::new(),
            REMOVE_KEY_EVENT: String::new(),
            STRAFE_LEFT: String::new(),
            STRAFE_RIGHT: String::new(),
            PROCESS_MOTION_INPUT: String::new(),
            UDPATE_CURRENT_TOUCH_INPUT_FACTORY: String::new(),
            ENABLE_PLAYER_GAME_INPUTS: String::new(),
            ENABLE_PLAYER_GAME_INPUT: String::new(),
        };
        this.PROCESS_INPUT = String::from("processInput");
        this.KEY_PRESSED = String::from("keyPressed");
        this.KEY_RELEASED = String::from("keyReleased");
        this.KEY_REPEATED = String::from("keyRepeated");
        this.ON_PRESS_GAME_KEY = String::from("onPressGameKeyEvent");
        this.ON_DOWN_GAME_KEY = String::from("onDownGameKeyEvent");
        this.ON_UP_GAME_KEY = String::from("onUpGameKeyEvent");
        this.NO_KEY = String::from("Key Code Not Mapped For Game: ");
        this.ADD_KEY_EVENT = String::from("addGameKeyEvent");
        this.REMOVE_KEY_EVENT = String::from("removeGameKeyEvent");
        this.STRAFE_LEFT = String::from("strafeLeft");
        this.STRAFE_RIGHT = String::from("strafeRight");
        this.PROCESS_MOTION_INPUT = String::from("processMotionInput");
        this.UDPATE_CURRENT_TOUCH_INPUT_FACTORY = String::from("updateCurrentTouchInputFactory");
        this.ENABLE_PLAYER_GAME_INPUTS = String::from("Enabling PlayerGameInputs: ");
        this.ENABLE_PLAYER_GAME_INPUT = String::from("Enabling PlayerGameInput: ");
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<GameInputStrings>>> {
        return GameInputStrings_instance.with(Clone::clone).clone();
    }
}

