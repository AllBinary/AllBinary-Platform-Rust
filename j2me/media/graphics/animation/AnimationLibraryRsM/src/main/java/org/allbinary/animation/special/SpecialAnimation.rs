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
// package org.allbinary.animation.special
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::animation::AnimationBehavior::AnimationBehavior;
use crate::org::allbinary::animation::IndexedAnimation::IndexedAnimation;
use crate::org::allbinary::animation::IndexedAnimationBehavior::IndexedAnimationBehavior;
use super::SpecialAnimationInterface::SpecialAnimationInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct SpecialAnimation {
    pub base: IndexedAnimation,
}

thread_local! { pub static SpecialAnimation_NULL_SPECIAL_ANIMATION: Option<Rc<RefCell<dyn SpecialAnimationDyn>>> = SpecialAnimation::new(IndexedAnimationBehavior::new(1, 250)).map(|reference| reference as Rc<RefCell<dyn SpecialAnimationDyn>>); }
// impl SpecialAnimationInterface for SpecialAnimation with the methods of the trait
impl crate::org::allbinary::animation::SequentialAnimationInterface::SequentialAnimationInterface for SpecialAnimation {
    fn getSize(&mut self) -> i32 { self.base.getSize() }
    fn previousFrame(&mut self) { self.base.previousFrame() }
}

impl SpecialAnimation {
    

    pub fn getInstance() -> Option<Rc<RefCell<dyn SpecialAnimationDyn>>> {
        return SpecialAnimation_NULL_SPECIAL_ANIMATION.with(Clone::clone).clone();
    }
    

    pub fn new_struct(mut animationBehavior: Option<Rc<RefCell<dyn crate::org::allbinary::animation::AnimationBehavior::AnimationBehaviorDyn>>>) -> Self {
        let mut this = Self {
            base: IndexedAnimation::new_struct(animationBehavior.clone()),
        };
        // super(...) is in the struct literal above
        this
    }

    pub fn new(mut animationBehavior: Option<Rc<RefCell<dyn crate::org::allbinary::animation::AnimationBehavior::AnimationBehaviorDyn>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(animationBehavior))))
    }
    

    pub fn nextFrame(&mut self) {
        let mut indexedAnimationBehavior: Option<Rc<RefCell<IndexedAnimationBehavior>>> = ((self.animationBehaviorP).clone() /* as IndexedAnimationBehavior */);
        jcall!(indexedAnimationBehavior).loopIndex += 1;
    }
    

    pub fn setLastFrame(&mut self) {
    }
    

    pub fn isComplete(&mut self) -> bool {
        return false;
    }
    

    pub fn open(&mut self) {
    }
    

    pub fn close(&mut self) {
    }
    

    pub fn process(&mut self) {
    }
}
impl std::ops::Deref for SpecialAnimation {
    type Target = IndexedAnimation;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for SpecialAnimation {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

pub trait SpecialAnimationDyn: crate::org::allbinary::animation::IndexedAnimation::IndexedAnimationDyn {
    fn as_SpecialAnimation(&self) -> &SpecialAnimation;
    fn as_SpecialAnimation_mut(&mut self) -> &mut SpecialAnimation;
    fn setLastFrame(&mut self) { SpecialAnimation::setLastFrame(self.as_SpecialAnimation_mut()) }
    fn isComplete(&mut self) -> bool { SpecialAnimation::isComplete(self.as_SpecialAnimation_mut()) }
    fn open(&mut self) { SpecialAnimation::open(self.as_SpecialAnimation_mut()) }
    fn close(&mut self) { SpecialAnimation::close(self.as_SpecialAnimation_mut()) }
    fn process(&mut self) { SpecialAnimation::process(self.as_SpecialAnimation_mut()) }
}
impl SpecialAnimationDyn for SpecialAnimation {
    fn as_SpecialAnimation(&self) -> &SpecialAnimation { self }
    fn as_SpecialAnimation_mut(&mut self) -> &mut SpecialAnimation { self }
}

impl crate::org::allbinary::animation::Animation::AnimationDyn for SpecialAnimation {
    fn as_Animation(&self) -> &crate::org::allbinary::animation::Animation::Animation { &self.base.base }
    fn as_Animation_mut(&mut self) -> &mut crate::org::allbinary::animation::Animation::Animation { &mut self.base.base }
    fn nextFrame(&mut self) { Self::nextFrame(self) }
}
impl crate::org::allbinary::animation::IndexedAnimation::IndexedAnimationDyn for SpecialAnimation {
    fn as_IndexedAnimation(&self) -> &crate::org::allbinary::animation::IndexedAnimation::IndexedAnimation { &self.base }
    fn as_IndexedAnimation_mut(&mut self) -> &mut crate::org::allbinary::animation::IndexedAnimation::IndexedAnimation { &mut self.base }
}
impl crate::org::allbinary::image::opengles::OpenGLSurfaceChangedInterface::OpenGLSurfaceChangedInterface for SpecialAnimation {
    fn set(&mut self, p1: Option<Rc<RefCell<dyn crate::javax::microedition::khronos::opengles::GL::GL>>>) { crate::org::allbinary::animation::Animation::Animation::set(crate::org::allbinary::animation::Animation::AnimationDyn::as_Animation_mut(self), p1) }
}


