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
// package org.allbinary.physics.movement
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::game::layer::AllBinaryGameLayer::AllBinaryGameLayer;
use crate::org::allbinary::logic::math::BasicDecimal::BasicDecimal;
use super::MovementInterface::MovementInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct Movement {
}

impl MovementInterface for Movement {
    fn init(&mut self, mut speedBasicDecimal: Option<Rc<RefCell<BasicDecimal>>>, mut angle: i32, mut otherAngle: i32) { Self::init(self, speedBasicDecimal, angle, otherAngle) }
    fn process(&mut self, mut layer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::AllBinaryGameLayer::AllBinaryGameLayerDyn>>>) { Self::process(self, layer) }
    fn stop(&mut self) { Self::stop(self) }
}

impl Movement {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn init(&mut self, mut speedBasicDecimal: Option<Rc<RefCell<BasicDecimal>>>, mut angle: i32, mut otherAngle: i32) {
    }
    

    // throws Exception
    pub fn process(&mut self, mut layer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::AllBinaryGameLayer::AllBinaryGameLayerDyn>>>) {
    }
    

    pub fn stop(&mut self) {
    }
}
pub trait MovementDyn: crate::org::allbinary::physics::movement::MovementInterface::MovementInterface {
    fn as_Movement(&self) -> &Movement;
    fn as_Movement_mut(&mut self) -> &mut Movement;
}
impl MovementDyn for Movement {
    fn as_Movement(&self) -> &Movement { self }
    fn as_Movement_mut(&mut self) -> &mut Movement { self }
}


