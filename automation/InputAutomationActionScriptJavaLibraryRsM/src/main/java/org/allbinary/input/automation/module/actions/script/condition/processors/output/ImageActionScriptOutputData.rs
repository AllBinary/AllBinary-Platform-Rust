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
// package org.allbinary.input.automation.module.actions.script.condition.processors.output
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

pub struct ImageActionScriptOutputData {
}

pub static mut ImageActionScriptOutputData_NAME: std::cell::LazyCell<String> = std::cell::LazyCell::new(|| String::from("IMAGE_ACTION_OUTPUT_NAME"));
pub static mut ImageActionScriptOutputData_SAVE: std::cell::LazyCell<String> = std::cell::LazyCell::new(|| String::from("SAVE_IMAGE_ACTION_OUTPUT_NAME"));
pub static mut ImageActionScriptOutputData_DISPLAY: std::cell::LazyCell<String> = std::cell::LazyCell::new(|| String::from("DISPLAY_IMAGE_ACTION_OUTPUT_NAME"));
pub static mut ImageActionScriptOutputData_TYPE: std::cell::LazyCell<String> = std::cell::LazyCell::new(|| String::from("TYPE_IMAGE_ACTION_OUTPUT_NAME"));
pub static mut ImageActionScriptOutputData_TYPES: std::cell::LazyCell<String> = std::cell::LazyCell::new(|| String::from("TYPES_IMAGE_ACTION_OUTPUT_NAME"));

impl ImageActionScriptOutputData {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
}

