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
// use crate::javax::microedition::lcdui::Image::Image;
use crate::org::allbinary::animation::AnimationBehavior::AnimationBehavior;
use crate::org::allbinary::math::AngleInfo::AngleInfo;
use super::ImageArrayBaseRotationAnimation::ImageArrayBaseRotationAnimation;
use super::ImageArrayRotationAnimationInfo::ImageArrayRotationAnimationInfo;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct ImageArrayRotationAnimation {
    pub base: ImageArrayBaseRotationAnimation,
    pub expectedTotalFrames: i32,
}

impl crate::org::allbinary::animation::SequentialAnimationInterface::SequentialAnimationInterface for ImageArrayRotationAnimation {
    fn getSize(&mut self) -> i32 { self.base.getSize() }
    fn previousFrame(&mut self) { self.base.previousFrame() }
}

impl ImageArrayRotationAnimation {
    

    // throws Exception
    pub fn createAnimation(mut anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>>, mut animationBehavior: Option<Rc<RefCell<dyn crate::org::allbinary::animation::AnimationBehavior::AnimationBehaviorDyn>>>) -> Option<Rc<RefCell<dyn ImageArrayRotationAnimationDyn>>> {
        let mut imageRotationAnimationInfo: Option<Rc<RefCell<ImageArrayRotationAnimationInfo>>> = (anyType).clone() /* as ImageArrayRotationAnimationInfo */;
        return ImageArrayRotationAnimation::new(jcall!(imageRotationAnimationInfo).getImageArray(), jcall!(imageRotationAnimationInfo).getAngleInfoP(), jcall!(imageRotationAnimationInfo).getTotalAngle(), animationBehavior.clone()).map(|reference| reference as Rc<RefCell<dyn ImageArrayRotationAnimationDyn>>);
    }
    

    pub fn new_struct(mut imageArray: Vec<Option<Rc<RefCell<Image>>>>, mut angleInfo: Option<Rc<RefCell<AngleInfo>>>, mut totalAngle: i32, mut animationBehavior: Option<Rc<RefCell<dyn crate::org::allbinary::animation::AnimationBehavior::AnimationBehaviorDyn>>>) -> Self {
        let mut this = Self {
            base: ImageArrayBaseRotationAnimation::new_struct(imageArray.clone(), angleInfo.clone(), animationBehavior.clone()),
            expectedTotalFrames: 0,
        };
        // super(...) is in the struct literal above
        this.init(imageArray.clone(), angleInfo.clone(), totalAngle.clone());
        this
    }

    pub fn new(mut imageArray: Vec<Option<Rc<RefCell<Image>>>>, mut angleInfo: Option<Rc<RefCell<AngleInfo>>>, mut totalAngle: i32, mut animationBehavior: Option<Rc<RefCell<dyn crate::org::allbinary::animation::AnimationBehavior::AnimationBehaviorDyn>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(imageArray, angleInfo, totalAngle, animationBehavior))))
    }
    

    // throws Exception
    pub fn init(&mut self, mut imageArray: Vec<Option<Rc<RefCell<Image>>>>, mut angleInfo: Option<Rc<RefCell<AngleInfo>>>, mut totalAngle: i32) {
        self.expectedTotalFrames = totalAngle / jcall!(jcall!(self.angleInfo).getAngleIncrementInfo()).getAngleIncrement();
        jcall!(self.angleInfo).adjustAngle(0);
        if self.expectedTotalFrames != self.getSize() {
            panic!("{:?}", format!("{}: {:?}", "Exception", (String::from("Wrong Number of Frames"))));
        }
    }
    

    pub fn setImageArray(&mut self, mut imageArray: Vec<Option<Rc<RefCell<Image>>>>) {
        self.base.setImageArray(imageArray.clone());
        jcall!(self.angleInfo).adjustAngle(0);
    }
}
impl std::ops::Deref for ImageArrayRotationAnimation {
    type Target = ImageArrayBaseRotationAnimation;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for ImageArrayRotationAnimation {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

pub trait ImageArrayRotationAnimationDyn: crate::org::allbinary::animation::image::ImageArrayBaseRotationAnimation::ImageArrayBaseRotationAnimationDyn {
    fn as_ImageArrayRotationAnimation(&self) -> &ImageArrayRotationAnimation;
    fn as_ImageArrayRotationAnimation_mut(&mut self) -> &mut ImageArrayRotationAnimation;
    fn init(&mut self, mut imageArray: Vec<Option<Rc<RefCell<Image>>>>, mut angleInfo: Option<Rc<RefCell<AngleInfo>>>, mut totalAngle: i32) { ImageArrayRotationAnimation::init(self.as_ImageArrayRotationAnimation_mut(), imageArray, angleInfo, totalAngle) }
}
impl ImageArrayRotationAnimationDyn for ImageArrayRotationAnimation {
    fn as_ImageArrayRotationAnimation(&self) -> &ImageArrayRotationAnimation { self }
    fn as_ImageArrayRotationAnimation_mut(&mut self) -> &mut ImageArrayRotationAnimation { self }
}

impl crate::org::allbinary::animation::image::ImageArrayBaseRotationAnimation::ImageArrayBaseRotationAnimationDyn for ImageArrayRotationAnimation {
    fn as_ImageArrayBaseRotationAnimation(&self) -> &crate::org::allbinary::animation::image::ImageArrayBaseRotationAnimation::ImageArrayBaseRotationAnimation { &self.base }
    fn as_ImageArrayBaseRotationAnimation_mut(&mut self) -> &mut crate::org::allbinary::animation::image::ImageArrayBaseRotationAnimation::ImageArrayBaseRotationAnimation { &mut self.base }
    fn setImageArray(&mut self, mut imageArray: Vec<Option<Rc<RefCell<Image>>>>) { Self::setImageArray(self, imageArray) }
}
impl crate::org::allbinary::animation::Animation::AnimationDyn for ImageArrayRotationAnimation {
    fn as_Animation(&self) -> &crate::org::allbinary::animation::Animation::Animation { &self.base.base.base.base }
    fn as_Animation_mut(&mut self) -> &mut crate::org::allbinary::animation::Animation::Animation { &mut self.base.base.base.base }
    fn setAlpha(&mut self, p1: i32) { crate::org::allbinary::animation::image::ImageArrayBaseRotationAnimation::ImageArrayBaseRotationAnimation::setAlpha(crate::org::allbinary::animation::image::ImageArrayBaseRotationAnimation::ImageArrayBaseRotationAnimationDyn::as_ImageArrayBaseRotationAnimation_mut(self), p1) }
}
impl crate::org::allbinary::animation::IndexedAnimation::IndexedAnimationDyn for ImageArrayRotationAnimation {
    fn as_IndexedAnimation(&self) -> &crate::org::allbinary::animation::IndexedAnimation::IndexedAnimation { &self.base.base.base }
    fn as_IndexedAnimation_mut(&mut self) -> &mut crate::org::allbinary::animation::IndexedAnimation::IndexedAnimation { &mut self.base.base.base }
    fn setFrame(&mut self, p1: i32) { crate::org::allbinary::animation::image::ImageArrayBaseRotationAnimation::ImageArrayBaseRotationAnimation::setFrame(crate::org::allbinary::animation::image::ImageArrayBaseRotationAnimation::ImageArrayBaseRotationAnimationDyn::as_ImageArrayBaseRotationAnimation_mut(self), p1) }
    fn getFrame(&mut self) -> i32 { crate::org::allbinary::animation::RotationAnimation::RotationAnimation::getFrame(crate::org::allbinary::animation::RotationAnimation::RotationAnimationDyn::as_RotationAnimation_mut(self)) }
    fn getAnimationSize(&mut self) -> i32 { crate::org::allbinary::animation::image::ImageArrayBaseRotationAnimation::ImageArrayBaseRotationAnimation::getAnimationSize(crate::org::allbinary::animation::image::ImageArrayBaseRotationAnimation::ImageArrayBaseRotationAnimationDyn::as_ImageArrayBaseRotationAnimation_mut(self)) }
    fn getSize(&mut self) -> i32 { crate::org::allbinary::animation::RotationAnimation::RotationAnimation::getSize(crate::org::allbinary::animation::RotationAnimation::RotationAnimationDyn::as_RotationAnimation_mut(self)) }
    fn setSequence(&mut self, p1: Vec<i32>) { crate::org::allbinary::animation::image::ImageArrayBaseRotationAnimation::ImageArrayBaseRotationAnimation::setSequence(crate::org::allbinary::animation::image::ImageArrayBaseRotationAnimation::ImageArrayBaseRotationAnimationDyn::as_ImageArrayBaseRotationAnimation_mut(self), p1) }
    fn getSequence(&mut self) -> Vec<i32> { crate::org::allbinary::animation::image::ImageArrayBaseRotationAnimation::ImageArrayBaseRotationAnimation::getSequence(crate::org::allbinary::animation::image::ImageArrayBaseRotationAnimation::ImageArrayBaseRotationAnimationDyn::as_ImageArrayBaseRotationAnimation_mut(self)) }
}
impl crate::org::allbinary::animation::RotationAnimation::RotationAnimationDyn for ImageArrayRotationAnimation {
    fn as_RotationAnimation(&self) -> &crate::org::allbinary::animation::RotationAnimation::RotationAnimation { &self.base.base }
    fn as_RotationAnimation_mut(&mut self) -> &mut crate::org::allbinary::animation::RotationAnimation::RotationAnimation { &mut self.base.base }
    fn nextRotation(&mut self) { crate::org::allbinary::animation::image::ImageArrayBaseRotationAnimation::ImageArrayBaseRotationAnimation::nextRotation(crate::org::allbinary::animation::image::ImageArrayBaseRotationAnimation::ImageArrayBaseRotationAnimationDyn::as_ImageArrayBaseRotationAnimation_mut(self)) }
    fn previousRotation(&mut self) { crate::org::allbinary::animation::image::ImageArrayBaseRotationAnimation::ImageArrayBaseRotationAnimation::previousRotation(crate::org::allbinary::animation::image::ImageArrayBaseRotationAnimation::ImageArrayBaseRotationAnimationDyn::as_ImageArrayBaseRotationAnimation_mut(self)) }
}
impl crate::org::allbinary::image::opengles::OpenGLSurfaceChangedInterface::OpenGLSurfaceChangedInterface for ImageArrayRotationAnimation {
    fn set(&mut self, p1: Option<Rc<RefCell<dyn crate::javax::microedition::khronos::opengles::GL::GL>>>) { crate::org::allbinary::animation::Animation::Animation::set(crate::org::allbinary::animation::Animation::AnimationDyn::as_Animation_mut(self), p1) }
}


