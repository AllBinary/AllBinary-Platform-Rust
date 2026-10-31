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
use crate::android::graphics::Bitmap::Bitmap;
use crate::android::graphics::Canvas::Canvas;
use crate::android::graphics::Color::Color;
use crate::android::graphics::Matrix::Matrix;
use crate::android::graphics::Paint::Paint;
// use crate::javax::microedition::lcdui::Image::Image;
use crate::org::microemu::android::device::AndroidImageInterface::AndroidImageInterface;
use super::graphics::graphics;
use super::android::android;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct AndroidImageUtil {
}

thread_local! { pub static AndroidImageUtil_instance: Option<Rc<RefCell<AndroidImageUtil>>> = AndroidImageUtil::new(); }

impl AndroidImageUtil {

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<AndroidImageUtil>>> {
        return AndroidImageUtil_instance.with(Clone::clone).clone();
    }
    

    pub fn paintBitmap(&mut self, mut bitmap: Option<Rc<RefCell<Bitmap>>>, mut originalBitmap: Option<Rc<RefCell<Bitmap>>>, mut paint: Option<Rc<RefCell<Paint>>>) {
        jcall!(bitmap).eraseColor(unsafe { crate::android::graphics::Color::Color_TRANSPARENT.clone() }.clone());
        let mut canvas: Option<Rc<RefCell<Canvas>>> = Canvas::new();
        jcall!(canvas).setBitmap(bitmap.clone());
        jcall!(canvas).drawBitmap(originalBitmap.clone(), 0.0_f32, 0.0_f32, paint.clone());
    }
    

    pub fn paint(&mut self, mut image: Option<Rc<RefCell<Image>>>, mut originalImage: Option<Rc<RefCell<Image>>>, mut paint: Option<Rc<RefCell<Paint>>>) {
        let mut androidImage: Option<Rc<RefCell<dyn AndroidImageInterface>>> = (image).clone() /* as AndroidImageInterface */;
        jcall!(jcall!(androidImage).getBitmap()).eraseColor(unsafe { crate::android::graphics::Color::Color_TRANSPARENT.clone() }.clone());
        let mut canvas: Option<Rc<RefCell<Canvas>>> = jcall!(androidImage).getCanvas();
        let mut originalAndroidImage: Option<Rc<RefCell<dyn AndroidImageInterface>>> = (originalImage).clone() /* as AndroidImageInterface */;
        jcall!(canvas).drawBitmap(jcall!(originalAndroidImage).getBitmap(), 0.0_f32, 0.0_f32, paint.clone());
    }
    

    pub fn rotate(&mut self, mut image: Option<Rc<RefCell<Image>>>, mut originalImage: Option<Rc<RefCell<Image>>>, mut matrix: Option<Rc<RefCell<Matrix>>>, mut paint: Option<Rc<RefCell<Paint>>>) {
        let mut androidImage: Option<Rc<RefCell<dyn AndroidImageInterface>>> = (image).clone() /* as AndroidImageInterface */;
        jcall!(jcall!(androidImage).getBitmap()).eraseColor(unsafe { crate::android::graphics::Color::Color_TRANSPARENT.clone() }.clone());
        let mut canvas: Option<Rc<RefCell<Canvas>>> = jcall!(androidImage).getCanvas();
        jcall!(canvas).concat(matrix.clone());
        let mut originalAndroidImage: Option<Rc<RefCell<dyn AndroidImageInterface>>> = (originalImage).clone() /* as AndroidImageInterface */;
        jcall!(canvas).drawBitmap(jcall!(originalAndroidImage).getBitmap(), 0.0_f32, 0.0_f32, paint.clone());
    }
}

