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
use crate::org::allbinary::logic::communication::log::ForcedLogUtil::ForcedLogUtil;
use super::CompoundAnimationInterfaceFactory::CompoundAnimationInterfaceFactory;
use super::CompoundIndexedAnimation::CompoundIndexedAnimation;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct CompoundIndexedAnimationInterfaceFactory {
    pub base: CompoundAnimationInterfaceFactory,
}

impl crate::org::allbinary::animation::BasicAnimationInterfaceFactoryInterface::BasicAnimationInterfaceFactoryInterface for CompoundIndexedAnimationInterfaceFactory {
}

impl CompoundIndexedAnimationInterfaceFactory {
    

    pub fn new_struct(mut basicAnimationInterfaceFactoryInterfaceArray: Vec<Option<Rc<RefCell<dyn AnimationInterfaceFactoryInterface>>>>, mut animationBehaviorFactory: Option<Rc<RefCell<dyn crate::org::allbinary::animation::AnimationBehaviorFactory::AnimationBehaviorFactoryDyn>>>) -> Self {
        let mut this = Self {
            base: CompoundAnimationInterfaceFactory::new_struct(basicAnimationInterfaceFactoryInterfaceArray.clone(), animationBehaviorFactory.clone()),
        };
        // super(...) is in the struct literal above
        this
    }

    pub fn new(mut basicAnimationInterfaceFactoryInterfaceArray: Vec<Option<Rc<RefCell<dyn AnimationInterfaceFactoryInterface>>>>, mut animationBehaviorFactory: Option<Rc<RefCell<dyn crate::org::allbinary::animation::AnimationBehaviorFactory::AnimationBehaviorFactoryDyn>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(basicAnimationInterfaceFactoryInterfaceArray, animationBehaviorFactory))))
    }
    

    pub fn createArray(&mut self, mut size: i32) -> Vec<Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>>> {
        return vec![None; (size) as usize];
    }
    

    pub fn createAnimation(&mut self, mut animationArray: Vec<Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>>>) -> Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>> {
        return CompoundIndexedAnimation::new(animationArray.clone(), jcall!(self.animationBehaviorFactory).getOrCreateInstance()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>);
    }
}
impl std::ops::Deref for CompoundIndexedAnimationInterfaceFactory {
    type Target = CompoundAnimationInterfaceFactory;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for CompoundIndexedAnimationInterfaceFactory {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::animation::compound::CompoundAnimationInterfaceFactory::CompoundAnimationInterfaceFactoryDyn for CompoundIndexedAnimationInterfaceFactory {
    fn as_CompoundAnimationInterfaceFactory(&self) -> &crate::org::allbinary::animation::compound::CompoundAnimationInterfaceFactory::CompoundAnimationInterfaceFactory { &self.base }
    fn as_CompoundAnimationInterfaceFactory_mut(&mut self) -> &mut crate::org::allbinary::animation::compound::CompoundAnimationInterfaceFactory::CompoundAnimationInterfaceFactory { &mut self.base }
    fn createArray(&mut self, mut size: i32) -> Vec<Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>>> { Self::createArray(self, size) }
    fn createAnimation(&mut self, mut animationArray: Vec<Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>>>) -> Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>> { Self::createAnimation(self, animationArray) }
}


