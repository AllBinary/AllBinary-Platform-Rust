//
// AllBinary Open License Version 1
// Copyright (c) 2023 AllBinary
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
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::microemu::device::playn::PlaynImmutableImage::PlaynImmutableImage;
use crate::org::microemu::device::playn::PlaynMutableImage::PlaynMutableImage;
use crate::playn::core::CanvasImage::CanvasImage;
use crate::playn::core::CanvasSurface::CanvasSurface;
use super::core::core;
use super::playn::playn;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct ImageJ2MEScaleUtil {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
}

thread_local! { pub static ImageJ2MEScaleUtil_instance: Option<Rc<RefCell<ImageJ2MEScaleUtil>>> = ImageJ2MEScaleUtil::new(); }

impl ImageJ2MEScaleUtil {
    

    pub fn getInstance() -> Option<Rc<RefCell<ImageJ2MEScaleUtil>>> {
        return ImageJ2MEScaleUtil_instance.with(Clone::clone).clone();
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            logUtil: None,
        };
        this.logUtil = LogUtil::getInstance();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    // throws Exception
    pub fn scaleArray(&mut self, mut images: Vec<Option<Rc<RefCell<Image>>>>, mut width: i32, mut height: i32) -> Vec<Option<Rc<RefCell<Image>>>> {
        let mut scaledImages: Vec<Option<Rc<RefCell<Image>>>> = vec![None; ((images.len() as i32)) as usize];
        {
            let mut index: i32 = (images.len() as i32).clone();
            while { index -= 1; index } >= 0 {
            scaledImages[(index) as usize] = self.scale(images[(index) as usize].clone(), width.clone(), height.clone());
            }
        }
        return scaledImages.clone();
    }
    

    // throws Exception
    pub fn scale(&mut self, mut image: Option<Rc<RefCell<Image>>>, mut width: i32, mut height: i32) -> Option<Rc<RefCell<Image>>> {
        let mut originalPlayNImage: Option<Rc<RefCell<Image>>> = None;
        if jcall!(image).isMutable() {
            let mut originalHTMLImage: Option<Rc<RefCell<PlaynMutableImage>>> = (image).clone() /* as PlaynMutableImage */;
            originalPlayNImage = (jcall!(originalHTMLImage).getImage()).clone() /* as Image */;
        } else {
            let mut originalHTMLImage: Option<Rc<RefCell<PlaynImmutableImage>>> = (image).clone() /* as PlaynImmutableImage */;
            originalPlayNImage = (jcall!(originalHTMLImage).getImage()).clone() /* as Image */;
        }
        let mut scaledImage: Option<Rc<RefCell<Image>>> = Image::createImage(width.clone(), height.clone());
        let mut htmlImage: Option<Rc<RefCell<PlaynMutableImage>>> = (scaledImage).clone() /* as PlaynMutableImage */;
        let mut canvasImage: Option<Rc<RefCell<CanvasImage>>> = (jcall!(htmlImage).getImage()).clone() /* as CanvasImage */;
        let mut canvasSurface: Option<Rc<RefCell<CanvasSurface>>> = jcall!(htmlImage).getCanvasSurface(canvasImage.clone());
        jcall!(canvasSurface).drawImage(originalPlayNImage.clone(), 0, 0, jcall!(scaledImage).getWidth(), jcall!(scaledImage).getHeight(), 0, 0, jcall!(image).getWidth(), jcall!(image).getHeight());
        return scaledImage.clone();
    }
}

