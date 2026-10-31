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
// package org.allbinary.game.midlet
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use super::DemoGameMidletState::DemoGameMidletState;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct DemoGameMidletStateFactory {
    pub NONE: Option<Rc<RefCell<DemoGameMidletState>>>,
    pub START_DEMO: Option<Rc<RefCell<DemoGameMidletState>>>,
    pub START_GAME: Option<Rc<RefCell<DemoGameMidletState>>>,
    pub START_INPUT_MAPPING: Option<Rc<RefCell<DemoGameMidletState>>>,
}

pub static mut DemoGameMidletStateFactory_STATE: std::cell::LazyCell<Option<Rc<RefCell<DemoGameMidletStateFactory>>>> = std::cell::LazyCell::new(|| DemoGameMidletStateFactory::new());

impl DemoGameMidletStateFactory {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            NONE: None,
            START_DEMO: None,
            START_GAME: None,
            START_INPUT_MAPPING: None,
        };
        this.NONE = DemoGameMidletState::new();
        this.START_DEMO = DemoGameMidletState::new();
        this.START_GAME = DemoGameMidletState::new();
        this.START_INPUT_MAPPING = DemoGameMidletState::new();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<DemoGameMidletStateFactory>>> {
        return unsafe { DemoGameMidletStateFactory_STATE.clone() }.clone();
    }
}

