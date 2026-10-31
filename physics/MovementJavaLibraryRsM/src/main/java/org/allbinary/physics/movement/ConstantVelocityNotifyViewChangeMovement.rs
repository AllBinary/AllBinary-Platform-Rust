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
use crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer;
use crate::org::allbinary::logic::math::BasicDecimal::BasicDecimal;
use crate::org::allbinary::logic::math::vector::AxisMathVectorUtil::AxisMathVectorUtil;
use crate::org::allbinary::math::AngleFactory::AngleFactory;
use super::Movement::Movement;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct ConstantVelocityNotifyViewChangeMovement {
    pub base: Movement,
    pub basicVelocityProperties: Option<Rc<RefCell<dyn crate::org::allbinary::game::physics::velocity::BasicVelocityProperties::BasicVelocityPropertiesDyn>>>,
    pub speedBasicDecimal: Option<Rc<RefCell<BasicDecimal>>>,
    pub axisMathVectorUtil: Option<Rc<RefCell<AxisMathVectorUtil>>>,
}


impl ConstantVelocityNotifyViewChangeMovement {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: Movement::new_struct(),
            basicVelocityProperties: None,
            speedBasicDecimal: None,
            axisMathVectorUtil: None,
        };
        this.axisMathVectorUtil = AxisMathVectorUtil::getInstance();
        this.speedBasicDecimal = crate::org::allbinary::logic::math::BasicDecimal::BasicDecimal_ZERO_BIGDECIMAL.with(Clone::clone).clone();
        this.basicVelocityProperties = BasicVelocityProperties::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::physics::velocity::BasicVelocityProperties::BasicVelocityPropertiesDyn>>);
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn init(&mut self, mut speedBasicDecimal: Option<Rc<RefCell<BasicDecimal>>>, mut angle: i32, mut otherAngle: i32) {
        self.speedBasicDecimal = speedBasicDecimal.clone();
        let mut angleFactory: Option<Rc<RefCell<AngleFactory>>> = AngleFactory::getInstance();
        { let java_receiver = self.basicVelocityProperties.clone(); let java_arguments = (speedBasicDecimal.clone(), jcall!(angleFactory).getAt(angle.clone()), jcall!(angleFactory).getAt(otherAngle.clone()),); jcall!(java_receiver).setVelocityWithBigDecimal(java_arguments.0, java_arguments.1, java_arguments.2); };
    }
    

    pub fn moveOutsideRadius(&mut self, mut layer: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>, mut radius: i64, mut angle: i32, mut otherAngle: i32) {
        let mut scaleFactorValue: i32 = jcall!(self.speedBasicDecimal).getScaledFactorValue();
        let mut xVector: i32 = (((jcall!(self.axisMathVectorUtil).calculateX(radius.clone(), angle.clone()) / ((scaleFactorValue) as i64))) as i32);
        let mut yVector: i32 = (((jcall!(self.axisMathVectorUtil).calculateY(radius.clone(), angle.clone()) / ((scaleFactorValue) as i64))) as i32);
        let mut zVector: i32 = (((jcall!(self.axisMathVectorUtil).calculateZ(radius.clone(), otherAngle.clone()) / ((scaleFactorValue) as i64))) as i32);
        jcall!(layer).moveDXYZ(xVector.clone(), yVector.clone(), zVector.clone());
    }
    

    // throws Exception
    pub fn process(&mut self, mut layer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::AllBinaryGameLayer::AllBinaryGameLayerDyn>>>) {
        let mut x: i32 = jcall!(jcall!(self.basicVelocityProperties).getVelocityXBasicDecimalP()).getScaled();
        let mut y: i32 = jcall!(jcall!(self.basicVelocityProperties).getVelocityYBasicDecimalP()).getScaled();
        let mut z: i32 = jcall!(jcall!(self.basicVelocityProperties).getVelocityZBasicDecimalP()).getScaled();
        jcall!(layer).moveDXYZ(x.clone(), y.clone(), z.clone());
        if x != 0 || y != 0 || z != 0 {
            jcall!(layer).as_AllBinaryLayer_mut().onViewPositionChangeEvent();
        }
    }
    

    pub fn stop(&mut self) {
        jcall!(self.basicVelocityProperties).zero();
    }
}
impl std::ops::Deref for ConstantVelocityNotifyViewChangeMovement {
    type Target = Movement;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for ConstantVelocityNotifyViewChangeMovement {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::physics::movement::Movement::MovementDyn for ConstantVelocityNotifyViewChangeMovement {
    fn as_Movement(&self) -> &crate::org::allbinary::physics::movement::Movement::Movement { &self.base }
    fn as_Movement_mut(&mut self) -> &mut crate::org::allbinary::physics::movement::Movement::Movement { &mut self.base }
}
impl crate::org::allbinary::physics::movement::MovementInterface::MovementInterface for ConstantVelocityNotifyViewChangeMovement {
    fn init(&mut self, mut speedBasicDecimal: Option<Rc<RefCell<BasicDecimal>>>, mut angle: i32, mut otherAngle: i32) { Self::init(self, speedBasicDecimal, angle, otherAngle) }
    fn process(&mut self, mut layer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::AllBinaryGameLayer::AllBinaryGameLayerDyn>>>) { Self::process(self, layer) }
    fn stop(&mut self) { Self::stop(self) }
}


