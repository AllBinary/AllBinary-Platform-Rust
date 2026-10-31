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
use crate::org::allbinary::animation::NullAnimationFactory::NullAnimationFactory;
use crate::org::allbinary::logic::communication::log::ForcedLogUtil::ForcedLogUtil;
use crate::org::allbinary::media::ScaleProperties::ScaleProperties;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct CompoundAnimationInterfaceFactory {
    pub basicAnimationInterfaceFactoryInterfaceArrayP: Vec<Option<Rc<RefCell<dyn AnimationInterfaceFactoryInterface>>>>,
    pub animationBehaviorFactory: Option<Rc<RefCell<dyn crate::org::allbinary::animation::AnimationBehaviorFactory::AnimationBehaviorFactoryDyn>>>,
}

impl AnimationInterfaceFactoryInterface for CompoundAnimationInterfaceFactory {
    fn getInstance(&mut self, mut instanceId: i32) -> Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>> { Self::getInstance(self, instanceId) }
    fn setInitialScale(&mut self, mut scaleProperties: Option<Rc<RefCell<ScaleProperties>>>) { Self::setInitialScale(self, scaleProperties) }
}
impl crate::org::allbinary::animation::BasicAnimationInterfaceFactoryInterface::BasicAnimationInterfaceFactoryInterface for CompoundAnimationInterfaceFactory {
}

impl CompoundAnimationInterfaceFactory {
    

    pub fn new_struct(mut basicAnimationInterfaceFactoryInterfaceArray: Vec<Option<Rc<RefCell<dyn AnimationInterfaceFactoryInterface>>>>, mut animationBehaviorFactory: Option<Rc<RefCell<dyn crate::org::allbinary::animation::AnimationBehaviorFactory::AnimationBehaviorFactoryDyn>>>) -> Self {
        let mut this = Self {
            basicAnimationInterfaceFactoryInterfaceArrayP: Vec::new(),
            animationBehaviorFactory: None,
        };
        this.basicAnimationInterfaceFactoryInterfaceArrayP = basicAnimationInterfaceFactoryInterfaceArray.clone();
        this.animationBehaviorFactory = animationBehaviorFactory.clone();
        this
    }

    pub fn new(mut basicAnimationInterfaceFactoryInterfaceArray: Vec<Option<Rc<RefCell<dyn AnimationInterfaceFactoryInterface>>>>, mut animationBehaviorFactory: Option<Rc<RefCell<dyn crate::org::allbinary::animation::AnimationBehaviorFactory::AnimationBehaviorFactoryDyn>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(basicAnimationInterfaceFactoryInterfaceArray, animationBehaviorFactory))))
    }
    

    // throws Exception
    pub fn getInstance(&mut self, mut instanceId: i32) -> Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>> {
        let mut size: i32 = (self.basicAnimationInterfaceFactoryInterfaceArrayP.len() as i32).clone();
        let mut animationInterfaceArray: Vec<Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>>> = self.createArray(size.clone());
        for index in (0..size) {
            animationInterfaceArray[(index) as usize] = jcall!(self.basicAnimationInterfaceFactoryInterfaceArrayP[(index) as usize]).getInstance(instanceId.clone());
        }
        return self.createAnimation(animationInterfaceArray.clone());
    }
    

    pub fn createArray(&mut self, mut size: i32) -> Vec<Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>>> {
        return jcall!(NullAnimationFactory::getFactoryInstance()).EMPTY_ARRAY.clone();
    }
    

    pub fn createAnimation(&mut self, mut animationInterfaceArray: Vec<Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>>>) -> Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>> {
        return jcall!(NullAnimationFactory::getFactoryInstance()).getInstance(0);
    }
    

    pub fn getBasicAnimationInterfaceFactoryInterfaceArray(&mut self) -> Vec<Option<Rc<RefCell<dyn AnimationInterfaceFactoryInterface>>>> {
        return self.basicAnimationInterfaceFactoryInterfaceArrayP.clone();
    }
    

    pub fn setInitialScale(&mut self, mut scaleProperties: Option<Rc<RefCell<ScaleProperties>>>) {
        let mut size: i32 = (self.basicAnimationInterfaceFactoryInterfaceArrayP.len() as i32).clone();
        for index in (0..size) {
            jcall!(self.basicAnimationInterfaceFactoryInterfaceArrayP[(index) as usize]).setInitialScale(scaleProperties.clone());
        }
    }
}
pub trait CompoundAnimationInterfaceFactoryDyn {
    fn as_CompoundAnimationInterfaceFactory(&self) -> &CompoundAnimationInterfaceFactory;
    fn as_CompoundAnimationInterfaceFactory_mut(&mut self) -> &mut CompoundAnimationInterfaceFactory;
    fn getInstance(&mut self, mut instanceId: i32) -> Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>> { CompoundAnimationInterfaceFactory::getInstance(self.as_CompoundAnimationInterfaceFactory_mut(), instanceId) }
    fn createArray(&mut self, mut size: i32) -> Vec<Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>>> { CompoundAnimationInterfaceFactory::createArray(self.as_CompoundAnimationInterfaceFactory_mut(), size) }
    fn createAnimation(&mut self, mut animationInterfaceArray: Vec<Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>>>) -> Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>> { CompoundAnimationInterfaceFactory::createAnimation(self.as_CompoundAnimationInterfaceFactory_mut(), animationInterfaceArray) }
    fn getBasicAnimationInterfaceFactoryInterfaceArray(&mut self) -> Vec<Option<Rc<RefCell<dyn AnimationInterfaceFactoryInterface>>>> { CompoundAnimationInterfaceFactory::getBasicAnimationInterfaceFactoryInterfaceArray(self.as_CompoundAnimationInterfaceFactory_mut()) }
    fn setInitialScale(&mut self, mut scaleProperties: Option<Rc<RefCell<ScaleProperties>>>) { CompoundAnimationInterfaceFactory::setInitialScale(self.as_CompoundAnimationInterfaceFactory_mut(), scaleProperties) }
}
impl CompoundAnimationInterfaceFactoryDyn for CompoundAnimationInterfaceFactory {
    fn as_CompoundAnimationInterfaceFactory(&self) -> &CompoundAnimationInterfaceFactory { self }
    fn as_CompoundAnimationInterfaceFactory_mut(&mut self) -> &mut CompoundAnimationInterfaceFactory { self }
}


