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
use crate::org::allbinary::math::AngleInfo::AngleInfo;
use crate::org::allbinary::graphics::color::BasicColor::BasicColor;
use crate::org::allbinary::media::image::ImageCopyUtil::ImageCopyUtil;
use crate::org::allbinary::media::image::ImageModifierUtil::ImageModifierUtil;
use crate::org::allbinary::media::image::ImageRotationUtil::ImageRotationUtil;
use crate::org::microemu::device::playn::PlaynMutableImage::PlaynMutableImage;
use crate::playn::core::CanvasImage::CanvasImage;
use crate::playn::core::CanvasSurface::CanvasSurface;
use super::ImageBaseRotationAnimation::ImageBaseRotationAnimation;
use super::AlphaBaseProcessor::AlphaBaseProcessor;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct AllBinaryHTMLImageRotationAnimation {
    pub base: ImageBaseRotationAnimation,
    pub imageRotationUtil: Option<Rc<RefCell<ImageRotationUtil>>>,
    pub imageModifierUtil: Option<Rc<RefCell<ImageModifierUtil>>>,
    pub originalImage: Option<Rc<RefCell<Image>>>,
    pub twoImages: Vec<Option<Rc<RefCell<Image>>>>,
    pub canvasSurfaceArray: Vec<Option<Rc<RefCell<CanvasSurface>>>>,
    pub imageToShow: Option<Rc<RefCell<Image>>>,
    pub bufferedImageIndex: i32,
    pub alphaProcessor: Option<Rc<RefCell<dyn crate::org::allbinary::animation::image::AlphaBaseProcessor::AlphaBaseProcessorDyn>>>,
}

impl crate::org::allbinary::animation::IndexedAnimationInterface::IndexedAnimationInterface for AllBinaryHTMLImageRotationAnimation {
    fn reset(&mut self) { self.base.reset() }
    fn setFrame(&mut self, mut index: i32) { Self::setFrame(self, index) }
    fn getFrame(&mut self) -> i32 { self.base.getFrame() }
}
impl crate::org::allbinary::animation::SequentialAnimationInterface::SequentialAnimationInterface for AllBinaryHTMLImageRotationAnimation {
    fn getSize(&mut self) -> i32 { self.base.getSize() }
    fn previousFrame(&mut self) { self.base.previousFrame() }
}
impl crate::org::allbinary::graphics::paint::LocationPaintableInterface::LocationPaintableInterface for AllBinaryHTMLImageRotationAnimation {
    fn paintXY(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>, mut x: i32, mut y: i32) { Self::paintXY(self, graphics, x, y) }
}

impl AllBinaryHTMLImageRotationAnimation {
    

    pub fn new_struct(mut originalImage: Option<Rc<RefCell<Image>>>, mut image: Option<Rc<RefCell<Image>>>, mut angleInfo: Option<Rc<RefCell<AngleInfo>>>, mut totalAngle: i16, mut animationBehavior: Option<Rc<RefCell<dyn crate::org::allbinary::animation::AnimationBehavior::AnimationBehaviorDyn>>>) -> Self {
        let mut this = Self {
            base: ImageBaseRotationAnimation::new_struct(image.clone(), angleInfo.clone(), totalAngle.clone(), animationBehavior.clone()),
            imageRotationUtil: None,
            imageModifierUtil: None,
            originalImage: None,
            twoImages: Vec::new(),
            canvasSurfaceArray: Vec::new(),
            imageToShow: None,
            bufferedImageIndex: 0,
            alphaProcessor: None,
        };
        this.imageRotationUtil = ImageRotationUtil::getInstance();
        this.imageModifierUtil = ImageModifierUtil::getInstanceOrCreate();
        this.twoImages = vec![None; (2) as usize];
        this.canvasSurfaceArray = vec![None; (2) as usize];
        this.alphaProcessor = AlphaBaseProcessor::getInstance();
        // super(...) is in the struct literal above
        this.originalImage = originalImage.clone();
        this.imageToShow = image.clone();
        this.twoImages[(0) as usize] = image.clone();
        this.twoImages[(1) as usize] = jcall!(ImageCopyUtil::getInstance()).createImage(image.clone());
        this.canvasSurfaceArray[(0) as usize] = this.getCanvasSurface(this.twoImages[(0) as usize].clone());
        this.canvasSurfaceArray[(1) as usize] = this.getCanvasSurface(this.twoImages[(1) as usize].clone());
        this
    }

    pub fn new(mut originalImage: Option<Rc<RefCell<Image>>>, mut image: Option<Rc<RefCell<Image>>>, mut angleInfo: Option<Rc<RefCell<AngleInfo>>>, mut totalAngle: i16, mut animationBehavior: Option<Rc<RefCell<dyn crate::org::allbinary::animation::AnimationBehavior::AnimationBehaviorDyn>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(originalImage, image, angleInfo, totalAngle, animationBehavior))))
    }
    

    pub fn getCanvasSurface(&mut self, mut image: Option<Rc<RefCell<Image>>>) -> Option<Rc<RefCell<CanvasSurface>>> {
        let mut htmlImage: Option<Rc<RefCell<PlaynMutableImage>>> = (image).clone() /* as PlaynMutableImage */;
        let mut canvasImage: Option<Rc<RefCell<CanvasImage>>> = (jcall!(htmlImage).getImage()).clone() /* as CanvasImage */;
        let mut canvasSurface: Option<Rc<RefCell<CanvasSurface>>> = jcall!(htmlImage).getCanvasSurface(canvasImage.clone());
        jcall!(canvasSurface).translate(jcall!(self.originalImage).getWidth() / 2, jcall!(self.originalImage).getHeight() / 2);
        return canvasSurface.clone();
    }
    

    pub fn setBasicColorP(&mut self, mut basicColor: Option<Rc<RefCell<BasicColor>>>) {
        let mut changed: bool = false;
        if self.getBasicColorP().is_none() || ((self.getBasicColorP()).unwrap() as i32) != jcall!(basicColor).intValue() {
            changed = true;
        }
        self.base.setBasicColorP(basicColor.clone());
        if changed {
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
            self.alphaProcessor = AlphaProcessor::AlphaProcessor::getInstance().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::animation::image::AlphaBaseProcessor::AlphaBaseProcessorDyn>>);
            self.updateImage();
        }
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
        let mut canvasSurface: Option<Rc<RefCell<CanvasSurface>>> = self.canvasSurfaceArray[(self.bufferedImageIndex) as usize].clone();
        jcall!(canvasSurface).save();
        { let java_receiver = self.imageRotationUtil.clone(); let java_arguments = (self.originalImage.clone(), self.twoImages[(self.bufferedImageIndex) as usize].clone(), canvasSurface.clone(), ((jcall!(self.angleInfo).getAngle()) as i32) + 90,); jcall!(java_receiver).rotateImageCanvasSurfaceClear(java_arguments.0, java_arguments.1, java_arguments.2, java_arguments.3); };
        jcall!(self.alphaProcessor).setAlpha(self.imageModifierUtil.clone(), self.originalImage.clone(), self.twoImages[(self.bufferedImageIndex) as usize].clone(), self.alphaP.clone());
        jcall!(self.imageRotationUtil).drawImage(self.originalImage.clone(), self.imageToShow.clone(), canvasSurface.clone());
        jcall!(canvasSurface).restore();
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
}
impl std::ops::Deref for AllBinaryHTMLImageRotationAnimation {
    type Target = ImageBaseRotationAnimation;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for AllBinaryHTMLImageRotationAnimation {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

pub trait AllBinaryHTMLImageRotationAnimationDyn: crate::org::allbinary::animation::image::ImageBaseRotationAnimation::ImageBaseRotationAnimationDyn {
    fn as_AllBinaryHTMLImageRotationAnimation(&self) -> &AllBinaryHTMLImageRotationAnimation;
    fn as_AllBinaryHTMLImageRotationAnimation_mut(&mut self) -> &mut AllBinaryHTMLImageRotationAnimation;
    fn getCanvasSurface(&mut self, mut image: Option<Rc<RefCell<Image>>>) -> Option<Rc<RefCell<CanvasSurface>>> { AllBinaryHTMLImageRotationAnimation::getCanvasSurface(self.as_AllBinaryHTMLImageRotationAnimation_mut(), image) }
    fn updateImage(&mut self) { AllBinaryHTMLImageRotationAnimation::updateImage(self.as_AllBinaryHTMLImageRotationAnimation_mut()) }
    fn swap(&mut self) { AllBinaryHTMLImageRotationAnimation::swap(self.as_AllBinaryHTMLImageRotationAnimation_mut()) }
}
impl AllBinaryHTMLImageRotationAnimationDyn for AllBinaryHTMLImageRotationAnimation {
    fn as_AllBinaryHTMLImageRotationAnimation(&self) -> &AllBinaryHTMLImageRotationAnimation { self }
    fn as_AllBinaryHTMLImageRotationAnimation_mut(&mut self) -> &mut AllBinaryHTMLImageRotationAnimation { self }
}

impl crate::org::allbinary::animation::image::ImageBaseRotationAnimation::ImageBaseRotationAnimationDyn for AllBinaryHTMLImageRotationAnimation {
    fn as_ImageBaseRotationAnimation(&self) -> &crate::org::allbinary::animation::image::ImageBaseRotationAnimation::ImageBaseRotationAnimation { &self.base }
    fn as_ImageBaseRotationAnimation_mut(&mut self) -> &mut crate::org::allbinary::animation::image::ImageBaseRotationAnimation::ImageBaseRotationAnimation { &mut self.base }
}
impl crate::org::allbinary::animation::Animation::AnimationDyn for AllBinaryHTMLImageRotationAnimation {
    fn as_Animation(&self) -> &crate::org::allbinary::animation::Animation::Animation { &self.base.base.base.base }
    fn as_Animation_mut(&mut self) -> &mut crate::org::allbinary::animation::Animation::Animation { &mut self.base.base.base.base }
    fn setAlpha(&mut self, mut alpha: i32) { Self::setAlpha(self, alpha) }
    fn paintXY(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>, mut x: i32, mut y: i32) { Self::paintXY(self, graphics, x, y) }
    fn setBasicColorP(&mut self, mut basicColor: Option<Rc<RefCell<BasicColor>>>) { Self::setBasicColorP(self, basicColor) }
}
impl crate::org::allbinary::animation::IndexedAnimation::IndexedAnimationDyn for AllBinaryHTMLImageRotationAnimation {
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
impl crate::org::allbinary::animation::RotationAnimation::RotationAnimationDyn for AllBinaryHTMLImageRotationAnimation {
    fn as_RotationAnimation(&self) -> &crate::org::allbinary::animation::RotationAnimation::RotationAnimation { &self.base.base }
    fn as_RotationAnimation_mut(&mut self) -> &mut crate::org::allbinary::animation::RotationAnimation::RotationAnimation { &mut self.base.base }
    fn nextRotation(&mut self) { Self::nextRotation(self) }
    fn previousRotation(&mut self) { Self::previousRotation(self) }
}
impl crate::org::allbinary::image::opengles::OpenGLSurfaceChangedInterface::OpenGLSurfaceChangedInterface for AllBinaryHTMLImageRotationAnimation {
    fn set(&mut self, p1: Option<Rc<RefCell<dyn crate::javax::microedition::khronos::opengles::GL::GL>>>) { crate::org::allbinary::animation::Animation::Animation::set(crate::org::allbinary::animation::Animation::AnimationDyn::as_Animation_mut(self), p1) }
}


