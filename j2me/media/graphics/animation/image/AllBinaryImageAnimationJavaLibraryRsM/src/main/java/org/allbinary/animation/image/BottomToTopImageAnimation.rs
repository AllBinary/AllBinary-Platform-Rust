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
// use crate::javax::microedition::lcdui::game::Sprite::Sprite;
use crate::org::allbinary::animation::AnimationBehavior::AnimationBehavior;
use crate::org::allbinary::graphics::Anchor::Anchor;
use crate::org::allbinary::logic::math::PrimitiveIntUtil::PrimitiveIntUtil;
use super::ImageSegmentAnimation::ImageSegmentAnimation;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct BottomToTopImageAnimation {
    pub base: ImageSegmentAnimation,
    pub startHeight: i32,
    pub currentHeight: i32,
    pub anchor: i32,
}

impl crate::org::allbinary::animation::IndexedAnimationInterface::IndexedAnimationInterface for BottomToTopImageAnimation {
    fn reset(&mut self) { self.base.reset() }
    fn setFrame(&mut self, mut index: i32) { Self::setFrame(self, index) }
    fn getFrame(&mut self) -> i32 { self.base.getFrame() }
}
impl crate::org::allbinary::animation::SequentialAnimationInterface::SequentialAnimationInterface for BottomToTopImageAnimation {
    fn getSize(&mut self) -> i32 { self.base.getSize() }
    fn previousFrame(&mut self) { Self::previousFrame(self) }
}
impl crate::org::allbinary::graphics::paint::LocationPaintableInterface::LocationPaintableInterface for BottomToTopImageAnimation {
    fn paintXY(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>, mut x: i32, mut y: i32) { Self::paintXY(self, graphics, x, y) }
}

impl BottomToTopImageAnimation {
    

    pub fn new_struct(mut image: Option<Rc<RefCell<Image>>>, mut animationBehavior: Option<Rc<RefCell<dyn crate::org::allbinary::animation::AnimationBehavior::AnimationBehaviorDyn>>>) -> Self {
        let mut this = Self {
            base: ImageSegmentAnimation::new_struct(image.clone(), PrimitiveIntUtil::getArrayInstance(), animationBehavior.clone()),
            startHeight: 0,
            currentHeight: 0,
            anchor: 0,
        };
        this.anchor = unsafe { crate::org::allbinary::graphics::Anchor::Anchor_TOP_LEFT.clone() }.clone();
        // super(...) is in the struct literal above
        { let java_arguments = (this.getSize() - 1,); this.setFrame(java_arguments.0) };
        this
    }

    pub fn new(mut image: Option<Rc<RefCell<Image>>>, mut animationBehavior: Option<Rc<RefCell<dyn crate::org::allbinary::animation::AnimationBehavior::AnimationBehaviorDyn>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(image, animationBehavior))))
    }
    

    pub fn nextFrame(&mut self) {
        self.base.previousFrame();
        self.update();
    }
    

    pub fn previousFrame(&mut self) {
        self.base.nextFrame();
        self.update();
    }
    

    pub fn setFrame(&mut self, mut index: i32) {
        self.base.setFrame(index.clone());
        self.update();
    }
    

    pub fn update(&mut self) {
        self.startHeight = self.getDrawHeight() * self.getFrame() / self.getSize();
        self.currentHeight = self.getDrawHeight() - self.startHeight;
    }
    

    pub fn paintXY(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>, mut x: i32, mut y: i32) {
        jcall!(graphics).drawRegion(self.getImage(), self.getStartX(), self.startHeight.clone(), self.getDrawWidth(), self.currentHeight.clone(), unsafe { crate::javax::microedition::lcdui::game::Sprite::Sprite_TRANS_NONE.clone() }.clone(), x.clone(), y + self.startHeight, self.anchor.clone());
    }
}
impl std::ops::Deref for BottomToTopImageAnimation {
    type Target = ImageSegmentAnimation;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for BottomToTopImageAnimation {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::animation::image::ImageAnimation::ImageAnimationDyn for BottomToTopImageAnimation {
    fn as_ImageAnimation(&self) -> &crate::org::allbinary::animation::image::ImageAnimation::ImageAnimation { &self.base.base }
    fn as_ImageAnimation_mut(&mut self) -> &mut crate::org::allbinary::animation::image::ImageAnimation::ImageAnimation { &mut self.base.base }
}
impl crate::org::allbinary::animation::image::ImageSegmentAnimation::ImageSegmentAnimationDyn for BottomToTopImageAnimation {
    fn as_ImageSegmentAnimation(&self) -> &crate::org::allbinary::animation::image::ImageSegmentAnimation::ImageSegmentAnimation { &self.base }
    fn as_ImageSegmentAnimation_mut(&mut self) -> &mut crate::org::allbinary::animation::image::ImageSegmentAnimation::ImageSegmentAnimation { &mut self.base }
}
impl crate::org::allbinary::animation::Animation::AnimationDyn for BottomToTopImageAnimation {
    fn as_Animation(&self) -> &crate::org::allbinary::animation::Animation::Animation { &self.base.base.base.base }
    fn as_Animation_mut(&mut self) -> &mut crate::org::allbinary::animation::Animation::Animation { &mut self.base.base.base.base }
    fn nextFrame(&mut self) { Self::nextFrame(self) }
    fn paintXY(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>, mut x: i32, mut y: i32) { Self::paintXY(self, graphics, x, y) }
}
impl crate::org::allbinary::animation::IndexedAnimation::IndexedAnimationDyn for BottomToTopImageAnimation {
    fn as_IndexedAnimation(&self) -> &crate::org::allbinary::animation::IndexedAnimation::IndexedAnimation { &self.base.base.base }
    fn as_IndexedAnimation_mut(&mut self) -> &mut crate::org::allbinary::animation::IndexedAnimation::IndexedAnimation { &mut self.base.base.base }
    fn setFrame(&mut self, mut index: i32) { Self::setFrame(self, index) }
    fn getFrame(&mut self) -> i32 { crate::org::allbinary::animation::image::ImageSegmentAnimation::ImageSegmentAnimation::getFrame(crate::org::allbinary::animation::image::ImageSegmentAnimation::ImageSegmentAnimationDyn::as_ImageSegmentAnimation_mut(self)) }
    fn getAnimationSize(&mut self) -> i32 { crate::org::allbinary::animation::image::ImageAnimation::ImageAnimation::getAnimationSize(crate::org::allbinary::animation::image::ImageAnimation::ImageAnimationDyn::as_ImageAnimation_mut(self)) }
    fn getSize(&mut self) -> i32 { crate::org::allbinary::animation::image::ImageSegmentAnimation::ImageSegmentAnimation::getSize(crate::org::allbinary::animation::image::ImageSegmentAnimation::ImageSegmentAnimationDyn::as_ImageSegmentAnimation_mut(self)) }
    fn previousFrame(&mut self) { Self::previousFrame(self) }
    fn setSequence(&mut self, p1: Vec<i32>) { crate::org::allbinary::animation::image::ImageSegmentAnimation::ImageSegmentAnimation::setSequence(crate::org::allbinary::animation::image::ImageSegmentAnimation::ImageSegmentAnimationDyn::as_ImageSegmentAnimation_mut(self), p1) }
    fn getSequence(&mut self) -> Vec<i32> { crate::org::allbinary::animation::image::ImageSegmentAnimation::ImageSegmentAnimation::getSequence(crate::org::allbinary::animation::image::ImageSegmentAnimation::ImageSegmentAnimationDyn::as_ImageSegmentAnimation_mut(self)) }
}
impl crate::org::allbinary::image::opengles::OpenGLSurfaceChangedInterface::OpenGLSurfaceChangedInterface for BottomToTopImageAnimation {
    fn set(&mut self, p1: Option<Rc<RefCell<dyn crate::javax::microedition::khronos::opengles::GL::GL>>>) { crate::org::allbinary::animation::Animation::Animation::set(crate::org::allbinary::animation::Animation::AnimationDyn::as_Animation_mut(self), p1) }
}


