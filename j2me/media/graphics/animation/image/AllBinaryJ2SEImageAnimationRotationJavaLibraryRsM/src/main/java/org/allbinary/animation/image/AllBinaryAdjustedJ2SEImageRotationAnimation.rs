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
use super::AllBinaryJ2SEImageRotationAnimation::AllBinaryJ2SEImageRotationAnimation;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct AllBinaryAdjustedJ2SEImageRotationAnimation {
    pub base: AllBinaryJ2SEImageRotationAnimation,
    pub dx: i32,
    pub dy: i32,
}

impl crate::org::allbinary::animation::SequentialAnimationInterface::SequentialAnimationInterface for AllBinaryAdjustedJ2SEImageRotationAnimation {
    fn getSize(&mut self) -> i32 { self.base.getSize() }
    fn previousFrame(&mut self) { self.base.previousFrame() }
}
impl crate::org::allbinary::graphics::paint::LocationPaintableInterface::LocationPaintableInterface for AllBinaryAdjustedJ2SEImageRotationAnimation {
    fn paintXY(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>, mut x: i32, mut y: i32) { Self::paintXY(self, graphics, x, y) }
}

impl AllBinaryAdjustedJ2SEImageRotationAnimation {
    

    pub fn new_struct(mut originalImage: Option<Rc<RefCell<Image>>>, mut image: Option<Rc<RefCell<Image>>>, mut angleInfo: Option<Rc<RefCell<AngleInfo>>>, mut totalAngle: i16, mut dx: i32, mut dy: i32, mut animationBehavior: Option<Rc<RefCell<dyn crate::org::allbinary::animation::AnimationBehavior::AnimationBehaviorDyn>>>) -> Self {
        let mut this = Self {
            base: AllBinaryJ2SEImageRotationAnimation::new_struct(originalImage.clone(), image.clone(), angleInfo.clone(), totalAngle.clone(), animationBehavior.clone()),
            dx: 0,
            dy: 0,
        };
        // super(...) is in the struct literal above
        this.init(dx.clone(), dy.clone());
        this
    }

    pub fn new(mut originalImage: Option<Rc<RefCell<Image>>>, mut image: Option<Rc<RefCell<Image>>>, mut angleInfo: Option<Rc<RefCell<AngleInfo>>>, mut totalAngle: i16, mut dx: i32, mut dy: i32, mut animationBehavior: Option<Rc<RefCell<dyn crate::org::allbinary::animation::AnimationBehavior::AnimationBehaviorDyn>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(originalImage, image, angleInfo, totalAngle, dx, dy, animationBehavior))))
    }
    

    // throws Exception
    pub fn init(&mut self, mut dx: i32, mut dy: i32) {
        self.dx = dx.clone();
        self.dy = dy.clone();
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
    

    pub fn paintXY(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>, mut x: i32, mut y: i32) {
        self.base.paintXY(graphics.clone(), x + self.dx, y + self.dy);
    }
}
impl std::ops::Deref for AllBinaryAdjustedJ2SEImageRotationAnimation {
    type Target = AllBinaryJ2SEImageRotationAnimation;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for AllBinaryAdjustedJ2SEImageRotationAnimation {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::animation::image::AllBinaryJ2SEImageRotationAnimation::AllBinaryJ2SEImageRotationAnimationDyn for AllBinaryAdjustedJ2SEImageRotationAnimation {
    fn as_AllBinaryJ2SEImageRotationAnimation(&self) -> &crate::org::allbinary::animation::image::AllBinaryJ2SEImageRotationAnimation::AllBinaryJ2SEImageRotationAnimation { &self.base }
    fn as_AllBinaryJ2SEImageRotationAnimation_mut(&mut self) -> &mut crate::org::allbinary::animation::image::AllBinaryJ2SEImageRotationAnimation::AllBinaryJ2SEImageRotationAnimation { &mut self.base }
}
impl crate::org::allbinary::animation::image::ImageBaseRotationAnimation::ImageBaseRotationAnimationDyn for AllBinaryAdjustedJ2SEImageRotationAnimation {
    fn as_ImageBaseRotationAnimation(&self) -> &crate::org::allbinary::animation::image::ImageBaseRotationAnimation::ImageBaseRotationAnimation { &self.base.base }
    fn as_ImageBaseRotationAnimation_mut(&mut self) -> &mut crate::org::allbinary::animation::image::ImageBaseRotationAnimation::ImageBaseRotationAnimation { &mut self.base.base }
    fn close(&mut self) { crate::org::allbinary::animation::image::AllBinaryJ2SEImageRotationAnimation::AllBinaryJ2SEImageRotationAnimation::close(crate::org::allbinary::animation::image::AllBinaryJ2SEImageRotationAnimation::AllBinaryJ2SEImageRotationAnimationDyn::as_AllBinaryJ2SEImageRotationAnimation_mut(self)) }
    fn finalize(&mut self) { crate::org::allbinary::animation::image::AllBinaryJ2SEImageRotationAnimation::AllBinaryJ2SEImageRotationAnimation::finalize(crate::org::allbinary::animation::image::AllBinaryJ2SEImageRotationAnimation::AllBinaryJ2SEImageRotationAnimationDyn::as_AllBinaryJ2SEImageRotationAnimation_mut(self)) }
}
impl crate::org::allbinary::animation::Animation::AnimationDyn for AllBinaryAdjustedJ2SEImageRotationAnimation {
    fn as_Animation(&self) -> &crate::org::allbinary::animation::Animation::Animation { &self.base.base.base.base.base }
    fn as_Animation_mut(&mut self) -> &mut crate::org::allbinary::animation::Animation::Animation { &mut self.base.base.base.base.base }
    fn setAlpha(&mut self, p1: i32) { crate::org::allbinary::animation::image::AllBinaryJ2SEImageRotationAnimation::AllBinaryJ2SEImageRotationAnimation::setAlpha(crate::org::allbinary::animation::image::AllBinaryJ2SEImageRotationAnimation::AllBinaryJ2SEImageRotationAnimationDyn::as_AllBinaryJ2SEImageRotationAnimation_mut(self), p1) }
    fn paintXY(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>, mut x: i32, mut y: i32) { Self::paintXY(self, graphics, x, y) }
    fn setBasicColorP(&mut self, p1: Option<Rc<RefCell<crate::org::allbinary::graphics::color::BasicColor::BasicColor>>>) { crate::org::allbinary::animation::image::AllBinaryJ2SEImageRotationAnimation::AllBinaryJ2SEImageRotationAnimation::setBasicColorP(crate::org::allbinary::animation::image::AllBinaryJ2SEImageRotationAnimation::AllBinaryJ2SEImageRotationAnimationDyn::as_AllBinaryJ2SEImageRotationAnimation_mut(self), p1) }
    fn changeBasicColor(&mut self, p1: Option<Rc<RefCell<crate::org::allbinary::graphics::color::BasicColor::BasicColor>>>) { crate::org::allbinary::animation::image::AllBinaryJ2SEImageRotationAnimation::AllBinaryJ2SEImageRotationAnimation::changeBasicColor(crate::org::allbinary::animation::image::AllBinaryJ2SEImageRotationAnimation::AllBinaryJ2SEImageRotationAnimationDyn::as_AllBinaryJ2SEImageRotationAnimation_mut(self), p1) }
    fn setScale(&mut self, p1: f32, p2: f32) { crate::org::allbinary::animation::image::AllBinaryJ2SEImageRotationAnimation::AllBinaryJ2SEImageRotationAnimation::setScale(crate::org::allbinary::animation::image::AllBinaryJ2SEImageRotationAnimation::AllBinaryJ2SEImageRotationAnimationDyn::as_AllBinaryJ2SEImageRotationAnimation_mut(self), p1, p2) }
    fn setMaxScale(&mut self, p1: f32, p2: f32) { crate::org::allbinary::animation::image::AllBinaryJ2SEImageRotationAnimation::AllBinaryJ2SEImageRotationAnimation::setMaxScale(crate::org::allbinary::animation::image::AllBinaryJ2SEImageRotationAnimation::AllBinaryJ2SEImageRotationAnimationDyn::as_AllBinaryJ2SEImageRotationAnimation_mut(self), p1, p2) }
    fn setDx(&mut self, mut dx: i32) { Self::setDx(self, dx) }
    fn getDx(&mut self) -> i32 { Self::getDx(self) }
    fn setDy(&mut self, mut dy: i32) { Self::setDy(self, dy) }
    fn getDy(&mut self) -> i32 { Self::getDy(self) }
}
impl crate::org::allbinary::animation::IndexedAnimation::IndexedAnimationDyn for AllBinaryAdjustedJ2SEImageRotationAnimation {
    fn as_IndexedAnimation(&self) -> &crate::org::allbinary::animation::IndexedAnimation::IndexedAnimation { &self.base.base.base.base }
    fn as_IndexedAnimation_mut(&mut self) -> &mut crate::org::allbinary::animation::IndexedAnimation::IndexedAnimation { &mut self.base.base.base.base }
    fn setFrame(&mut self, p1: i32) { crate::org::allbinary::animation::image::AllBinaryJ2SEImageRotationAnimation::AllBinaryJ2SEImageRotationAnimation::setFrame(crate::org::allbinary::animation::image::AllBinaryJ2SEImageRotationAnimation::AllBinaryJ2SEImageRotationAnimationDyn::as_AllBinaryJ2SEImageRotationAnimation_mut(self), p1) }
    fn getFrame(&mut self) -> i32 { crate::org::allbinary::animation::RotationAnimation::RotationAnimation::getFrame(crate::org::allbinary::animation::RotationAnimation::RotationAnimationDyn::as_RotationAnimation_mut(self)) }
    fn getAnimationSize(&mut self) -> i32 { crate::org::allbinary::animation::image::ImageBaseRotationAnimation::ImageBaseRotationAnimation::getAnimationSize(crate::org::allbinary::animation::image::ImageBaseRotationAnimation::ImageBaseRotationAnimationDyn::as_ImageBaseRotationAnimation_mut(self)) }
    fn getSize(&mut self) -> i32 { crate::org::allbinary::animation::RotationAnimation::RotationAnimation::getSize(crate::org::allbinary::animation::RotationAnimation::RotationAnimationDyn::as_RotationAnimation_mut(self)) }
    fn setSequence(&mut self, p1: Vec<i32>) { crate::org::allbinary::animation::image::ImageBaseRotationAnimation::ImageBaseRotationAnimation::setSequence(crate::org::allbinary::animation::image::ImageBaseRotationAnimation::ImageBaseRotationAnimationDyn::as_ImageBaseRotationAnimation_mut(self), p1) }
    fn getSequence(&mut self) -> Vec<i32> { crate::org::allbinary::animation::image::ImageBaseRotationAnimation::ImageBaseRotationAnimation::getSequence(crate::org::allbinary::animation::image::ImageBaseRotationAnimation::ImageBaseRotationAnimationDyn::as_ImageBaseRotationAnimation_mut(self)) }
    fn getWidth(&mut self) -> i32 { crate::org::allbinary::animation::image::ImageBaseRotationAnimation::ImageBaseRotationAnimation::getWidth(crate::org::allbinary::animation::image::ImageBaseRotationAnimation::ImageBaseRotationAnimationDyn::as_ImageBaseRotationAnimation_mut(self)) }
    fn getHeight(&mut self) -> i32 { crate::org::allbinary::animation::image::ImageBaseRotationAnimation::ImageBaseRotationAnimation::getHeight(crate::org::allbinary::animation::image::ImageBaseRotationAnimation::ImageBaseRotationAnimationDyn::as_ImageBaseRotationAnimation_mut(self)) }
}
impl crate::org::allbinary::animation::RotationAnimation::RotationAnimationDyn for AllBinaryAdjustedJ2SEImageRotationAnimation {
    fn as_RotationAnimation(&self) -> &crate::org::allbinary::animation::RotationAnimation::RotationAnimation { &self.base.base.base }
    fn as_RotationAnimation_mut(&mut self) -> &mut crate::org::allbinary::animation::RotationAnimation::RotationAnimation { &mut self.base.base.base }
    fn nextRotation(&mut self) { crate::org::allbinary::animation::image::AllBinaryJ2SEImageRotationAnimation::AllBinaryJ2SEImageRotationAnimation::nextRotation(crate::org::allbinary::animation::image::AllBinaryJ2SEImageRotationAnimation::AllBinaryJ2SEImageRotationAnimationDyn::as_AllBinaryJ2SEImageRotationAnimation_mut(self)) }
    fn previousRotation(&mut self) { crate::org::allbinary::animation::image::AllBinaryJ2SEImageRotationAnimation::AllBinaryJ2SEImageRotationAnimation::previousRotation(crate::org::allbinary::animation::image::AllBinaryJ2SEImageRotationAnimation::AllBinaryJ2SEImageRotationAnimationDyn::as_AllBinaryJ2SEImageRotationAnimation_mut(self)) }
}
impl crate::org::allbinary::image::opengles::OpenGLSurfaceChangedInterface::OpenGLSurfaceChangedInterface for AllBinaryAdjustedJ2SEImageRotationAnimation {
    fn set(&mut self, p1: Option<Rc<RefCell<dyn crate::javax::microedition::khronos::opengles::GL::GL>>>) { crate::org::allbinary::animation::Animation::Animation::set(crate::org::allbinary::animation::Animation::AnimationDyn::as_Animation_mut(self), p1) }
}


