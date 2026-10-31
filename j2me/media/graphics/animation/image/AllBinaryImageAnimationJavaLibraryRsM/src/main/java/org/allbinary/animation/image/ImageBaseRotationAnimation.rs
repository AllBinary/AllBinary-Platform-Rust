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
// use crate::javax::microedition::lcdui::Graphics::Graphics;
// use crate::javax::microedition::lcdui::Image::Image;
use crate::org::allbinary::DisposalUtil::DisposalUtil;
use crate::org::allbinary::animation::AnimationBehavior::AnimationBehavior;
use crate::org::allbinary::animation::RotationAnimation::RotationAnimation;
use crate::org::allbinary::graphics::Anchor::Anchor;
use crate::org::allbinary::logic::math::PrimitiveIntUtil::PrimitiveIntUtil;
use crate::org::allbinary::math::AngleInfo::AngleInfo;
use crate::org::allbinary::util::CircularIndexUtil::CircularIndexUtil;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct ImageBaseRotationAnimation {
    pub base: RotationAnimation,
    pub image: Option<Rc<RefCell<Image>>>,
    pub anchor: i32,
}

impl crate::org::allbinary::animation::SequentialAnimationInterface::SequentialAnimationInterface for ImageBaseRotationAnimation {
    fn getSize(&mut self) -> i32 { self.base.getSize() }
    fn previousFrame(&mut self) { self.base.previousFrame() }
}
impl crate::org::allbinary::graphics::paint::LocationPaintableInterface::LocationPaintableInterface for ImageBaseRotationAnimation {
    fn paintXY(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>, mut x: i32, mut y: i32) { Self::paintXY(self, graphics, x, y) }
}

impl ImageBaseRotationAnimation {
    

    pub fn new_struct(mut image: Option<Rc<RefCell<Image>>>, mut angleInfo: Option<Rc<RefCell<AngleInfo>>>, mut totalAngle: i16, mut animationBehavior: Option<Rc<RefCell<dyn crate::org::allbinary::animation::AnimationBehavior::AnimationBehaviorDyn>>>) -> Self {
        let mut this = Self {
            base: RotationAnimation::new_struct(angleInfo.clone(), CircularIndexUtil::createInstance(totalAngle / jcall!(jcall!(angleInfo).getAngleIncrementInfo()).getAngleIncrement()), animationBehavior.clone()),
            image: None,
            anchor: 0,
        };
        this.anchor = unsafe { crate::org::allbinary::graphics::Anchor::Anchor_TOP_LEFT.clone() }.clone();
        // super(...) is in the struct literal above
        this.image = image.clone();
        this
    }

    pub fn new(mut image: Option<Rc<RefCell<Image>>>, mut angleInfo: Option<Rc<RefCell<AngleInfo>>>, mut totalAngle: i16, mut animationBehavior: Option<Rc<RefCell<dyn crate::org::allbinary::animation::AnimationBehavior::AnimationBehaviorDyn>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(image, angleInfo, totalAngle, animationBehavior))))
    }
    

    // throws Exception
    pub fn getAnimationSize(&mut self) -> i32 {
        return self.getSize();
    }
    

    pub fn setSequence(&mut self, mut sequence: Vec<i32>) {
    }
    

    pub fn getSequence(&mut self) -> Vec<i32> {
        return PrimitiveIntUtil::getArrayInstance();
    }
    

    pub fn paintXY(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>, mut x: i32, mut y: i32) {
        jcall!(graphics).drawImage(self.image.clone(), x.clone(), y.clone(), self.anchor.clone());
    }
    

    pub fn getImage(&mut self) -> Option<Rc<RefCell<Image>>> {
        return self.image.clone();
    }
    

    pub fn getWidth(&mut self) -> i32 {
        return jcall!(self.image).getWidth();
    }
    

    pub fn getHeight(&mut self) -> i32 {
        return jcall!(self.image).getHeight();
    }
    

    // throws Exception
    pub fn close(&mut self) {
        jcall!(DisposalUtil::getInstance()).disposeImage(self.image.clone());
    }
    

    // throws Throwable
    pub fn finalize(&mut self) {
        jcall!(DisposalUtil::getInstance()).disposeImage(self.image.clone());
    }
}
impl std::ops::Deref for ImageBaseRotationAnimation {
    type Target = RotationAnimation;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for ImageBaseRotationAnimation {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

pub trait ImageBaseRotationAnimationDyn: crate::org::allbinary::animation::RotationAnimation::RotationAnimationDyn {
    fn as_ImageBaseRotationAnimation(&self) -> &ImageBaseRotationAnimation;
    fn as_ImageBaseRotationAnimation_mut(&mut self) -> &mut ImageBaseRotationAnimation;
    fn getImage(&mut self) -> Option<Rc<RefCell<Image>>> { ImageBaseRotationAnimation::getImage(self.as_ImageBaseRotationAnimation_mut()) }
    fn close(&mut self) { ImageBaseRotationAnimation::close(self.as_ImageBaseRotationAnimation_mut()) }
    fn finalize(&mut self) { ImageBaseRotationAnimation::finalize(self.as_ImageBaseRotationAnimation_mut()) }
}
impl ImageBaseRotationAnimationDyn for ImageBaseRotationAnimation {
    fn as_ImageBaseRotationAnimation(&self) -> &ImageBaseRotationAnimation { self }
    fn as_ImageBaseRotationAnimation_mut(&mut self) -> &mut ImageBaseRotationAnimation { self }
}

impl crate::org::allbinary::animation::Animation::AnimationDyn for ImageBaseRotationAnimation {
    fn as_Animation(&self) -> &crate::org::allbinary::animation::Animation::Animation { &self.base.base.base }
    fn as_Animation_mut(&mut self) -> &mut crate::org::allbinary::animation::Animation::Animation { &mut self.base.base.base }
    fn paintXY(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>, mut x: i32, mut y: i32) { Self::paintXY(self, graphics, x, y) }
}
impl crate::org::allbinary::animation::IndexedAnimation::IndexedAnimationDyn for ImageBaseRotationAnimation {
    fn as_IndexedAnimation(&self) -> &crate::org::allbinary::animation::IndexedAnimation::IndexedAnimation { &self.base.base }
    fn as_IndexedAnimation_mut(&mut self) -> &mut crate::org::allbinary::animation::IndexedAnimation::IndexedAnimation { &mut self.base.base }
    fn setFrame(&mut self, p1: i32) { crate::org::allbinary::animation::RotationAnimation::RotationAnimation::setFrame(crate::org::allbinary::animation::RotationAnimation::RotationAnimationDyn::as_RotationAnimation_mut(self), p1) }
    fn getFrame(&mut self) -> i32 { crate::org::allbinary::animation::RotationAnimation::RotationAnimation::getFrame(crate::org::allbinary::animation::RotationAnimation::RotationAnimationDyn::as_RotationAnimation_mut(self)) }
    fn getAnimationSize(&mut self) -> i32 { Self::getAnimationSize(self) }
    fn getSize(&mut self) -> i32 { crate::org::allbinary::animation::RotationAnimation::RotationAnimation::getSize(crate::org::allbinary::animation::RotationAnimation::RotationAnimationDyn::as_RotationAnimation_mut(self)) }
    fn setSequence(&mut self, mut sequence: Vec<i32>) { Self::setSequence(self, sequence) }
    fn getSequence(&mut self) -> Vec<i32> { Self::getSequence(self) }
    fn getWidth(&mut self) -> i32 { Self::getWidth(self) }
    fn getHeight(&mut self) -> i32 { Self::getHeight(self) }
}
impl crate::org::allbinary::animation::RotationAnimation::RotationAnimationDyn for ImageBaseRotationAnimation {
    fn as_RotationAnimation(&self) -> &crate::org::allbinary::animation::RotationAnimation::RotationAnimation { &self.base }
    fn as_RotationAnimation_mut(&mut self) -> &mut crate::org::allbinary::animation::RotationAnimation::RotationAnimation { &mut self.base }
}
impl crate::org::allbinary::image::opengles::OpenGLSurfaceChangedInterface::OpenGLSurfaceChangedInterface for ImageBaseRotationAnimation {
    fn set(&mut self, p1: Option<Rc<RefCell<dyn crate::javax::microedition::khronos::opengles::GL::GL>>>) { crate::org::allbinary::animation::Animation::Animation::set(crate::org::allbinary::animation::Animation::AnimationDyn::as_Animation_mut(self), p1) }
}


