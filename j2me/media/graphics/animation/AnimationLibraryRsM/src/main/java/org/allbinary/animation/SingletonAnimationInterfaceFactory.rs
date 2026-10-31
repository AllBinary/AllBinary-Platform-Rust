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
use super::AnimationInterfaceFactoryInterface::AnimationInterfaceFactoryInterface;
use super::ProceduralAnimationInterfaceFactoryInterface::ProceduralAnimationInterfaceFactoryInterface;
use super::Animation::Animation;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct SingletonAnimationInterfaceFactory {
    pub animationInterface: Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>>,
}

impl AnimationInterfaceFactoryInterface for SingletonAnimationInterfaceFactory {
    fn getInstance(&mut self, mut instanceId: i32) -> Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>> { Self::getInstance(self, instanceId) }
    fn setInitialScale(&mut self, mut scaleProperties: Option<Rc<RefCell<ScaleProperties>>>) { Self::setInitialScale(self, scaleProperties) }
}
impl crate::org::allbinary::animation::BasicAnimationInterfaceFactoryInterface::BasicAnimationInterfaceFactoryInterface for SingletonAnimationInterfaceFactory {
}
impl ProceduralAnimationInterfaceFactoryInterface for SingletonAnimationInterfaceFactory {
    fn getInstanceAnimation(&mut self, mut animationInterface: Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>>) -> Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>> { Self::getInstanceAnimation(self, animationInterface) }
}
impl crate::org::allbinary::animation::BasicAnimationInterfaceFactoryInterface::BasicAnimationInterfaceFactoryInterface for SingletonAnimationInterfaceFactory {
}

impl SingletonAnimationInterfaceFactory {
    

    pub fn new_struct(mut animationInterface: Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>>) -> Self {
        let mut this = Self {
            animationInterface: None,
        };
        this.animationInterface = animationInterface.clone();
        this
    }

    pub fn new(mut animationInterface: Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(animationInterface))))
    }
    

    // throws Exception
    pub fn getInstance(&mut self, mut instanceId: i32) -> Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>> {
        return self.animationInterface.clone();
    }
    

    // throws Exception
    pub fn getInstanceAnimation(&mut self, mut animationInterface: Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>>) -> Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>> {
        return self.animationInterface.clone();
    }
    

    pub fn setInitialScale(&mut self, mut scaleProperties: Option<Rc<RefCell<ScaleProperties>>>) {
    }
}
pub trait SingletonAnimationInterfaceFactoryDyn {
    fn as_SingletonAnimationInterfaceFactory(&self) -> &SingletonAnimationInterfaceFactory;
    fn as_SingletonAnimationInterfaceFactory_mut(&mut self) -> &mut SingletonAnimationInterfaceFactory;
    fn getInstance(&mut self, mut instanceId: i32) -> Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>> { SingletonAnimationInterfaceFactory::getInstance(self.as_SingletonAnimationInterfaceFactory_mut(), instanceId) }
    fn getInstanceAnimation(&mut self, mut animationInterface: Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>>) -> Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>> { SingletonAnimationInterfaceFactory::getInstanceAnimation(self.as_SingletonAnimationInterfaceFactory_mut(), animationInterface) }
    fn setInitialScale(&mut self, mut scaleProperties: Option<Rc<RefCell<ScaleProperties>>>) { SingletonAnimationInterfaceFactory::setInitialScale(self.as_SingletonAnimationInterfaceFactory_mut(), scaleProperties) }
}
impl SingletonAnimationInterfaceFactoryDyn for SingletonAnimationInterfaceFactory {
    fn as_SingletonAnimationInterfaceFactory(&self) -> &SingletonAnimationInterfaceFactory { self }
    fn as_SingletonAnimationInterfaceFactory_mut(&mut self) -> &mut SingletonAnimationInterfaceFactory { self }
}


