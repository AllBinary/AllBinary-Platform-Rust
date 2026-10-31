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
// package org.allbinary.media.image.comparison
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::java::awt::*;
// use crate::java::awt::image::BufferedImage::BufferedImage;
use crate::org::allbinary::graphics::GPoint::GPoint;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub trait ImageComparatorConstraintsInterface {
    

    fn getMaxNonMatchingPixelDeltas(&mut self) -> i32;
    

    fn isCollisionWithAvoidRectangles(&mut self, rectangle: Option<Rc<RefCell<Rectangle>>>) -> bool;
    

    // Java overloaded method: Rust needs a different name
    fn isCollisionWithAvoidRectangles(&mut self, point: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::GPoint::GPointDyn>>>) -> bool;
    

    fn isColorAllowed(&mut self, frame: i32, point: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::GPoint::GPointDyn>>>, color: Option<Rc<RefCell<Color>>>) -> bool;
    

    // throws Exception
    fn isImageValid(&mut self, bufferedImage: Option<Rc<RefCell<BufferedImage>>>) -> bool;
    

    fn isFrameAllowed(&mut self, frame: i32) -> bool;

    fn toString(&mut self) -> String { String::from(std::any::type_name::<Self>()) }
}


