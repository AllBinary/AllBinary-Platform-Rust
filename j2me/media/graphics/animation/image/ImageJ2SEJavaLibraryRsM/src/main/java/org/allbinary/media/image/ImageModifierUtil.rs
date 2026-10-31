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
// use crate::javax::microedition::lcdui::Image::Image;
use crate::org::allbinary::graphics::Anchor::Anchor;
use crate::org::allbinary::graphics::color::BasicColor::BasicColor;
use crate::org::allbinary::logic::communication::log::PreLogUtil::PreLogUtil;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
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

thread_local! { pub static ImageModifierUtil_instance: Option<Rc<RefCell<ImageModifierUtil>>> = ImageModifierUtil::new(); }

impl ImageModifierUtil {

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
    

    pub fn setColor(&mut self, mut unusedOriginalImage: Option<Rc<RefCell<Image>>>, mut image: Option<Rc<RefCell<Image>>>, mut imageIndex: i32, mut basicColor: Option<Rc<RefCell<BasicColor>>>) {
        let mut newBufferedImage: Option<Rc<RefCell<BufferedImage>>> = jcall!(self.imageUtil).getBufferedImage(image.clone());
        let mut r: i16 = jcall!(basicColor).red.clone();
        let mut g: i16 = jcall!(basicColor).green.clone();
        let mut b: i16 = jcall!(basicColor).blue.clone();
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
                colorModel = jcall!(newBufferedImage).getColorModel();
                dataElements = jcall!(jcall!(newBufferedImage).getRaster()).getDataElements(index.clone(), index2.clone(), None);
                ax = jcall!(colorModel).getAlpha(dataElements.clone());
                rx = jcall!(colorModel).getRed(dataElements.clone());
                gx = jcall!(colorModel).getGreen(dataElements.clone());
                bx = jcall!(colorModel).getBlue(dataElements.clone());
                if rx != 0 || gx != 0 || bx != 0 {
                    jcall!(newBufferedImage).setRGB(index.clone(), index2.clone(), (ax << 24) | (r << 16) | (g << 8) | (((b)) as i32));
                }
            }
        }
    }
    

    pub fn changeColor(&mut self, mut unusedOriginalImage: Option<Rc<RefCell<Image>>>, mut image: Option<Rc<RefCell<Image>>>, mut imageIndex: i32, mut basicColor: Option<Rc<RefCell<BasicColor>>>) {
        let mut newBufferedImage: Option<Rc<RefCell<BufferedImage>>> = jcall!(self.imageUtil).getBufferedImage(image.clone());
        let mut MAX: f32 = 255.0_f32;
        let mut r: f32 = (((jcall!(basicColor).red) as f32)) / MAX;
        let mut g: f32 = (((jcall!(basicColor).green) as f32)) / MAX;
        let mut b: f32 = (((jcall!(basicColor).blue) as f32)) / MAX;
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
                colorModel = jcall!(newBufferedImage).getColorModel();
                dataElements = jcall!(jcall!(newBufferedImage).getRaster()).getDataElements(index.clone(), index2.clone(), None);
                ax = jcall!(colorModel).getAlpha(dataElements.clone());
                rx = jcall!(colorModel).getRed(dataElements.clone());
                gx = jcall!(colorModel).getGreen(dataElements.clone());
                bx = jcall!(colorModel).getBlue(dataElements.clone());
                rx = ((rx * r) as i32);
                gx = ((gx * g) as i32);
                bx = ((bx * b) as i32);
                jcall!(newBufferedImage).setRGB(index.clone(), index2.clone(), (ax << 24) | (rx << 16) | (gx << 8) | (bx));
            }
        }
    }
    

    pub fn setAlpha(&mut self, mut unusedOriginalImage: Option<Rc<RefCell<Image>>>, mut image: Option<Rc<RefCell<Image>>>, mut imageIndex: i32, mut alphaInt: i32) {
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
    

    pub fn getImageArray(&mut self, mut originalImageArray: Vec<Option<Rc<RefCell<Image>>>>) -> Vec<Option<Rc<RefCell<Image>>>> {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut newImageArray: Vec<Option<Rc<RefCell<Image>>>> = vec![None; ((originalImageArray.len() as i32)) as usize];
            let mut size: i32 = (newImageArray.len() as i32).clone();
            let mut originalImage: Option<Rc<RefCell<Image>>> = None;
            for index in (0..size) {
                originalImage = originalImageArray[(index) as usize].clone();
                let mut image: Option<Rc<RefCell<Image>>> = jcall!(ImageCreationUtil::ImageCreationUtil::getInstance()).createImageWH(jcall!(originalImage).getWidth(), jcall!(originalImage).getHeight());
                jcall!(jcall!(image).getGraphics()).drawImage(originalImage.clone(), 0, 0, unsafe { crate::org::allbinary::graphics::Anchor::Anchor_TOP_LEFT.clone() }.clone());
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

