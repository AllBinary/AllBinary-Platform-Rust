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

pub struct ImageRotationUtil {
}

thread_local! { pub static ImageRotationUtil_instance: Option<Rc<RefCell<ImageRotationUtil>>> = ImageRotationUtil::new(); }

impl ImageRotationUtil {
    

    pub fn getInstance() -> Option<Rc<RefCell<ImageRotationUtil>>> {
        return ImageRotationUtil_instance.with(Clone::clone).clone();
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn rotateImage(&mut self, mut originalImage: Option<Rc<RefCell<Image>>>, mut image: Option<Rc<RefCell<Image>>>, mut totalAngle: i32) -> Option<Rc<RefCell<Image>>> {
        if jcall!(image).isMutable() {
            let mut htmlImage: Option<Rc<RefCell<PlaynMutableImage>>> = (image).clone() /* as PlaynMutableImage */;
            let mut canvasImage: Option<Rc<RefCell<CanvasImage>>> = (jcall!(htmlImage).getImage()).clone() /* as CanvasImage */;
            let mut canvasSurface: Option<Rc<RefCell<CanvasSurface>>> = jcall!(htmlImage).getCanvasSurface(canvasImage.clone());
            jcall!(canvasSurface).translate(jcall!(originalImage).getWidth() / 2, jcall!(originalImage).getHeight() / 2);
            let mut image2: Option<Rc<RefCell<Image>>> = self.rotateImageCanvasSurface(originalImage.clone(), image.clone(), canvasSurface.clone(), totalAngle.clone());
            self.drawImage(originalImage.clone(), image.clone(), canvasSurface.clone());
            return image2.clone();
        } else {
            return None;
        }
    }
    

    pub fn rotateImageCanvasSurfaceClear(&mut self, mut originalImage: Option<Rc<RefCell<Image>>>, mut image: Option<Rc<RefCell<Image>>>, mut canvasSurface: Option<Rc<RefCell<CanvasSurface>>>, mut totalAngle: i32) -> Option<Rc<RefCell<Image>>> {
        if jcall!(image).isMutable() {
            jcall!(canvasSurface).translate(-jcall!(originalImage).getWidth() / 2, -jcall!(originalImage).getHeight() / 2);
            jcall!(canvasSurface).clear();
            jcall!(canvasSurface).translate(jcall!(originalImage).getWidth() / 2, jcall!(originalImage).getHeight() / 2);
            return self.rotateImageCanvasSurface(originalImage.clone(), image.clone(), canvasSurface.clone(), totalAngle.clone());
        } else {
            return None;
        }
    }
    

    pub fn rotateImageCanvasSurface(&mut self, mut originalImage: Option<Rc<RefCell<Image>>>, mut image: Option<Rc<RefCell<Image>>>, mut canvasSurface: Option<Rc<RefCell<CanvasSurface>>>, mut totalAngle: i32) -> Option<Rc<RefCell<Image>>> {
        if jcall!(image).isMutable() {
            jcall!(canvasSurface).rotate(((((totalAngle.clone()) as f64).to_radians()) as f32));
            return image.clone();
        } else {
            return None;
        }
    }
    

    pub fn drawImage(&mut self, mut originalImage: Option<Rc<RefCell<Image>>>, mut image: Option<Rc<RefCell<Image>>>, mut canvasSurface: Option<Rc<RefCell<CanvasSurface>>>) {
        let mut originalPlayNImage: Option<Rc<RefCell<Image>>> = None;
        if jcall!(originalImage).isMutable() {
            let mut originalHTMLImage: Option<Rc<RefCell<PlaynMutableImage>>> = (originalImage).clone() /* as PlaynMutableImage */;
            originalPlayNImage = (jcall!(originalHTMLImage).getImage()).clone() /* as Image */;
        } else {
            let mut originalHTMLImage: Option<Rc<RefCell<PlaynImmutableImage>>> = (originalImage).clone() /* as PlaynImmutableImage */;
            originalPlayNImage = (jcall!(originalHTMLImage).getImage()).clone() /* as Image */;
        }
        jcall!(canvasSurface).drawImage(originalPlayNImage.clone(), -jcall!(originalImage).getWidth() / 2, -jcall!(originalImage).getHeight() / 2);
    }
    

    // throws Exception
    pub fn createRotatedImage(&mut self, mut originalImage: Option<Rc<RefCell<Image>>>, mut rotationInDegrees: i32) -> Option<Rc<RefCell<Image>>> {
        let mut image: Option<Rc<RefCell<Image>>> = jcall!(ImageCreationUtil::ImageCreationUtil::getInstance()).createImageWH(jcall!(originalImage).getWidth(), jcall!(originalImage).getHeight());
        let mut rotatedImage: Option<Rc<RefCell<Image>>> = self.rotateImage(originalImage.clone(), image.clone(), rotationInDegrees.clone());
        if rotatedImage.is_none() {
            panic!("{:?}", format!("{}: {:?}", "Exception", (String::from("Not Mutable"))));
        }
        return rotatedImage.clone();
    }
}

