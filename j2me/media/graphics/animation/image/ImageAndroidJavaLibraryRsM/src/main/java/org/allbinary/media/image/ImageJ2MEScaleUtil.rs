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
use crate::javax::microedition::lcdui::Image::Image;
use crate::org::allbinary::image::GameFeatureImageCacheFactory::GameFeatureImageCacheFactory;
use crate::java::lang::Exception::*;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct ImageJ2MEScaleUtil {
}

pub static ImageJ2MEScaleUtil_instance: crate::JavaStatic<Option<Rc<RefCell<ImageJ2MEScaleUtil>>>> = crate::JavaStatic::new(|| ImageJ2MEScaleUtil::new());

impl ImageJ2MEScaleUtil {

    pub fn toString(&mut self) -> String {
        String::from(std::any::type_name::<Self>())
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<ImageJ2MEScaleUtil>>> {
        return ImageJ2MEScaleUtil_instance.with(Clone::clone).clone();
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
    pub fn scaleArray(&mut self, mut images: Vec<Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Image::ImageDyn>>>>, mut width: i32, mut height: i32) -> Vec<Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Image::ImageDyn>>>> {
        for index in (0..(images.len() as i32)) {
            let mut image: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Image::ImageDyn>>> = images[(index) as usize].clone();
            images[(index) as usize] = jcall!(ImageScaleUtil::ImageScaleUtil::getInstance()).createImage2(GameFeatureImageCacheFactory::getInstance(), image.clone(), ((width) as f32) / 2.0_f32, ((jcall!(image).getWidth()) as f32), ((height) as f32), ((jcall!(image).getHeight()) as f32), false);
        }
        return images.clone();
    }
    

    // throws Exception
    pub fn scale(&mut self, mut image: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Image::ImageDyn>>>, mut width: i32, mut height: i32) -> Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Image::ImageDyn>>> {
        return jcall!(ImageScaleUtil::ImageScaleUtil::getInstance()).createImage2(GameFeatureImageCacheFactory::getInstance(), image.clone(), ((width) as f32), ((jcall!(image).getWidth()) as f32), ((height) as f32), ((jcall!(image).getHeight()) as f32), false);
    }
}

