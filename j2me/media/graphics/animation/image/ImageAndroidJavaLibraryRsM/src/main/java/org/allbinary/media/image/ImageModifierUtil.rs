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
use crate::android::graphics::Paint::Paint;
use crate::android::graphics::PorterDuff::PorterDuff;
use crate::android::graphics::PorterDuffColorFilter::PorterDuffColorFilter;
// use crate::javax::microedition::lcdui::Image::Image;
use crate::org::allbinary::graphics::color::BasicColor::BasicColor;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct ImageModifierUtil {
    pub paint: Option<Rc<RefCell<Paint>>>,
}


impl ImageModifierUtil {

    pub fn new_struct() -> Self {
        let mut this = Self {
            paint: None,
        };
        this.paint = Paint::new(unsafe { crate::android::graphics::Paint::Paint_ANTI_ALIAS_FLAG.clone() }.clone());
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstanceOrCreate() -> Option<Rc<RefCell<ImageModifierUtil>>> {
        return ImageModifierUtil::new();
    }
    

    pub fn setColor(&mut self, mut unusedOriginalImage: Option<Rc<RefCell<Image>>>, mut image: Option<Rc<RefCell<Image>>>, mut imageIndex: i32, mut basicColor: Option<Rc<RefCell<BasicColor>>>) {
        { let java_receiver = self.paint.clone(); let java_arguments = (PorterDuffColorFilter::new(jcall!(basicColor).intValue(), jcall!(unsafe { crate::android::graphics::PorterDuff::PorterDuff_Mode.clone() }).SRC_IN.clone()),); jcall!(java_receiver).setColorFilter(java_arguments.0); };
    }
    

    pub fn changeColor(&mut self, mut unusedOriginalImage: Option<Rc<RefCell<Image>>>, mut image: Option<Rc<RefCell<Image>>>, mut imageIndex: i32, mut basicColor: Option<Rc<RefCell<BasicColor>>>) {
        { let java_receiver = self.paint.clone(); let java_arguments = (PorterDuffColorFilter::new(jcall!(basicColor).intValue(), jcall!(unsafe { crate::android::graphics::PorterDuff::PorterDuff_Mode.clone() }).SRC_IN.clone()),); jcall!(java_receiver).setColorFilter(java_arguments.0); };
    }
    

    pub fn setAlpha(&mut self, mut originalImage: Option<Rc<RefCell<Image>>>, mut image: Option<Rc<RefCell<Image>>>, mut imageIndex: i32, mut alphaInt: i32) {
        if alphaInt != jcall!(self.paint).getAlpha() {
            jcall!(self.paint).setAlpha(alphaInt.clone());
        }
    }
    

    pub fn getImageArray(&mut self, mut originalImageArray: Vec<Option<Rc<RefCell<Image>>>>) -> Vec<Option<Rc<RefCell<Image>>>> {
        return originalImageArray.clone();
    }
    

    pub fn handleImage(&mut self, mut imageArray: Vec<Option<Rc<RefCell<Image>>>>, mut index: i32, mut image: Option<Rc<RefCell<Image>>>) {
    }
    

    pub fn reset(&mut self) {
    }
}

