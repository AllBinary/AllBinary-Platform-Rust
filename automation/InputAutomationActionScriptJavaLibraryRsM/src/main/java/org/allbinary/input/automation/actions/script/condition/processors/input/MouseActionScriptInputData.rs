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
// package org.allbinary.input.automation.actions.script.condition.processors.input
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

pub struct MouseActionScriptInputData {
}

pub static mut MouseActionScriptInputData_NAME: std::cell::LazyCell<String> = std::cell::LazyCell::new(|| String::from("MOUSE_ACTION_INPUT_NAME"));
pub static mut MouseActionScriptInputData_BUTTONS: std::cell::LazyCell<String> = std::cell::LazyCell::new(|| String::from("MOUSE_ACTION_INPUT_BUTTONS"));
pub static mut MouseActionScriptInputData_MOVE: std::cell::LazyCell<String> = std::cell::LazyCell::new(|| String::from("MOUSE_ACTION_INPUT_MOVE"));
pub static mut MouseActionScriptInputData_MOVE_X: std::cell::LazyCell<String> = std::cell::LazyCell::new(|| String::from("MOUSE_ACTION_INPUT_MOVE_X"));
pub static mut MouseActionScriptInputData_MOVE_Y: std::cell::LazyCell<String> = std::cell::LazyCell::new(|| String::from("MOUSE_ACTION_INPUT_MOVE_Y"));

impl MouseActionScriptInputData {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
}

