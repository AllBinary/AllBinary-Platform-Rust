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
use crate::javax::microedition::lcdui::Image::Image;
use crate::org::microemu::android::device::AndroidImageInterface::AndroidImageInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct AndroidImageUtil {
}

pub static AndroidImageUtil_instance: crate::JavaStatic<Option<Rc<RefCell<AndroidImageUtil>>>> = crate::JavaStatic::new(|| AndroidImageUtil::new());

impl AndroidImageUtil {

    pub fn toString(&mut self) -> String {
        String::from(std::any::type_name::<Self>())
    }

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
        let mut canvas: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Canvas::CanvasDyn>>> = Canvas::new().map(|reference| reference as Rc<RefCell<dyn crate::javax::microedition::lcdui::Canvas::CanvasDyn>>);
        jcall!(canvas).as_Canvas_mut().setBitmap(bitmap.clone());
        jcall!(canvas).as_Canvas_mut().drawBitmap(originalBitmap.clone(), 0.0_f32, 0.0_f32, paint.clone());
    }
    

    pub fn paint(&mut self, mut image: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Image::ImageDyn>>>, mut originalImage: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Image::ImageDyn>>>, mut paint: Option<Rc<RefCell<Paint>>>) {
        let mut androidImage: Option<Rc<RefCell<dyn AndroidImageInterface>>> = (|| -> Option<Rc<RefCell<dyn AndroidImageInterface>>> { panic!("ClassCastException: a Rust dyn Image cannot be cast to AndroidImageInterface") })();
        jcall!(jcall!(androidImage).getBitmap()).eraseColor(unsafe { crate::android::graphics::Color::Color_TRANSPARENT.clone() }.clone());
        let mut canvas: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Canvas::CanvasDyn>>> = jcall!(androidImage).getCanvas();
        let mut originalAndroidImage: Option<Rc<RefCell<dyn AndroidImageInterface>>> = (|| -> Option<Rc<RefCell<dyn AndroidImageInterface>>> { panic!("ClassCastException: a Rust dyn Image cannot be cast to AndroidImageInterface") })();
        jcall!(canvas).as_Canvas_mut().drawBitmap(jcall!(originalAndroidImage).getBitmap(), 0.0_f32, 0.0_f32, paint.clone());
    }
    

    pub fn rotate(&mut self, mut image: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Image::ImageDyn>>>, mut originalImage: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Image::ImageDyn>>>, mut matrix: Option<Rc<RefCell<Matrix>>>, mut paint: Option<Rc<RefCell<Paint>>>) {
        let mut androidImage: Option<Rc<RefCell<dyn AndroidImageInterface>>> = (|| -> Option<Rc<RefCell<dyn AndroidImageInterface>>> { panic!("ClassCastException: a Rust dyn Image cannot be cast to AndroidImageInterface") })();
        jcall!(jcall!(androidImage).getBitmap()).eraseColor(unsafe { crate::android::graphics::Color::Color_TRANSPARENT.clone() }.clone());
        let mut canvas: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Canvas::CanvasDyn>>> = jcall!(androidImage).getCanvas();
        jcall!(canvas).as_Canvas_mut().concat(matrix.clone());
        let mut originalAndroidImage: Option<Rc<RefCell<dyn AndroidImageInterface>>> = (|| -> Option<Rc<RefCell<dyn AndroidImageInterface>>> { panic!("ClassCastException: a Rust dyn Image cannot be cast to AndroidImageInterface") })();
        jcall!(canvas).as_Canvas_mut().drawBitmap(jcall!(originalAndroidImage).getBitmap(), 0.0_f32, 0.0_f32, paint.clone());
    }
}

