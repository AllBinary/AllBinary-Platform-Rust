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
// package org.allbinary.input.automation.module.generic
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

pub struct GenericConfigData {
}

pub static mut GenericConfigData_RELATIVE_PATH: std::cell::LazyCell<String> = std::cell::LazyCell::new(|| String::from("./"));
pub static mut GenericConfigData_CONFIG_PATH: std::cell::LazyCell<String> = std::cell::LazyCell::new(|| String::from("modules/configs/"));
pub static mut GenericConfigData_MODULE_RELATIVE_PATH: std::cell::LazyCell<String> = std::cell::LazyCell::new(|| format!("{}{}", format!("{}{}", unsafe { GenericConfigData_RELATIVE_PATH.clone() }, unsafe { GenericConfigData_CONFIG_PATH.clone() }), String::from("allbinary/input/automation/module/")));
pub static mut GenericConfigData_GAME_MODULE_RELATIVE_PATH: std::cell::LazyCell<String> = std::cell::LazyCell::new(|| format!("{}{}", unsafe { GenericConfigData_MODULE_RELATIVE_PATH.clone() }, String::from("game/")));
pub static mut GenericConfigData_MMO_GAME_MODULE_RELATIVE_PATH: std::cell::LazyCell<String> = std::cell::LazyCell::new(|| format!("{}{}", unsafe { GenericConfigData_GAME_MODULE_RELATIVE_PATH.clone() }, String::from("mmog/")));
pub static mut GenericConfigData_FREE_MMO_GAME_MODULE_RELATIVE_PATH: std::cell::LazyCell<String> = std::cell::LazyCell::new(|| String::from("free/"));

impl GenericConfigData {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
}

