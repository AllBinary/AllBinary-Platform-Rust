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
use crate::org::allbinary::time::GameTickTimeDelayHelperFactory::GameTickTimeDelayHelperFactory;
use crate::org::allbinary::time::TimeDelayHelper::TimeDelayHelper;
use super::BasicConstantVelocityMovement::BasicConstantVelocityMovement;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct DelayedConstantVelocityMovement {
    pub base: BasicConstantVelocityMovement,
    pub timeDelayHelper: Option<Rc<RefCell<dyn crate::org::allbinary::time::TimeDelayHelper::TimeDelayHelperDyn>>>,
}


impl DelayedConstantVelocityMovement {
    

    pub fn new_struct(mut basicDecimal: Option<Rc<RefCell<BasicDecimal>>>, mut velocityProperties: Option<Rc<RefCell<dyn crate::org::allbinary::game::physics::velocity::BasicVelocityProperties::BasicVelocityPropertiesDyn>>>) -> Self {
        let mut this = Self {
            base: BasicConstantVelocityMovement::new_struct(basicDecimal.clone(), velocityProperties.clone()),
            timeDelayHelper: None,
        };
        this.timeDelayHelper = TimeDelayHelper::new(60).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::time::TimeDelayHelper::TimeDelayHelperDyn>>);
        // super(...) is in the struct literal above
        this
    }

    pub fn new(mut basicDecimal: Option<Rc<RefCell<BasicDecimal>>>, mut velocityProperties: Option<Rc<RefCell<dyn crate::org::allbinary::game::physics::velocity::BasicVelocityProperties::BasicVelocityPropertiesDyn>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(basicDecimal, velocityProperties))))
    }
    

    pub fn init(&mut self, mut speedBasicDecimal: Option<Rc<RefCell<BasicDecimal>>>, mut angle: i32, mut otherAngle: i32) {
    }
    

    // throws Exception
    pub fn process(&mut self, mut layer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::AllBinaryGameLayer::AllBinaryGameLayerDyn>>>) {
        if { let java_receiver = self.timeDelayHelper.clone(); let java_arguments = (jcall!(GameTickTimeDelayHelperFactory::getInstance()).startTime.clone(),); let java_result = jcall!(java_receiver).isTime(java_arguments.0); java_result } {
            self.base.process(layer.clone());
        }
    }
}
impl std::ops::Deref for DelayedConstantVelocityMovement {
    type Target = BasicConstantVelocityMovement;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for DelayedConstantVelocityMovement {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::physics::movement::BasicConstantVelocityMovement::BasicConstantVelocityMovementDyn for DelayedConstantVelocityMovement {
    fn as_BasicConstantVelocityMovement(&self) -> &crate::org::allbinary::physics::movement::BasicConstantVelocityMovement::BasicConstantVelocityMovement { &self.base }
    fn as_BasicConstantVelocityMovement_mut(&mut self) -> &mut crate::org::allbinary::physics::movement::BasicConstantVelocityMovement::BasicConstantVelocityMovement { &mut self.base }
}
impl crate::org::allbinary::physics::movement::Movement::MovementDyn for DelayedConstantVelocityMovement {
    fn as_Movement(&self) -> &crate::org::allbinary::physics::movement::Movement::Movement { &self.base.base }
    fn as_Movement_mut(&mut self) -> &mut crate::org::allbinary::physics::movement::Movement::Movement { &mut self.base.base }
}
impl crate::org::allbinary::physics::movement::MovementInterface::MovementInterface for DelayedConstantVelocityMovement {
    fn init(&mut self, mut speedBasicDecimal: Option<Rc<RefCell<BasicDecimal>>>, mut angle: i32, mut otherAngle: i32) { Self::init(self, speedBasicDecimal, angle, otherAngle) }
    fn process(&mut self, mut layer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::AllBinaryGameLayer::AllBinaryGameLayerDyn>>>) { Self::process(self, layer) }
    fn stop(&mut self) { crate::org::allbinary::physics::movement::BasicConstantVelocityMovement::BasicConstantVelocityMovement::stop(crate::org::allbinary::physics::movement::BasicConstantVelocityMovement::BasicConstantVelocityMovementDyn::as_BasicConstantVelocityMovement_mut(self)) }
}
impl crate::org::allbinary::game::physics::velocity::VelocityInterfaceCompositeInterface::VelocityInterfaceCompositeInterface for DelayedConstantVelocityMovement {
    fn getVelocityProperties(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::game::physics::velocity::BasicVelocityProperties::BasicVelocityPropertiesDyn>>> { crate::org::allbinary::physics::movement::BasicConstantVelocityMovement::BasicConstantVelocityMovement::getVelocityProperties(crate::org::allbinary::physics::movement::BasicConstantVelocityMovement::BasicConstantVelocityMovementDyn::as_BasicConstantVelocityMovement_mut(self)) }
}


