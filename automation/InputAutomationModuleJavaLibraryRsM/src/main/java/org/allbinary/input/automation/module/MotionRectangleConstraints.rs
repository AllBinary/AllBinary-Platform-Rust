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
// package org.allbinary.input.automation.module
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::java::awt::*;
// use crate::java::awt::image::BufferedImage::BufferedImage;
use crate::org::allbinary::media::image::comparison::motion::MotionRectangleConstraintsInterface::MotionRectangleConstraintsInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct MotionRectangleConstraints {
    pub minDimension: Option<Rc<RefCell<Dimension>>>,
    pub maxDimension: Option<Rc<RefCell<Dimension>>>,
    pub maxMotionRectangles: i32,
    pub minArea: i32,
}

impl MotionRectangleConstraintsInterface for MotionRectangleConstraints {
    fn getMaxMotionRectangles(&mut self) -> i32 { Self::getMaxMotionRectangles(self) }
    fn getMinDimension(&mut self) -> Option<Rc<RefCell<Dimension>>> { Self::getMinDimension(self) }
    fn getMinArea(&mut self) -> i32 { Self::getMinArea(self) }
    fn getMaxDimension(&mut self) -> Option<Rc<RefCell<Dimension>>> { Self::getMaxDimension(self) }
    fn isTooSmall(&mut self, mut rectangle: Option<Rc<RefCell<Rectangle>>>) -> bool { Self::isTooSmall(self, rectangle) }
    fn isAreaTooSmall(&mut self, mut rectangle: Option<Rc<RefCell<Rectangle>>>) -> bool { Self::isAreaTooSmall(self, rectangle) }
    fn isTooBig(&mut self, mut rectangle: Option<Rc<RefCell<Rectangle>>>) -> bool { Self::isTooBig(self, rectangle) }
    fn isValid(&mut self, mut frame: Option<i64>, mut bufferedImage: Option<Rc<RefCell<BufferedImage>>>, mut rectangle: Option<Rc<RefCell<Rectangle>>>) -> bool { Self::isValid(self, frame, bufferedImage, rectangle) }
}

impl MotionRectangleConstraints {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            minDimension: None,
            maxDimension: None,
            maxMotionRectangles: 0,
            minArea: 0,
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getMinDimension(&mut self) -> Option<Rc<RefCell<Dimension>>> {
        return self.minDimension.clone();
    }
    

    pub fn getMaxDimension(&mut self) -> Option<Rc<RefCell<Dimension>>> {
        return self.maxDimension.clone();
    }
    

    pub fn isTooSmall(&mut self, mut rectangle: Option<Rc<RefCell<Rectangle>>>) -> bool {
        if jcall!(rectangle).width < jcall!(self.getMinDimension()).width || jcall!(rectangle).height < jcall!(self.getMinDimension()).height {
            return true;
        } else {
            return false;
        }
    }
    

    pub fn isAreaTooSmall(&mut self, mut rectangle: Option<Rc<RefCell<Rectangle>>>) -> bool {
        if jcall!(rectangle).width * jcall!(rectangle).height < self.getMinArea() {
            return true;
        } else {
            return false;
        }
    }
    

    pub fn isTooBig(&mut self, mut rectangle: Option<Rc<RefCell<Rectangle>>>) -> bool {
        if jcall!(rectangle).width > jcall!(self.getMaxDimension()).width || jcall!(rectangle).height > jcall!(self.getMaxDimension()).height {
            return true;
        } else {
            return false;
        }
    }
    

    // throws Exception
    pub fn isValid(&mut self, mut frame: Option<i64>, mut bufferedImage: Option<Rc<RefCell<BufferedImage>>>, mut rectangle: Option<Rc<RefCell<Rectangle>>>) -> bool {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    pub fn setMinDimension(&mut self, mut minDimension: Option<Rc<RefCell<Dimension>>>) {
        self.minDimension = minDimension.clone();
    }
    

    pub fn setMaxDimension(&mut self, mut maxDimension: Option<Rc<RefCell<Dimension>>>) {
        self.maxDimension = maxDimension.clone();
    }
    

    pub fn getMaxMotionRectangles(&mut self) -> i32 {
        return self.maxMotionRectangles.clone();
    }
    

    pub fn setMaxMotionRectangles(&mut self, mut maxMotionRectangles: i32) {
        self.maxMotionRectangles = maxMotionRectangles.clone();
    }
    

    pub fn getMinArea(&mut self) -> i32 {
        return self.minArea.clone();
    }
    

    pub fn setMinArea(&mut self, mut minArea: i32) {
        self.minArea = minArea.clone();
    }
}
pub trait MotionRectangleConstraintsDyn {
    fn as_MotionRectangleConstraints(&self) -> &MotionRectangleConstraints;
    fn as_MotionRectangleConstraints_mut(&mut self) -> &mut MotionRectangleConstraints;
    fn getMinDimension(&mut self) -> Option<Rc<RefCell<Dimension>>> { MotionRectangleConstraints::getMinDimension(self.as_MotionRectangleConstraints_mut()) }
    fn getMaxDimension(&mut self) -> Option<Rc<RefCell<Dimension>>> { MotionRectangleConstraints::getMaxDimension(self.as_MotionRectangleConstraints_mut()) }
    fn isTooSmall(&mut self, mut rectangle: Option<Rc<RefCell<Rectangle>>>) -> bool { MotionRectangleConstraints::isTooSmall(self.as_MotionRectangleConstraints_mut(), rectangle) }
    fn isAreaTooSmall(&mut self, mut rectangle: Option<Rc<RefCell<Rectangle>>>) -> bool { MotionRectangleConstraints::isAreaTooSmall(self.as_MotionRectangleConstraints_mut(), rectangle) }
    fn isTooBig(&mut self, mut rectangle: Option<Rc<RefCell<Rectangle>>>) -> bool { MotionRectangleConstraints::isTooBig(self.as_MotionRectangleConstraints_mut(), rectangle) }
    fn isValid(&mut self, mut frame: Option<i64>, mut bufferedImage: Option<Rc<RefCell<BufferedImage>>>, mut rectangle: Option<Rc<RefCell<Rectangle>>>) -> bool { MotionRectangleConstraints::isValid(self.as_MotionRectangleConstraints_mut(), frame, bufferedImage, rectangle) }
    fn setMinDimension(&mut self, mut minDimension: Option<Rc<RefCell<Dimension>>>) { MotionRectangleConstraints::setMinDimension(self.as_MotionRectangleConstraints_mut(), minDimension) }
    fn setMaxDimension(&mut self, mut maxDimension: Option<Rc<RefCell<Dimension>>>) { MotionRectangleConstraints::setMaxDimension(self.as_MotionRectangleConstraints_mut(), maxDimension) }
    fn getMaxMotionRectangles(&mut self) -> i32 { MotionRectangleConstraints::getMaxMotionRectangles(self.as_MotionRectangleConstraints_mut()) }
    fn setMaxMotionRectangles(&mut self, mut maxMotionRectangles: i32) { MotionRectangleConstraints::setMaxMotionRectangles(self.as_MotionRectangleConstraints_mut(), maxMotionRectangles) }
    fn getMinArea(&mut self) -> i32 { MotionRectangleConstraints::getMinArea(self.as_MotionRectangleConstraints_mut()) }
    fn setMinArea(&mut self, mut minArea: i32) { MotionRectangleConstraints::setMinArea(self.as_MotionRectangleConstraints_mut(), minArea) }
}
impl MotionRectangleConstraintsDyn for MotionRectangleConstraints {
    fn as_MotionRectangleConstraints(&self) -> &MotionRectangleConstraints { self }
    fn as_MotionRectangleConstraints_mut(&mut self) -> &mut MotionRectangleConstraints { self }
}


