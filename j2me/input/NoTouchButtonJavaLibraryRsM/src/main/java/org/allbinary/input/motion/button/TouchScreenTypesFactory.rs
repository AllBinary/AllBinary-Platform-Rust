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
// package org.allbinary.input.motion.button
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use super::TouchScreenType::TouchScreenType;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct TouchScreenTypesFactory {
    pub FINGER: Option<Rc<RefCell<TouchScreenType>>>,
    pub NOTOUCH: Option<Rc<RefCell<TouchScreenType>>>,
    pub STYLUS: Option<Rc<RefCell<TouchScreenType>>>,
    pub UNDEFINED: Option<Rc<RefCell<TouchScreenType>>>,
}

thread_local! { pub static TouchScreenTypesFactory_instance: Option<Rc<RefCell<TouchScreenTypesFactory>>> = TouchScreenTypesFactory::new(); }

impl TouchScreenTypesFactory {

    pub fn new_struct() -> Self {
        let mut this = Self {
            FINGER: None,
            NOTOUCH: None,
            STYLUS: None,
            UNDEFINED: None,
        };
        this.FINGER = TouchScreenType::new(String::from("Finger"));
        this.NOTOUCH = TouchScreenType::new(String::from("No Touch"));
        this.STYLUS = TouchScreenType::new(String::from("Stylus"));
        this.UNDEFINED = TouchScreenType::new(String::from("Undefined"));
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<TouchScreenTypesFactory>>> {
        return TouchScreenTypesFactory_instance.with(Clone::clone).clone();
    }
}

