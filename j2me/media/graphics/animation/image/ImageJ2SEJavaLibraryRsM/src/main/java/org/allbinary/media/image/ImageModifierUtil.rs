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
// use crate::java::awt::image::BufferedImage::BufferedImage;
// use crate::java::awt::image::ColorModel::ColorModel;
// use crate::java::awt::image::WritableRaster::WritableRaster;
use crate::javax::microedition::lcdui::Image::Image;
use crate::org::allbinary::graphics::Anchor::Anchor;
use crate::org::allbinary::graphics::color::BasicColor::BasicColor;
use crate::org::allbinary::logic::communication::log::PreLogUtil::PreLogUtil;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::java::lang::Exception::*;
use super::ImageUtil::ImageUtil;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct ImageModifierUtil {
    pub imageUtil: Option<Rc<RefCell<ImageUtil>>>,
    pub colorArray: Vec<i32>,
}

pub static ImageModifierUtil_instance: crate::JavaStatic<Option<Rc<RefCell<ImageModifierUtil>>>> = crate::JavaStatic::new(|| ImageModifierUtil::new());

impl ImageModifierUtil {

    pub fn toString(&mut self) -> String {
        String::from(std::any::type_name::<Self>())
    }

    pub fn new_struct() -> Self {
        let mut this = Self {
            imageUtil: None,
            colorArray: Vec::new(),
        };
        this.imageUtil = ImageUtil::getInstance();
        this.colorArray = vec![0; (1) as usize];
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
        let mut newBufferedImage: Option<Rc<RefCell<BufferedImage>>> = jcall!(self.imageUtil).getBufferedImage(image.clone());
        let mut r: i16 = { let java_field = jcall!(basicColor).red.clone(); java_field }.clone();
        let mut g: i16 = { let java_field = jcall!(basicColor).green.clone(); java_field }.clone();
        let mut b: i16 = { let java_field = jcall!(basicColor).blue.clone(); java_field }.clone();
        let mut width: i32 = jcall!(image).getWidth();
        let mut height: i32 = jcall!(image).getHeight();
        let mut colorModel: Option<Rc<RefCell<ColorModel>>> = None;
        let mut dataElements: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = None;
        let mut ax: i32 = 0;
        let mut rx: i32 = 0;
        let mut gx: i32 = 0;
        let mut bx: i32 = 0;
        for index in (0..width) {
            for index2 in (0..height) {
                colorModel = { let java_value = (jcall!(newBufferedImage).getColorModel()).clone(); java_value };
                dataElements = crate::java_into_object({ let java_value = (jcall!(jcall!(newBufferedImage).getRaster()).getDataElements(index.clone(), index2.clone(), None)).clone(); java_value });
                ax = { let java_value = (jcall!(colorModel).getAlpha(dataElements.clone())).clone(); java_value };
                rx = { let java_value = (jcall!(colorModel).getRed(dataElements.clone())).clone(); java_value };
                gx = { let java_value = (jcall!(colorModel).getGreen(dataElements.clone())).clone(); java_value };
                bx = { let java_value = (jcall!(colorModel).getBlue(dataElements.clone())).clone(); java_value };
                if rx != 0 || gx != 0 || bx != 0 {
                    jcall!(newBufferedImage).setRGB(index.clone(), index2.clone(), (ax << 24) | (r << 16) | (g << 8) | (((b)) as i32));
                }
            }
        }
    }
    

    pub fn changeColor(&mut self, mut unusedOriginalImage: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Image::ImageDyn>>>, mut image: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Image::ImageDyn>>>, mut imageIndex: i32, mut basicColor: Option<Rc<RefCell<BasicColor>>>) {
        let mut newBufferedImage: Option<Rc<RefCell<BufferedImage>>> = jcall!(self.imageUtil).getBufferedImage(image.clone());
        let mut MAX: f32 = 255.0_f32;
        let mut r: f32 = ((({ let java_field = jcall!(basicColor).red.clone(); java_field }) as f32)) / MAX;
        let mut g: f32 = ((({ let java_field = jcall!(basicColor).green.clone(); java_field }) as f32)) / MAX;
        let mut b: f32 = ((({ let java_field = jcall!(basicColor).blue.clone(); java_field }) as f32)) / MAX;
        let mut width: i32 = jcall!(image).getWidth();
        let mut height: i32 = jcall!(image).getHeight();
        let mut colorModel: Option<Rc<RefCell<ColorModel>>> = None;
        let mut dataElements: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = None;
        let mut ax: i32 = 0;
        let mut rx: i32 = 0;
        let mut gx: i32 = 0;
        let mut bx: i32 = 0;
        for index in (0..width) {
            for index2 in (0..height) {
                colorModel = { let java_value = (jcall!(newBufferedImage).getColorModel()).clone(); java_value };
                dataElements = crate::java_into_object({ let java_value = (jcall!(jcall!(newBufferedImage).getRaster()).getDataElements(index.clone(), index2.clone(), None)).clone(); java_value });
                ax = { let java_value = (jcall!(colorModel).getAlpha(dataElements.clone())).clone(); java_value };
                rx = { let java_value = (jcall!(colorModel).getRed(dataElements.clone())).clone(); java_value };
                gx = { let java_value = (jcall!(colorModel).getGreen(dataElements.clone())).clone(); java_value };
                bx = { let java_value = (jcall!(colorModel).getBlue(dataElements.clone())).clone(); java_value };
                rx = ((rx * r) as i32);
                gx = ((gx * g) as i32);
                bx = ((bx * b) as i32);
                jcall!(newBufferedImage).setRGB(index.clone(), index2.clone(), (ax << 24) | (rx << 16) | (gx << 8) | (bx));
            }
        }
    }
    

    pub fn setAlpha(&mut self, mut unusedOriginalImage: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Image::ImageDyn>>>, mut image: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Image::ImageDyn>>>, mut imageIndex: i32, mut alphaInt: i32) {
        let mut alpha: i8 = ((alphaInt) as i8);
        let mut newBufferedImage: Option<Rc<RefCell<BufferedImage>>> = jcall!(self.imageUtil).getBufferedImage(image.clone());
        let mut width: i32 = jcall!(image).getWidth();
        let mut height: i32 = jcall!(image).getHeight();
        let mut raster: Option<Rc<RefCell<WritableRaster>>> = jcall!(newBufferedImage).getAlphaRaster();
        for index in (0..width) {
            for index2 in (0..height) {
                jcall!(raster).getPixel(index.clone(), index2.clone(), self.colorArray.clone());
                self.colorArray[(0) as usize] = ((alpha) as i32) & self.colorArray[(0) as usize];
                jcall!(raster).setPixel(index.clone(), index2.clone(), self.colorArray.clone());
            }
        }
    }
    

    pub fn getImageArray(&mut self, mut originalImageArray: Vec<Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Image::ImageDyn>>>>) -> Vec<Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Image::ImageDyn>>>> {
        let java_try_result = match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut newImageArray: Vec<Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Image::ImageDyn>>>> = vec![None; ((originalImageArray.len() as i32)) as usize];
            let mut size: i32 = (newImageArray.len() as i32).clone();
            let mut originalImage: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Image::ImageDyn>>> = None;
            for index in (0..size) {
                originalImage = { let java_value = (originalImageArray[(index) as usize]).clone(); java_value }.clone();
                let mut image: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Image::ImageDyn>>> = jcall!(ImageCreationUtil::ImageCreationUtil::getInstance()).createImageWH(jcall!(originalImage).getWidth(), jcall!(originalImage).getHeight());
                jcall!(jcall!(image).getGraphics()).drawImage(originalImage.clone(), 0, 0, unsafe { crate::org::allbinary::graphics::Anchor::Anchor_TOP_LEFT.clone() }.clone());
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

