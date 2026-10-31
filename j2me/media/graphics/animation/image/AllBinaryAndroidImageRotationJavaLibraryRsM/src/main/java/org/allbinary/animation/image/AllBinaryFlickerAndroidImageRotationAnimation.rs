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
use crate::android::graphics::Matrix::Matrix;
// use crate::javax::microedition::lcdui::Image::Image;
use crate::org::allbinary::animation::AnimationBehavior::AnimationBehavior;
use crate::org::allbinary::graphics::color::BasicColor::BasicColor;
use crate::org::allbinary::math::AngleInfo::AngleInfo;
use crate::org::allbinary::media::image::AndroidImageUtil::AndroidImageUtil;
use crate::org::allbinary::media::image::ImageModifierUtil::ImageModifierUtil;
use super::ImageBaseRotationAnimation::ImageBaseRotationAnimation;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct AllBinaryFlickerAndroidImageRotationAnimation {
    pub base: ImageBaseRotationAnimation,
    pub imageModifierUtil: Option<Rc<RefCell<ImageModifierUtil>>>,
    pub androidImageUtil: Option<Rc<RefCell<AndroidImageUtil>>>,
    pub matrix: Option<Rc<RefCell<Matrix>>>,
    pub halfWidth: i32,
    pub halfHeight: i32,
    pub increment: i16,
    pub originalImage: Option<Rc<RefCell<Image>>>,
}

impl crate::org::allbinary::animation::IndexedAnimationInterface::IndexedAnimationInterface for AllBinaryFlickerAndroidImageRotationAnimation {
    fn reset(&mut self) { self.base.reset() }
    fn setFrame(&mut self, mut index: i32) { Self::setFrame(self, index) }
    fn getFrame(&mut self) -> i32 { self.base.getFrame() }
}
impl crate::org::allbinary::animation::SequentialAnimationInterface::SequentialAnimationInterface for AllBinaryFlickerAndroidImageRotationAnimation {
    fn getSize(&mut self) -> i32 { self.base.getSize() }
    fn previousFrame(&mut self) { self.base.previousFrame() }
}

impl AllBinaryFlickerAndroidImageRotationAnimation {
    

    pub fn new_struct(mut originalImage: Option<Rc<RefCell<Image>>>, mut image: Option<Rc<RefCell<Image>>>, mut angleInfo: Option<Rc<RefCell<AngleInfo>>>, mut totalAngle: i16, mut animationBehavior: Option<Rc<RefCell<dyn crate::org::allbinary::animation::AnimationBehavior::AnimationBehaviorDyn>>>) -> Self {
        let mut this = Self {
            base: ImageBaseRotationAnimation::new_struct(image.clone(), angleInfo.clone(), totalAngle.clone(), animationBehavior.clone()),
            imageModifierUtil: None,
            androidImageUtil: None,
            matrix: None,
            halfWidth: 0,
            halfHeight: 0,
            increment: 0,
            originalImage: None,
        };
        this.imageModifierUtil = ImageModifierUtil::getInstanceOrCreate();
        this.androidImageUtil = AndroidImageUtil::getInstance();
        this.matrix = Matrix::new();
        // super(...) is in the struct literal above
        this.originalImage = originalImage.clone();
        this.halfWidth = (jcall!(image).getWidth() >> 1);
        this.halfHeight = (jcall!(image).getHeight() >> 1);
        this.increment = (((jcall!(jcall!(this.angleInfo).getAngleIncrementInfo()).getAngleIncrement())) as i16);
        this
    }

    pub fn new(mut originalImage: Option<Rc<RefCell<Image>>>, mut image: Option<Rc<RefCell<Image>>>, mut angleInfo: Option<Rc<RefCell<AngleInfo>>>, mut totalAngle: i16, mut animationBehavior: Option<Rc<RefCell<dyn crate::org::allbinary::animation::AnimationBehavior::AnimationBehaviorDyn>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(originalImage, image, angleInfo, totalAngle, animationBehavior))))
    }
    

    pub fn setBasicColorP(&mut self, mut basicColor: Option<Rc<RefCell<BasicColor>>>) {
        let mut changed: bool = false;
        if self.getBasicColorP().is_none() || ((self.getBasicColorP()).unwrap() as i32) != jcall!(basicColor).intValue() {
            { let java_receiver = self.imageModifierUtil.clone(); let java_arguments = (self.originalImage.clone(), self.getImage(), 0, basicColor.clone(),); jcall!(java_receiver).setColor(java_arguments.0, java_arguments.1, java_arguments.2, java_arguments.3); };
            changed = true;
        }
        self.base.setBasicColorP(basicColor.clone());
        if changed {
            jcall!(self.matrix).setRotate(0.0_f32, ((self.halfWidth) as f32), ((self.halfHeight) as f32));
            self.updateImage();
        }
    }
    

    pub fn changeBasicColor(&mut self, mut basicColor: Option<Rc<RefCell<BasicColor>>>) {
        self.setBasicColorP(basicColor.clone());
    }
    

    pub fn setAlpha(&mut self, mut alpha: i32) {
        let mut changed: bool = false;
        if self.alphaP != alpha {
            changed = true;
        }
        self.base.setAlpha(alpha.clone());
        { let java_receiver = self.imageModifierUtil.clone(); let java_arguments = (self.originalImage.clone(), self.getImage(), 0, self.alphaP.clone(),); jcall!(java_receiver).setAlpha(java_arguments.0, java_arguments.1, java_arguments.2, java_arguments.3); };
        if changed {
            jcall!(self.matrix).setRotate(0.0_f32, ((self.halfWidth) as f32), ((self.halfHeight) as f32));
            self.updateImage();
        }
    }
    

    pub fn nextRotation(&mut self) {
        self.base.nextRotation();
        jcall!(self.matrix).setRotate(((self.increment) as f32), ((self.halfWidth) as f32), ((self.halfHeight) as f32));
        self.updateImage();
    }
    

    pub fn previousRotation(&mut self) {
        self.base.previousRotation();
        jcall!(self.matrix).setRotate(((-self.increment) as f32), ((self.halfWidth) as f32), ((self.halfHeight) as f32));
        self.updateImage();
    }
    

    pub fn updateImage(&mut self) {
        { let java_receiver = self.androidImageUtil.clone(); let java_arguments = (self.getImage(), self.originalImage.clone(), self.matrix.clone(), jcall!(self.imageModifierUtil).paint.clone(),); jcall!(java_receiver).rotate(java_arguments.0, java_arguments.1, java_arguments.2, java_arguments.3); };
    }
    

    pub fn setFrame(&mut self, mut index: i32) {
        let mut currentFrame: i32 = jcall!(self.circularIndexUtil).getIndex();
        jcall!(self.circularIndexUtil).setIndex(index.clone());
        let mut newFrame: i32 = jcall!(self.circularIndexUtil).getIndex();
        jcall!(self.angleInfo).adjustAngle(newFrame.clone());
        jcall!(self.matrix).setRotate((((newFrame - currentFrame)) as f32) * ((self.increment) as f32), ((self.halfWidth) as f32), ((self.halfHeight) as f32));
        self.updateImage();
    }
}
impl std::ops::Deref for AllBinaryFlickerAndroidImageRotationAnimation {
    type Target = ImageBaseRotationAnimation;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for AllBinaryFlickerAndroidImageRotationAnimation {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

pub trait AllBinaryFlickerAndroidImageRotationAnimationDyn: crate::org::allbinary::animation::image::ImageBaseRotationAnimation::ImageBaseRotationAnimationDyn {
    fn as_AllBinaryFlickerAndroidImageRotationAnimation(&self) -> &AllBinaryFlickerAndroidImageRotationAnimation;
    fn as_AllBinaryFlickerAndroidImageRotationAnimation_mut(&mut self) -> &mut AllBinaryFlickerAndroidImageRotationAnimation;
    fn updateImage(&mut self) { AllBinaryFlickerAndroidImageRotationAnimation::updateImage(self.as_AllBinaryFlickerAndroidImageRotationAnimation_mut()) }
}
impl AllBinaryFlickerAndroidImageRotationAnimationDyn for AllBinaryFlickerAndroidImageRotationAnimation {
    fn as_AllBinaryFlickerAndroidImageRotationAnimation(&self) -> &AllBinaryFlickerAndroidImageRotationAnimation { self }
    fn as_AllBinaryFlickerAndroidImageRotationAnimation_mut(&mut self) -> &mut AllBinaryFlickerAndroidImageRotationAnimation { self }
}

impl crate::org::allbinary::animation::image::ImageBaseRotationAnimation::ImageBaseRotationAnimationDyn for AllBinaryFlickerAndroidImageRotationAnimation {
    fn as_ImageBaseRotationAnimation(&self) -> &crate::org::allbinary::animation::image::ImageBaseRotationAnimation::ImageBaseRotationAnimation { &self.base }
    fn as_ImageBaseRotationAnimation_mut(&mut self) -> &mut crate::org::allbinary::animation::image::ImageBaseRotationAnimation::ImageBaseRotationAnimation { &mut self.base }
}
impl crate::org::allbinary::animation::Animation::AnimationDyn for AllBinaryFlickerAndroidImageRotationAnimation {
    fn as_Animation(&self) -> &crate::org::allbinary::animation::Animation::Animation { &self.base.base.base.base }
    fn as_Animation_mut(&mut self) -> &mut crate::org::allbinary::animation::Animation::Animation { &mut self.base.base.base.base }
    fn setAlpha(&mut self, mut alpha: i32) { Self::setAlpha(self, alpha) }
    fn setBasicColorP(&mut self, mut basicColor: Option<Rc<RefCell<BasicColor>>>) { Self::setBasicColorP(self, basicColor) }
    fn changeBasicColor(&mut self, mut basicColor: Option<Rc<RefCell<BasicColor>>>) { Self::changeBasicColor(self, basicColor) }
}
impl crate::org::allbinary::animation::IndexedAnimation::IndexedAnimationDyn for AllBinaryFlickerAndroidImageRotationAnimation {
    fn as_IndexedAnimation(&self) -> &crate::org::allbinary::animation::IndexedAnimation::IndexedAnimation { &self.base.base.base }
    fn as_IndexedAnimation_mut(&mut self) -> &mut crate::org::allbinary::animation::IndexedAnimation::IndexedAnimation { &mut self.base.base.base }
    fn setFrame(&mut self, mut index: i32) { Self::setFrame(self, index) }
    fn getFrame(&mut self) -> i32 { crate::org::allbinary::animation::RotationAnimation::RotationAnimation::getFrame(crate::org::allbinary::animation::RotationAnimation::RotationAnimationDyn::as_RotationAnimation_mut(self)) }
    fn getAnimationSize(&mut self) -> i32 { crate::org::allbinary::animation::image::ImageBaseRotationAnimation::ImageBaseRotationAnimation::getAnimationSize(crate::org::allbinary::animation::image::ImageBaseRotationAnimation::ImageBaseRotationAnimationDyn::as_ImageBaseRotationAnimation_mut(self)) }
    fn getSize(&mut self) -> i32 { crate::org::allbinary::animation::RotationAnimation::RotationAnimation::getSize(crate::org::allbinary::animation::RotationAnimation::RotationAnimationDyn::as_RotationAnimation_mut(self)) }
    fn setSequence(&mut self, p1: Vec<i32>) { crate::org::allbinary::animation::image::ImageBaseRotationAnimation::ImageBaseRotationAnimation::setSequence(crate::org::allbinary::animation::image::ImageBaseRotationAnimation::ImageBaseRotationAnimationDyn::as_ImageBaseRotationAnimation_mut(self), p1) }
    fn getSequence(&mut self) -> Vec<i32> { crate::org::allbinary::animation::image::ImageBaseRotationAnimation::ImageBaseRotationAnimation::getSequence(crate::org::allbinary::animation::image::ImageBaseRotationAnimation::ImageBaseRotationAnimationDyn::as_ImageBaseRotationAnimation_mut(self)) }
    fn getWidth(&mut self) -> i32 { crate::org::allbinary::animation::image::ImageBaseRotationAnimation::ImageBaseRotationAnimation::getWidth(crate::org::allbinary::animation::image::ImageBaseRotationAnimation::ImageBaseRotationAnimationDyn::as_ImageBaseRotationAnimation_mut(self)) }
    fn getHeight(&mut self) -> i32 { crate::org::allbinary::animation::image::ImageBaseRotationAnimation::ImageBaseRotationAnimation::getHeight(crate::org::allbinary::animation::image::ImageBaseRotationAnimation::ImageBaseRotationAnimationDyn::as_ImageBaseRotationAnimation_mut(self)) }
}
impl crate::org::allbinary::animation::RotationAnimation::RotationAnimationDyn for AllBinaryFlickerAndroidImageRotationAnimation {
    fn as_RotationAnimation(&self) -> &crate::org::allbinary::animation::RotationAnimation::RotationAnimation { &self.base.base }
    fn as_RotationAnimation_mut(&mut self) -> &mut crate::org::allbinary::animation::RotationAnimation::RotationAnimation { &mut self.base.base }
    fn nextRotation(&mut self) { Self::nextRotation(self) }
    fn previousRotation(&mut self) { Self::previousRotation(self) }
}
impl crate::org::allbinary::image::opengles::OpenGLSurfaceChangedInterface::OpenGLSurfaceChangedInterface for AllBinaryFlickerAndroidImageRotationAnimation {
    fn set(&mut self, p1: Option<Rc<RefCell<dyn crate::javax::microedition::khronos::opengles::GL::GL>>>) { crate::org::allbinary::animation::Animation::Animation::set(crate::org::allbinary::animation::Animation::AnimationDyn::as_Animation_mut(self), p1) }
}


