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
use crate::org::allbinary::DisposalUtil::DisposalUtil;
use crate::org::allbinary::animation::AnimationBehavior::AnimationBehavior;
use crate::org::allbinary::animation::RotationAnimation::RotationAnimation;
use crate::org::allbinary::graphics::Anchor::Anchor;
use crate::org::allbinary::logic::math::PrimitiveIntUtil::PrimitiveIntUtil;
use crate::org::allbinary::math::AngleInfo::AngleInfo;
use crate::org::allbinary::media::image::ImageModifierUtil::ImageModifierUtil;
use crate::org::allbinary::util::CircularIndexUtil::CircularIndexUtil;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct ImageArrayBaseRotationAnimation {
    pub base: RotationAnimation,
    pub imageModifierUtil: Option<Rc<RefCell<ImageModifierUtil>>>,
    pub originalImageArray: Vec<Option<Rc<RefCell<Image>>>>,
    pub imageArray: Vec<Option<Rc<RefCell<Image>>>>,
    pub currentImage: Option<Rc<RefCell<Image>>>,
    pub totalFrames: i32,
    pub anchor: i32,
}

impl crate::org::allbinary::animation::IndexedAnimationInterface::IndexedAnimationInterface for ImageArrayBaseRotationAnimation {
    fn reset(&mut self) { self.base.reset() }
    fn setFrame(&mut self, mut index2: i32) { Self::setFrame(self, index2) }
    fn getFrame(&mut self) -> i32 { self.base.getFrame() }
}
impl crate::org::allbinary::animation::SequentialAnimationInterface::SequentialAnimationInterface for ImageArrayBaseRotationAnimation {
    fn getSize(&mut self) -> i32 { self.base.getSize() }
    fn previousFrame(&mut self) { self.base.previousFrame() }
}
impl crate::org::allbinary::graphics::paint::LocationPaintableInterface::LocationPaintableInterface for ImageArrayBaseRotationAnimation {
    fn paintXY(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>, mut x: i32, mut y: i32) { Self::paintXY(self, graphics, x, y) }
}

impl ImageArrayBaseRotationAnimation {
    

    pub fn new_struct(mut originalImageArray: Vec<Option<Rc<RefCell<Image>>>>, mut angleInfo: Option<Rc<RefCell<AngleInfo>>>, mut animationBehavior: Option<Rc<RefCell<dyn crate::org::allbinary::animation::AnimationBehavior::AnimationBehaviorDyn>>>) -> Self {
        let mut this = Self {
            base: RotationAnimation::new_struct(angleInfo.clone(), CircularIndexUtil::createInstance(360 / jcall!(jcall!(angleInfo).getAngleIncrementInfo()).getAngleIncrement()), animationBehavior.clone()),
            imageModifierUtil: None,
            originalImageArray: Vec::new(),
            imageArray: Vec::new(),
            currentImage: None,
            totalFrames: 0,
            anchor: 0,
        };
        this.imageModifierUtil = ImageModifierUtil::getInstanceOrCreate();
        this.imageArray = crate::javax::microedition::lcdui::NullImage::NullImage_NULL_IMAGE_ARRAY.with(Clone::clone).clone();
        this.anchor = unsafe { crate::org::allbinary::graphics::Anchor::Anchor_TOP_LEFT.clone() }.clone();
        // super(...) is in the struct literal above
        this.originalImageArray = originalImageArray.clone();
        { let java_arguments = (jcall!(this.imageModifierUtil).getImageArray(originalImageArray.clone()),); this.setImageArray(java_arguments.0) };
        this.currentImage = this.imageArray[(jcall!(this.circularIndexUtil).getIndex()) as usize].clone();
        this
    }

    pub fn new(mut originalImageArray: Vec<Option<Rc<RefCell<Image>>>>, mut angleInfo: Option<Rc<RefCell<AngleInfo>>>, mut animationBehavior: Option<Rc<RefCell<dyn crate::org::allbinary::animation::AnimationBehavior::AnimationBehaviorDyn>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(originalImageArray, angleInfo, animationBehavior))))
    }
    

    // throws Exception
    pub fn getAnimationSize(&mut self) -> i32 {
        return self.getSize();
    }
    

    pub fn setAlpha(&mut self, mut alpha: i32) {
        if self.alphaP != alpha {
            self.base.setAlpha(alpha.clone());
            jcall!(self.imageModifierUtil).reset();
            let mut index: i32 = jcall!(self.circularIndexUtil).getIndex();
            jcall!(self.imageModifierUtil).setAlpha(self.originalImageArray[(index) as usize].clone(), self.imageArray[(index) as usize].clone(), index.clone(), self.alphaP.clone());
        }
    }
    

    pub fn nextRotation(&mut self) {
        self.base.nextRotation();
        let mut index: i32 = jcall!(self.circularIndexUtil).getIndex();
        jcall!(self.imageModifierUtil).setAlpha(self.originalImageArray[(index) as usize].clone(), self.imageArray[(index) as usize].clone(), index.clone(), self.alphaP.clone());
        self.currentImage = self.imageArray[(index) as usize].clone();
    }
    

    pub fn previousRotation(&mut self) {
        self.base.previousRotation();
        let mut index: i32 = jcall!(self.circularIndexUtil).getIndex();
        jcall!(self.imageModifierUtil).setAlpha(self.originalImageArray[(index) as usize].clone(), self.imageArray[(index) as usize].clone(), index.clone(), self.alphaP.clone());
        self.currentImage = self.imageArray[(index) as usize].clone();
    }
    

    pub fn setFrame(&mut self, mut index2: i32) {
        self.base.setFrame(index2.clone());
        let mut index: i32 = jcall!(self.circularIndexUtil).getIndex();
        jcall!(self.imageModifierUtil).setAlpha(self.originalImageArray[(index) as usize].clone(), self.imageArray[(index) as usize].clone(), index.clone(), self.alphaP.clone());
        self.currentImage = self.imageArray[(index) as usize].clone();
    }
    

    pub fn setSequence(&mut self, mut sequence: Vec<i32>) {
    }
    

    pub fn getSequence(&mut self) -> Vec<i32> {
        return PrimitiveIntUtil::getArrayInstance();
    }
    

    pub fn setImageArray(&mut self, mut imageArray: Vec<Option<Rc<RefCell<Image>>>>) {
        self.imageArray = imageArray.clone();
        self.totalFrames = (imageArray.len() as i32).clone();
        self.circularIndexUtil = CircularIndexUtil::createInstance(self.totalFrames.clone());
    }
    

    pub fn paintXY(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>, mut x: i32, mut y: i32) {
        jcall!(graphics).drawImage(self.currentImage.clone(), x.clone(), y.clone(), self.anchor.clone());
    }
    

    // throws Exception
    pub fn close(&mut self) {
        let mut disposalUtil: Option<Rc<RefCell<DisposalUtil>>> = DisposalUtil::getInstance();
        let mut size2: i32 = (self.imageArray.len() as i32).clone();
        for index in (0..size2) {
            jcall!(disposalUtil).disposeImage(self.imageArray[(index) as usize].clone());
        }
        let mut size: i32 = (self.originalImageArray.len() as i32).clone();
        for index in (0..size) {
            jcall!(disposalUtil).disposeImage(self.originalImageArray[(index) as usize].clone());
        }
        jcall!(disposalUtil).disposeImage(self.currentImage.clone());
    }
    

    // throws Throwable
    pub fn finalize(&mut self) {
        let mut disposalUtil: Option<Rc<RefCell<DisposalUtil>>> = DisposalUtil::getInstance();
        let mut size2: i32 = (self.imageArray.len() as i32).clone();
        for index in (0..size2) {
            jcall!(disposalUtil).disposeImage(self.imageArray[(index) as usize].clone());
        }
        let mut size: i32 = (self.originalImageArray.len() as i32).clone();
        for index in (0..size) {
            jcall!(disposalUtil).disposeImage(self.originalImageArray[(index) as usize].clone());
        }
        jcall!(disposalUtil).disposeImage(self.currentImage.clone());
    }
}
impl std::ops::Deref for ImageArrayBaseRotationAnimation {
    type Target = RotationAnimation;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for ImageArrayBaseRotationAnimation {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

pub trait ImageArrayBaseRotationAnimationDyn: crate::org::allbinary::animation::RotationAnimation::RotationAnimationDyn {
    fn as_ImageArrayBaseRotationAnimation(&self) -> &ImageArrayBaseRotationAnimation;
    fn as_ImageArrayBaseRotationAnimation_mut(&mut self) -> &mut ImageArrayBaseRotationAnimation;
    fn setImageArray(&mut self, mut imageArray: Vec<Option<Rc<RefCell<Image>>>>) { ImageArrayBaseRotationAnimation::setImageArray(self.as_ImageArrayBaseRotationAnimation_mut(), imageArray) }
    fn close(&mut self) { ImageArrayBaseRotationAnimation::close(self.as_ImageArrayBaseRotationAnimation_mut()) }
    fn finalize(&mut self) { ImageArrayBaseRotationAnimation::finalize(self.as_ImageArrayBaseRotationAnimation_mut()) }
}
impl ImageArrayBaseRotationAnimationDyn for ImageArrayBaseRotationAnimation {
    fn as_ImageArrayBaseRotationAnimation(&self) -> &ImageArrayBaseRotationAnimation { self }
    fn as_ImageArrayBaseRotationAnimation_mut(&mut self) -> &mut ImageArrayBaseRotationAnimation { self }
}

impl crate::org::allbinary::animation::Animation::AnimationDyn for ImageArrayBaseRotationAnimation {
    fn as_Animation(&self) -> &crate::org::allbinary::animation::Animation::Animation { &self.base.base.base }
    fn as_Animation_mut(&mut self) -> &mut crate::org::allbinary::animation::Animation::Animation { &mut self.base.base.base }
    fn setAlpha(&mut self, mut alpha: i32) { Self::setAlpha(self, alpha) }
    fn paintXY(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>, mut x: i32, mut y: i32) { Self::paintXY(self, graphics, x, y) }
}
impl crate::org::allbinary::animation::IndexedAnimation::IndexedAnimationDyn for ImageArrayBaseRotationAnimation {
    fn as_IndexedAnimation(&self) -> &crate::org::allbinary::animation::IndexedAnimation::IndexedAnimation { &self.base.base }
    fn as_IndexedAnimation_mut(&mut self) -> &mut crate::org::allbinary::animation::IndexedAnimation::IndexedAnimation { &mut self.base.base }
    fn setFrame(&mut self, mut index2: i32) { Self::setFrame(self, index2) }
    fn getFrame(&mut self) -> i32 { crate::org::allbinary::animation::RotationAnimation::RotationAnimation::getFrame(crate::org::allbinary::animation::RotationAnimation::RotationAnimationDyn::as_RotationAnimation_mut(self)) }
    fn getAnimationSize(&mut self) -> i32 { Self::getAnimationSize(self) }
    fn getSize(&mut self) -> i32 { crate::org::allbinary::animation::RotationAnimation::RotationAnimation::getSize(crate::org::allbinary::animation::RotationAnimation::RotationAnimationDyn::as_RotationAnimation_mut(self)) }
    fn setSequence(&mut self, mut sequence: Vec<i32>) { Self::setSequence(self, sequence) }
    fn getSequence(&mut self) -> Vec<i32> { Self::getSequence(self) }
}
impl crate::org::allbinary::animation::RotationAnimation::RotationAnimationDyn for ImageArrayBaseRotationAnimation {
    fn as_RotationAnimation(&self) -> &crate::org::allbinary::animation::RotationAnimation::RotationAnimation { &self.base }
    fn as_RotationAnimation_mut(&mut self) -> &mut crate::org::allbinary::animation::RotationAnimation::RotationAnimation { &mut self.base }
    fn nextRotation(&mut self) { Self::nextRotation(self) }
    fn previousRotation(&mut self) { Self::previousRotation(self) }
}
impl crate::org::allbinary::image::opengles::OpenGLSurfaceChangedInterface::OpenGLSurfaceChangedInterface for ImageArrayBaseRotationAnimation {
    fn set(&mut self, p1: Option<Rc<RefCell<dyn crate::javax::microedition::khronos::opengles::GL::GL>>>) { crate::org::allbinary::animation::Animation::Animation::set(crate::org::allbinary::animation::Animation::AnimationDyn::as_Animation_mut(self), p1) }
}


