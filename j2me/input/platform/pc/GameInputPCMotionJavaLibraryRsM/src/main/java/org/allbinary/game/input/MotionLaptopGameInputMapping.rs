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
use crate::org::allbinary::game::input::mapping::InputToGameKeyMapping::InputToGameKeyMapping;
use super::PCGameInputMapping::PCGameInputMapping;
use super::MotionLaptopDefaultGameInputMapping::MotionLaptopDefaultGameInputMapping;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct MotionLaptopGameInputMapping {
    pub base: PCGameInputMapping,
}


impl MotionLaptopGameInputMapping {

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: PCGameInputMapping::new_struct(),
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getDefault(&mut self) -> Option<Rc<RefCell<InputToGameKeyMapping>>> {
        return jcall!(MotionLaptopDefaultGameInputMapping::new()).getInputMapping();
    }
}
impl std::ops::Deref for MotionLaptopGameInputMapping {
    type Target = PCGameInputMapping;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for MotionLaptopGameInputMapping {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::game::input::PCGameInputMapping::PCGameInputMappingDyn for MotionLaptopGameInputMapping {
    fn as_PCGameInputMapping(&self) -> &crate::org::allbinary::game::input::PCGameInputMapping::PCGameInputMapping { &self.base }
    fn as_PCGameInputMapping_mut(&mut self) -> &mut crate::org::allbinary::game::input::PCGameInputMapping::PCGameInputMapping { &mut self.base }
}


