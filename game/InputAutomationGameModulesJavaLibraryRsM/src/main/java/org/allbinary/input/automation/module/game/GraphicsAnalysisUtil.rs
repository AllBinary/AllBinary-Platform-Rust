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
// package org.allbinary.input.automation.module.game
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::java::awt::*;
// use crate::java::awt::image::BufferedImage::BufferedImage;
use crate::org::allbinary::graphics::color::ColorCacheFactory::ColorCacheFactory;
use crate::org::allbinary::graphics::color::ColorCacheable::ColorCacheable;
use crate::org::allbinary::logic::util::cache::AutomaticCacheInterface::AutomaticCacheInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct GraphicsAnalysisUtil {
}

thread_local! { pub static GraphicsAnalysisUtil_MAX: i32 = 122; }

impl GraphicsAnalysisUtil {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    // throws Exception
    pub fn getNominator(mut bufferedImage: Option<Rc<RefCell<BufferedImage>>>, mut min_x: Option<i32>, mut max_x: Option<i32>, mut y: Option<i32>) -> f64 {
        for index in ((((min_x).unwrap() as i32)) + 1..=((max_x).unwrap() as i32)).rev() {
            let mut colorInteger: Option<i32> = (jcall!(bufferedImage).getRGB(index.clone(), ((y).unwrap() as i32))).to_string();
            let mut automaticCacheInterface: Option<Rc<RefCell<dyn AutomaticCacheInterface>>> = ColorCacheFactory::getInstance().map(|reference| reference as Rc<RefCell<dyn AutomaticCacheInterface>>);
            let mut colorCacheable: Option<Rc<RefCell<ColorCacheable>>> = (jcall!(automaticCacheInterface).get(crate::java_into_object((colorInteger.clone()).expect("NullPointerException")))).clone() /* as ColorCacheable */;
            let mut color: Option<Rc<RefCell<Color>>> = jcall!(colorCacheable).getColor();
            if jcall!(color).getRed() < GraphicsAnalysisUtil_MAX.with(Clone::clone) && jcall!(color).getGreen() < GraphicsAnalysisUtil_MAX.with(Clone::clone) && jcall!(color).getBlue() < GraphicsAnalysisUtil_MAX.with(Clone::clone) {
            } else {
                return ((index - ((min_x).unwrap() as i32)) as f64);
            }
        }
        return 0.0;
    }
}

