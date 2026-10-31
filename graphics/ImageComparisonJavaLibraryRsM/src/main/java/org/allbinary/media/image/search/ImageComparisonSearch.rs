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
// package org.allbinary.media.image.search
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::java::awt::image::BufferedImage::BufferedImage;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
use crate::org::allbinary::logic::StdUtil::StdUtil;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::media::image::ImageUtil::ImageUtil;
use crate::org::allbinary::media::image::comparison::pixel::PixelDelta::PixelDelta;
use crate::org::allbinary::string::CommonStrings::CommonStrings;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct ImageComparisonSearch {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub commonStrings: Option<Rc<RefCell<CommonStrings>>>,
    pub isSameHeight: bool,
    pub isSameWidth: bool,
    pub pixelsThatMatch: i32,
    pub pixelsIgnored: i32,
    pub imageHeight: i32,
    pub imageWidth: i32,
    pub tolerance: i32,
    pub nonMatchingPixelVector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>,
    pub bufferedImages: Vec<Option<Rc<RefCell<BufferedImage>>>>,
    pub matchingPercent: f32,
}


impl ImageComparisonSearch {
    

    pub fn new_struct(mut bufferedImage: Option<Rc<RefCell<BufferedImage>>>, mut bufferedImage2: Option<Rc<RefCell<BufferedImage>>>, mut tolerance: i32) -> Self {
        let mut this = Self {
            logUtil: None,
            commonStrings: None,
            isSameHeight: false,
            isSameWidth: false,
            pixelsThatMatch: 0,
            pixelsIgnored: 0,
            imageHeight: 0,
            imageWidth: 0,
            tolerance: 0,
            nonMatchingPixelVector: None,
            bufferedImages: Vec::new(),
            matchingPercent: 0.0_f32,
        };
        this.logUtil = LogUtil::getInstance();
        this.commonStrings = CommonStrings::getInstance();
        this.matchingPercent = ((-1) as f32);
        this.nonMatchingPixelVector = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        this.setBufferedImages(vec![None; (2) as usize]);
        this.getBufferedImages()[(0) as usize] = bufferedImage.clone();
        this.getBufferedImages()[(1) as usize] = bufferedImage2.clone();
        this.imageHeight = jcall!(bufferedImage).getHeight();
        this.imageWidth = jcall!(bufferedImage).getWidth();
        if !match (&jcall!(bufferedImage).getHeight(), &jcall!(bufferedImage2).getHeight()) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            this.isSameHeight = false;
            if this.imageHeight > jcall!(bufferedImage2).getHeight() {
                this.imageHeight = jcall!(bufferedImage2).getHeight();
            }
        } else {
            this.isSameHeight = true;
        }
        if !match (&jcall!(bufferedImage).getWidth(), &jcall!(bufferedImage2).getWidth()) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            this.isSameWidth = false;
            if this.imageWidth > jcall!(bufferedImage2).getWidth() {
                this.imageWidth = jcall!(bufferedImage2).getWidth();
            }
        } else {
            this.isSameWidth = true;
        }
        if !this.isSameWidth || !this.isSameHeight {
            let mut imageUtil: Option<Rc<RefCell<ImageUtil>>> = ImageUtil::getInstance();
            jcall!(this.logUtil).putF(String::from("Images were not the same size? Most likely a resolution change."), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(this.commonStrings).CONSTRUCTOR.clone());
            { let java_receiver = this.logUtil.clone(); let java_arguments = (format!("{}{}", String::from("1: "), jcall!(imageUtil).toString(bufferedImage.clone())), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(this.commonStrings).CONSTRUCTOR.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
            { let java_receiver = this.logUtil.clone(); let java_arguments = (format!("{}{}", String::from("2: "), jcall!(imageUtil).toString(bufferedImage2.clone())), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(this.commonStrings).CONSTRUCTOR.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        }
        this
    }

    pub fn new(mut bufferedImage: Option<Rc<RefCell<BufferedImage>>>, mut bufferedImage2: Option<Rc<RefCell<BufferedImage>>>, mut tolerance: i32) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(bufferedImage, bufferedImage2, tolerance))))
    }
    

    pub fn getMatchingPercent(&mut self) -> f32 {
        if self.matchingPercent == ((-1) as f32) {
            self.matchingPercent = ((self.pixelsThatMatch) as f32) / (((self.imageWidth * self.imageHeight)) as f32);
        }
        return self.matchingPercent.clone();
    }
    

    pub fn add(&mut self, mut pixel: Option<Rc<RefCell<PixelDelta>>>) {
        jcall!(self.nonMatchingPixelVector).add(crate::java_into_object(pixel.clone()));
    }
    

    pub fn getNonMatchingPixelVector(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        return self.nonMatchingPixelVector.clone();
    }
    

    pub fn toString(&mut self) -> String {
        return format!("{}{}", format!("{}{}", format!("{}{}", format!("{}{}", format!("{}{}", format!("{}{}", format!("{}{}", format!("{}{}", String::from("ImageComparisonInfo: "), String::from(" Number Of Matching Pixels: ")), self.pixelsThatMatch), String::from("\nNumber Of Non-Matching Pixels: ")), jcall!(self.getNonMatchingPixelVector()).size()), String::from("\nNumber Of Pixels Ignored: ")), self.pixelsIgnored), String::from("\nMatching Percentage: ")), self.getMatchingPercent());
    }
    

    pub fn getBufferedImages(&mut self) -> Vec<Option<Rc<RefCell<BufferedImage>>>> {
        return self.bufferedImages.clone();
    }
    

    pub fn setBufferedImages(&mut self, mut bufferedImages: Vec<Option<Rc<RefCell<BufferedImage>>>>) {
        self.bufferedImages = bufferedImages.clone();
    }
    

    pub fn getTolerance(&mut self) -> i32 {
        return self.tolerance.clone();
    }
    

    pub fn setTolerance(&mut self, mut tolerance: i32) {
        self.tolerance = tolerance.clone();
    }
}

