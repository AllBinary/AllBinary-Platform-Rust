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
// use crate::java::io::IOException::IOException;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use super::ImageProcessorInput::ImageProcessorInput;
use super::ImageProcessedVisitor::ImageProcessedVisitor;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct ImageArrayRotationUtil {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub commonStrings: Option<Rc<RefCell<CommonStrings>>>,
    pub TOTAL_ANGLE: String,
}

thread_local! { pub static ImageArrayRotationUtil_instance: Option<Rc<RefCell<ImageArrayRotationUtil>>> = ImageArrayRotationUtil::new(); }

impl ImageArrayRotationUtil {

    pub fn new_struct() -> Self {
        let mut this = Self {
            logUtil: None,
            commonStrings: None,
            TOTAL_ANGLE: String::new(),
        };
        this.logUtil = LogUtil::getInstance();
        this.commonStrings = CommonStrings::getInstance();
        this.TOTAL_ANGLE = String::from("totalAngle: ");
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<ImageArrayRotationUtil>>> {
        return ImageArrayRotationUtil_instance.with(Clone::clone).clone();
    }
    

    // throws IOException
    pub fn process(&mut self, mut imageProcessorInput: Option<Rc<RefCell<ImageProcessorInput>>>, mut input: String, mut visitor: Option<Rc<RefCell<dyn ImageProcessedVisitor>>>) {
        let mut totalAngle: Option<i32> = None;
        if input == jcall!(self.commonStrings).UP {
            totalAngle = (-90).to_string();
        } else if input == jcall!(self.commonStrings).DOWN {
            totalAngle = (90).to_string();
        } else {
            totalAngle = (input.clone()).to_string();
        }
        let mut generatedBufferedImage: Option<Rc<RefCell<BufferedImage>>> = None;
        let mut bufferedImageArray: Vec<Option<Rc<RefCell<BufferedImage>>>> = jcall!(imageProcessorInput).getBufferedImageArray();
        for index in (0..(bufferedImageArray.len() as i32)) {
            jcall!(self.logUtil).putF(format!("{}{}", self.TOTAL_ANGLE, totalAngle), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).RUN.clone());
            generatedBufferedImage = jcall!(ImageJ2SERotationUtil::ImageJ2SERotationUtil::getInstance()).getRotatedImage(bufferedImageArray[(index) as usize].clone(), ((totalAngle).unwrap() as i32));
            jcall!(visitor).visit(generatedBufferedImage.clone(), input.clone(), index.clone());
        }
    }
}

