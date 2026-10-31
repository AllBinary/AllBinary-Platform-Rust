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
// package org.allbinary.game.configuration
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use super::GameConfiguration::GameConfiguration;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct GameSpeed {
}

thread_local! { pub static GameSpeed_instance: Option<Rc<RefCell<GameSpeed>>> = GameSpeed::new(); }

impl GameSpeed {

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getSpeed(&mut self) -> i32 {
        let mut speedGameConfiguration: Option<Rc<RefCell<dyn crate::org::allbinary::game::configuration::GameConfiguration::GameConfigurationDyn>>> = jcall!(GameConfigurationCentral::GameConfigurationCentral::getInstance()).SPEED.clone();
        return (((jcall!(speedGameConfiguration).getMaxValue()).unwrap() as i32) - ((jcall!(speedGameConfiguration).getValue()).unwrap() as i32) + 1);
    }
    

    pub fn getDelay(&mut self) -> i32 {
        let mut speedGameConfiguration: Option<Rc<RefCell<dyn crate::org::allbinary::game::configuration::GameConfiguration::GameConfigurationDyn>>> = jcall!(GameConfigurationCentral::GameConfigurationCentral::getInstance()).SPEED.clone();
        return 19 * (((jcall!(speedGameConfiguration).getMaxValue()).unwrap() as i32) - ((jcall!(speedGameConfiguration).getValue()).unwrap() as i32));
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<GameSpeed>>> {
        return GameSpeed_instance.with(Clone::clone).clone();
    }
}

