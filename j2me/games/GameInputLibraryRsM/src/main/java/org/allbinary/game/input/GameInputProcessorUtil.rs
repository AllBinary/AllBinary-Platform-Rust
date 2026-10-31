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
use super::GameInputProcessor::GameInputProcessor;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct GameInputProcessorUtil {
}


impl GameInputProcessorUtil {

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn init(mut inputProcessorArray: Vec<Option<Rc<RefCell<dyn crate::org::allbinary::game::input::GameInputProcessor::GameInputProcessorDyn>>>>) {
        let mut noInputProcessor: Option<Rc<RefCell<dyn crate::org::allbinary::game::input::GameInputProcessor::GameInputProcessorDyn>>> = GameInputProcessor::getInstance();
        let mut total: i32 = jcall!(InputFactory::InputFactory::getInstance()).MAX.clone();
        for index in (0..total) {
            let mut inputProcessorCanBeNull: Option<Rc<RefCell<dyn crate::org::allbinary::game::input::GameInputProcessor::GameInputProcessorDyn>>> = inputProcessorArray[(index) as usize].clone();
            if inputProcessorCanBeNull.is_none() {
                inputProcessorArray[(index) as usize] = noInputProcessor.clone();
            }
        }
    }
}

