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
// package org.allbinary.input.automation.actions.script.condition
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

pub struct ColorAtActionScriptConditionData {
}

pub static mut ColorAtActionScriptConditionData_NAME: std::cell::LazyCell<String> = std::cell::LazyCell::new(|| String::from("COLOR_AT_ACTION_CONDITION_NAME"));
pub static mut ColorAtActionScriptConditionData_LOCATION_X: std::cell::LazyCell<String> = std::cell::LazyCell::new(|| String::from("COLOR_AT_ACTION_CONDITION_LOCATION_X"));
pub static mut ColorAtActionScriptConditionData_LOCATION_Y: std::cell::LazyCell<String> = std::cell::LazyCell::new(|| String::from("COLOR_AT_ACTION_CONDITION_LOCATION_Y"));
pub static mut ColorAtActionScriptConditionData_MIN_RED: std::cell::LazyCell<String> = std::cell::LazyCell::new(|| String::from("COLOR_AT_ACTION_CONDITION_MIN_RED"));
pub static mut ColorAtActionScriptConditionData_MAX_RED: std::cell::LazyCell<String> = std::cell::LazyCell::new(|| String::from("COLOR_AT_ACTION_CONDITION_MAX_RED"));
pub static mut ColorAtActionScriptConditionData_MIN_GREEN: std::cell::LazyCell<String> = std::cell::LazyCell::new(|| String::from("COLOR_AT_ACTION_CONDITION_MIN_GREEN"));
pub static mut ColorAtActionScriptConditionData_MAX_GREEN: std::cell::LazyCell<String> = std::cell::LazyCell::new(|| String::from("COLOR_AT_ACTION_CONDITION_MAX_GREEN"));
pub static mut ColorAtActionScriptConditionData_MIN_BLUE: std::cell::LazyCell<String> = std::cell::LazyCell::new(|| String::from("COLOR_AT_ACTION_CONDITION_MIN_BLUE"));
pub static mut ColorAtActionScriptConditionData_MAX_BLUE: std::cell::LazyCell<String> = std::cell::LazyCell::new(|| String::from("COLOR_AT_ACTION_CONDITION_MAX_BLUE"));

impl ColorAtActionScriptConditionData {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
}

