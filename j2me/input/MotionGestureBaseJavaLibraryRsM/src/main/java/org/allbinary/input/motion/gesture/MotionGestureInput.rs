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
// package org.allbinary.input.motion.gesture
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::game::input::Input::Input;
use crate::org::allbinary::game::input::InputFactory::InputFactory;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct MotionGestureInput {
    pub base: Input,
}


impl MotionGestureInput {
    

    pub fn new_struct(mut id: i32, mut name: String) -> Self {
        let mut this = Self {
            base: Input::new_struct(id.clone(), name.clone()),
        };
        // super(...) is in the struct literal above
        let mut inputFactory: Option<Rc<RefCell<InputFactory>>> = InputFactory::getInstance();
        this
    }

    pub fn new(mut id: i32, mut name: String) -> Option<Rc<RefCell<Self>>> {
        let this = Some(Rc::new(RefCell::new(Self::new_struct(id, name))));
            jcall!(inputFactory).add(this.getId(), (|java_value: Option<Rc<RefCell<_>>>| java_value.map(|reference| Rc::new(RefCell::new(Box::new(Some(reference)) as Box<dyn std::any::Any>))))(this.clone()));
        this
    }
}
impl std::ops::Deref for MotionGestureInput {
    type Target = Input;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for MotionGestureInput {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::game::input::Input::InputDyn for MotionGestureInput {
    fn as_Input(&self) -> &crate::org::allbinary::game::input::Input::Input { &self.base }
    fn as_Input_mut(&mut self) -> &mut crate::org::allbinary::game::input::Input::Input { &mut self.base }
}


