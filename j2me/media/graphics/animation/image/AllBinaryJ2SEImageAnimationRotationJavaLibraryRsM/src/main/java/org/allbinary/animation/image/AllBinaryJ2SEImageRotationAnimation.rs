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
use crate::org::allbinary::graphics::color::BasicColor::BasicColor;
use crate::org::allbinary::math::AngleInfo::AngleInfo;
use crate::org::allbinary::media::image::ImageCopyUtil::ImageCopyUtil;
use crate::org::allbinary::media::image::ImageModifierUtil::ImageModifierUtil;
use crate::org::allbinary::media::image::ImageRotationUtil::ImageRotationUtil;
use super::ImageBaseRotationAnimation::ImageBaseRotationAnimation;
use super::ModifierBaseProcessor::ModifierBaseProcessor;
use super::ScaleBaseProcessor::ScaleBaseProcessor;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct AllBinaryJ2SEImageRotationAnimation {
    pub base: ImageBaseRotationAnimation,
    pub imageRotationUtil: Option<Rc<RefCell<ImageRotationUtil>>>,
    pub imageModifierUtil: Option<Rc<RefCell<ImageModifierUtil>>>,
    pub realOriginalImage: Option<Rc<RefCell<Image>>>,
    pub originalImageArray: Vec<Option<Rc<RefCell<Image>>>>,
    pub twoImages: Vec<Option<Rc<RefCell<Image>>>>,
    pub imageToShow: Option<Rc<RefCell<Image>>>,
    pub bufferedImageIndex: i32,
    pub lastScaleX: f32,
    pub lastScaleY: f32,
    pub scaleX: f32,
    pub scaleY: f32,
    pub maxScaleX: f32,
    pub maxScaleY: f32,
    pub alphaProcessor: Option<Rc<RefCell<dyn crate::org::allbinary::animation::image::ModifierBaseProcessor::ModifierBaseProcessorDyn>>>,
    pub setColorProcessor: Option<Rc<RefCell<dyn crate::org::allbinary::animation::image::ModifierBaseProcessor::ModifierBaseProcessorDyn>>>,
    pub changeColorProcessor: Option<Rc<RefCell<dyn crate::org::allbinary::animation::image::ModifierBaseProcessor::ModifierBaseProcessorDyn>>>,
    pub scaleProcessor: Option<Rc<RefCell<dyn crate::org::allbinary::animation::image::ScaleBaseProcessor::ScaleBaseProcessorDyn>>>,
}

impl crate::org::allbinary::animation::IndexedAnimationInterface::IndexedAnimationInterface for AllBinaryJ2SEImageRotationAnimation {
    fn reset(&mut self) { self.base.reset() }
    fn setFrame(&mut self, mut index: i32) { Self::setFrame(self, index) }
    fn getFrame(&mut self) -> i32 { self.base.getFrame() }
}
impl crate::org::allbinary::animation::SequentialAnimationInterface::SequentialAnimationInterface for AllBinaryJ2SEImageRotationAnimation {
    fn getSize(&mut self) -> i32 { self.base.getSize() }
    fn previousFrame(&mut self) { self.base.previousFrame() }
}
impl crate::org::allbinary::graphics::paint::LocationPaintableInterface::LocationPaintableInterface for AllBinaryJ2SEImageRotationAnimation {
    fn paintXY(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>, mut x: i32, mut y: i32) { Self::paintXY(self, graphics, x, y) }
}

impl AllBinaryJ2SEImageRotationAnimation {
    

    pub fn new_struct(mut originalImage: Option<Rc<RefCell<Image>>>, mut image: Option<Rc<RefCell<Image>>>, mut angleInfo: Option<Rc<RefCell<AngleInfo>>>, mut totalAngle: i16, mut animationBehavior: Option<Rc<RefCell<dyn crate::org::allbinary::animation::AnimationBehavior::AnimationBehaviorDyn>>>) -> Self {
        let mut this = Self {
            base: ImageBaseRotationAnimation::new_struct(image.clone(), angleInfo.clone(), totalAngle.clone(), animationBehavior.clone()),
            imageRotationUtil: None,
            imageModifierUtil: None,
            realOriginalImage: None,
            originalImageArray: Vec::new(),
            twoImages: Vec::new(),
            imageToShow: None,
            bufferedImageIndex: 0,
            lastScaleX: 0.0_f32,
            lastScaleY: 0.0_f32,
            scaleX: 0.0_f32,
            scaleY: 0.0_f32,
            maxScaleX: 0.0_f32,
            maxScaleY: 0.0_f32,
            alphaProcessor: None,
            setColorProcessor: None,
            changeColorProcessor: None,
            scaleProcessor: None,
        };
        this.imageRotationUtil = ImageRotationUtil::getInstance();
        this.imageModifierUtil = ImageModifierUtil::getInstanceOrCreate();
        this.originalImageArray = vec![None; (1) as usize];
        this.twoImages = vec![None; (2) as usize];
        this.lastScaleX = 1.0_f32;
        this.lastScaleY = 1.0_f32;
        this.scaleX = 1.0_f32;
        this.scaleY = 1.0_f32;
        this.alphaProcessor = ModifierBaseProcessor::getInstance();
        this.setColorProcessor = ModifierBaseProcessor::getInstance();
        this.changeColorProcessor = ModifierBaseProcessor::getInstance();
        this.scaleProcessor = ScaleProcessor::ScaleProcessor::getInstance();
        // super(...) is in the struct literal above
        this.realOriginalImage = originalImage.clone();
        this.originalImageArray[(0) as usize] = originalImage.clone();
        this.imageToShow = image.clone();
        this.twoImages[(0) as usize] = image.clone();
        this.twoImages[(1) as usize] = jcall!(ImageCopyUtil::getInstance()).createImageForRotation(image.clone());
        this
    }

    pub fn new(mut originalImage: Option<Rc<RefCell<Image>>>, mut image: Option<Rc<RefCell<Image>>>, mut angleInfo: Option<Rc<RefCell<AngleInfo>>>, mut totalAngle: i16, mut animationBehavior: Option<Rc<RefCell<dyn crate::org::allbinary::animation::AnimationBehavior::AnimationBehaviorDyn>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(originalImage, image, angleInfo, totalAngle, animationBehavior))))
    }
    

    pub fn setBasicColorP(&mut self, mut basicColor: Option<Rc<RefCell<BasicColor>>>) {
        let mut changed: bool = false;
        if self.getBasicColorP().is_none() || ((self.getBasicColorP()).unwrap() as i32) != jcall!(basicColor).intValue() {
            changed = true;
        }
        self.base.setBasicColorP(basicColor.clone());
        if changed {
            self.setColorProcessor = SetColorProcessor::SetColorProcessor::getInstance().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::animation::image::ModifierBaseProcessor::ModifierBaseProcessorDyn>>);
            self.updateImage();
        }
    }
    

    pub fn changeBasicColor(&mut self, mut basicColor: Option<Rc<RefCell<BasicColor>>>) {
        let mut changed: bool = false;
        if self.getChangeBasicColor().is_none() || ((self.getChangeBasicColor()).unwrap() as i32) != jcall!(basicColor).intValue() {
            changed = true;
        }
        self.base.changeBasicColor(basicColor.clone());
        if changed {
            self.changeColorProcessor = ChangeColorProcessor::ChangeColorProcessor::getInstance().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::animation::image::ModifierBaseProcessor::ModifierBaseProcessorDyn>>);
            self.updateImage();
        }
    }
    

    pub fn setAlpha(&mut self, mut alpha: i32) {
        let mut changed: bool = false;
        if self.alphaP != alpha {
            changed = true;
        }
        self.base.setAlpha(alpha.clone());
        if changed {
            self.alphaProcessor = AlphaProcessor::AlphaProcessor::getInstance().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::animation::image::ModifierBaseProcessor::ModifierBaseProcessorDyn>>);
            self.updateImage();
        }
    }
    

    pub fn setScale(&mut self, mut scaleX: f32, mut scaleY: f32) {
        self.scaleX = scaleX.clone();
        self.scaleY = scaleY.clone();
        if self.scaleX != self.lastScaleX || self.scaleY != self.lastScaleY {
            jcall!(self.scaleProcessor).update(self.realOriginalImage.clone(), self.originalImageArray.clone(), self.twoImages.clone(), self.bufferedImageIndex.clone(), self.scaleX.clone(), self.scaleY.clone(), self.maxScaleX.clone(), self.maxScaleY.clone());
            self.updateImage();
        }
        self.lastScaleX = self.scaleX.clone();
        self.lastScaleY = self.scaleY.clone();
    }
    

    pub fn setMaxScale(&mut self, mut maxScaleX: f32, mut maxScaleY: f32) {
        self.maxScaleX = maxScaleX.clone();
        self.maxScaleY = maxScaleY.clone();
        jcall!(self.scaleProcessor).update(self.realOriginalImage.clone(), self.originalImageArray.clone(), self.twoImages.clone(), self.bufferedImageIndex.clone(), self.scaleX.clone(), self.scaleY.clone(), self.maxScaleX.clone(), self.maxScaleY.clone());
        self.updateImage();
    }
    

    pub fn nextRotation(&mut self) {
        self.base.nextRotation();
        self.updateImage();
    }
    

    pub fn previousRotation(&mut self) {
        self.base.previousRotation();
        self.updateImage();
    }
    

    pub fn updateImage(&mut self) {
        { let java_receiver = self.imageRotationUtil.clone(); let java_arguments = (self.originalImageArray[(0) as usize].clone(), self.twoImages[(self.bufferedImageIndex) as usize].clone(), ((jcall!(self.angleInfo).getAngle()) as i32) + 90,); jcall!(java_receiver).rotateImage(java_arguments.0, java_arguments.1, java_arguments.2); };
        jcall!(self.alphaProcessor).updateAlpha(self.imageModifierUtil.clone(), crate::javax::microedition::lcdui::NullImage::NullImage_NULL_IMAGE.with(Clone::clone).clone(), self.twoImages[(self.bufferedImageIndex) as usize].clone(), 0, self.alphaP.clone());
        jcall!(self.setColorProcessor).updateColor(self.imageModifierUtil.clone(), crate::javax::microedition::lcdui::NullImage::NullImage_NULL_IMAGE.with(Clone::clone).clone(), self.twoImages[(self.bufferedImageIndex) as usize].clone(), 0, self.basicColor.clone());
        jcall!(self.changeColorProcessor).updateColor(self.imageModifierUtil.clone(), crate::javax::microedition::lcdui::NullImage::NullImage_NULL_IMAGE.with(Clone::clone).clone(), self.twoImages[(self.bufferedImageIndex) as usize].clone(), 0, self.changeBasicColorP.clone());
        self.swap();
    }
    

    pub fn setFrame(&mut self, mut index: i32) {
        self.base.setFrame(index.clone());
        self.updateImage();
    }
    

    pub fn swap(&mut self) {
        self.imageToShow = self.twoImages[(self.bufferedImageIndex) as usize].clone();
        if self.bufferedImageIndex == 0 {
            self.bufferedImageIndex = 1;
        } else {
            self.bufferedImageIndex = 0;
        }
    }
    

    pub fn paintXY(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>, mut x: i32, mut y: i32) {
        jcall!(graphics).drawImage(self.imageToShow.clone(), x.clone(), y.clone(), self.anchor.clone());
    }
    

    // throws Exception
    pub fn close(&mut self) {
        self.base.close();
        let mut disposalUtil: Option<Rc<RefCell<DisposalUtil>>> = DisposalUtil::getInstance();
        let mut size2: i32 = (self.twoImages.len() as i32).clone();
        for index in (0..size2) {
            jcall!(disposalUtil).disposeImage(self.twoImages[(index) as usize].clone());
        }
        let mut size: i32 = (self.originalImageArray.len() as i32).clone();
        for index in (0..size) {
            jcall!(disposalUtil).disposeImage(self.originalImageArray[(index) as usize].clone());
        }
        jcall!(disposalUtil).disposeImage(self.realOriginalImage.clone());
        jcall!(disposalUtil).disposeImage(self.imageToShow.clone());
    }
    

    // throws Throwable
    pub fn finalize(&mut self) {
        self.base.finalize();
        let mut disposalUtil: Option<Rc<RefCell<DisposalUtil>>> = DisposalUtil::getInstance();
        let mut size2: i32 = (self.twoImages.len() as i32).clone();
        for index in (0..size2) {
            jcall!(disposalUtil).disposeImage(self.twoImages[(index) as usize].clone());
        }
        let mut size: i32 = (self.originalImageArray.len() as i32).clone();
        for index in (0..size) {
            jcall!(disposalUtil).disposeImage(self.originalImageArray[(index) as usize].clone());
        }
        jcall!(disposalUtil).disposeImage(self.realOriginalImage.clone());
        jcall!(disposalUtil).disposeImage(self.imageToShow.clone());
    }
}
impl std::ops::Deref for AllBinaryJ2SEImageRotationAnimation {
    type Target = ImageBaseRotationAnimation;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for AllBinaryJ2SEImageRotationAnimation {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

pub trait AllBinaryJ2SEImageRotationAnimationDyn: crate::org::allbinary::animation::image::ImageBaseRotationAnimation::ImageBaseRotationAnimationDyn {
    fn as_AllBinaryJ2SEImageRotationAnimation(&self) -> &AllBinaryJ2SEImageRotationAnimation;
    fn as_AllBinaryJ2SEImageRotationAnimation_mut(&mut self) -> &mut AllBinaryJ2SEImageRotationAnimation;
    fn updateImage(&mut self) { AllBinaryJ2SEImageRotationAnimation::updateImage(self.as_AllBinaryJ2SEImageRotationAnimation_mut()) }
    fn swap(&mut self) { AllBinaryJ2SEImageRotationAnimation::swap(self.as_AllBinaryJ2SEImageRotationAnimation_mut()) }
}
impl AllBinaryJ2SEImageRotationAnimationDyn for AllBinaryJ2SEImageRotationAnimation {
    fn as_AllBinaryJ2SEImageRotationAnimation(&self) -> &AllBinaryJ2SEImageRotationAnimation { self }
    fn as_AllBinaryJ2SEImageRotationAnimation_mut(&mut self) -> &mut AllBinaryJ2SEImageRotationAnimation { self }
}

impl crate::org::allbinary::animation::image::ImageBaseRotationAnimation::ImageBaseRotationAnimationDyn for AllBinaryJ2SEImageRotationAnimation {
    fn as_ImageBaseRotationAnimation(&self) -> &crate::org::allbinary::animation::image::ImageBaseRotationAnimation::ImageBaseRotationAnimation { &self.base }
    fn as_ImageBaseRotationAnimation_mut(&mut self) -> &mut crate::org::allbinary::animation::image::ImageBaseRotationAnimation::ImageBaseRotationAnimation { &mut self.base }
    fn close(&mut self) { Self::close(self) }
    fn finalize(&mut self) { Self::finalize(self) }
}
impl crate::org::allbinary::animation::Animation::AnimationDyn for AllBinaryJ2SEImageRotationAnimation {
    fn as_Animation(&self) -> &crate::org::allbinary::animation::Animation::Animation { &self.base.base.base.base }
    fn as_Animation_mut(&mut self) -> &mut crate::org::allbinary::animation::Animation::Animation { &mut self.base.base.base.base }
    fn setAlpha(&mut self, mut alpha: i32) { Self::setAlpha(self, alpha) }
    fn paintXY(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>, mut x: i32, mut y: i32) { Self::paintXY(self, graphics, x, y) }
    fn setBasicColorP(&mut self, mut basicColor: Option<Rc<RefCell<BasicColor>>>) { Self::setBasicColorP(self, basicColor) }
    fn changeBasicColor(&mut self, mut basicColor: Option<Rc<RefCell<BasicColor>>>) { Self::changeBasicColor(self, basicColor) }
    fn setScale(&mut self, mut scaleX: f32, mut scaleY: f32) { Self::setScale(self, scaleX, scaleY) }
    fn setMaxScale(&mut self, mut maxScaleX: f32, mut maxScaleY: f32) { Self::setMaxScale(self, maxScaleX, maxScaleY) }
}
impl crate::org::allbinary::animation::IndexedAnimation::IndexedAnimationDyn for AllBinaryJ2SEImageRotationAnimation {
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
impl crate::org::allbinary::animation::RotationAnimation::RotationAnimationDyn for AllBinaryJ2SEImageRotationAnimation {
    fn as_RotationAnimation(&self) -> &crate::org::allbinary::animation::RotationAnimation::RotationAnimation { &self.base.base }
    fn as_RotationAnimation_mut(&mut self) -> &mut crate::org::allbinary::animation::RotationAnimation::RotationAnimation { &mut self.base.base }
    fn nextRotation(&mut self) { Self::nextRotation(self) }
    fn previousRotation(&mut self) { Self::previousRotation(self) }
}
impl crate::org::allbinary::image::opengles::OpenGLSurfaceChangedInterface::OpenGLSurfaceChangedInterface for AllBinaryJ2SEImageRotationAnimation {
    fn set(&mut self, p1: Option<Rc<RefCell<dyn crate::javax::microedition::khronos::opengles::GL::GL>>>) { crate::org::allbinary::animation::Animation::Animation::set(crate::org::allbinary::animation::Animation::AnimationDyn::as_Animation_mut(self), p1) }
}


