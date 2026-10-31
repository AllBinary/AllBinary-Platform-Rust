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

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct ImageAnimation {
    pub base: IndexedAnimation,
    pub image: Option<Rc<RefCell<Image>>>,
    pub anchor: i32,
}

impl crate::org::allbinary::animation::IndexedAnimationInterface::IndexedAnimationInterface for ImageAnimation {
    fn reset(&mut self) { self.base.reset() }
    fn setFrame(&mut self, mut index: i32) { Self::setFrame(self, index) }
    fn getFrame(&mut self) -> i32 { Self::getFrame(self) }
}
impl crate::org::allbinary::animation::SequentialAnimationInterface::SequentialAnimationInterface for ImageAnimation {
    fn getSize(&mut self) -> i32 { Self::getSize(self) }
    fn previousFrame(&mut self) { Self::previousFrame(self) }
}
impl crate::org::allbinary::graphics::paint::LocationPaintableInterface::LocationPaintableInterface for ImageAnimation {
    fn paintXY(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>, mut x: i32, mut y: i32) { Self::paintXY(self, graphics, x, y) }
}

impl ImageAnimation {
    

    pub fn new_struct(mut image: Option<Rc<RefCell<Image>>>, mut animationBehavior: Option<Rc<RefCell<dyn crate::org::allbinary::animation::AnimationBehavior::AnimationBehaviorDyn>>>) -> Self {
        let mut this = Self {
            base: IndexedAnimation::new_struct(animationBehavior.clone()),
            image: None,
            anchor: 0,
        };
        this.anchor = unsafe { crate::org::allbinary::graphics::Anchor::Anchor_TOP_LEFT.clone() }.clone();
        // super(...) is in the struct literal above
        this.image = image.clone();
        this
    }

    pub fn new(mut image: Option<Rc<RefCell<Image>>>, mut animationBehavior: Option<Rc<RefCell<dyn crate::org::allbinary::animation::AnimationBehavior::AnimationBehaviorDyn>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(image, animationBehavior))))
    }
    

    // throws Exception
    pub fn getAnimationSize(&mut self) -> i32 {
        return self.getSize();
    }
    

    pub fn nextFrame(&mut self) {
    }
    

    pub fn previousFrame(&mut self) {
    }
    

    pub fn setFrame(&mut self, mut index: i32) {
    }
    

    pub fn getFrame(&mut self) -> i32 {
        return 0;
    }
    

    pub fn getSize(&mut self) -> i32 {
        return 1;
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
    

    // throws Exception
    pub fn close(&mut self) {
        jcall!(DisposalUtil::getInstance()).disposeImage(self.image.clone());
    }
    

    // throws Throwable
    pub fn finalize(&mut self) {
        jcall!(DisposalUtil::getInstance()).disposeImage(self.image.clone());
    }
}
impl std::ops::Deref for ImageAnimation {
    type Target = IndexedAnimation;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for ImageAnimation {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

pub trait ImageAnimationDyn: crate::org::allbinary::animation::IndexedAnimation::IndexedAnimationDyn {
    fn as_ImageAnimation(&self) -> &ImageAnimation;
    fn as_ImageAnimation_mut(&mut self) -> &mut ImageAnimation;
    fn getImage(&mut self) -> Option<Rc<RefCell<Image>>> { ImageAnimation::getImage(self.as_ImageAnimation_mut()) }
    fn close(&mut self) { ImageAnimation::close(self.as_ImageAnimation_mut()) }
    fn finalize(&mut self) { ImageAnimation::finalize(self.as_ImageAnimation_mut()) }
}
impl ImageAnimationDyn for ImageAnimation {
    fn as_ImageAnimation(&self) -> &ImageAnimation { self }
    fn as_ImageAnimation_mut(&mut self) -> &mut ImageAnimation { self }
}

impl crate::org::allbinary::animation::Animation::AnimationDyn for ImageAnimation {
    fn as_Animation(&self) -> &crate::org::allbinary::animation::Animation::Animation { &self.base.base }
    fn as_Animation_mut(&mut self) -> &mut crate::org::allbinary::animation::Animation::Animation { &mut self.base.base }
    fn nextFrame(&mut self) { Self::nextFrame(self) }
    fn paintXY(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>, mut x: i32, mut y: i32) { Self::paintXY(self, graphics, x, y) }
}
impl crate::org::allbinary::animation::IndexedAnimation::IndexedAnimationDyn for ImageAnimation {
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
impl crate::org::allbinary::image::opengles::OpenGLSurfaceChangedInterface::OpenGLSurfaceChangedInterface for ImageAnimation {
    fn set(&mut self, p1: Option<Rc<RefCell<dyn crate::javax::microedition::khronos::opengles::GL::GL>>>) { crate::org::allbinary::animation::Animation::Animation::set(crate::org::allbinary::animation::Animation::AnimationDyn::as_Animation_mut(self), p1) }
}


