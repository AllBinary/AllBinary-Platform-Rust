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
use crate::javax::microedition::lcdui::Image::Image;
use crate::javax::microedition::lcdui::Graphics::Graphics;
use crate::org::allbinary::math::AngleInfo::AngleInfo;
use crate::org::allbinary::animation::AnimationBehavior::AnimationBehavior;
use crate::org::allbinary::graphics::color::BasicColor::BasicColor;
use crate::org::allbinary::media::image::AndroidImageUtil::AndroidImageUtil;
use crate::org::allbinary::media::image::ImageCopyUtil::ImageCopyUtil;
use crate::org::allbinary::media::image::ImageModifierUtil::ImageModifierUtil;
use crate::java::lang::Exception::*;
use super::ImageBaseRotationAnimation::ImageBaseRotationAnimation;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct AllBinaryNoFlickerAndroidImageRotationAnimation {
    pub base: ImageBaseRotationAnimation,
    pub imageModifierUtil: Option<Rc<RefCell<ImageModifierUtil>>>,
    pub androidImageUtil: Option<Rc<RefCell<AndroidImageUtil>>>,
    pub matrix: Option<Rc<RefCell<Matrix>>>,
    pub halfWidth: i32,
    pub halfHeight: i32,
    pub increment: i16,
    pub originalImage: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Image::ImageDyn>>>,
    pub twoImages: Vec<Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Image::ImageDyn>>>>,
    pub imageToShow: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Image::ImageDyn>>>,
    pub bufferedImageIndex: i32,
}

impl crate::org::allbinary::animation::AnimationInterface::AnimationInterface for AllBinaryNoFlickerAndroidImageRotationAnimation {
    fn nextFrame(&mut self) { crate::org::allbinary::animation::Animation::Animation::nextFrame(crate::org::allbinary::animation::Animation::AnimationDyn::as_Animation_mut(self)) }
    fn paintThreedXYZ(&mut self, p1: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>, p2: i32, p3: i32, p4: i32) { crate::org::allbinary::animation::Animation::Animation::paintThreedXYZ(crate::org::allbinary::animation::Animation::AnimationDyn::as_Animation_mut(self), p1, p2, p3, p4) }
}
impl crate::org::allbinary::graphics::paint::LocationPaintableInterface::LocationPaintableInterface for AllBinaryNoFlickerAndroidImageRotationAnimation {
    fn paintXY(&mut self, mut graphics: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>, mut x: i32, mut y: i32) { Self::paintXY(self, graphics, x, y) }
}
impl crate::org::allbinary::direction::DirectionalInterface::DirectionalInterface for AllBinaryNoFlickerAndroidImageRotationAnimation {
    fn setFrameByDirection(&mut self, p1: Option<Rc<RefCell<crate::org::allbinary::direction::Direction::Direction>>>) { crate::org::allbinary::animation::RotationAnimation::RotationAnimation::setFrameByDirection(crate::org::allbinary::animation::RotationAnimation::RotationAnimationDyn::as_RotationAnimation_mut(self), p1) }
    fn setFrameToAngle(&mut self, p1: Option<Rc<RefCell<dyn crate::org::allbinary::math::Angle::AngleDyn>>>) { crate::org::allbinary::animation::RotationAnimation::RotationAnimation::setFrameToAngle(crate::org::allbinary::animation::RotationAnimation::RotationAnimationDyn::as_RotationAnimation_mut(self), p1) }
    fn adjustFrameToAngle(&mut self, p1: Option<Rc<RefCell<dyn crate::org::allbinary::math::Angle::AngleDyn>>>) { crate::org::allbinary::animation::RotationAnimation::RotationAnimation::adjustFrameToAngle(crate::org::allbinary::animation::RotationAnimation::RotationAnimationDyn::as_RotationAnimation_mut(self), p1) }
    fn adjustFrame(&mut self, p1: i16) { crate::org::allbinary::animation::RotationAnimation::RotationAnimation::adjustFrame(crate::org::allbinary::animation::RotationAnimation::RotationAnimationDyn::as_RotationAnimation_mut(self), p1) }
}

impl AllBinaryNoFlickerAndroidImageRotationAnimation {
    

    pub fn new_struct(mut originalImage: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Image::ImageDyn>>>, mut image: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Image::ImageDyn>>>, mut angleInfo: Option<Rc<RefCell<AngleInfo>>>, mut totalAngle: i16, mut animationBehavior: Option<Rc<RefCell<dyn crate::org::allbinary::animation::AnimationBehavior::AnimationBehaviorDyn>>>) -> Self {
        let mut this = Self {
            base: ImageBaseRotationAnimation::new_struct(image.clone(), angleInfo.clone(), totalAngle.clone(), animationBehavior.clone()),
            imageModifierUtil: None,
            androidImageUtil: None,
            matrix: None,
            halfWidth: 0,
            halfHeight: 0,
            increment: 0,
            originalImage: None,
            twoImages: Vec::new(),
            imageToShow: None,
            bufferedImageIndex: 0,
        };
        this.imageModifierUtil = ImageModifierUtil::getInstanceOrCreate();
        this.androidImageUtil = AndroidImageUtil::getInstance();
        this.matrix = Matrix::new();
        this.twoImages = vec![None; (2) as usize];
        // super(...) is in the struct literal above
        this.originalImage = originalImage.clone();
        this.halfWidth = { let java_value = ((jcall!(image).getWidth() >> 1)).clone(); java_value };
        this.halfHeight = { let java_value = ((jcall!(image).getHeight() >> 1)).clone(); java_value };
        this.increment = { let java_value = ((jcall!(jcall!(this.angleInfo).getAngleIncrementInfo()).getAngleIncrement())).clone(); java_value };
        this.imageToShow = image.clone();
        this.twoImages[(0) as usize] = image.clone();
        this.twoImages[(1) as usize] = jcall!(ImageCopyUtil::getInstance()).createImage(image.clone());
        this
    }

    pub fn new(mut originalImage: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Image::ImageDyn>>>, mut image: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Image::ImageDyn>>>, mut angleInfo: Option<Rc<RefCell<AngleInfo>>>, mut totalAngle: i16, mut animationBehavior: Option<Rc<RefCell<dyn crate::org::allbinary::animation::AnimationBehavior::AnimationBehaviorDyn>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(originalImage, image, angleInfo, totalAngle, animationBehavior))))
    }
    

    pub fn setBasicColorP(&mut self, mut basicColor: Option<Rc<RefCell<BasicColor>>>) {
        let mut changed: bool = false;
        if self.getBasicColorP().is_none() || { let java_value = jcall!(self.getBasicColorP()).intValue(); java_value } != { let java_value = jcall!(basicColor).intValue(); java_value } {
            jcall!(self.imageModifierUtil).setColor(self.originalImage.clone(), self.imageToShow.clone(), 0, basicColor.clone());
            changed = true;
        }
        { let java_arguments = (basicColor.clone(),); self.base.setBasicColorP(java_arguments.0) };
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
        { let java_arguments = (alpha.clone(),); self.base.setAlpha(java_arguments.0) };
        jcall!(self.imageModifierUtil).setAlpha(self.originalImage.clone(), self.imageToShow.clone(), 0, self.alphaP.clone());
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
        jcall!(self.androidImageUtil).rotate(self.twoImages[(self.bufferedImageIndex) as usize].clone(), self.originalImage.clone(), self.matrix.clone(), { let java_field = jcall!(self.imageModifierUtil).paint.clone(); java_field }.clone());
        self.swap();
    }
    

    pub fn setFrame(&mut self, mut index: i32) {
        let mut currentFrame: i32 = jcall!(self.circularIndexUtil).getIndex();
        jcall!(self.circularIndexUtil).setIndex(index.clone());
        let mut newFrame: i32 = jcall!(self.circularIndexUtil).getIndex();
        jcall!(self.angleInfo).adjustAngle(newFrame.clone());
        jcall!(self.matrix).setRotate((((newFrame - currentFrame)) as f32) * ((self.increment) as f32), ((self.halfWidth) as f32), ((self.halfHeight) as f32));
        self.updateImage();
    }
    

    pub fn swap(&mut self) {
        self.imageToShow = { let java_value = (self.twoImages[(self.bufferedImageIndex) as usize]).clone(); java_value }.clone();
        if self.bufferedImageIndex == 0 {
            self.bufferedImageIndex = 1;
        } else {
            self.bufferedImageIndex = 0;
        }
    }
    

    pub fn paintXY(&mut self, mut graphics: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>, mut x: i32, mut y: i32) {
        jcall!(graphics).drawImage(self.imageToShow.clone(), x.clone(), y.clone(), self.anchor.clone());
    }
}
impl std::ops::Deref for AllBinaryNoFlickerAndroidImageRotationAnimation {
    type Target = ImageBaseRotationAnimation;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for AllBinaryNoFlickerAndroidImageRotationAnimation {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::animation::image::ImageBaseRotationAnimation::ImageBaseRotationAnimationDyn for AllBinaryNoFlickerAndroidImageRotationAnimation {
    fn as_ImageBaseRotationAnimation(&self) -> &crate::org::allbinary::animation::image::ImageBaseRotationAnimation::ImageBaseRotationAnimation { &self.base }
    fn as_ImageBaseRotationAnimation_mut(&mut self) -> &mut crate::org::allbinary::animation::image::ImageBaseRotationAnimation::ImageBaseRotationAnimation { &mut self.base }
}
impl crate::org::allbinary::animation::Animation::AnimationDyn for AllBinaryNoFlickerAndroidImageRotationAnimation {
    fn as_Animation(&self) -> &crate::org::allbinary::animation::Animation::Animation { &self.base.base.base.base }
    fn as_Animation_mut(&mut self) -> &mut crate::org::allbinary::animation::Animation::Animation { &mut self.base.base.base.base }
    fn setAlpha(&mut self, mut alpha: i32) { Self::setAlpha(self, alpha) }
    fn paintXY(&mut self, mut graphics: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>, mut x: i32, mut y: i32) { Self::paintXY(self, graphics, x, y) }
    fn setBasicColorP(&mut self, mut basicColor: Option<Rc<RefCell<BasicColor>>>) { Self::setBasicColorP(self, basicColor) }
    fn changeBasicColor(&mut self, mut basicColor: Option<Rc<RefCell<BasicColor>>>) { Self::changeBasicColor(self, basicColor) }
}
impl crate::org::allbinary::animation::IndexedAnimation::IndexedAnimationDyn for AllBinaryNoFlickerAndroidImageRotationAnimation {
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
impl crate::org::allbinary::animation::RotationAnimation::RotationAnimationDyn for AllBinaryNoFlickerAndroidImageRotationAnimation {
    fn as_RotationAnimation(&self) -> &crate::org::allbinary::animation::RotationAnimation::RotationAnimation { &self.base.base }
    fn as_RotationAnimation_mut(&mut self) -> &mut crate::org::allbinary::animation::RotationAnimation::RotationAnimation { &mut self.base.base }
    fn nextRotation(&mut self) { Self::nextRotation(self) }
    fn previousRotation(&mut self) { Self::previousRotation(self) }
}
impl crate::org::allbinary::image::opengles::OpenGLSurfaceChangedInterface::OpenGLSurfaceChangedInterface for AllBinaryNoFlickerAndroidImageRotationAnimation {
    fn set(&mut self, p1: Option<Rc<RefCell<dyn crate::javax::microedition::khronos::opengles::GL::GL>>>) { crate::org::allbinary::animation::Animation::Animation::set(crate::org::allbinary::animation::Animation::AnimationDyn::as_Animation_mut(self), p1) }
}


