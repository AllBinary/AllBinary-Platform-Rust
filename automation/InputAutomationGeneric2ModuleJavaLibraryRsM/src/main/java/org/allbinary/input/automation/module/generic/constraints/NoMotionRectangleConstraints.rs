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
// package org.allbinary.input.automation.module.generic.constraints
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::java::awt::*;
// use crate::java::awt::image::BufferedImage::BufferedImage;
use crate::org::allbinary::input::automation::module::MotionRectangleConstraints::MotionRectangleConstraints;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct NoMotionRectangleConstraints {
    pub base: MotionRectangleConstraints,
}


impl NoMotionRectangleConstraints {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: MotionRectangleConstraints::new_struct(),
        };
        this.setMinDimension(Dimension::new(0, 0));
        this.setMinArea(0);
        this.setMaxDimension(Dimension::new(i32::MAX.clone(), i32::MAX.clone()));
        this.setMaxMotionRectangles(i32::MAX.clone());
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn isValid(&mut self, mut frame: Option<i64>, mut bufferedImage: Option<Rc<RefCell<BufferedImage>>>, mut rectangle: Option<Rc<RefCell<Rectangle>>>) -> bool {
        return true;
    }
}
impl std::ops::Deref for NoMotionRectangleConstraints {
    type Target = MotionRectangleConstraints;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for NoMotionRectangleConstraints {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::input::automation::module::MotionRectangleConstraints::MotionRectangleConstraintsDyn for NoMotionRectangleConstraints {
    fn as_MotionRectangleConstraints(&self) -> &crate::org::allbinary::input::automation::module::MotionRectangleConstraints::MotionRectangleConstraints { &self.base }
    fn as_MotionRectangleConstraints_mut(&mut self) -> &mut crate::org::allbinary::input::automation::module::MotionRectangleConstraints::MotionRectangleConstraints { &mut self.base }
    fn isValid(&mut self, mut frame: Option<i64>, mut bufferedImage: Option<Rc<RefCell<BufferedImage>>>, mut rectangle: Option<Rc<RefCell<Rectangle>>>) -> bool { Self::isValid(self, frame, bufferedImage, rectangle) }
}


