// 
// AllBinary Open License Version 1 
// Copyright (c) 2022 AllBinary 
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
// package org.allbinary.media.image
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::javax::microedition::lcdui::Image::Image;
use crate::org::allbinary::game::configuration::feature::Features::Features;
use crate::org::allbinary::graphics::color::BasicColor::BasicColor;
use crate::org::allbinary::graphics::opengles::OpenGLFeatureFactory::OpenGLFeatureFactory;
use crate::org::allbinary::image::opengles::ModifierOpenGLESImageProcessor::ModifierOpenGLESImageProcessor;
use crate::org::allbinary::image::opengles::OpenGLESImage::OpenGLESImage;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::org::allbinary::logic::communication::log::PreLogUtil::PreLogUtil;
use crate::org::eclipse::swt::graphics::ImageData::ImageData;
use crate::org::microemu::device::swt::SwtImmutableImage::SwtImmutableImage;
use crate::org::microemu::device::swt::SwtMutableImage::SwtMutableImage;
use super::ImageCopyUtil::ImageCopyUtil;
use super::graphics::graphics;
use super::swt::swt;
use super::eclipse::eclipse;
use super::org::org;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct ImageModifierUtil {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub imageCopyUtil: Option<Rc<RefCell<ImageCopyUtil>>>,
    pub features: Option<Rc<RefCell<Features>>>,
}

thread_local! { pub static ImageModifierUtil_instance: Option<Rc<RefCell<ImageModifierUtil>>> = ImageModifierUtil::new(); }

impl ImageModifierUtil {

    pub fn new_struct() -> Self {
        let mut this = Self {
            logUtil: None,
            imageCopyUtil: None,
            features: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.imageCopyUtil = ImageCopyUtil::getInstance();
        this.features = Features::getInstance();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstanceOrCreate() -> Option<Rc<RefCell<ImageModifierUtil>>> {
        return ImageModifierUtil_instance.with(Clone::clone).clone();
    }
    

    pub fn setBasicColorP(&mut self, mut basicColor: Option<Rc<RefCell<BasicColor>>>) {
    }
    

    pub fn setColor(&mut self, mut unusedOriginalImage: Option<Rc<RefCell<Image>>>, mut image: Option<Rc<RefCell<Image>>>, mut imageIndex: i32, mut basicColor: Option<Rc<RefCell<BasicColor>>>) {
        let mut newBufferedImage: Option<Rc<RefCell<Image>>> = None;
        if jcall!(image).isMutable() {
            let mut swtImage: Option<Rc<RefCell<SwtMutableImage>>> = (image).clone() /* as SwtMutableImage */;
            newBufferedImage = (jcall!(swtImage).getImage()).clone() /* as Image */;
        } else {
            let mut swtImage: Option<Rc<RefCell<SwtImmutableImage>>> = (image).clone() /* as SwtImmutableImage */;
            newBufferedImage = (jcall!(swtImage).getImage()).clone() /* as Image */;
        }
        let mut r: i16 = jcall!(basicColor).red.clone();
        let mut g: i16 = jcall!(basicColor).green.clone();
        let mut b: i16 = jcall!(basicColor).blue.clone();
        let mut width: i32 = jcall!(image).getWidth();
        let mut height: i32 = jcall!(image).getHeight();
        let mut imageData: Option<Rc<RefCell<ImageData>>> = jcall!(newBufferedImage).getImageData();
        if jcall!(imageData).depth == 24 {
            let mut rx: i32 = 0;
            let mut gx: i32 = 0;
            let mut bx: i32 = 0;
            let mut index3: i32 = 0;
            for index in (0..width) {
                for index2 in (0..height) {
                    index3 = (index2 * jcall!(imageData).bytesPerLine) + (index * 4);
                    rx = (jcall!(imageData).data[(index3) as usize] & 0xFF_u32 as i32) << 16;
                    gx = (jcall!(imageData).data[(index3 + 1) as usize] & 0xFF_u32 as i32) << 8;
                    bx = (jcall!(imageData).data[(index3 + 2) as usize] & 0xFF_u32 as i32);
                    if rx != 0 || gx != 0 || bx != 0 {
                        jcall!(imageData).data[(index3) as usize] = ((r) as i8);
                        jcall!(imageData).data[(index3 + 1) as usize] = ((g) as i8);
                        jcall!(imageData).data[(index3 + 2) as usize] = ((b) as i8);
                    }
                }
            }
        }
    }
    

    pub fn changeColor(&mut self, mut unusedOriginalImage: Option<Rc<RefCell<Image>>>, mut image: Option<Rc<RefCell<Image>>>, mut imageIndex: i32, mut basicColor: Option<Rc<RefCell<BasicColor>>>) {
        if { let java_receiver = self.features.clone(); let java_arguments = (jcall!(OpenGLFeatureFactory::getInstance()).OPENGL.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::configuration::feature::Feature::FeatureDyn>>),); let java_result = jcall!(java_receiver).isFeature(java_arguments.0); java_result } {
            if jcall!(image).getType() >= crate::org::allbinary::image::opengles::OpenGLESImage::OpenGLESImage_TYPE.with(Clone::clone) {
                let mut openGLESImage: Option<Rc<RefCell<dyn crate::org::allbinary::image::opengles::OpenGLESImage::OpenGLESImageDyn>>> = ((image).clone() /* as OpenGLESImage */);
                jcall!(openGLESImage).as_OpenGLESImage_mut().imageProcessor = ModifierOpenGLESImageProcessor::getInstance();
                jcall!(jcall!(openGLESImage).as_OpenGLESImage_mut().openGLESImageProperties).redf = jcall!(basicColor).getRedComponent();
                jcall!(jcall!(openGLESImage).as_OpenGLESImage_mut().openGLESImageProperties).greenf = jcall!(basicColor).getGreenComponent();
                jcall!(jcall!(openGLESImage).as_OpenGLESImage_mut().openGLESImageProperties).bluef = jcall!(basicColor).getBlueComponent();
                return;
            }
        }
        let mut newBufferedImage: Option<Rc<RefCell<Image>>> = None;
        if jcall!(image).isMutable() {
            let mut swtImage: Option<Rc<RefCell<SwtMutableImage>>> = (image).clone() /* as SwtMutableImage */;
            newBufferedImage = (jcall!(swtImage).getImage()).clone() /* as Image */;
        } else {
            let mut swtImage: Option<Rc<RefCell<SwtImmutableImage>>> = (image).clone() /* as SwtImmutableImage */;
            newBufferedImage = (jcall!(swtImage).getImage()).clone() /* as Image */;
        }
        let mut MAX: f32 = 255.0_f32;
        let mut r: f32 = (((jcall!(basicColor).red) as f32)) / MAX;
        let mut g: f32 = (((jcall!(basicColor).green) as f32)) / MAX;
        let mut b: f32 = (((jcall!(basicColor).blue) as f32)) / MAX;
        let mut rx: i32 = 0;
        let mut gx: i32 = 0;
        let mut bx: i32 = 0;
        let mut imageData: Option<Rc<RefCell<ImageData>>> = jcall!(newBufferedImage).getImageData();
        if jcall!(imageData).depth == 24 {
            let mut size: i32 = (jcall!(imageData).data.len() as i32).clone();
            for index in (0..size).step_by((4) as usize) {
                rx = (jcall!(imageData).data[(index) as usize] & 0xFF_u32 as i32) << 16;
                gx = (jcall!(imageData).data[(index + 1) as usize] & 0xFF_u32 as i32) << 8;
                bx = (jcall!(imageData).data[(index + 2) as usize] & 0xFF_u32 as i32);
                rx = ((rx * r) as i32);
                gx = ((gx * g) as i32);
                bx = ((bx * b) as i32);
                jcall!(imageData).data[(index) as usize] = ((rx) as i8);
                jcall!(imageData).data[(index + 1) as usize] = ((gx) as i8);
                jcall!(imageData).data[(index + 2) as usize] = ((bx) as i8);
            }
        }
    }
    

    pub fn setAlpha(&mut self, mut unusedOriginalImage: Option<Rc<RefCell<Image>>>, mut image: Option<Rc<RefCell<Image>>>, mut imageIndex: i32, mut alphaInt: i32) {
        if { let java_receiver = self.features.clone(); let java_arguments = (jcall!(OpenGLFeatureFactory::getInstance()).OPENGL.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::configuration::feature::Feature::FeatureDyn>>),); let java_result = jcall!(java_receiver).isFeature(java_arguments.0); java_result } {
            if jcall!(image).getType() >= crate::org::allbinary::image::opengles::OpenGLESImage::OpenGLESImage_TYPE.with(Clone::clone) {
                let mut openGLESImage: Option<Rc<RefCell<dyn crate::org::allbinary::image::opengles::OpenGLESImage::OpenGLESImageDyn>>> = ((image).clone() /* as OpenGLESImage */);
                jcall!(openGLESImage).as_OpenGLESImage_mut().imageProcessor = ModifierOpenGLESImageProcessor::getInstance();
                jcall!(jcall!(openGLESImage).as_OpenGLESImage_mut().openGLESImageProperties).alphaf = (((alphaInt) as f32)) / 255.0_f32;
                return;
            }
        }
        self.setAlphaSWT(image.clone(), alphaInt.clone());
    }
    

    pub fn setAlphaSWT(&mut self, mut image: Option<Rc<RefCell<Image>>>, mut alphaInt: i32) {
        let mut alpha: i8 = ((alphaInt) as i8);
        let mut newBufferedImage: Option<Rc<RefCell<Image>>> = None;
        if jcall!(image).isMutable() {
            let mut swtImage: Option<Rc<RefCell<SwtMutableImage>>> = (image).clone() /* as SwtMutableImage */;
            newBufferedImage = (jcall!(swtImage).getImage()).clone() /* as Image */;
        } else {
            let mut swtImage: Option<Rc<RefCell<SwtImmutableImage>>> = (image).clone() /* as SwtImmutableImage */;
            newBufferedImage = (jcall!(swtImage).getImage()).clone() /* as Image */;
        }
        jcall!(jcall!(newBufferedImage).getImageData()).alpha = alpha.clone();
    }
    

    pub fn getImageArray(&mut self, mut originalImageArray: Vec<Option<Rc<RefCell<Image>>>>) -> Vec<Option<Rc<RefCell<Image>>>> {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut newImageArray: Vec<Option<Rc<RefCell<Image>>>> = vec![None; ((originalImageArray.len() as i32)) as usize];
            let mut size: i32 = (newImageArray.len() as i32).clone();
            let mut originalImage: Option<Rc<RefCell<Image>>> = None;
            for index in (0..size) {
                originalImage = originalImageArray[(index) as usize].clone();
                let mut image: Option<Rc<RefCell<Image>>> = jcall!(self.imageCopyUtil).createImage(originalImage.clone());
                newImageArray[(index) as usize] = image.clone();
            }
            return newImageArray.clone();
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            let mut commonStrings: Option<Rc<RefCell<CommonStrings>>> = CommonStrings::getInstance();
            PreLogUtil::putOE(jcall!(commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("getImageArray"), e.clone());
            return originalImageArray.clone();
            }
        }
    }
    

    pub fn handleImage(&mut self, mut imageArray: Vec<Option<Rc<RefCell<Image>>>>, mut index: i32, mut image: Option<Rc<RefCell<Image>>>) {
    }
    

    pub fn reset(&mut self) {
    }
}

