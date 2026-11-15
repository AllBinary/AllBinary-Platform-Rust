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
use crate::javax::microedition::lcdui::Image::Image;
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
use crate::java::lang::Exception::*;
use super::ImageCopyUtil::ImageCopyUtil;

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

pub static ImageModifierUtil_instance: crate::JavaStatic<Option<Rc<RefCell<ImageModifierUtil>>>> = crate::JavaStatic::new(|| ImageModifierUtil::new());

impl ImageModifierUtil {

    pub fn toString(&mut self) -> String {
        String::from(std::any::type_name::<Self>())
    }

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
    

    pub fn setColor(&mut self, mut unusedOriginalImage: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Image::ImageDyn>>>, mut image: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Image::ImageDyn>>>, mut imageIndex: i32, mut basicColor: Option<Rc<RefCell<BasicColor>>>) {
        let mut newBufferedImage: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Image::ImageDyn>>> = None;
        if jcall!(image).isMutable() {
            let mut swtImage: Option<Rc<RefCell<SwtMutableImage>>> = (image).clone().map(|reference| { assert!({ let java_borrowed = reference.borrow(); let java_any: &dyn std::any::Any = &*java_borrowed; java_any.is::<SwtMutableImage>() }, "ClassCastException: not a SwtMutableImage"); unsafe { Rc::from_raw(Rc::into_raw(reference) as *const RefCell<SwtMutableImage>) } });
            newBufferedImage = { let java_value = ({ let java_object = (jcall!(swtImage).getImage()).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Image::ImageDyn>>>>().expect("ClassCastException").clone()) }).clone(); java_value };
        } else {
            let mut swtImage: Option<Rc<RefCell<SwtImmutableImage>>> = (image).clone().map(|reference| { assert!({ let java_borrowed = reference.borrow(); let java_any: &dyn std::any::Any = &*java_borrowed; java_any.is::<SwtImmutableImage>() }, "ClassCastException: not a SwtImmutableImage"); unsafe { Rc::from_raw(Rc::into_raw(reference) as *const RefCell<SwtImmutableImage>) } });
            newBufferedImage = { let java_value = ({ let java_object = (jcall!(swtImage).getImage()).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Image::ImageDyn>>>>().expect("ClassCastException").clone()) }).clone(); java_value };
        }
        let mut r: i16 = { let java_field = jcall!(basicColor).red.clone(); java_field }.clone();
        let mut g: i16 = { let java_field = jcall!(basicColor).green.clone(); java_field }.clone();
        let mut b: i16 = { let java_field = jcall!(basicColor).blue.clone(); java_field }.clone();
        let mut width: i32 = jcall!(image).getWidth();
        let mut height: i32 = jcall!(image).getHeight();
        let mut imageData: Option<Rc<RefCell<ImageData>>> = jcall!(newBufferedImage).as_Image_mut().getImageData();
        if { let java_field = jcall!(imageData).depth.clone(); java_field } == 24 {
            let mut rx: i32 = 0;
            let mut gx: i32 = 0;
            let mut bx: i32 = 0;
            let mut index3: i32 = 0;
            for index in (0..width) {
                for index2 in (0..height) {
                    index3 = { let java_value = ((index2 * { let java_field = jcall!(imageData).bytesPerLine.clone(); java_field }) + (index * 4)).clone(); java_value };
                    rx = { let java_value = ((jcall!(imageData).data[(index3) as usize] & 0xFF) << 16).clone(); java_value };
                    gx = { let java_value = ((jcall!(imageData).data[(index3 + 1) as usize] & 0xFF) << 8).clone(); java_value };
                    bx = { let java_value = ((jcall!(imageData).data[(index3 + 2) as usize] & 0xFF)).clone(); java_value };
                    if rx != 0 || gx != 0 || bx != 0 {
                        jcall!(imageData).data[(index3) as usize] = ((r) as i8);
                        jcall!(imageData).data[(index3 + 1) as usize] = ((g) as i8);
                        jcall!(imageData).data[(index3 + 2) as usize] = ((b) as i8);
                    }
                }
            }
        }
    }
    

    pub fn changeColor(&mut self, mut unusedOriginalImage: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Image::ImageDyn>>>, mut image: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Image::ImageDyn>>>, mut imageIndex: i32, mut basicColor: Option<Rc<RefCell<BasicColor>>>) {
        if { let java_receiver = self.features.clone(); let java_arguments = ({ let java_field = jcall!(OpenGLFeatureFactory::getInstance()).OPENGL.clone(); java_field }.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::configuration::feature::Feature::FeatureDyn>>),); let java_result = jcall!(java_receiver).isFeature(java_arguments.0); java_result } {
            if jcall!(image).getType() >= crate::org::allbinary::image::opengles::OpenGLESImage::OpenGLESImage_TYPE.with(Clone::clone) {
                let mut openGLESImage: Option<Rc<RefCell<dyn crate::org::allbinary::image::opengles::OpenGLESImage::OpenGLESImageDyn>>> = ((|| -> Option<Rc<RefCell<dyn crate::org::allbinary::image::opengles::OpenGLESImage::OpenGLESImageDyn>>> { panic!("ClassCastException: a Rust dyn Image cannot be cast to OpenGLESImage") })());
                jcall!(openGLESImage).as_OpenGLESImage_mut().imageProcessor = ModifierOpenGLESImageProcessor::getInstance().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::image::opengles::OpenGLESImageProcessor::OpenGLESImageProcessorDyn>>);
                jcall!({ let java_field = jcall!(openGLESImage).as_OpenGLESImage_mut().openGLESImageProperties.clone(); java_field }).redf = jcall!(basicColor).getRedComponent();
                jcall!({ let java_field = jcall!(openGLESImage).as_OpenGLESImage_mut().openGLESImageProperties.clone(); java_field }).greenf = jcall!(basicColor).getGreenComponent();
                jcall!({ let java_field = jcall!(openGLESImage).as_OpenGLESImage_mut().openGLESImageProperties.clone(); java_field }).bluef = jcall!(basicColor).getBlueComponent();
                return;
            }
        }
        let mut newBufferedImage: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Image::ImageDyn>>> = None;
        if jcall!(image).isMutable() {
            let mut swtImage: Option<Rc<RefCell<SwtMutableImage>>> = (image).clone().map(|reference| { assert!({ let java_borrowed = reference.borrow(); let java_any: &dyn std::any::Any = &*java_borrowed; java_any.is::<SwtMutableImage>() }, "ClassCastException: not a SwtMutableImage"); unsafe { Rc::from_raw(Rc::into_raw(reference) as *const RefCell<SwtMutableImage>) } });
            newBufferedImage = { let java_value = ({ let java_object = (jcall!(swtImage).getImage()).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Image::ImageDyn>>>>().expect("ClassCastException").clone()) }).clone(); java_value };
        } else {
            let mut swtImage: Option<Rc<RefCell<SwtImmutableImage>>> = (image).clone().map(|reference| { assert!({ let java_borrowed = reference.borrow(); let java_any: &dyn std::any::Any = &*java_borrowed; java_any.is::<SwtImmutableImage>() }, "ClassCastException: not a SwtImmutableImage"); unsafe { Rc::from_raw(Rc::into_raw(reference) as *const RefCell<SwtImmutableImage>) } });
            newBufferedImage = { let java_value = ({ let java_object = (jcall!(swtImage).getImage()).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Image::ImageDyn>>>>().expect("ClassCastException").clone()) }).clone(); java_value };
        }
        let mut MAX: f32 = 255.0_f32;
        let mut r: f32 = ((({ let java_field = jcall!(basicColor).red.clone(); java_field }) as f32)) / MAX;
        let mut g: f32 = ((({ let java_field = jcall!(basicColor).green.clone(); java_field }) as f32)) / MAX;
        let mut b: f32 = ((({ let java_field = jcall!(basicColor).blue.clone(); java_field }) as f32)) / MAX;
        let mut rx: i32 = 0;
        let mut gx: i32 = 0;
        let mut bx: i32 = 0;
        let mut imageData: Option<Rc<RefCell<ImageData>>> = jcall!(newBufferedImage).as_Image_mut().getImageData();
        if { let java_field = jcall!(imageData).depth.clone(); java_field } == 24 {
            let mut size: i32 = ({ let java_field = jcall!(imageData).data.clone(); java_field }.len() as i32).clone();
            for index in (0..size).step_by((4) as usize) {
                rx = { let java_value = ((jcall!(imageData).data[(index) as usize] & 0xFF) << 16).clone(); java_value };
                gx = { let java_value = ((jcall!(imageData).data[(index + 1) as usize] & 0xFF) << 8).clone(); java_value };
                bx = { let java_value = ((jcall!(imageData).data[(index + 2) as usize] & 0xFF)).clone(); java_value };
                rx = ((rx * r) as i32);
                gx = ((gx * g) as i32);
                bx = ((bx * b) as i32);
                jcall!(imageData).data[(index) as usize] = ((rx) as i8);
                jcall!(imageData).data[(index + 1) as usize] = ((gx) as i8);
                jcall!(imageData).data[(index + 2) as usize] = ((bx) as i8);
            }
        }
    }
    

    pub fn setAlpha(&mut self, mut unusedOriginalImage: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Image::ImageDyn>>>, mut image: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Image::ImageDyn>>>, mut imageIndex: i32, mut alphaInt: i32) {
        if { let java_receiver = self.features.clone(); let java_arguments = ({ let java_field = jcall!(OpenGLFeatureFactory::getInstance()).OPENGL.clone(); java_field }.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::configuration::feature::Feature::FeatureDyn>>),); let java_result = jcall!(java_receiver).isFeature(java_arguments.0); java_result } {
            if jcall!(image).getType() >= crate::org::allbinary::image::opengles::OpenGLESImage::OpenGLESImage_TYPE.with(Clone::clone) {
                let mut openGLESImage: Option<Rc<RefCell<dyn crate::org::allbinary::image::opengles::OpenGLESImage::OpenGLESImageDyn>>> = ((|| -> Option<Rc<RefCell<dyn crate::org::allbinary::image::opengles::OpenGLESImage::OpenGLESImageDyn>>> { panic!("ClassCastException: a Rust dyn Image cannot be cast to OpenGLESImage") })());
                jcall!(openGLESImage).as_OpenGLESImage_mut().imageProcessor = ModifierOpenGLESImageProcessor::getInstance().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::image::opengles::OpenGLESImageProcessor::OpenGLESImageProcessorDyn>>);
                jcall!({ let java_field = jcall!(openGLESImage).as_OpenGLESImage_mut().openGLESImageProperties.clone(); java_field }).alphaf = (((alphaInt) as f32)) / 255.0_f32;
                return;
            }
        }
        self.setAlphaSWT(image.clone(), alphaInt.clone());
    }
    

    pub fn setAlphaSWT(&mut self, mut image: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Image::ImageDyn>>>, mut alphaInt: i32) {
        let mut alpha: i8 = ((alphaInt) as i8);
        let mut newBufferedImage: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Image::ImageDyn>>> = None;
        if jcall!(image).isMutable() {
            let mut swtImage: Option<Rc<RefCell<SwtMutableImage>>> = (image).clone().map(|reference| { assert!({ let java_borrowed = reference.borrow(); let java_any: &dyn std::any::Any = &*java_borrowed; java_any.is::<SwtMutableImage>() }, "ClassCastException: not a SwtMutableImage"); unsafe { Rc::from_raw(Rc::into_raw(reference) as *const RefCell<SwtMutableImage>) } });
            newBufferedImage = { let java_value = ({ let java_object = (jcall!(swtImage).getImage()).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Image::ImageDyn>>>>().expect("ClassCastException").clone()) }).clone(); java_value };
        } else {
            let mut swtImage: Option<Rc<RefCell<SwtImmutableImage>>> = (image).clone().map(|reference| { assert!({ let java_borrowed = reference.borrow(); let java_any: &dyn std::any::Any = &*java_borrowed; java_any.is::<SwtImmutableImage>() }, "ClassCastException: not a SwtImmutableImage"); unsafe { Rc::from_raw(Rc::into_raw(reference) as *const RefCell<SwtImmutableImage>) } });
            newBufferedImage = { let java_value = ({ let java_object = (jcall!(swtImage).getImage()).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Image::ImageDyn>>>>().expect("ClassCastException").clone()) }).clone(); java_value };
        }
        jcall!(jcall!(newBufferedImage).as_Image_mut().getImageData()).alpha = alpha.clone();
    }
    

    pub fn getImageArray(&mut self, mut originalImageArray: Vec<Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Image::ImageDyn>>>>) -> Vec<Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Image::ImageDyn>>>> {
        let java_try_result = match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut newImageArray: Vec<Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Image::ImageDyn>>>> = vec![None; ((originalImageArray.len() as i32)) as usize];
            let mut size: i32 = (newImageArray.len() as i32).clone();
            let mut originalImage: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Image::ImageDyn>>> = None;
            for index in (0..size) {
                originalImage = { let java_value = (originalImageArray[(index) as usize]).clone(); java_value }.clone();
                let mut image: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Image::ImageDyn>>> = jcall!(self.imageCopyUtil).createImage(originalImage.clone());
                newImageArray[(index) as usize] = image.clone();
            }
            return Some(newImageArray.clone());
        })) {
            Ok(java_return) => java_return,
            Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            let mut commonStrings: Option<Rc<RefCell<CommonStrings>>> = CommonStrings::getInstance();
            PreLogUtil::putOE({ let java_field = jcall!(commonStrings).EXCEPTION.clone(); java_field }.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("getImageArray"), crate::java_into_object(e.clone()));
            return originalImageArray.clone();
            None
            }
        };if let Some(java_return) = java_try_result { return java_return; }unreachable!("try statement")
    }
    

    pub fn handleImage(&mut self, mut imageArray: Vec<Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Image::ImageDyn>>>>, mut index: i32, mut image: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Image::ImageDyn>>>) {
    }
    

    pub fn reset(&mut self) {
    }
}

