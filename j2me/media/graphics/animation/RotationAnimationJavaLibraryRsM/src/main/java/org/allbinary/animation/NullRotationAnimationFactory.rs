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
use crate::org::allbinary::media::ScaleProperties::ScaleProperties;
use crate::java::lang::Exception::*;
use super::AnimationInterfaceFactoryInterface::AnimationInterfaceFactoryInterface;
use super::ProceduralAnimationInterfaceFactoryInterface::ProceduralAnimationInterfaceFactoryInterface;
use super::RotationAnimation::RotationAnimation;
use super::NullRotationAnimation2::NullRotationAnimation2;
use super::Animation::Animation;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct NullRotationAnimationFactory {
    pub NULL_ROTATION_ANIMATION_ARRAY: Vec<Option<Rc<RefCell<dyn crate::org::allbinary::animation::RotationAnimation::RotationAnimationDyn>>>>,
    pub NULL_ANIMATION: Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>>,
}

pub static mut NullRotationAnimationFactory_NULL_ROTATION_ANIMATION_FACTORY: std::cell::LazyCell<Option<Rc<RefCell<NullRotationAnimationFactory>>>> = std::cell::LazyCell::new(|| NullRotationAnimationFactory::new());
impl AnimationInterfaceFactoryInterface for NullRotationAnimationFactory {
    fn getInstance(&mut self, mut instanceId: i32) -> Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>> { Self::getInstance(self, instanceId) }
    fn setInitialScale(&mut self, mut scaleProperties: Option<Rc<RefCell<ScaleProperties>>>) { Self::setInitialScale(self, scaleProperties) }
}
impl crate::org::allbinary::animation::BasicAnimationInterfaceFactoryInterface::BasicAnimationInterfaceFactoryInterface for NullRotationAnimationFactory {
}
impl ProceduralAnimationInterfaceFactoryInterface for NullRotationAnimationFactory {
    fn getInstanceAnimation(&mut self, mut animationInterface: Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>>) -> Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>> { Self::getInstanceAnimation(self, animationInterface) }
}

impl NullRotationAnimationFactory {

    pub fn toString(&mut self) -> String {
        String::from(std::any::type_name::<Self>())
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            NULL_ROTATION_ANIMATION_ARRAY: Vec::new(),
            NULL_ANIMATION: None,
        };
        this.NULL_ROTATION_ANIMATION_ARRAY = vec![None; (0) as usize];
        this.NULL_ANIMATION = NullRotationAnimation2::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>);
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getFactoryInstance() -> Option<Rc<RefCell<NullRotationAnimationFactory>>> {
        return unsafe { NullRotationAnimationFactory_NULL_ROTATION_ANIMATION_FACTORY.clone() }.clone();
    }
    

    pub fn getInstance(&mut self, mut instanceId: i32) -> Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>> {
        return self.NULL_ANIMATION.clone();
    }
    

    // throws Exception
    pub fn getInstanceAnimation(&mut self, mut animationInterface: Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>>) -> Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>> {
        return self.NULL_ANIMATION.clone();
    }
    

    pub fn setInitialScale(&mut self, mut scaleProperties: Option<Rc<RefCell<ScaleProperties>>>) {
    }
}

