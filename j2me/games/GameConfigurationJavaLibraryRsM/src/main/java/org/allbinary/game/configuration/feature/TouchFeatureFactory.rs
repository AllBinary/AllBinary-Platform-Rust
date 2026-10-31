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
// package org.allbinary.game.configuration.feature
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use super::TouchFeature::TouchFeature;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct TouchFeatureFactory {
    pub TOUCH_ENABLED: Option<Rc<RefCell<TouchFeature>>>,
    pub SHOW_SCREEN_BUTTONS: Option<Rc<RefCell<TouchFeature>>>,
    pub AUTO_HIDE_SHOW_SCREEN_BUTTONS: Option<Rc<RefCell<TouchFeature>>>,
    pub HIDE_SCREEN_BUTTONS: Option<Rc<RefCell<TouchFeature>>>,
}

thread_local! { pub static TouchFeatureFactory_instance: Option<Rc<RefCell<TouchFeatureFactory>>> = TouchFeatureFactory::new(); }

impl TouchFeatureFactory {
    

    pub fn getInstance() -> Option<Rc<RefCell<TouchFeatureFactory>>> {
        return TouchFeatureFactory_instance.with(Clone::clone).clone();
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            TOUCH_ENABLED: None,
            SHOW_SCREEN_BUTTONS: None,
            AUTO_HIDE_SHOW_SCREEN_BUTTONS: None,
            HIDE_SCREEN_BUTTONS: None,
        };
        this.TOUCH_ENABLED = TouchFeature::new(String::from("Touch Enabled"));
        this.SHOW_SCREEN_BUTTONS = TouchFeature::new(String::from("Show"));
        this.AUTO_HIDE_SHOW_SCREEN_BUTTONS = TouchFeature::new(String::from("Auto Hide (One Level)"));
        this.HIDE_SCREEN_BUTTONS = TouchFeature::new(String::from("Hide"));
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
}

