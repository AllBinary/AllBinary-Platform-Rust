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
// package org.allbinary.animation.image
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::javax::microedition::lcdui::Graphics::Graphics;
// use crate::javax::microedition::lcdui::Image::Image;
use crate::org::allbinary::animation::AnimationBehavior::AnimationBehavior;
use super::ImageAnimation::ImageAnimation;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct AdjustedImageAnimation {
    pub base: ImageAnimation,
    pub dx: i32,
    pub dy: i32,
}

impl crate::org::allbinary::animation::SequentialAnimationInterface::SequentialAnimationInterface for AdjustedImageAnimation {
    fn getSize(&mut self) -> i32 { self.base.getSize() }
    fn previousFrame(&mut self) { self.base.previousFrame() }
}
impl crate::org::allbinary::graphics::paint::LocationPaintableInterface::LocationPaintableInterface for AdjustedImageAnimation {
    fn paintXY(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>, mut x: i32, mut y: i32) { Self::paintXY(self, graphics, x, y) }
}

impl AdjustedImageAnimation {
    

    pub fn new_struct(mut image: Option<Rc<RefCell<Image>>>, mut dx: i32, mut dy: i32, mut animationBehavior: Option<Rc<RefCell<dyn crate::org::allbinary::animation::AnimationBehavior::AnimationBehaviorDyn>>>) -> Self {
        let mut this = Self {
            base: ImageAnimation::new_struct(image.clone(), animationBehavior.clone()),
            dx: 0,
            dy: 0,
        };
        // super(...) is in the struct literal above
        this.init(dx.clone(), dy.clone());
        this
    }

    pub fn new(mut image: Option<Rc<RefCell<Image>>>, mut dx: i32, mut dy: i32, mut animationBehavior: Option<Rc<RefCell<dyn crate::org::allbinary::animation::AnimationBehavior::AnimationBehaviorDyn>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(image, dx, dy, animationBehavior))))
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
impl std::ops::Deref for AdjustedImageAnimation {
    type Target = ImageAnimation;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for AdjustedImageAnimation {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::animation::image::ImageAnimation::ImageAnimationDyn for AdjustedImageAnimation {
    fn as_ImageAnimation(&self) -> &crate::org::allbinary::animation::image::ImageAnimation::ImageAnimation { &self.base }
    fn as_ImageAnimation_mut(&mut self) -> &mut crate::org::allbinary::animation::image::ImageAnimation::ImageAnimation { &mut self.base }
}
impl crate::org::allbinary::animation::Animation::AnimationDyn for AdjustedImageAnimation {
    fn as_Animation(&self) -> &crate::org::allbinary::animation::Animation::Animation { &self.base.base.base }
    fn as_Animation_mut(&mut self) -> &mut crate::org::allbinary::animation::Animation::Animation { &mut self.base.base.base }
    fn nextFrame(&mut self) { crate::org::allbinary::animation::image::ImageAnimation::ImageAnimation::nextFrame(crate::org::allbinary::animation::image::ImageAnimation::ImageAnimationDyn::as_ImageAnimation_mut(self)) }
    fn paintXY(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>, mut x: i32, mut y: i32) { Self::paintXY(self, graphics, x, y) }
    fn setDx(&mut self, mut dx: i32) { Self::setDx(self, dx) }
    fn getDx(&mut self) -> i32 { Self::getDx(self) }
    fn setDy(&mut self, mut dy: i32) { Self::setDy(self, dy) }
    fn getDy(&mut self) -> i32 { Self::getDy(self) }
}
impl crate::org::allbinary::animation::IndexedAnimation::IndexedAnimationDyn for AdjustedImageAnimation {
    fn as_IndexedAnimation(&self) -> &crate::org::allbinary::animation::IndexedAnimation::IndexedAnimation { &self.base.base }
    fn as_IndexedAnimation_mut(&mut self) -> &mut crate::org::allbinary::animation::IndexedAnimation::IndexedAnimation { &mut self.base.base }
    fn setFrame(&mut self, p1: i32) { crate::org::allbinary::animation::image::ImageAnimation::ImageAnimation::setFrame(crate::org::allbinary::animation::image::ImageAnimation::ImageAnimationDyn::as_ImageAnimation_mut(self), p1) }
    fn getFrame(&mut self) -> i32 { crate::org::allbinary::animation::image::ImageAnimation::ImageAnimation::getFrame(crate::org::allbinary::animation::image::ImageAnimation::ImageAnimationDyn::as_ImageAnimation_mut(self)) }
    fn getAnimationSize(&mut self) -> i32 { crate::org::allbinary::animation::image::ImageAnimation::ImageAnimation::getAnimationSize(crate::org::allbinary::animation::image::ImageAnimation::ImageAnimationDyn::as_ImageAnimation_mut(self)) }
    fn getSize(&mut self) -> i32 { crate::org::allbinary::animation::image::ImageAnimation::ImageAnimation::getSize(crate::org::allbinary::animation::image::ImageAnimation::ImageAnimationDyn::as_ImageAnimation_mut(self)) }
    fn previousFrame(&mut self) { crate::org::allbinary::animation::image::ImageAnimation::ImageAnimation::previousFrame(crate::org::allbinary::animation::image::ImageAnimation::ImageAnimationDyn::as_ImageAnimation_mut(self)) }
    fn setSequence(&mut self, p1: Vec<i32>) { crate::org::allbinary::animation::image::ImageAnimation::ImageAnimation::setSequence(crate::org::allbinary::animation::image::ImageAnimation::ImageAnimationDyn::as_ImageAnimation_mut(self), p1) }
    fn getSequence(&mut self) -> Vec<i32> { crate::org::allbinary::animation::image::ImageAnimation::ImageAnimation::getSequence(crate::org::allbinary::animation::image::ImageAnimation::ImageAnimationDyn::as_ImageAnimation_mut(self)) }
}
impl crate::org::allbinary::image::opengles::OpenGLSurfaceChangedInterface::OpenGLSurfaceChangedInterface for AdjustedImageAnimation {
    fn set(&mut self, p1: Option<Rc<RefCell<dyn crate::javax::microedition::khronos::opengles::GL::GL>>>) { crate::org::allbinary::animation::Animation::Animation::set(crate::org::allbinary::animation::Animation::AnimationDyn::as_Animation_mut(self), p1) }
}


