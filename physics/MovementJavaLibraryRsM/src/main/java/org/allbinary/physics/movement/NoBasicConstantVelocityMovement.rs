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
use crate::org::allbinary::game::physics::velocity::BasicVelocityProperties::BasicVelocityProperties;
use crate::org::allbinary::logic::math::BasicDecimal::BasicDecimal;
use super::BasicConstantVelocityMovement::BasicConstantVelocityMovement;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct NoBasicConstantVelocityMovement {
    pub base: BasicConstantVelocityMovement,
}


impl NoBasicConstantVelocityMovement {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: BasicConstantVelocityMovement::new_struct(crate::org::allbinary::logic::math::BasicDecimal::BasicDecimal_ZERO_BIGDECIMAL.with(Clone::clone).clone(), BasicVelocityProperties::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::physics::velocity::BasicVelocityProperties::BasicVelocityPropertiesDyn>>)),
        };
        // super(...) is in the struct literal above
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    // throws Exception
    pub fn process(&mut self, mut layer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::AllBinaryGameLayer::AllBinaryGameLayerDyn>>>) {
    }
}
impl std::ops::Deref for NoBasicConstantVelocityMovement {
    type Target = BasicConstantVelocityMovement;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for NoBasicConstantVelocityMovement {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::physics::movement::BasicConstantVelocityMovement::BasicConstantVelocityMovementDyn for NoBasicConstantVelocityMovement {
    fn as_BasicConstantVelocityMovement(&self) -> &crate::org::allbinary::physics::movement::BasicConstantVelocityMovement::BasicConstantVelocityMovement { &self.base }
    fn as_BasicConstantVelocityMovement_mut(&mut self) -> &mut crate::org::allbinary::physics::movement::BasicConstantVelocityMovement::BasicConstantVelocityMovement { &mut self.base }
}
impl crate::org::allbinary::physics::movement::Movement::MovementDyn for NoBasicConstantVelocityMovement {
    fn as_Movement(&self) -> &crate::org::allbinary::physics::movement::Movement::Movement { &self.base.base }
    fn as_Movement_mut(&mut self) -> &mut crate::org::allbinary::physics::movement::Movement::Movement { &mut self.base.base }
}
impl crate::org::allbinary::physics::movement::MovementInterface::MovementInterface for NoBasicConstantVelocityMovement {
    fn init(&mut self, p1: Option<Rc<RefCell<crate::org::allbinary::logic::math::BasicDecimal::BasicDecimal>>>, p2: i32, p3: i32) { crate::org::allbinary::physics::movement::BasicConstantVelocityMovement::BasicConstantVelocityMovement::init(crate::org::allbinary::physics::movement::BasicConstantVelocityMovement::BasicConstantVelocityMovementDyn::as_BasicConstantVelocityMovement_mut(self), p1, p2, p3) }
    fn process(&mut self, mut layer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::AllBinaryGameLayer::AllBinaryGameLayerDyn>>>) { Self::process(self, layer) }
    fn stop(&mut self) { crate::org::allbinary::physics::movement::BasicConstantVelocityMovement::BasicConstantVelocityMovement::stop(crate::org::allbinary::physics::movement::BasicConstantVelocityMovement::BasicConstantVelocityMovementDyn::as_BasicConstantVelocityMovement_mut(self)) }
}
impl crate::org::allbinary::game::physics::velocity::VelocityInterfaceCompositeInterface::VelocityInterfaceCompositeInterface for NoBasicConstantVelocityMovement {
    fn getVelocityProperties(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::game::physics::velocity::BasicVelocityProperties::BasicVelocityPropertiesDyn>>> { crate::org::allbinary::physics::movement::BasicConstantVelocityMovement::BasicConstantVelocityMovement::getVelocityProperties(crate::org::allbinary::physics::movement::BasicConstantVelocityMovement::BasicConstantVelocityMovementDyn::as_BasicConstantVelocityMovement_mut(self)) }
}


