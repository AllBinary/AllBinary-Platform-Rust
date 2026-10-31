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
use crate::org::allbinary::animation::IndexedAnimation::IndexedAnimation;
use crate::org::allbinary::graphics::Anchor::Anchor;
use crate::org::allbinary::logic::math::PrimitiveIntUtil::PrimitiveIntUtil;
use crate::org::allbinary::util::CircularIndexUtil::CircularIndexUtil;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct TweenedImageAnimation {
    pub base: IndexedAnimation,
    pub image: Option<Rc<RefCell<Image>>>,
    pub dx: i32,
    pub dy: i32,
    pub totalFrames: i32,
    pub circularIndexUtil: Option<Rc<RefCell<CircularIndexUtil>>>,
    pub anchor: i32,
}

impl crate::org::allbinary::animation::IndexedAnimationInterface::IndexedAnimationInterface for TweenedImageAnimation {
    fn reset(&mut self) { self.base.reset() }
    fn setFrame(&mut self, mut index: i32) { Self::setFrame(self, index) }
    fn getFrame(&mut self) -> i32 { Self::getFrame(self) }
}
impl crate::org::allbinary::animation::SequentialAnimationInterface::SequentialAnimationInterface for TweenedImageAnimation {
    fn getSize(&mut self) -> i32 { Self::getSize(self) }
    fn previousFrame(&mut self) { Self::previousFrame(self) }
}
impl crate::org::allbinary::graphics::paint::LocationPaintableInterface::LocationPaintableInterface for TweenedImageAnimation {
    fn paintXY(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>, mut x: i32, mut y: i32) { Self::paintXY(self, graphics, x, y) }
}

impl TweenedImageAnimation {
    

    pub fn new_struct(mut image: Option<Rc<RefCell<Image>>>, mut dx: i32, mut dy: i32, mut totalFrames: i32, mut animationBehavior: Option<Rc<RefCell<dyn crate::org::allbinary::animation::AnimationBehavior::AnimationBehaviorDyn>>>) -> Self {
        let mut this = Self {
            base: IndexedAnimation::new_struct(animationBehavior.clone()),
            image: None,
            dx: 0,
            dy: 0,
            totalFrames: 0,
            circularIndexUtil: None,
            anchor: 0,
        };
        this.anchor = unsafe { crate::org::allbinary::graphics::Anchor::Anchor_TOP_LEFT.clone() }.clone();
        // super(...) is in the struct literal above
        this.image = image.clone();
        this.dx = dx.clone();
        this.dy = dy.clone();
        this.totalFrames = totalFrames.clone();
        this.circularIndexUtil = CircularIndexUtil::createInstance(this.totalFrames.clone());
        this
    }

    pub fn new(mut image: Option<Rc<RefCell<Image>>>, mut dx: i32, mut dy: i32, mut totalFrames: i32, mut animationBehavior: Option<Rc<RefCell<dyn crate::org::allbinary::animation::AnimationBehavior::AnimationBehaviorDyn>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(image, dx, dy, totalFrames, animationBehavior))))
    }
    

    // throws Exception
    pub fn getAnimationSize(&mut self) -> i32 {
        return self.getSize();
    }
    

    pub fn nextFrame(&mut self) {
        jcall!(self.circularIndexUtil).next();
    }
    

    pub fn previousFrame(&mut self) {
        jcall!(self.circularIndexUtil).previous();
    }
    

    pub fn setFrame(&mut self, mut index: i32) {
        jcall!(self.circularIndexUtil).setIndex(index.clone());
    }
    

    pub fn getFrame(&mut self) -> i32 {
        return jcall!(self.circularIndexUtil).getIndex();
    }
    

    pub fn getSize(&mut self) -> i32 {
        return self.totalFrames.clone();
    }
    

    pub fn setSequence(&mut self, mut sequence: Vec<i32>) {
    }
    

    pub fn getSequence(&mut self) -> Vec<i32> {
        return PrimitiveIntUtil::getArrayInstance();
    }
    

    pub fn paintXY(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>, mut x: i32, mut y: i32) {
        let mut frame: i32 = self.getFrame();
        let mut currentX: i32 = self.dx * frame;
        let mut currentY: i32 = self.dy * frame;
        jcall!(graphics).drawImage(self.image.clone(), x + currentX, y + currentY, self.anchor.clone());
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
impl std::ops::Deref for TweenedImageAnimation {
    type Target = IndexedAnimation;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for TweenedImageAnimation {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::animation::Animation::AnimationDyn for TweenedImageAnimation {
    fn as_Animation(&self) -> &crate::org::allbinary::animation::Animation::Animation { &self.base.base }
    fn as_Animation_mut(&mut self) -> &mut crate::org::allbinary::animation::Animation::Animation { &mut self.base.base }
    fn nextFrame(&mut self) { Self::nextFrame(self) }
    fn paintXY(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>, mut x: i32, mut y: i32) { Self::paintXY(self, graphics, x, y) }
}
impl crate::org::allbinary::animation::IndexedAnimation::IndexedAnimationDyn for TweenedImageAnimation {
    fn as_IndexedAnimation(&self) -> &crate::org::allbinary::animation::IndexedAnimation::IndexedAnimation { &self.base }
    fn as_IndexedAnimation_mut(&mut self) -> &mut crate::org::allbinary::animation::IndexedAnimation::IndexedAnimation { &mut self.base }
    fn setFrame(&mut self, mut index: i32) { Self::setFrame(self, index) }
    fn getFrame(&mut self) -> i32 { Self::getFrame(self) }
    fn getAnimationSize(&mut self) -> i32 { Self::getAnimationSize(self) }
    fn getSize(&mut self) -> i32 { Self::getSize(self) }
    fn previousFrame(&mut self) { Self::previousFrame(self) }
    fn setSequence(&mut self, mut sequence: Vec<i32>) { Self::setSequence(self, sequence) }
    fn getSequence(&mut self) -> Vec<i32> { Self::getSequence(self) }
}
impl crate::org::allbinary::image::opengles::OpenGLSurfaceChangedInterface::OpenGLSurfaceChangedInterface for TweenedImageAnimation {
    fn set(&mut self, p1: Option<Rc<RefCell<dyn crate::javax::microedition::khronos::opengles::GL::GL>>>) { crate::org::allbinary::animation::Animation::Animation::set(crate::org::allbinary::animation::Animation::AnimationDyn::as_Animation_mut(self), p1) }
}


