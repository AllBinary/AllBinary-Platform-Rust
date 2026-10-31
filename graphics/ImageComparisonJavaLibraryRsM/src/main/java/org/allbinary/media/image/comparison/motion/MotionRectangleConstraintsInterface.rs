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
// package org.allbinary.media.image.comparison.motion
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::java::awt::*;
// use crate::java::awt::image::BufferedImage::BufferedImage;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub trait MotionRectangleConstraintsInterface {
    

    fn getMaxMotionRectangles(&mut self) -> i32;
    

    fn getMinDimension(&mut self) -> Option<Rc<RefCell<Dimension>>>;
    

    fn getMinArea(&mut self) -> i32;
    

    fn getMaxDimension(&mut self) -> Option<Rc<RefCell<Dimension>>>;
    

    fn isTooSmall(&mut self, rectangle: Option<Rc<RefCell<Rectangle>>>) -> bool;
    

    fn isAreaTooSmall(&mut self, rectangle: Option<Rc<RefCell<Rectangle>>>) -> bool;
    

    fn isTooBig(&mut self, rectangle: Option<Rc<RefCell<Rectangle>>>) -> bool;
    

    // throws Exception
    fn isValid(&mut self, frame: Option<i64>, bufferedImage: Option<Rc<RefCell<BufferedImage>>>, rectangle: Option<Rc<RefCell<Rectangle>>>) -> bool;

    fn toString(&mut self) -> String { String::from(std::any::type_name::<Self>()) }
}


