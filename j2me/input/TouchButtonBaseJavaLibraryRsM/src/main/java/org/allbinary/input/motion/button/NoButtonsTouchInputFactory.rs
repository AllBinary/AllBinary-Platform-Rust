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
// package org.allbinary.input.motion.button
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use super::BaseTouchInput::BaseTouchInput;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct NoButtonsTouchInputFactory {
}

thread_local! { pub static NoButtonsTouchInputFactory_instance: Option<Rc<RefCell<dyn crate::org::allbinary::input::motion::button::BaseTouchInput::BaseTouchInputDyn>>> = BaseTouchInput::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::input::motion::button::BaseTouchInput::BaseTouchInputDyn>>); }

impl NoButtonsTouchInputFactory {

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<dyn crate::org::allbinary::input::motion::button::BaseTouchInput::BaseTouchInputDyn>>> {
        return NoButtonsTouchInputFactory_instance.with(Clone::clone).clone();
    }
}

