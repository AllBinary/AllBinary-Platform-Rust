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
use super::LeftToRightImageAnimation::LeftToRightImageAnimation;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct AdjustedLeftToRightImageAnimation {
    pub base: LeftToRightImageAnimation,
    pub dx: i32,
    pub dy: i32,
}

impl crate::org::allbinary::animation::SequentialAnimationInterface::SequentialAnimationInterface for AdjustedLeftToRightImageAnimation {
    fn getSize(&mut self) -> i32 { self.base.getSize() }
    fn previousFrame(&mut self) { self.base.previousFrame() }
}
impl crate::org::allbinary::graphics::paint::LocationPaintableInterface::LocationPaintableInterface for AdjustedLeftToRightImageAnimation {
    fn paintXY(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>, mut x: i32, mut y: i32) { Self::paintXY(self, graphics, x, y) }
}

impl AdjustedLeftToRightImageAnimation {
    

    pub fn new_struct(mut image: Option<Rc<RefCell<Image>>>, mut sequenceArray: Vec<i32>, mut dx: i32, mut dy: i32, mut animationBehavior: Option<Rc<RefCell<dyn crate::org::allbinary::animation::AnimationBehavior::AnimationBehaviorDyn>>>) -> Self {
        let mut this = Self {
            base: LeftToRightImageAnimation::new_struct(image.clone(), sequenceArray.clone(), animationBehavior.clone()),
            dx: 0,
            dy: 0,
        };
        // super(...) is in the struct literal above
        this.init(dx.clone(), dy.clone());
        this
    }

    pub fn new(mut image: Option<Rc<RefCell<Image>>>, mut sequenceArray: Vec<i32>, mut dx: i32, mut dy: i32, mut animationBehavior: Option<Rc<RefCell<dyn crate::org::allbinary::animation::AnimationBehavior::AnimationBehaviorDyn>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(image, sequenceArray, dx, dy, animationBehavior))))
    }
    

    // throws Exception
    pub fn init(&mut self, mut dx: i32, mut dy: i32) {
        self.dx = dx.clone();
        self.dy = dy.clone();
    }
    

    pub fn paintXY(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>, mut x: i32, mut y: i32) {
        self.base.paintXY(graphics.clone(), x + self.dx, y + self.dy);
    }
}
impl std::ops::Deref for AdjustedLeftToRightImageAnimation {
    type Target = LeftToRightImageAnimation;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for AdjustedLeftToRightImageAnimation {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::animation::image::ImageAnimation::ImageAnimationDyn for AdjustedLeftToRightImageAnimation {
    fn as_ImageAnimation(&self) -> &crate::org::allbinary::animation::image::ImageAnimation::ImageAnimation { &self.base.base.base }
    fn as_ImageAnimation_mut(&mut self) -> &mut crate::org::allbinary::animation::image::ImageAnimation::ImageAnimation { &mut self.base.base.base }
}
impl crate::org::allbinary::animation::image::ImageSegmentAnimation::ImageSegmentAnimationDyn for AdjustedLeftToRightImageAnimation {
    fn as_ImageSegmentAnimation(&self) -> &crate::org::allbinary::animation::image::ImageSegmentAnimation::ImageSegmentAnimation { &self.base.base }
    fn as_ImageSegmentAnimation_mut(&mut self) -> &mut crate::org::allbinary::animation::image::ImageSegmentAnimation::ImageSegmentAnimation { &mut self.base.base }
}
impl crate::org::allbinary::animation::image::LeftToRightImageAnimation::LeftToRightImageAnimationDyn for AdjustedLeftToRightImageAnimation {
    fn as_LeftToRightImageAnimation(&self) -> &crate::org::allbinary::animation::image::LeftToRightImageAnimation::LeftToRightImageAnimation { &self.base }
    fn as_LeftToRightImageAnimation_mut(&mut self) -> &mut crate::org::allbinary::animation::image::LeftToRightImageAnimation::LeftToRightImageAnimation { &mut self.base }
}
impl crate::org::allbinary::animation::Animation::AnimationDyn for AdjustedLeftToRightImageAnimation {
    fn as_Animation(&self) -> &crate::org::allbinary::animation::Animation::Animation { &self.base.base.base.base.base }
    fn as_Animation_mut(&mut self) -> &mut crate::org::allbinary::animation::Animation::Animation { &mut self.base.base.base.base.base }
    fn nextFrame(&mut self) { crate::org::allbinary::animation::image::LeftToRightImageAnimation::LeftToRightImageAnimation::nextFrame(crate::org::allbinary::animation::image::LeftToRightImageAnimation::LeftToRightImageAnimationDyn::as_LeftToRightImageAnimation_mut(self)) }
    fn paintXY(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>, mut x: i32, mut y: i32) { Self::paintXY(self, graphics, x, y) }
}
impl crate::org::allbinary::animation::IndexedAnimation::IndexedAnimationDyn for AdjustedLeftToRightImageAnimation {
    fn as_IndexedAnimation(&self) -> &crate::org::allbinary::animation::IndexedAnimation::IndexedAnimation { &self.base.base.base.base }
    fn as_IndexedAnimation_mut(&mut self) -> &mut crate::org::allbinary::animation::IndexedAnimation::IndexedAnimation { &mut self.base.base.base.base }
    fn setFrame(&mut self, p1: i32) { crate::org::allbinary::animation::image::LeftToRightImageAnimation::LeftToRightImageAnimation::setFrame(crate::org::allbinary::animation::image::LeftToRightImageAnimation::LeftToRightImageAnimationDyn::as_LeftToRightImageAnimation_mut(self), p1) }
    fn getFrame(&mut self) -> i32 { crate::org::allbinary::animation::image::ImageSegmentAnimation::ImageSegmentAnimation::getFrame(crate::org::allbinary::animation::image::ImageSegmentAnimation::ImageSegmentAnimationDyn::as_ImageSegmentAnimation_mut(self)) }
    fn getAnimationSize(&mut self) -> i32 { crate::org::allbinary::animation::image::ImageAnimation::ImageAnimation::getAnimationSize(crate::org::allbinary::animation::image::ImageAnimation::ImageAnimationDyn::as_ImageAnimation_mut(self)) }
    fn getSize(&mut self) -> i32 { crate::org::allbinary::animation::image::ImageSegmentAnimation::ImageSegmentAnimation::getSize(crate::org::allbinary::animation::image::ImageSegmentAnimation::ImageSegmentAnimationDyn::as_ImageSegmentAnimation_mut(self)) }
    fn previousFrame(&mut self) { crate::org::allbinary::animation::image::LeftToRightImageAnimation::LeftToRightImageAnimation::previousFrame(crate::org::allbinary::animation::image::LeftToRightImageAnimation::LeftToRightImageAnimationDyn::as_LeftToRightImageAnimation_mut(self)) }
    fn setSequence(&mut self, p1: Vec<i32>) { crate::org::allbinary::animation::image::ImageSegmentAnimation::ImageSegmentAnimation::setSequence(crate::org::allbinary::animation::image::ImageSegmentAnimation::ImageSegmentAnimationDyn::as_ImageSegmentAnimation_mut(self), p1) }
    fn getSequence(&mut self) -> Vec<i32> { crate::org::allbinary::animation::image::ImageSegmentAnimation::ImageSegmentAnimation::getSequence(crate::org::allbinary::animation::image::ImageSegmentAnimation::ImageSegmentAnimationDyn::as_ImageSegmentAnimation_mut(self)) }
}
impl crate::org::allbinary::image::opengles::OpenGLSurfaceChangedInterface::OpenGLSurfaceChangedInterface for AdjustedLeftToRightImageAnimation {
    fn set(&mut self, p1: Option<Rc<RefCell<dyn crate::javax::microedition::khronos::opengles::GL::GL>>>) { crate::org::allbinary::animation::Animation::Animation::set(crate::org::allbinary::animation::Animation::AnimationDyn::as_Animation_mut(self), p1) }
}


