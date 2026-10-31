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
// package org.allbinary.animation.compound
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::animation::Animation::Animation;
use crate::org::allbinary::animation::AnimationBehaviorFactory::AnimationBehaviorFactory;
use crate::org::allbinary::animation::AnimationInterfaceFactoryInterface::AnimationInterfaceFactoryInterface;
use crate::org::allbinary::animation::IndexedAnimation::IndexedAnimation;
use crate::org::allbinary::logic::communication::log::ForcedLogUtil::ForcedLogUtil;
use crate::org::allbinary::media::ScaleProperties::ScaleProperties;
use super::SimultaneousCompoundIndexAnimation::SimultaneousCompoundIndexAnimation;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct SimultaneousCompoundIndexAnimationInterfaceFactory {
    pub basicAnimationInterfaceFactoryInterfaceArray: Vec<Option<Rc<RefCell<dyn AnimationInterfaceFactoryInterface>>>>,
    pub animationBehaviorFactory: Option<Rc<RefCell<dyn crate::org::allbinary::animation::AnimationBehaviorFactory::AnimationBehaviorFactoryDyn>>>,
}

impl AnimationInterfaceFactoryInterface for SimultaneousCompoundIndexAnimationInterfaceFactory {
    fn getInstance(&mut self, mut instanceId: i32) -> Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>> { Self::getInstance(self, instanceId) }
    fn setInitialScale(&mut self, mut scaleProperties: Option<Rc<RefCell<ScaleProperties>>>) { Self::setInitialScale(self, scaleProperties) }
}
impl crate::org::allbinary::animation::BasicAnimationInterfaceFactoryInterface::BasicAnimationInterfaceFactoryInterface for SimultaneousCompoundIndexAnimationInterfaceFactory {
}

impl SimultaneousCompoundIndexAnimationInterfaceFactory {
    

    pub fn new_struct(mut basicAnimationInterfaceFactoryInterfaceArray: Vec<Option<Rc<RefCell<dyn AnimationInterfaceFactoryInterface>>>>, mut animationBehaviorFactory: Option<Rc<RefCell<dyn crate::org::allbinary::animation::AnimationBehaviorFactory::AnimationBehaviorFactoryDyn>>>) -> Self {
        let mut this = Self {
            basicAnimationInterfaceFactoryInterfaceArray: Vec::new(),
            animationBehaviorFactory: None,
        };
        this.basicAnimationInterfaceFactoryInterfaceArray = basicAnimationInterfaceFactoryInterfaceArray.clone();
        this.animationBehaviorFactory = animationBehaviorFactory.clone();
        this
    }

    pub fn new(mut basicAnimationInterfaceFactoryInterfaceArray: Vec<Option<Rc<RefCell<dyn AnimationInterfaceFactoryInterface>>>>, mut animationBehaviorFactory: Option<Rc<RefCell<dyn crate::org::allbinary::animation::AnimationBehaviorFactory::AnimationBehaviorFactoryDyn>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(basicAnimationInterfaceFactoryInterfaceArray, animationBehaviorFactory))))
    }
    

    // throws Exception
    pub fn getInstance(&mut self, mut instanceId: i32) -> Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>> {
        let mut size: i32 = (self.basicAnimationInterfaceFactoryInterfaceArray.len() as i32).clone();
        let mut animationInterfaceArray: Vec<Option<Rc<RefCell<dyn crate::org::allbinary::animation::IndexedAnimation::IndexedAnimationDyn>>>> = vec![None; (size) as usize];
        for index in (0..size) {
            animationInterfaceArray[(index) as usize] = (jcall!(self.basicAnimationInterfaceFactoryInterfaceArray[(index) as usize]).getInstance(instanceId.clone())).clone() /* as IndexedAnimation */;
        }
        return self.getOrCreate(animationInterfaceArray.clone());
    }
    

    pub fn getOrCreate(&mut self, mut animationInterfaceArray: Vec<Option<Rc<RefCell<dyn crate::org::allbinary::animation::IndexedAnimation::IndexedAnimationDyn>>>>) -> Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>> {
        return SimultaneousCompoundIndexAnimation::new(animationInterfaceArray.clone(), jcall!(self.animationBehaviorFactory).getOrCreateInstance()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>);
    }
    

    pub fn getBasicAnimationInterfaceFactoryInterfaceArray(&mut self) -> Vec<Option<Rc<RefCell<dyn AnimationInterfaceFactoryInterface>>>> {
        return self.basicAnimationInterfaceFactoryInterfaceArray.clone();
    }
    

    pub fn setInitialScale(&mut self, mut scaleProperties: Option<Rc<RefCell<ScaleProperties>>>) {
    }
}

