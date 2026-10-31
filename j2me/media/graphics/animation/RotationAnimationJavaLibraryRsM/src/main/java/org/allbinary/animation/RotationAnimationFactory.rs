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
// package org.allbinary.animation
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::game::configuration::GameConfigurationCentral::GameConfigurationCentral;
use crate::org::allbinary::math::AngleFactory::AngleFactory;
use crate::org::allbinary::math::AngleInfo::AngleInfo;
use crate::org::allbinary::media::ScaleProperties::ScaleProperties;
use super::AnimationInterfaceFactoryInterface::AnimationInterfaceFactoryInterface;
use super::ProceduralAnimationInterfaceFactoryInterface::ProceduralAnimationInterfaceFactoryInterface;
use super::Animation::Animation;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct RotationAnimationFactory {
    pub totalAngle: i16,
    pub angleIncrement: i16,
}

impl AnimationInterfaceFactoryInterface for RotationAnimationFactory {
    fn getInstance(&mut self, mut instanceId: i32) -> Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>> { Self::getInstance(self, instanceId) }
    fn setInitialScale(&mut self, mut scaleProperties: Option<Rc<RefCell<ScaleProperties>>>) { Self::setInitialScale(self, scaleProperties) }
}
impl crate::org::allbinary::animation::BasicAnimationInterfaceFactoryInterface::BasicAnimationInterfaceFactoryInterface for RotationAnimationFactory {
}
impl ProceduralAnimationInterfaceFactoryInterface for RotationAnimationFactory {
    fn getInstanceAnimation(&mut self, mut animationInterface: Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>>) -> Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>> { Self::getInstanceAnimation(self, animationInterface) }
}
impl crate::org::allbinary::animation::BasicAnimationInterfaceFactoryInterface::BasicAnimationInterfaceFactoryInterface for RotationAnimationFactory {
}

impl RotationAnimationFactory {
    

    pub fn createFactory() -> Option<Rc<RefCell<RotationAnimationFactory>>> {
        let mut totalAngle: i16 = jcall!(AngleFactory::getInstance()).TOTAL_ANGLE.clone();
        return RotationAnimationFactory::new(totalAngle.clone(), (((((totalAngle) as i32) / jcall!(GameConfigurationCentral::getInstance()).getGameControlFidelity())) as i16));
    }
    

    pub fn new_struct(mut totalAngle: i16, mut angleIncrement: i16) -> Self {
        let mut this = Self {
            totalAngle: 0,
            angleIncrement: 0,
        };
        this.totalAngle = totalAngle.clone();
        this.angleIncrement = angleIncrement.clone();
        this
    }

    pub fn new(mut totalAngle: i16, mut angleIncrement: i16) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(totalAngle, angleIncrement))))
    }
    

    // throws Exception
    pub fn getInstance(&mut self, mut instanceId: i32) -> Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>> {
        return NullRotationAnimation::NullRotationAnimation::createTotalAngle(AngleInfo::getInstance(self.angleIncrement.clone()), self.totalAngle.clone(), AnimationBehavior::AnimationBehavior::getInstance());
    }
    

    // throws Exception
    pub fn getInstanceAnimation(&mut self, mut animationInterface: Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>>) -> Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>> {
        return NullRotationAnimation::NullRotationAnimation::createTotalAngle(AngleInfo::getInstance(self.angleIncrement.clone()), self.totalAngle.clone(), AnimationBehavior::AnimationBehavior::getInstance());
    }
    

    pub fn setInitialScale(&mut self, mut scaleProperties: Option<Rc<RefCell<ScaleProperties>>>) {
    }
}

