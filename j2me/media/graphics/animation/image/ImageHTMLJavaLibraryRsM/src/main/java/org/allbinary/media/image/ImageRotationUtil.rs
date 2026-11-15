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
use crate::javax::microedition::lcdui::Image::Image;
use crate::org::microemu::device::playn::PlaynImmutableImage::PlaynImmutableImage;
use crate::org::microemu::device::playn::PlaynMutableImage::PlaynMutableImage;
use crate::playn::core::CanvasImage::CanvasImage;
use crate::playn::core::CanvasSurface::CanvasSurface;
use crate::java::lang::Exception::*;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct ImageRotationUtil {
}

pub static ImageRotationUtil_instance: crate::JavaStatic<Option<Rc<RefCell<ImageRotationUtil>>>> = crate::JavaStatic::new(|| ImageRotationUtil::new());

impl ImageRotationUtil {

    pub fn toString(&mut self) -> String {
        String::from(std::any::type_name::<Self>())
    }
    

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
    

    // throws Exception
    pub fn createRotatedImage(&mut self, mut originalImage: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Image::ImageDyn>>>, mut rotationInDegrees: i32) -> Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Image::ImageDyn>>> {
        let mut image: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Image::ImageDyn>>> = jcall!(ImageCreationUtil::ImageCreationUtil::getInstance()).getInstance(jcall!(originalImage).getWidth(), jcall!(originalImage).getHeight());
        if jcall!(image).isMutable() {
            let mut originalPlayNImage: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Image::ImageDyn>>> = None;
            if jcall!(originalImage).isMutable() {
                let mut originalHTMLImage: Option<Rc<RefCell<PlaynMutableImage>>> = (originalImage).clone() /* as PlaynMutableImage */;
                originalPlayNImage = { let java_value = ((jcall!(originalHTMLImage).getImage()).clone()).clone(); java_value };
            } else {
                let mut originalHTMLImage: Option<Rc<RefCell<PlaynImmutableImage>>> = (originalImage).clone() /* as PlaynImmutableImage */;
                originalPlayNImage = { let java_value = ((jcall!(originalHTMLImage).getImage()).clone()).clone(); java_value };
            }
            let mut htmlImage: Option<Rc<RefCell<PlaynMutableImage>>> = (image).clone() /* as PlaynMutableImage */;
            let mut canvasImage: Option<Rc<RefCell<CanvasImage>>> = (jcall!(htmlImage).getImage()).clone() /* as CanvasImage */;
            let mut canvasSurface: Option<Rc<RefCell<CanvasSurface>>> = jcall!(htmlImage).getCanvasSurface(canvasImage.clone());
            jcall!(canvasSurface).translate(jcall!(originalPlayNImage).as_Image_mut().width() / 2, jcall!(originalPlayNImage).as_Image_mut().height() / 2);
            jcall!(canvasSurface).rotate(((((rotationInDegrees.clone()) as f64).to_radians()) as f32));
            jcall!(canvasSurface).drawImage(originalPlayNImage.clone(), -jcall!(originalPlayNImage).as_Image_mut().width() / 2, -jcall!(originalPlayNImage).as_Image_mut().height() / 2);
            return image.clone();
        } else {
            panic!("{:?}", format!("{}: {:?}", "Exception", (String::from("Not Mutable"))));
        }
    }
}

