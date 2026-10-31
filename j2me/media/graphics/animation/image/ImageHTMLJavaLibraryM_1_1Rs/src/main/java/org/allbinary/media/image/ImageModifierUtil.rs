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
// use crate::javax::microedition::lcdui::Graphics::Graphics;
// use crate::javax::microedition::lcdui::Image::Image;
use crate::org::allbinary::graphics::Anchor::Anchor;
use crate::org::allbinary::graphics::color::BasicColor::BasicColor;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::microemu::device::playn::PlaynImage::PlaynImage;
use crate::org::microemu::device::ResourceCallbackStrings::ResourceCallbackStrings;
use crate::playn::core::Canvas::Canvas;
use crate::playn::core::CanvasImage::CanvasImage;
use crate::playn::core::util::Callback::Callback;
use super::core::core;
use super::playn::playn;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct ImageModifierUtil {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub commonStrings: Option<Rc<RefCell<CommonStrings>>>,
    pub resourceCallbackStrings: Option<Rc<RefCell<ResourceCallbackStrings>>>,
    pub alphaArray: Vec<bool>,
}


impl ImageModifierUtil {

    pub fn new_struct() -> Self {
        let mut this = Self {
            logUtil: None,
            commonStrings: None,
            resourceCallbackStrings: None,
            alphaArray: Vec::new(),
        };
        this.logUtil = LogUtil::getInstance();
        this.commonStrings = CommonStrings::getInstance();
        this.resourceCallbackStrings = ResourceCallbackStrings::getInstance();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstanceOrCreate() -> Option<Rc<RefCell<ImageModifierUtil>>> {
        return ImageModifierUtil::new();
    }
    

    pub fn setColor(&mut self, mut unusedOriginalImage: Option<Rc<RefCell<Image>>>, mut image: Option<Rc<RefCell<Image>>>, mut imageIndex: i32, mut basicColor: Option<Rc<RefCell<BasicColor>>>) {
    }
    

    pub fn changeColor(&mut self, mut unusedOriginalImage: Option<Rc<RefCell<Image>>>, mut image: Option<Rc<RefCell<Image>>>, mut imageIndex: i32, mut basicColor: Option<Rc<RefCell<BasicColor>>>) {
    }
    

    pub fn setAlpha(&mut self, mut originalImage: Option<Rc<RefCell<Image>>>, mut image: Option<Rc<RefCell<Image>>>, mut imageIndex: i32, mut alpha: i32) {
        let mut alphaF: f32 = ((alpha) as f32).clone();
        let mut alphaFloat: f32 = alphaF / 255.0_f32;
        self.setAlphaF(originalImage.clone(), image.clone(), imageIndex.clone(), alphaFloat.clone());
    }
    

    pub fn setAlphaF(&mut self, mut originalImage: Option<Rc<RefCell<Image>>>, mut image: Option<Rc<RefCell<Image>>>, mut imageIndex: i32, mut alpha: f32) {
        if self.alphaArray[(imageIndex) as usize] {
            self.alphaArray[(imageIndex) as usize] = false;
            self.setAlpha2(originalImage.clone(), image.clone(), imageIndex.clone(), alpha.clone());
        }
    }
    

    pub fn setAlpha2(&mut self, mut originalImage: Option<Rc<RefCell<Image>>>, mut image: Option<Rc<RefCell<Image>>>, mut imageIndex: i32, mut alpha: f32) {
        let mut htmlImage: Option<Rc<RefCell<PlaynImage>>> = (image).clone() /* as PlaynImage */;
        let mut canvasImage: Option<Rc<RefCell<CanvasImage>>> = (jcall!(htmlImage).getImage()).clone() /* as CanvasImage */;
        let mut originalPlaynImage: Option<Rc<RefCell<Image>>> = (jcall!(((originalImage).clone() /* as PlaynImage */)).getImage()).clone() /* as Image */;
        let mut canvas: Option<Rc<RefCell<Canvas>>> = jcall!(canvasImage).canvas();
        jcall!(canvas).clear();
        jcall!(canvas).setAlpha(alpha.clone());
        jcall!(canvas).drawImage(originalPlaynImage.clone(), 0, 0);
    }
    

    pub fn setAlpha3(&mut self, mut image: Option<Rc<RefCell<Image>>>, mut alpha: f32) {
        let mut alphaF: f32 = alpha.clone();
        let mut alphaFloat: f32 = alphaF / 255.0_f32;
        let mut htmlImage: Option<Rc<RefCell<PlaynImage>>> = (image).clone() /* as PlaynImage */;
        let mut canvasImage: Option<Rc<RefCell<CanvasImage>>> = (jcall!(htmlImage).getImage()).clone() /* as CanvasImage */;
        let mut canvas: Option<Rc<RefCell<Canvas>>> = jcall!(canvasImage).canvas();
        jcall!(canvas).setAlpha(alphaFloat.clone());
    }
    

    pub fn getImageArray(&mut self, mut originalImageArray: Vec<Option<Rc<RefCell<Image>>>>) -> Vec<Option<Rc<RefCell<Image>>>> {
        let mut size: i32 = (originalImageArray.len() as i32).clone();
        self.alphaArray = vec![false; (size) as usize];
        let mut imageArray: Vec<Option<Rc<RefCell<Image>>>> = vec![None; (size) as usize];
        for index in (0..size) {
            imageArray[(index) as usize] = originalImageArray[(index) as usize].clone();
            self.handleImage(imageArray.clone(), index.clone(), originalImageArray[(index) as usize].clone());
        }
        return imageArray.clone();
    }
    

    pub fn handleImage(&mut self, mut imageArray: Vec<Option<Rc<RefCell<Image>>>>, mut index: i32, mut image: Option<Rc<RefCell<Image>>>) {
        let mut image3: Option<Rc<RefCell<Image>>> = (jcall!(((image).clone() /* as PlaynImage */)).getImage()).clone() /* as Image */;
        if image3.is_some() {
            if jcall!(image3).isReady() || jcall!(image3).width() + jcall!(image3).height() <= 0 || jcall!(image).getName() == jcall!(self.resourceCallbackStrings).FROM_DATA {
                self.copy(imageArray.clone(), index.clone(), image.clone(), image3.clone());
            } else {
                let mut callback: Option<Rc<RefCell<Callback>>> = Callback::new() /* Java anonymous class body: implement it as a named type
                    

                    pub fn onSuccess(&mut self, mut resource: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) {
                        let mut logUtil: Option<Rc<RefCell<LogUtil>>> = LogUtil::getInstance();
                        jcall!(logUtil).putF(format!("{}{}", jcall!(self.resourceCallbackStrings).DONE, jcall!(image).getName()), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.resourceCallbackStrings).HANDLE_IMAGE.clone());
                        self.copy(imageArray.clone(), index.clone(), image.clone(), image3.clone());
                    }
                    

                    pub fn onFailure(&mut self, mut e: String) {
                        let mut logUtil: Option<Rc<RefCell<LogUtil>>> = LogUtil::getInstance();
                        jcall!(logUtil).putF(jcall!(StringMaker::new()).append(jcall!(self.commonStrings).EXCEPTION_LABEL.clone()).append(jcall!(self.resourceCallbackStrings).ERROR.clone()).append(jcall!(image).getName()).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.resourceCallbackStrings).HANDLE_IMAGE.clone());
                    }
*/;
                jcall!(image3).addCallback(callback.clone());
            }
        } else {
            { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(self.resourceCallbackStrings).NULL + jcall!(image).isMutable(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.resourceCallbackStrings).HANDLE_IMAGE.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        }
    }
    

    pub fn copy(&mut self, mut imageArray: Vec<Option<Rc<RefCell<Image>>>>, mut index: i32, mut image: Option<Rc<RefCell<Image>>>, mut image3: Option<Rc<RefCell<Image>>>) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut image2: Option<Rc<RefCell<Image>>> = Image::createImage(jcall!(image3).width(), jcall!(image3).height());
            let mut graphics: Option<Rc<RefCell<Graphics>>> = jcall!(image2).getGraphics();
            jcall!(graphics).drawImage(image.clone(), 0, 0, unsafe { crate::org::allbinary::graphics::Anchor::Anchor_TOP_LEFT.clone() }.clone());
            imageArray[(index) as usize] = image2.clone();
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            jcall!(self.logUtil).putF(jcall!(self.commonStrings).EXCEPTION_LABEL + jcall!(self.resourceCallbackStrings).DONE, crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.resourceCallbackStrings).HANDLE_IMAGE.clone());
            }
        }
    }
    

    pub fn reset(&mut self) {
        let mut size: i32 = (self.alphaArray.len() as i32).clone();
        for index in (0..size) {
            self.alphaArray[(index) as usize] = true;
        }
    }
}

