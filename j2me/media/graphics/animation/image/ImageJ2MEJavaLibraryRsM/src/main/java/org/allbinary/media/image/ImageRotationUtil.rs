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
use crate::org::allbinary::graphics::Anchor::Anchor;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct ImageRotationUtil {
    pub anchor: i32,
}

thread_local! { pub static ImageRotationUtil_instance: Option<Rc<RefCell<ImageRotationUtil>>> = ImageRotationUtil::new(); }

impl ImageRotationUtil {
    

    pub fn getInstance() -> Option<Rc<RefCell<ImageRotationUtil>>> {
        return ImageRotationUtil_instance.with(Clone::clone).clone();
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            anchor: 0,
        };
        this.anchor = unsafe { crate::org::allbinary::graphics::Anchor::Anchor_TOP_LEFT.clone() }.clone();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    // throws Exception
    pub fn createRotatedImage(&mut self, mut originalImage: Option<Rc<RefCell<Image>>>, mut rotationInDegrees: i32) -> Option<Rc<RefCell<Image>>> {
        return originalImage.clone();
    }
}

