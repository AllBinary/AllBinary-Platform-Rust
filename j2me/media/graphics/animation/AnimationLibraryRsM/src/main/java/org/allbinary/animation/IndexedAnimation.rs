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
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::math::PrimitiveIntUtil::PrimitiveIntUtil;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use super::Animation::Animation;
use super::IndexedAnimationInterface::IndexedAnimationInterface;
use super::AnimationBehavior::AnimationBehavior;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct IndexedAnimation {
    pub base: Animation,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub commonStrings: Option<Rc<RefCell<CommonStrings>>>,
    pub animationBehaviorP: Option<Rc<RefCell<dyn crate::org::allbinary::animation::AnimationBehavior::AnimationBehaviorDyn>>>,
}

// impl IndexedAnimationInterface for IndexedAnimation with the methods of the trait

impl IndexedAnimation {
    

    pub fn new_struct(mut animationBehavior: Option<Rc<RefCell<dyn crate::org::allbinary::animation::AnimationBehavior::AnimationBehaviorDyn>>>) -> Self {
        let mut this = Self {
            base: Animation::new_struct(),
            logUtil: None,
            commonStrings: None,
            animationBehaviorP: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.commonStrings = CommonStrings::getInstance();
        this.animationBehaviorP = animationBehavior.clone();
        this
    }

    pub fn new(mut animationBehavior: Option<Rc<RefCell<dyn crate::org::allbinary::animation::AnimationBehavior::AnimationBehaviorDyn>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(animationBehavior))))
    }
    

    pub fn getAnimationBehavior(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::animation::AnimationBehavior::AnimationBehaviorDyn>>> {
        return self.animationBehaviorP.clone();
    }
    

    pub fn reset(&mut self) {
        jcall!(self.animationBehaviorP).reset();
        self.setFrame(0);
    }
    

    pub fn setFrame(&mut self, mut index: i32) {
    }
    

    pub fn getFrame(&mut self) -> i32 {
        return 0;
    }
    

    // throws Exception
    pub fn getAnimationSize(&mut self) -> i32 {
        panic!("{:?}", format!("{}: {:?}", "Exception", (jcall!(self.commonStrings).NOT_IMPLEMENTED.clone())));
    }
    

    pub fn getSize(&mut self) -> i32 {
        return 0;
    }
    

    pub fn previousFrame(&mut self) {
    }
    

    pub fn isLastFrame(&mut self) -> bool {
        if self.getFrame() == self.getSize() - 1 {
            return true;
        } else {
            return false;
        }
    }
    

    pub fn setSequence(&mut self, mut sequence: Vec<i32>) {
    }
    

    pub fn getSequence(&mut self) -> Vec<i32> {
        return PrimitiveIntUtil::getArrayInstance();
    }
    

    pub fn setState(&mut self, mut indexedAnimation: Option<Rc<RefCell<dyn IndexedAnimationDyn>>>) {
        { let java_arguments = (jcall!(indexedAnimation).getFrame(),); self.setFrame(java_arguments.0) };
        { let java_arguments = (jcall!(indexedAnimation).getDx(),); self.setDx(java_arguments.0) };
        { let java_arguments = (jcall!(indexedAnimation).getDy(),); self.setDy(java_arguments.0) };
    }
    

    pub fn getWidth(&mut self) -> i32 {
        return 0;
    }
    

    pub fn getHeight(&mut self) -> i32 {
        return 0;
    }
}
impl std::ops::Deref for IndexedAnimation {
    type Target = Animation;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for IndexedAnimation {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

pub trait IndexedAnimationDyn: crate::org::allbinary::animation::Animation::AnimationDyn {
    fn as_IndexedAnimation(&self) -> &IndexedAnimation;
    fn as_IndexedAnimation_mut(&mut self) -> &mut IndexedAnimation;
    fn getAnimationBehavior(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::animation::AnimationBehavior::AnimationBehaviorDyn>>> { IndexedAnimation::getAnimationBehavior(self.as_IndexedAnimation_mut()) }
    fn reset(&mut self) { IndexedAnimation::reset(self.as_IndexedAnimation_mut()) }
    fn setFrame(&mut self, mut index: i32) { IndexedAnimation::setFrame(self.as_IndexedAnimation_mut(), index) }
    fn getFrame(&mut self) -> i32 { IndexedAnimation::getFrame(self.as_IndexedAnimation_mut()) }
    fn getAnimationSize(&mut self) -> i32 { IndexedAnimation::getAnimationSize(self.as_IndexedAnimation_mut()) }
    fn getSize(&mut self) -> i32 { IndexedAnimation::getSize(self.as_IndexedAnimation_mut()) }
    fn previousFrame(&mut self) { IndexedAnimation::previousFrame(self.as_IndexedAnimation_mut()) }
    fn isLastFrame(&mut self) -> bool { IndexedAnimation::isLastFrame(self.as_IndexedAnimation_mut()) }
    fn setSequence(&mut self, mut sequence: Vec<i32>) { IndexedAnimation::setSequence(self.as_IndexedAnimation_mut(), sequence) }
    fn getSequence(&mut self) -> Vec<i32> { IndexedAnimation::getSequence(self.as_IndexedAnimation_mut()) }
    fn setState(&mut self, mut indexedAnimation: Option<Rc<RefCell<dyn IndexedAnimationDyn>>>) { IndexedAnimation::setState(self.as_IndexedAnimation_mut(), indexedAnimation) }
    fn getWidth(&mut self) -> i32 { IndexedAnimation::getWidth(self.as_IndexedAnimation_mut()) }
    fn getHeight(&mut self) -> i32 { IndexedAnimation::getHeight(self.as_IndexedAnimation_mut()) }
}
impl IndexedAnimationDyn for IndexedAnimation {
    fn as_IndexedAnimation(&self) -> &IndexedAnimation { self }
    fn as_IndexedAnimation_mut(&mut self) -> &mut IndexedAnimation { self }
}

impl crate::org::allbinary::animation::Animation::AnimationDyn for IndexedAnimation {
    fn as_Animation(&self) -> &crate::org::allbinary::animation::Animation::Animation { &self.base }
    fn as_Animation_mut(&mut self) -> &mut crate::org::allbinary::animation::Animation::Animation { &mut self.base }
}
impl crate::org::allbinary::image::opengles::OpenGLSurfaceChangedInterface::OpenGLSurfaceChangedInterface for IndexedAnimation {
    fn set(&mut self, p1: Option<Rc<RefCell<dyn crate::javax::microedition::khronos::opengles::GL::GL>>>) { crate::org::allbinary::animation::Animation::Animation::set(crate::org::allbinary::animation::Animation::AnimationDyn::as_Animation_mut(self), p1) }
}


