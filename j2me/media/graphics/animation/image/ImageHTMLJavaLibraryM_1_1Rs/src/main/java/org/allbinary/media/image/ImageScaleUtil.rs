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
// package org.allbinary.media.image
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::javax::microedition::lcdui::Image::Image;
use crate::org::allbinary::image::ImageCache::ImageCache;
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

pub struct ImageScaleUtil {
}

thread_local! { pub static ImageScaleUtil_instance: Option<Rc<RefCell<ImageScaleUtil>>> = ImageScaleUtil::new(); }

impl ImageScaleUtil {
    

    pub fn getInstance() -> Option<Rc<RefCell<ImageScaleUtil>>> {
        return ImageScaleUtil_instance.with(Clone::clone).clone();
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    // throws Exception
    pub fn createImage2(&mut self, mut imageCache: Option<Rc<RefCell<ImageCache>>>, mut originalImage: Option<Rc<RefCell<Image>>>, mut scaleNominatorX: f32, mut scaleDenominatorX: f32, mut scaleNominatorY: f32, mut scaleDenominatorY: f32, mut cached: bool) -> Option<Rc<RefCell<Image>>> {
        return self.createImage3(imageCache.clone(), originalImage.clone(), scaleNominatorX.clone(), scaleDenominatorX.clone(), scaleNominatorY.clone(), scaleDenominatorY.clone(), cached.clone(), true);
    }
    

    // throws Exception
    pub fn createImage3(&mut self, mut imageCache: Option<Rc<RefCell<ImageCache>>>, mut originalImage: Option<Rc<RefCell<Image>>>, mut scaleNominatorX: f32, mut scaleDenominatorX: f32, mut scaleNominatorY: f32, mut scaleDenominatorY: f32, mut cached: bool, mut mutable: bool) -> Option<Rc<RefCell<Image>>> {
        let mut scaleX: f32 = scaleNominatorX / scaleDenominatorX;
        let mut scaleY: f32 = scaleNominatorY / scaleDenominatorY;
        return self.createImage(imageCache.clone(), originalImage.clone(), scaleX.clone(), scaleY.clone(), cached.clone());
    }
    

    // throws Exception
    pub fn createImage(&mut self, mut imageCache: Option<Rc<RefCell<ImageCache>>>, mut originalImage: Option<Rc<RefCell<Image>>>, mut scaleX: f32, mut scaleY: f32, mut cached: bool) -> Option<Rc<RefCell<Image>>> {
        return self.createImageInternal(originalImage.clone(), scaleX.clone(), scaleY.clone());
    }
    

    // throws Exception
    pub fn createImageInternal(&mut self, mut originalImage: Option<Rc<RefCell<Image>>>, mut scaleX: f32, mut scaleY: f32) -> Option<Rc<RefCell<Image>>> {
        let mut image: Option<Rc<RefCell<Image>>> = jcall!(ImageCreationUtil::ImageCreationUtil::getInstance()).createImageWH((((jcall!(originalImage).getWidth() * scaleX)) as i32), (((jcall!(originalImage).getHeight() * scaleY)) as i32));
        if jcall!(image).isMutable() {
            let mut originalPlayNImage: Option<Rc<RefCell<Image>>> = None;
            if jcall!(originalImage).isMutable() {
                let mut originalHTMLImage: Option<Rc<RefCell<PlaynMutableImage>>> = (originalImage).clone() /* as PlaynMutableImage */;
                originalPlayNImage = (jcall!(originalHTMLImage).getImage()).clone() /* as Image */;
            } else {
                let mut originalHTMLImage: Option<Rc<RefCell<PlaynImmutableImage>>> = (originalImage).clone() /* as PlaynImmutableImage */;
                originalPlayNImage = (jcall!(originalHTMLImage).getImage()).clone() /* as Image */;
            }
            let mut htmlImage: Option<Rc<RefCell<PlaynMutableImage>>> = (image).clone() /* as PlaynMutableImage */;
            let mut canvasImage: Option<Rc<RefCell<CanvasImage>>> = (jcall!(htmlImage).getImage()).clone() /* as CanvasImage */;
            let mut canvasSurface: Option<Rc<RefCell<CanvasSurface>>> = jcall!(htmlImage).getCanvasSurface(canvasImage.clone());
            jcall!(canvasSurface).drawImage(originalPlayNImage.clone(), 0, 0, jcall!(image).getWidth(), jcall!(image).getHeight(), 0, 0, jcall!(originalImage).getWidth(), jcall!(originalImage).getHeight());
            return image.clone();
        } else {
            panic!("{:?}", format!("{}: {:?}", "Exception", (String::from("Not Mutable"))));
        }
    }
}

