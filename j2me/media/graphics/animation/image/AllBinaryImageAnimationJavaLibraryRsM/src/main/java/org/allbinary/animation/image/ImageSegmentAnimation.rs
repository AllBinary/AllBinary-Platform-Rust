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
use crate::org::allbinary::util::CircularIndexUtil::CircularIndexUtil;
use super::ImageAnimation::ImageAnimation;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct ImageSegmentAnimation {
    pub base: ImageAnimation,
    pub startX: i32,
    pub startY: i32,
    pub drawWidth: i32,
    pub drawHeight: i32,
    pub circularIndexUtil: Option<Rc<RefCell<CircularIndexUtil>>>,
    pub sequenceArray: Vec<i32>,
}

impl crate::org::allbinary::animation::IndexedAnimationInterface::IndexedAnimationInterface for ImageSegmentAnimation {
    fn reset(&mut self) { self.base.reset() }
    fn setFrame(&mut self, mut index: i32) { Self::setFrame(self, index) }
    fn getFrame(&mut self) -> i32 { Self::getFrame(self) }
}
impl crate::org::allbinary::animation::SequentialAnimationInterface::SequentialAnimationInterface for ImageSegmentAnimation {
    fn getSize(&mut self) -> i32 { Self::getSize(self) }
    fn previousFrame(&mut self) { Self::previousFrame(self) }
}
impl crate::org::allbinary::graphics::paint::LocationPaintableInterface::LocationPaintableInterface for ImageSegmentAnimation {
    fn paintXY(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>, mut x: i32, mut y: i32) { Self::paintXY(self, graphics, x, y) }
}

impl ImageSegmentAnimation {
    

    pub fn new_struct(mut image: Option<Rc<RefCell<Image>>>, mut sequenceArray: Vec<i32>, mut animationBehavior: Option<Rc<RefCell<dyn crate::org::allbinary::animation::AnimationBehavior::AnimationBehaviorDyn>>>) -> Self {
        let mut this = Self {
            base: ImageAnimation::new_struct(image.clone(), animationBehavior.clone()),
            startX: 0,
            startY: 0,
            drawWidth: 0,
            drawHeight: 0,
            circularIndexUtil: None,
            sequenceArray: Vec::new(),
        };
        // super(...) is in the struct literal above
        this.startX = 0;
        this.startY = 0;
        this.drawWidth = jcall!(this.getImage()).getWidth();
        this.drawHeight = jcall!(this.getImage()).getHeight();
        this.circularIndexUtil = CircularIndexUtil::createInstance(this.drawHeight.clone());
        this.sequenceArray = sequenceArray.clone();
        this
    }

    pub fn new(mut image: Option<Rc<RefCell<Image>>>, mut sequenceArray: Vec<i32>, mut animationBehavior: Option<Rc<RefCell<dyn crate::org::allbinary::animation::AnimationBehavior::AnimationBehaviorDyn>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(image, sequenceArray, animationBehavior))))
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
        return jcall!(self.circularIndexUtil).getSize();
    }
    

    pub fn setSequence(&mut self, mut sequenceArray: Vec<i32>) {
        self.sequenceArray = sequenceArray.clone();
    }
    

    pub fn getSequence(&mut self) -> Vec<i32> {
        return self.sequenceArray.clone();
    }
    

    pub fn paintXY(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>, mut x: i32, mut y: i32) {
    }
    

    pub fn getStartX(&mut self) -> i32 {
        return self.startX.clone();
    }
    

    pub fn setStartX(&mut self, mut startX: i32) {
        self.startX = startX.clone();
    }
    

    pub fn getStartY(&mut self) -> i32 {
        return self.startY.clone();
    }
    

    pub fn setStartY(&mut self, mut startY: i32) {
        self.startY = startY.clone();
    }
    

    pub fn getDrawWidth(&mut self) -> i32 {
        return self.drawWidth.clone();
    }
    

    pub fn setDrawWidth(&mut self, mut drawWidth: i32) {
        self.drawWidth = drawWidth.clone();
    }
    

    pub fn getDrawHeight(&mut self) -> i32 {
        return self.drawHeight.clone();
    }
    

    pub fn setDrawHeight(&mut self, mut drawHeight: i32) {
        self.drawHeight = drawHeight.clone();
    }
}
impl std::ops::Deref for ImageSegmentAnimation {
    type Target = ImageAnimation;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for ImageSegmentAnimation {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

pub trait ImageSegmentAnimationDyn: crate::org::allbinary::animation::image::ImageAnimation::ImageAnimationDyn {
    fn as_ImageSegmentAnimation(&self) -> &ImageSegmentAnimation;
    fn as_ImageSegmentAnimation_mut(&mut self) -> &mut ImageSegmentAnimation;
    fn getStartX(&mut self) -> i32 { ImageSegmentAnimation::getStartX(self.as_ImageSegmentAnimation_mut()) }
    fn setStartX(&mut self, mut startX: i32) { ImageSegmentAnimation::setStartX(self.as_ImageSegmentAnimation_mut(), startX) }
    fn getStartY(&mut self) -> i32 { ImageSegmentAnimation::getStartY(self.as_ImageSegmentAnimation_mut()) }
    fn setStartY(&mut self, mut startY: i32) { ImageSegmentAnimation::setStartY(self.as_ImageSegmentAnimation_mut(), startY) }
    fn getDrawWidth(&mut self) -> i32 { ImageSegmentAnimation::getDrawWidth(self.as_ImageSegmentAnimation_mut()) }
    fn setDrawWidth(&mut self, mut drawWidth: i32) { ImageSegmentAnimation::setDrawWidth(self.as_ImageSegmentAnimation_mut(), drawWidth) }
    fn getDrawHeight(&mut self) -> i32 { ImageSegmentAnimation::getDrawHeight(self.as_ImageSegmentAnimation_mut()) }
    fn setDrawHeight(&mut self, mut drawHeight: i32) { ImageSegmentAnimation::setDrawHeight(self.as_ImageSegmentAnimation_mut(), drawHeight) }
}
impl ImageSegmentAnimationDyn for ImageSegmentAnimation {
    fn as_ImageSegmentAnimation(&self) -> &ImageSegmentAnimation { self }
    fn as_ImageSegmentAnimation_mut(&mut self) -> &mut ImageSegmentAnimation { self }
}

impl crate::org::allbinary::animation::image::ImageAnimation::ImageAnimationDyn for ImageSegmentAnimation {
    fn as_ImageAnimation(&self) -> &crate::org::allbinary::animation::image::ImageAnimation::ImageAnimation { &self.base }
    fn as_ImageAnimation_mut(&mut self) -> &mut crate::org::allbinary::animation::image::ImageAnimation::ImageAnimation { &mut self.base }
}
impl crate::org::allbinary::animation::Animation::AnimationDyn for ImageSegmentAnimation {
    fn as_Animation(&self) -> &crate::org::allbinary::animation::Animation::Animation { &self.base.base.base }
    fn as_Animation_mut(&mut self) -> &mut crate::org::allbinary::animation::Animation::Animation { &mut self.base.base.base }
    fn nextFrame(&mut self) { Self::nextFrame(self) }
    fn paintXY(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>, mut x: i32, mut y: i32) { Self::paintXY(self, graphics, x, y) }
}
impl crate::org::allbinary::animation::IndexedAnimation::IndexedAnimationDyn for ImageSegmentAnimation {
    fn as_IndexedAnimation(&self) -> &crate::org::allbinary::animation::IndexedAnimation::IndexedAnimation { &self.base.base }
    fn as_IndexedAnimation_mut(&mut self) -> &mut crate::org::allbinary::animation::IndexedAnimation::IndexedAnimation { &mut self.base.base }
    fn setFrame(&mut self, mut index: i32) { Self::setFrame(self, index) }
    fn getFrame(&mut self) -> i32 { Self::getFrame(self) }
    fn getAnimationSize(&mut self) -> i32 { crate::org::allbinary::animation::image::ImageAnimation::ImageAnimation::getAnimationSize(crate::org::allbinary::animation::image::ImageAnimation::ImageAnimationDyn::as_ImageAnimation_mut(self)) }
    fn getSize(&mut self) -> i32 { Self::getSize(self) }
    fn previousFrame(&mut self) { Self::previousFrame(self) }
    fn setSequence(&mut self, mut sequenceArray: Vec<i32>) { Self::setSequence(self, sequenceArray) }
    fn getSequence(&mut self) -> Vec<i32> { Self::getSequence(self) }
}
impl crate::org::allbinary::image::opengles::OpenGLSurfaceChangedInterface::OpenGLSurfaceChangedInterface for ImageSegmentAnimation {
    fn set(&mut self, p1: Option<Rc<RefCell<dyn crate::javax::microedition::khronos::opengles::GL::GL>>>) { crate::org::allbinary::animation::Animation::Animation::set(crate::org::allbinary::animation::Animation::AnimationDyn::as_Animation_mut(self), p1) }
}


