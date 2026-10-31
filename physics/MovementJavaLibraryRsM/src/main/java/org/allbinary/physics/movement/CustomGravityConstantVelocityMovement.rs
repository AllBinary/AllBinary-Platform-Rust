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
use crate::org::allbinary::game::physics::acceleration::GravityUtil::GravityUtil;
use crate::org::allbinary::game::physics::velocity::VelocityProperties::VelocityProperties;
use crate::org::allbinary::logic::math::BasicDecimal::BasicDecimal;
use super::BasicConstantVelocityMovement::BasicConstantVelocityMovement;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct CustomGravityConstantVelocityMovement {
    pub base: BasicConstantVelocityMovement,
    pub customGravity: i32,
    pub gravityUtil: Option<Rc<RefCell<GravityUtil>>>,
}


impl CustomGravityConstantVelocityMovement {
    

    pub fn new_struct(mut speedBasicDecimal: Option<Rc<RefCell<BasicDecimal>>>, mut customGravity: i32) -> Self {
        let mut this = Self {
            base: BasicConstantVelocityMovement::new_struct(speedBasicDecimal.clone(), VelocityProperties::new(((jcall!(speedBasicDecimal).getUnscaled()) as i32), ((jcall!(speedBasicDecimal).getUnscaled()) as i32))),
            customGravity: 0,
            gravityUtil: None,
        };
        this.gravityUtil = GravityUtil::getInstance();
        // super(...) is in the struct literal above
        this.customGravity = customGravity.clone();
        this
    }

    pub fn new(mut speedBasicDecimal: Option<Rc<RefCell<BasicDecimal>>>, mut customGravity: i32) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(speedBasicDecimal, customGravity))))
    }
    

    // throws Exception
    pub fn process(&mut self, mut layer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::AllBinaryGameLayer::AllBinaryGameLayerDyn>>>) {
        self.base.process(layer.clone());
        { let java_receiver = self.gravityUtil.clone(); let java_arguments = (self.getVelocityProperties(), self.customGravity.clone(),); jcall!(java_receiver).process(java_arguments.0, java_arguments.1); };
        let mut velocityProperties: Option<Rc<RefCell<VelocityProperties>>> = (self.getVelocityProperties()).clone() /* as VelocityProperties */;
        jcall!(velocityProperties).limitMaxYForwardVelocity();
    }
}
impl std::ops::Deref for CustomGravityConstantVelocityMovement {
    type Target = BasicConstantVelocityMovement;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for CustomGravityConstantVelocityMovement {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::physics::movement::BasicConstantVelocityMovement::BasicConstantVelocityMovementDyn for CustomGravityConstantVelocityMovement {
    fn as_BasicConstantVelocityMovement(&self) -> &crate::org::allbinary::physics::movement::BasicConstantVelocityMovement::BasicConstantVelocityMovement { &self.base }
    fn as_BasicConstantVelocityMovement_mut(&mut self) -> &mut crate::org::allbinary::physics::movement::BasicConstantVelocityMovement::BasicConstantVelocityMovement { &mut self.base }
}
impl crate::org::allbinary::physics::movement::Movement::MovementDyn for CustomGravityConstantVelocityMovement {
    fn as_Movement(&self) -> &crate::org::allbinary::physics::movement::Movement::Movement { &self.base.base }
    fn as_Movement_mut(&mut self) -> &mut crate::org::allbinary::physics::movement::Movement::Movement { &mut self.base.base }
}
impl crate::org::allbinary::physics::movement::MovementInterface::MovementInterface for CustomGravityConstantVelocityMovement {
    fn init(&mut self, p1: Option<Rc<RefCell<crate::org::allbinary::logic::math::BasicDecimal::BasicDecimal>>>, p2: i32, p3: i32) { crate::org::allbinary::physics::movement::BasicConstantVelocityMovement::BasicConstantVelocityMovement::init(crate::org::allbinary::physics::movement::BasicConstantVelocityMovement::BasicConstantVelocityMovementDyn::as_BasicConstantVelocityMovement_mut(self), p1, p2, p3) }
    fn process(&mut self, mut layer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::AllBinaryGameLayer::AllBinaryGameLayerDyn>>>) { Self::process(self, layer) }
    fn stop(&mut self) { crate::org::allbinary::physics::movement::BasicConstantVelocityMovement::BasicConstantVelocityMovement::stop(crate::org::allbinary::physics::movement::BasicConstantVelocityMovement::BasicConstantVelocityMovementDyn::as_BasicConstantVelocityMovement_mut(self)) }
}
impl crate::org::allbinary::game::physics::velocity::VelocityInterfaceCompositeInterface::VelocityInterfaceCompositeInterface for CustomGravityConstantVelocityMovement {
    fn getVelocityProperties(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::game::physics::velocity::BasicVelocityProperties::BasicVelocityPropertiesDyn>>> { crate::org::allbinary::physics::movement::BasicConstantVelocityMovement::BasicConstantVelocityMovement::getVelocityProperties(crate::org::allbinary::physics::movement::BasicConstantVelocityMovement::BasicConstantVelocityMovementDyn::as_BasicConstantVelocityMovement_mut(self)) }
}


