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
use crate::org::allbinary::animation::AnimationBehavior::AnimationBehavior;
use crate::org::allbinary::math::AngleInfo::AngleInfo;
use super::ImageArrayRotationAnimation::ImageArrayRotationAnimation;
use super::ImageArrayRotationAnimationInfo::ImageArrayRotationAnimationInfo;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

//TWB - Adjustments should be done in the resource creation and not at the animation level
pub struct AdjustedImageArrayRotationAnimation {
    pub base: ImageArrayRotationAnimation,
    pub dx: i32,
    pub dy: i32,
}

impl crate::org::allbinary::animation::SequentialAnimationInterface::SequentialAnimationInterface for AdjustedImageArrayRotationAnimation {
    fn getSize(&mut self) -> i32 { self.base.getSize() }
    fn previousFrame(&mut self) { self.base.previousFrame() }
}
impl crate::org::allbinary::graphics::paint::LocationPaintableInterface::LocationPaintableInterface for AdjustedImageArrayRotationAnimation {
    fn paintXY(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>, mut x: i32, mut y: i32) { Self::paintXY(self, graphics, x, y) }
}

impl AdjustedImageArrayRotationAnimation {
    

    // throws Exception
    pub fn createAdjustedImageArrayRotationAnimationInfo(mut anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>>, mut animationBehavior: Option<Rc<RefCell<dyn crate::org::allbinary::animation::AnimationBehavior::AnimationBehaviorDyn>>>) -> Option<Rc<RefCell<dyn AdjustedImageArrayRotationAnimationDyn>>> {
        let mut imageRotationAnimationInfo: Option<Rc<RefCell<ImageArrayRotationAnimationInfo>>> = (anyType).clone() /* as ImageArrayRotationAnimationInfo */;
        return AdjustedImageArrayRotationAnimation::new(jcall!(imageRotationAnimationInfo).getImageArray(), jcall!(imageRotationAnimationInfo).getAngleInfoP(), jcall!(imageRotationAnimationInfo).getTotalAngle(), jcall!(imageRotationAnimationInfo).getDx(), jcall!(imageRotationAnimationInfo).getDy(), animationBehavior.clone()).map(|reference| reference as Rc<RefCell<dyn AdjustedImageArrayRotationAnimationDyn>>);
    }
    

    // throws Exception
    pub fn createAnimation(mut imageArray: Vec<Option<Rc<RefCell<Image>>>>, mut angleInfo: Option<Rc<RefCell<AngleInfo>>>, mut totalAngle: i32, mut animationBehavior: Option<Rc<RefCell<dyn crate::org::allbinary::animation::AnimationBehavior::AnimationBehaviorDyn>>>) -> Option<Rc<RefCell<dyn AdjustedImageArrayRotationAnimationDyn>>> {
        return AdjustedImageArrayRotationAnimation::new(imageArray.clone(), angleInfo.clone(), totalAngle.clone(), -(jcall!(imageArray[(0) as usize]).getWidth() >> 2), -(jcall!(imageArray[(0) as usize]).getHeight() >> 2), animationBehavior.clone()).map(|reference| reference as Rc<RefCell<dyn AdjustedImageArrayRotationAnimationDyn>>);
    }
    

    pub fn new_struct(mut imageArray: Vec<Option<Rc<RefCell<Image>>>>, mut angleInfo: Option<Rc<RefCell<AngleInfo>>>, mut totalAngle: i32, mut dx: i32, mut dy: i32, mut animationBehavior: Option<Rc<RefCell<dyn crate::org::allbinary::animation::AnimationBehavior::AnimationBehaviorDyn>>>) -> Self {
        let mut this = Self {
            base: ImageArrayRotationAnimation::new_struct(imageArray.clone(), angleInfo.clone(), totalAngle.clone(), animationBehavior.clone()),
            dx: 0,
            dy: 0,
        };
        // super(...) is in the struct literal above
        this.init(dx.clone(), dy.clone());
        this
    }

    pub fn new(mut imageArray: Vec<Option<Rc<RefCell<Image>>>>, mut angleInfo: Option<Rc<RefCell<AngleInfo>>>, mut totalAngle: i32, mut dx: i32, mut dy: i32, mut animationBehavior: Option<Rc<RefCell<dyn crate::org::allbinary::animation::AnimationBehavior::AnimationBehaviorDyn>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(imageArray, angleInfo, totalAngle, dx, dy, animationBehavior))))
    }
    

    // throws Exception
    pub fn init(&mut self, mut dx: i32, mut dy: i32) {
        self.setDx(dx.clone());
        self.setDy(dy.clone());
    }
    

    pub fn paintXY(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>, mut x: i32, mut y: i32) {
        self.base.paintXY(graphics.clone(), x + self.dx, y + self.dy);
    }
    

    pub fn setDx(&mut self, mut dx: i32) {
        self.dx = dx.clone();
    }
    

    pub fn getDx(&mut self) -> i32 {
        return self.dx.clone();
    }
    

    pub fn setDy(&mut self, mut dy: i32) {
        self.dy = dy.clone();
    }
    

    pub fn getDy(&mut self) -> i32 {
        return self.dy.clone();
    }
}
impl std::ops::Deref for AdjustedImageArrayRotationAnimation {
    type Target = ImageArrayRotationAnimation;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for AdjustedImageArrayRotationAnimation {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

pub trait AdjustedImageArrayRotationAnimationDyn: crate::org::allbinary::animation::image::ImageArrayRotationAnimation::ImageArrayRotationAnimationDyn {
    fn as_AdjustedImageArrayRotationAnimation(&self) -> &AdjustedImageArrayRotationAnimation;
    fn as_AdjustedImageArrayRotationAnimation_mut(&mut self) -> &mut AdjustedImageArrayRotationAnimation;
    fn init(&mut self, mut dx: i32, mut dy: i32) { AdjustedImageArrayRotationAnimation::init(self.as_AdjustedImageArrayRotationAnimation_mut(), dx, dy) }
}
impl AdjustedImageArrayRotationAnimationDyn for AdjustedImageArrayRotationAnimation {
    fn as_AdjustedImageArrayRotationAnimation(&self) -> &AdjustedImageArrayRotationAnimation { self }
    fn as_AdjustedImageArrayRotationAnimation_mut(&mut self) -> &mut AdjustedImageArrayRotationAnimation { self }
}

impl crate::org::allbinary::animation::image::ImageArrayRotationAnimation::ImageArrayRotationAnimationDyn for AdjustedImageArrayRotationAnimation {
    fn as_ImageArrayRotationAnimation(&self) -> &crate::org::allbinary::animation::image::ImageArrayRotationAnimation::ImageArrayRotationAnimation { &self.base }
    fn as_ImageArrayRotationAnimation_mut(&mut self) -> &mut crate::org::allbinary::animation::image::ImageArrayRotationAnimation::ImageArrayRotationAnimation { &mut self.base }
}
impl crate::org::allbinary::animation::image::ImageArrayBaseRotationAnimation::ImageArrayBaseRotationAnimationDyn for AdjustedImageArrayRotationAnimation {
    fn as_ImageArrayBaseRotationAnimation(&self) -> &crate::org::allbinary::animation::image::ImageArrayBaseRotationAnimation::ImageArrayBaseRotationAnimation { &self.base.base }
    fn as_ImageArrayBaseRotationAnimation_mut(&mut self) -> &mut crate::org::allbinary::animation::image::ImageArrayBaseRotationAnimation::ImageArrayBaseRotationAnimation { &mut self.base.base }
}
impl crate::org::allbinary::animation::Animation::AnimationDyn for AdjustedImageArrayRotationAnimation {
    fn as_Animation(&self) -> &crate::org::allbinary::animation::Animation::Animation { &self.base.base.base.base.base }
    fn as_Animation_mut(&mut self) -> &mut crate::org::allbinary::animation::Animation::Animation { &mut self.base.base.base.base.base }
    fn setAlpha(&mut self, p1: i32) { crate::org::allbinary::animation::image::ImageArrayBaseRotationAnimation::ImageArrayBaseRotationAnimation::setAlpha(crate::org::allbinary::animation::image::ImageArrayBaseRotationAnimation::ImageArrayBaseRotationAnimationDyn::as_ImageArrayBaseRotationAnimation_mut(self), p1) }
    fn paintXY(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>, mut x: i32, mut y: i32) { Self::paintXY(self, graphics, x, y) }
    fn setDx(&mut self, mut dx: i32) { Self::setDx(self, dx) }
    fn getDx(&mut self) -> i32 { Self::getDx(self) }
    fn setDy(&mut self, mut dy: i32) { Self::setDy(self, dy) }
    fn getDy(&mut self) -> i32 { Self::getDy(self) }
}
impl crate::org::allbinary::animation::IndexedAnimation::IndexedAnimationDyn for AdjustedImageArrayRotationAnimation {
    fn as_IndexedAnimation(&self) -> &crate::org::allbinary::animation::IndexedAnimation::IndexedAnimation { &self.base.base.base.base }
    fn as_IndexedAnimation_mut(&mut self) -> &mut crate::org::allbinary::animation::IndexedAnimation::IndexedAnimation { &mut self.base.base.base.base }
    fn setFrame(&mut self, p1: i32) { crate::org::allbinary::animation::image::ImageArrayBaseRotationAnimation::ImageArrayBaseRotationAnimation::setFrame(crate::org::allbinary::animation::image::ImageArrayBaseRotationAnimation::ImageArrayBaseRotationAnimationDyn::as_ImageArrayBaseRotationAnimation_mut(self), p1) }
    fn getFrame(&mut self) -> i32 { crate::org::allbinary::animation::RotationAnimation::RotationAnimation::getFrame(crate::org::allbinary::animation::RotationAnimation::RotationAnimationDyn::as_RotationAnimation_mut(self)) }
    fn getAnimationSize(&mut self) -> i32 { crate::org::allbinary::animation::image::ImageArrayBaseRotationAnimation::ImageArrayBaseRotationAnimation::getAnimationSize(crate::org::allbinary::animation::image::ImageArrayBaseRotationAnimation::ImageArrayBaseRotationAnimationDyn::as_ImageArrayBaseRotationAnimation_mut(self)) }
    fn getSize(&mut self) -> i32 { crate::org::allbinary::animation::RotationAnimation::RotationAnimation::getSize(crate::org::allbinary::animation::RotationAnimation::RotationAnimationDyn::as_RotationAnimation_mut(self)) }
    fn setSequence(&mut self, p1: Vec<i32>) { crate::org::allbinary::animation::image::ImageArrayBaseRotationAnimation::ImageArrayBaseRotationAnimation::setSequence(crate::org::allbinary::animation::image::ImageArrayBaseRotationAnimation::ImageArrayBaseRotationAnimationDyn::as_ImageArrayBaseRotationAnimation_mut(self), p1) }
    fn getSequence(&mut self) -> Vec<i32> { crate::org::allbinary::animation::image::ImageArrayBaseRotationAnimation::ImageArrayBaseRotationAnimation::getSequence(crate::org::allbinary::animation::image::ImageArrayBaseRotationAnimation::ImageArrayBaseRotationAnimationDyn::as_ImageArrayBaseRotationAnimation_mut(self)) }
}
impl crate::org::allbinary::animation::RotationAnimation::RotationAnimationDyn for AdjustedImageArrayRotationAnimation {
    fn as_RotationAnimation(&self) -> &crate::org::allbinary::animation::RotationAnimation::RotationAnimation { &self.base.base.base }
    fn as_RotationAnimation_mut(&mut self) -> &mut crate::org::allbinary::animation::RotationAnimation::RotationAnimation { &mut self.base.base.base }
    fn nextRotation(&mut self) { crate::org::allbinary::animation::image::ImageArrayBaseRotationAnimation::ImageArrayBaseRotationAnimation::nextRotation(crate::org::allbinary::animation::image::ImageArrayBaseRotationAnimation::ImageArrayBaseRotationAnimationDyn::as_ImageArrayBaseRotationAnimation_mut(self)) }
    fn previousRotation(&mut self) { crate::org::allbinary::animation::image::ImageArrayBaseRotationAnimation::ImageArrayBaseRotationAnimation::previousRotation(crate::org::allbinary::animation::image::ImageArrayBaseRotationAnimation::ImageArrayBaseRotationAnimationDyn::as_ImageArrayBaseRotationAnimation_mut(self)) }
}
impl crate::org::allbinary::image::opengles::OpenGLSurfaceChangedInterface::OpenGLSurfaceChangedInterface for AdjustedImageArrayRotationAnimation {
    fn set(&mut self, p1: Option<Rc<RefCell<dyn crate::javax::microedition::khronos::opengles::GL::GL>>>) { crate::org::allbinary::animation::Animation::Animation::set(crate::org::allbinary::animation::Animation::AnimationDyn::as_Animation_mut(self), p1) }
}


