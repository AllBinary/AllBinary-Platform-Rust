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
// use crate::javax::microedition::lcdui::Graphics::Graphics;
// use crate::javax::microedition::lcdui::Image::Image;
use crate::org::allbinary::graphics::Anchor::Anchor;
use crate::org::allbinary::graphics::color::BasicColorFactory::BasicColorFactory;
use crate::org::allbinary::graphics::color::BasicColorSetUtil::BasicColorSetUtil;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use super::ImageCreationUtil::ImageCreationUtil;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct ImageCopyUtil {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub imageCreationUtil: Option<Rc<RefCell<ImageCreationUtil>>>,
    pub anchor: i32,
}

thread_local! { pub static ImageCopyUtil_instance: Option<Rc<RefCell<ImageCopyUtil>>> = ImageCopyUtil::new(); }

impl ImageCopyUtil {
    

    pub fn getInstance() -> Option<Rc<RefCell<ImageCopyUtil>>> {
        return ImageCopyUtil_instance.with(Clone::clone).clone();
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            logUtil: None,
            imageCreationUtil: None,
            anchor: 0,
        };
        this.logUtil = LogUtil::getInstance();
        this.imageCreationUtil = ImageCreationUtil::getInstance();
        this.anchor = unsafe { crate::org::allbinary::graphics::Anchor::Anchor_TOP_LEFT.clone() }.clone();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    // throws Exception
    pub fn createImageForRotation(&mut self, mut originalImage: Option<Rc<RefCell<Image>>>) -> Option<Rc<RefCell<Image>>> {
        return self.createImage(originalImage.clone());
    }
    

    // throws Exception
    pub fn createImage(&mut self, mut originalImage: Option<Rc<RefCell<Image>>>) -> Option<Rc<RefCell<Image>>> {
        let mut image: Option<Rc<RefCell<Image>>> = { let java_receiver = self.imageCreationUtil.clone(); let java_arguments = (jcall!(originalImage).getWidth(), jcall!(originalImage).getHeight(),); let java_result = jcall!(java_receiver).getInstance(java_arguments.0, java_arguments.1); java_result };
        if jcall!(image).isMutable() {
            jcall!(jcall!(image).getGraphics()).drawImage(originalImage.clone(), 0, 0, self.anchor.clone());
            return image.clone();
        } else {
            panic!("{:?}", format!("{}: {:?}", "Exception", (String::from("Not Mutable"))));
        }
    }
    

    // throws Exception
    pub fn createImageScale(&mut self, mut originalImage: Option<Rc<RefCell<Image>>>, mut canvasScale: f32, mut resize: bool) -> Option<Rc<RefCell<Image>>> {
        let mut newWidth: i32 = (((jcall!(originalImage).getWidth() * canvasScale)) as i32);
        let mut newHeight: i32 = (((jcall!(originalImage).getHeight() * canvasScale)) as i32);
        if resize {
            if newWidth < newHeight {
                newWidth = newHeight.clone();
            }
            if newHeight < newWidth {
                newHeight = newWidth.clone();
            }
        }
        let mut image: Option<Rc<RefCell<Image>>> = jcall!(self.imageCreationUtil).getInstance(newWidth.clone(), newHeight.clone());
        if jcall!(image).isMutable() {
            let mut halfWidthDelta: i32 = (newWidth - jcall!(originalImage).getWidth()) / 2;
            let mut halfHeightDelta: i32 = (newHeight - jcall!(originalImage).getHeight()) / 2;
            let mut graphics: Option<Rc<RefCell<Graphics>>> = jcall!(image).getGraphics();
            jcall!(graphics).drawImage(originalImage.clone(), halfWidthDelta.clone(), halfHeightDelta.clone(), self.anchor.clone());
            return image.clone();
        } else {
            panic!("{:?}", format!("{}: {:?}", "Exception", (String::from("Not Mutable"))));
        }
    }
}

