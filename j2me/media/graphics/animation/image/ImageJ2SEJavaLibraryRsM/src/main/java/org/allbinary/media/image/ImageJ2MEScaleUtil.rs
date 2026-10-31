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

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct ImageJ2MEScaleUtil {
}

thread_local! { pub static ImageJ2MEScaleUtil_instance: Option<Rc<RefCell<ImageJ2MEScaleUtil>>> = ImageJ2MEScaleUtil::new(); }

impl ImageJ2MEScaleUtil {
    

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
        let mut sourceWidth: i32 = jcall!(image).getWidth();
        let mut sourceHeight: i32 = jcall!(image).getHeight();
        let mut originalData: Vec<i32> = vec![0; (jcall!(image).getWidth() * jcall!(image).getHeight()) as usize];
        jcall!(image).getRGB(originalData.clone(), 0, jcall!(image).getWidth(), 0, 0, jcall!(image).getWidth(), jcall!(image).getHeight());
        let mut scaledData: Vec<i32> = vec![0; (width * height) as usize];
        let mut heightRatioFactor: i32 = (8 * sourceHeight) / height;
        let mut widthRatioFactor: i32 = (8 * sourceWidth) / width;
        let mut scaledIndex: i32 = (scaledData.len() as i32) - 1;
        let mut dx: i32 = 0;
        let mut dy: i32 = 0;
        {
            let mut index: i32 = height.clone();
            while { index -= 1; index } >= 0 {
            dy = (index * heightRatioFactor) >> 3;
            {
                let mut index2: i32 = width.clone();
                while { index2 -= 1; index2 } >= 0 {
                dx = (index2 * widthRatioFactor) >> 3;
                scaledData[({ let value = scaledIndex; scaledIndex -= 1; value }) as usize] = originalData[((sourceWidth * dy) + dx) as usize].clone();
                }
            }
            }
        }
        let mut scaledImage: Option<Rc<RefCell<Image>>> = Image::createRGBImage(scaledData.clone(), width.clone(), height.clone(), true);
        return scaledImage.clone();
    }
}

