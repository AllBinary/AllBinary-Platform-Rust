// 
// AllBinary Open License Version 1 
// Copyright (c) 2006 AllBinary 
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
// package org.allbinary.game.layer
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use super::SensorAction::SensorAction;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct SensorActionFactory {
    pub ATTACK: Option<Rc<RefCell<SensorAction>>>,
    pub EVADE: Option<Rc<RefCell<SensorAction>>>,
}

thread_local! { pub static SensorActionFactory_instance: Option<Rc<RefCell<SensorActionFactory>>> = SensorActionFactory::new(); }

impl SensorActionFactory {
    

    pub fn getInstance() -> Option<Rc<RefCell<SensorActionFactory>>> {
        return SensorActionFactory_instance.with(Clone::clone).clone();
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            ATTACK: None,
            EVADE: None,
        };
        this.ATTACK = SensorAction::new(String::from("Attack"));
        this.EVADE = SensorAction::new(String::from("Evade"));
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
}

