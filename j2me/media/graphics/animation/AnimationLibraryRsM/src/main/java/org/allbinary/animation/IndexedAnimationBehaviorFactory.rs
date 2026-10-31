// 
// AllBinary Open License Version 1 
// Copyright (c) 2022 AllBinary 
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
use super::AnimationBehaviorFactory::AnimationBehaviorFactory;
use super::IndexedAnimationBehavior::IndexedAnimationBehavior;
use super::AnimationBehavior::AnimationBehavior;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct IndexedAnimationBehaviorFactory {
    pub base: AnimationBehaviorFactory,
    pub frameDelayTime: i64,
    pub loopTotal: i32,
}


impl IndexedAnimationBehaviorFactory {
    

    pub fn new_struct(mut loopTotal: i32, mut frameDelayTime: i64) -> Self {
        let mut this = Self {
            base: AnimationBehaviorFactory::new_struct(),
            frameDelayTime: 0_i64,
            loopTotal: 0,
        };
        this.loopTotal = loopTotal.clone();
        this.frameDelayTime = frameDelayTime.clone();
        this
    }

    pub fn new(mut loopTotal: i32, mut frameDelayTime: i64) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(loopTotal, frameDelayTime))))
    }
    

    pub fn getOrCreateInstance(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::animation::AnimationBehavior::AnimationBehaviorDyn>>> {
        return IndexedAnimationBehavior::new(self.loopTotal.clone(), self.frameDelayTime.clone()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::animation::AnimationBehavior::AnimationBehaviorDyn>>);
    }
}
impl std::ops::Deref for IndexedAnimationBehaviorFactory {
    type Target = AnimationBehaviorFactory;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for IndexedAnimationBehaviorFactory {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::animation::AnimationBehaviorFactory::AnimationBehaviorFactoryDyn for IndexedAnimationBehaviorFactory {
    fn as_AnimationBehaviorFactory(&self) -> &crate::org::allbinary::animation::AnimationBehaviorFactory::AnimationBehaviorFactory { &self.base }
    fn as_AnimationBehaviorFactory_mut(&mut self) -> &mut crate::org::allbinary::animation::AnimationBehaviorFactory::AnimationBehaviorFactory { &mut self.base }
    fn getOrCreateInstance(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::animation::AnimationBehavior::AnimationBehaviorDyn>>> { Self::getOrCreateInstance(self) }
}


