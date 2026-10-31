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
// use crate::javax::microedition::lcdui::NullImage::NullImage;
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

pub struct ImageArrayAnimation {
    pub base: IndexedAnimation,
    pub imageArray: Vec<Option<Rc<RefCell<Image>>>>,
    pub totalFrames: i32,
    pub circularIndexUtil: Option<Rc<RefCell<CircularIndexUtil>>>,
    pub anchor: i32,
}

impl crate::org::allbinary::animation::IndexedAnimationInterface::IndexedAnimationInterface for ImageArrayAnimation {
    fn reset(&mut self) { self.base.reset() }
    fn setFrame(&mut self, mut index: i32) { Self::setFrame(self, index) }
    fn getFrame(&mut self) -> i32 { Self::getFrame(self) }
}
impl crate::org::allbinary::animation::SequentialAnimationInterface::SequentialAnimationInterface for ImageArrayAnimation {
    fn getSize(&mut self) -> i32 { Self::getSize(self) }
    fn previousFrame(&mut self) { Self::previousFrame(self) }
}
impl crate::org::allbinary::graphics::paint::LocationPaintableInterface::LocationPaintableInterface for ImageArrayAnimation {
    fn paintXY(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>, mut x: i32, mut y: i32) { Self::paintXY(self, graphics, x, y) }
}

impl ImageArrayAnimation {
    

    pub fn new_struct(mut imageArray: Vec<Option<Rc<RefCell<Image>>>>, mut animationBehavior: Option<Rc<RefCell<dyn crate::org::allbinary::animation::AnimationBehavior::AnimationBehaviorDyn>>>) -> Self {
        let mut this = Self {
            base: IndexedAnimation::new_struct(animationBehavior.clone()),
            imageArray: Vec::new(),
            totalFrames: 0,
            circularIndexUtil: None,
            anchor: 0,
        };
        this.imageArray = crate::javax::microedition::lcdui::NullImage::NullImage_NULL_IMAGE_ARRAY.with(Clone::clone).clone();
        this.circularIndexUtil = crate::org::allbinary::util::CircularIndexUtil::CircularIndexUtil_NULL_CIRCULAR_INDEX_UTIL.with(Clone::clone).clone();
        this.anchor = unsafe { crate::org::allbinary::graphics::Anchor::Anchor_TOP_LEFT.clone() }.clone();
        // super(...) is in the struct literal above
        this.setImageArray(imageArray.clone());
        this
    }

    pub fn new(mut imageArray: Vec<Option<Rc<RefCell<Image>>>>, mut animationBehavior: Option<Rc<RefCell<dyn crate::org::allbinary::animation::AnimationBehavior::AnimationBehaviorDyn>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(imageArray, animationBehavior))))
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
    

    pub fn getImage(&mut self, mut index: i32) -> Option<Rc<RefCell<Image>>> {
        return self.imageArray[(index) as usize].clone();
    }
    

    pub fn getImageArray(&mut self) -> Vec<Option<Rc<RefCell<Image>>>> {
        return self.imageArray.clone();
    }
    

    pub fn setImageArray(&mut self, mut imageArray: Vec<Option<Rc<RefCell<Image>>>>) {
        self.imageArray = imageArray.clone();
        self.totalFrames = (imageArray.len() as i32).clone();
        self.circularIndexUtil = CircularIndexUtil::createInstance(self.totalFrames.clone());
    }
    

    pub fn paintXY(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>, mut x: i32, mut y: i32) {
        jcall!(graphics).drawImage(self.imageArray[(jcall!(self.circularIndexUtil).getIndex()) as usize].clone(), x.clone(), y.clone(), self.anchor.clone());
    }
}
impl std::ops::Deref for ImageArrayAnimation {
    type Target = IndexedAnimation;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for ImageArrayAnimation {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

pub trait ImageArrayAnimationDyn: crate::org::allbinary::animation::IndexedAnimation::IndexedAnimationDyn {
    fn as_ImageArrayAnimation(&self) -> &ImageArrayAnimation;
    fn as_ImageArrayAnimation_mut(&mut self) -> &mut ImageArrayAnimation;
    fn getImage(&mut self, mut index: i32) -> Option<Rc<RefCell<Image>>> { ImageArrayAnimation::getImage(self.as_ImageArrayAnimation_mut(), index) }
    fn getImageArray(&mut self) -> Vec<Option<Rc<RefCell<Image>>>> { ImageArrayAnimation::getImageArray(self.as_ImageArrayAnimation_mut()) }
    fn setImageArray(&mut self, mut imageArray: Vec<Option<Rc<RefCell<Image>>>>) { ImageArrayAnimation::setImageArray(self.as_ImageArrayAnimation_mut(), imageArray) }
}
impl ImageArrayAnimationDyn for ImageArrayAnimation {
    fn as_ImageArrayAnimation(&self) -> &ImageArrayAnimation { self }
    fn as_ImageArrayAnimation_mut(&mut self) -> &mut ImageArrayAnimation { self }
}

impl crate::org::allbinary::animation::Animation::AnimationDyn for ImageArrayAnimation {
    fn as_Animation(&self) -> &crate::org::allbinary::animation::Animation::Animation { &self.base.base }
    fn as_Animation_mut(&mut self) -> &mut crate::org::allbinary::animation::Animation::Animation { &mut self.base.base }
    fn nextFrame(&mut self) { Self::nextFrame(self) }
    fn paintXY(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>, mut x: i32, mut y: i32) { Self::paintXY(self, graphics, x, y) }
}
impl crate::org::allbinary::animation::IndexedAnimation::IndexedAnimationDyn for ImageArrayAnimation {
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
impl crate::org::allbinary::image::opengles::OpenGLSurfaceChangedInterface::OpenGLSurfaceChangedInterface for ImageArrayAnimation {
    fn set(&mut self, p1: Option<Rc<RefCell<dyn crate::javax::microedition::khronos::opengles::GL::GL>>>) { crate::org::allbinary::animation::Animation::Animation::set(crate::org::allbinary::animation::Animation::AnimationDyn::as_Animation_mut(self), p1) }
}


