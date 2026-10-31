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
use super::ImageRotationUtil::ImageRotationUtil;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct ImageToRotationImageArrayUtil {
}

thread_local! { pub static ImageToRotationImageArrayUtil_instance: Option<Rc<RefCell<ImageToRotationImageArrayUtil>>> = ImageToRotationImageArrayUtil::new(); }

impl ImageToRotationImageArrayUtil {
    

    pub fn getInstance() -> Option<Rc<RefCell<ImageToRotationImageArrayUtil>>> {
        return ImageToRotationImageArrayUtil_instance.with(Clone::clone).clone();
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
    pub fn generate(&mut self, mut image: Option<Rc<RefCell<Image>>>, mut angleIncrement: i32, mut totalAngle: i32) -> Vec<Option<Rc<RefCell<Image>>>> {
        let mut imageRotationUtil: Option<Rc<RefCell<ImageRotationUtil>>> = ImageRotationUtil::getInstance();
        let mut totalFrames: i32 = totalAngle / angleIncrement;
        let mut imageArray: Vec<Option<Rc<RefCell<Image>>>> = vec![None; (totalFrames) as usize];
        imageArray[(0) as usize] = image.clone();
        for index in (1..totalFrames) {
            imageArray[(index) as usize] = jcall!(imageRotationUtil).createRotatedImage(image.clone(), index * angleIncrement);
        }
        return imageArray.clone();
    }
}

