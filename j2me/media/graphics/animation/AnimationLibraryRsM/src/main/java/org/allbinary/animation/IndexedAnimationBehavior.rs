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
use super::AnimationBehavior::AnimationBehavior;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct IndexedAnimationBehavior {
    pub base: AnimationBehavior,
    pub frameDelayTime: i64,
    pub loopTotal: i32,
    pub elapsedTime: i64,
    pub loopIndex: i32,
}


impl IndexedAnimationBehavior {
    

    pub fn new_struct(mut loopTotal: i32, mut frameDelayTime: i64) -> Self {
        let mut this = Self {
            base: AnimationBehavior::new_struct(),
            frameDelayTime: 0_i64,
            loopTotal: 0,
            elapsedTime: 0_i64,
            loopIndex: 0,
        };
        this.loopTotal = loopTotal.clone();
        this.frameDelayTime = frameDelayTime.clone();
        this
    }

    pub fn new(mut loopTotal: i32, mut frameDelayTime: i64) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(loopTotal, frameDelayTime))))
    }
    

    pub fn reset(&mut self) {
        self.loopIndex = 0;
    }
}
impl std::ops::Deref for IndexedAnimationBehavior {
    type Target = AnimationBehavior;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for IndexedAnimationBehavior {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::animation::AnimationBehavior::AnimationBehaviorDyn for IndexedAnimationBehavior {
    fn as_AnimationBehavior(&self) -> &crate::org::allbinary::animation::AnimationBehavior::AnimationBehavior { &self.base }
    fn as_AnimationBehavior_mut(&mut self) -> &mut crate::org::allbinary::animation::AnimationBehavior::AnimationBehavior { &mut self.base }
    fn reset(&mut self) { Self::reset(self) }
}


