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
// package org.allbinary.animation.image
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::animation::AnimationBehavior::AnimationBehavior;
use crate::org::allbinary::logic::NullUtil::NullUtil;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use crate::org::allbinary::logic::util::cache::CacheableInterface::CacheableInterface;
use super::AdjustedImageArrayRotationAnimation::AdjustedImageArrayRotationAnimation;
use super::ImageArrayRotationAnimationInfo::ImageArrayRotationAnimationInfo;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct ImageArrayRotationAnimationCacheable {
    pub base: AdjustedImageArrayRotationAnimation,
}

impl CacheableInterface for ImageArrayRotationAnimationCacheable {
    fn getKey(&mut self) -> Option<Rc<RefCell<Box<dyn std::any::Any>>>> { Self::getKey(self) }
}
impl crate::org::allbinary::animation::SequentialAnimationInterface::SequentialAnimationInterface for ImageArrayRotationAnimationCacheable {
    fn getSize(&mut self) -> i32 { self.base.getSize() }
    fn previousFrame(&mut self) { self.base.previousFrame() }
}

impl ImageArrayRotationAnimationCacheable {
    

    pub fn new_struct(mut anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>>, mut animationBehavior: Option<Rc<RefCell<dyn crate::org::allbinary::animation::AnimationBehavior::AnimationBehaviorDyn>>>) -> Self {
        let mut this = Self {
            base: AdjustedImageArrayRotationAnimation::new_struct(jcall!(((anyType).clone() /* as ImageArrayRotationAnimationInfo */)).getImageArray(), jcall!(((anyType).clone() /* as ImageArrayRotationAnimationInfo */)).getAngleInfoP(), jcall!(((anyType).clone() /* as ImageArrayRotationAnimationInfo */)).getTotalAngle(), jcall!(((anyType).clone() /* as ImageArrayRotationAnimationInfo */)).getDx(), jcall!(((anyType).clone() /* as ImageArrayRotationAnimationInfo */)).getDy(), animationBehavior.clone()),
        };
        // super(...) is in the struct literal above
        this
    }

    pub fn new(mut anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>>, mut animationBehavior: Option<Rc<RefCell<dyn crate::org::allbinary::animation::AnimationBehavior::AnimationBehaviorDyn>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(anyType, animationBehavior))))
    }
    

    pub fn getKey(&mut self) -> Option<Rc<RefCell<Box<dyn std::any::Any>>>> {
        return jcall!(NullUtil::getInstance()).NULL_OBJECT.clone();
    }
    

    pub fn toString(&mut self) -> String {
        return jcall!(StringUtil::getInstance()).EMPTY_STRING.clone();
    }
}
impl std::ops::Deref for ImageArrayRotationAnimationCacheable {
    type Target = AdjustedImageArrayRotationAnimation;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for ImageArrayRotationAnimationCacheable {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::animation::image::AdjustedImageArrayRotationAnimation::AdjustedImageArrayRotationAnimationDyn for ImageArrayRotationAnimationCacheable {
    fn as_AdjustedImageArrayRotationAnimation(&self) -> &crate::org::allbinary::animation::image::AdjustedImageArrayRotationAnimation::AdjustedImageArrayRotationAnimation { &self.base }
    fn as_AdjustedImageArrayRotationAnimation_mut(&mut self) -> &mut crate::org::allbinary::animation::image::AdjustedImageArrayRotationAnimation::AdjustedImageArrayRotationAnimation { &mut self.base }
}
impl crate::org::allbinary::animation::image::ImageArrayRotationAnimation::ImageArrayRotationAnimationDyn for ImageArrayRotationAnimationCacheable {
    fn as_ImageArrayRotationAnimation(&self) -> &crate::org::allbinary::animation::image::ImageArrayRotationAnimation::ImageArrayRotationAnimation { &self.base.base }
    fn as_ImageArrayRotationAnimation_mut(&mut self) -> &mut crate::org::allbinary::animation::image::ImageArrayRotationAnimation::ImageArrayRotationAnimation { &mut self.base.base }
}
impl crate::org::allbinary::animation::image::ImageArrayBaseRotationAnimation::ImageArrayBaseRotationAnimationDyn for ImageArrayRotationAnimationCacheable {
    fn as_ImageArrayBaseRotationAnimation(&self) -> &crate::org::allbinary::animation::image::ImageArrayBaseRotationAnimation::ImageArrayBaseRotationAnimation { &self.base.base.base }
    fn as_ImageArrayBaseRotationAnimation_mut(&mut self) -> &mut crate::org::allbinary::animation::image::ImageArrayBaseRotationAnimation::ImageArrayBaseRotationAnimation { &mut self.base.base.base }
}
impl crate::org::allbinary::animation::Animation::AnimationDyn for ImageArrayRotationAnimationCacheable {
    fn as_Animation(&self) -> &crate::org::allbinary::animation::Animation::Animation { &self.base.base.base.base.base.base }
    fn as_Animation_mut(&mut self) -> &mut crate::org::allbinary::animation::Animation::Animation { &mut self.base.base.base.base.base.base }
    fn setAlpha(&mut self, p1: i32) { crate::org::allbinary::animation::image::ImageArrayBaseRotationAnimation::ImageArrayBaseRotationAnimation::setAlpha(crate::org::allbinary::animation::image::ImageArrayBaseRotationAnimation::ImageArrayBaseRotationAnimationDyn::as_ImageArrayBaseRotationAnimation_mut(self), p1) }
    fn setDx(&mut self, p1: i32) { crate::org::allbinary::animation::image::AdjustedImageArrayRotationAnimation::AdjustedImageArrayRotationAnimation::setDx(crate::org::allbinary::animation::image::AdjustedImageArrayRotationAnimation::AdjustedImageArrayRotationAnimationDyn::as_AdjustedImageArrayRotationAnimation_mut(self), p1) }
    fn getDx(&mut self) -> i32 { crate::org::allbinary::animation::image::AdjustedImageArrayRotationAnimation::AdjustedImageArrayRotationAnimation::getDx(crate::org::allbinary::animation::image::AdjustedImageArrayRotationAnimation::AdjustedImageArrayRotationAnimationDyn::as_AdjustedImageArrayRotationAnimation_mut(self)) }
    fn setDy(&mut self, p1: i32) { crate::org::allbinary::animation::image::AdjustedImageArrayRotationAnimation::AdjustedImageArrayRotationAnimation::setDy(crate::org::allbinary::animation::image::AdjustedImageArrayRotationAnimation::AdjustedImageArrayRotationAnimationDyn::as_AdjustedImageArrayRotationAnimation_mut(self), p1) }
    fn getDy(&mut self) -> i32 { crate::org::allbinary::animation::image::AdjustedImageArrayRotationAnimation::AdjustedImageArrayRotationAnimation::getDy(crate::org::allbinary::animation::image::AdjustedImageArrayRotationAnimation::AdjustedImageArrayRotationAnimationDyn::as_AdjustedImageArrayRotationAnimation_mut(self)) }
}
impl crate::org::allbinary::animation::IndexedAnimation::IndexedAnimationDyn for ImageArrayRotationAnimationCacheable {
    fn as_IndexedAnimation(&self) -> &crate::org::allbinary::animation::IndexedAnimation::IndexedAnimation { &self.base.base.base.base.base }
    fn as_IndexedAnimation_mut(&mut self) -> &mut crate::org::allbinary::animation::IndexedAnimation::IndexedAnimation { &mut self.base.base.base.base.base }
    fn setFrame(&mut self, p1: i32) { crate::org::allbinary::animation::image::ImageArrayBaseRotationAnimation::ImageArrayBaseRotationAnimation::setFrame(crate::org::allbinary::animation::image::ImageArrayBaseRotationAnimation::ImageArrayBaseRotationAnimationDyn::as_ImageArrayBaseRotationAnimation_mut(self), p1) }
    fn getFrame(&mut self) -> i32 { crate::org::allbinary::animation::RotationAnimation::RotationAnimation::getFrame(crate::org::allbinary::animation::RotationAnimation::RotationAnimationDyn::as_RotationAnimation_mut(self)) }
    fn getAnimationSize(&mut self) -> i32 { crate::org::allbinary::animation::image::ImageArrayBaseRotationAnimation::ImageArrayBaseRotationAnimation::getAnimationSize(crate::org::allbinary::animation::image::ImageArrayBaseRotationAnimation::ImageArrayBaseRotationAnimationDyn::as_ImageArrayBaseRotationAnimation_mut(self)) }
    fn getSize(&mut self) -> i32 { crate::org::allbinary::animation::RotationAnimation::RotationAnimation::getSize(crate::org::allbinary::animation::RotationAnimation::RotationAnimationDyn::as_RotationAnimation_mut(self)) }
    fn setSequence(&mut self, p1: Vec<i32>) { crate::org::allbinary::animation::image::ImageArrayBaseRotationAnimation::ImageArrayBaseRotationAnimation::setSequence(crate::org::allbinary::animation::image::ImageArrayBaseRotationAnimation::ImageArrayBaseRotationAnimationDyn::as_ImageArrayBaseRotationAnimation_mut(self), p1) }
    fn getSequence(&mut self) -> Vec<i32> { crate::org::allbinary::animation::image::ImageArrayBaseRotationAnimation::ImageArrayBaseRotationAnimation::getSequence(crate::org::allbinary::animation::image::ImageArrayBaseRotationAnimation::ImageArrayBaseRotationAnimationDyn::as_ImageArrayBaseRotationAnimation_mut(self)) }
}
impl crate::org::allbinary::animation::RotationAnimation::RotationAnimationDyn for ImageArrayRotationAnimationCacheable {
    fn as_RotationAnimation(&self) -> &crate::org::allbinary::animation::RotationAnimation::RotationAnimation { &self.base.base.base.base }
    fn as_RotationAnimation_mut(&mut self) -> &mut crate::org::allbinary::animation::RotationAnimation::RotationAnimation { &mut self.base.base.base.base }
    fn nextRotation(&mut self) { crate::org::allbinary::animation::image::ImageArrayBaseRotationAnimation::ImageArrayBaseRotationAnimation::nextRotation(crate::org::allbinary::animation::image::ImageArrayBaseRotationAnimation::ImageArrayBaseRotationAnimationDyn::as_ImageArrayBaseRotationAnimation_mut(self)) }
    fn previousRotation(&mut self) { crate::org::allbinary::animation::image::ImageArrayBaseRotationAnimation::ImageArrayBaseRotationAnimation::previousRotation(crate::org::allbinary::animation::image::ImageArrayBaseRotationAnimation::ImageArrayBaseRotationAnimationDyn::as_ImageArrayBaseRotationAnimation_mut(self)) }
}
impl crate::org::allbinary::image::opengles::OpenGLSurfaceChangedInterface::OpenGLSurfaceChangedInterface for ImageArrayRotationAnimationCacheable {
    fn set(&mut self, p1: Option<Rc<RefCell<dyn crate::javax::microedition::khronos::opengles::GL::GL>>>) { crate::org::allbinary::animation::Animation::Animation::set(crate::org::allbinary::animation::Animation::AnimationDyn::as_Animation_mut(self), p1) }
}


