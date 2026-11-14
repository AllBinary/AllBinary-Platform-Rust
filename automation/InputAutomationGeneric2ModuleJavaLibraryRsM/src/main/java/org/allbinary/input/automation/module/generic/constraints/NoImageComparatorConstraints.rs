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
use crate::org::allbinary::graphics::GPoint::GPoint;
use crate::org::allbinary::input::automation::module::ImageComparatorConstraints::ImageComparatorConstraints;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct NoImageComparatorConstraints {
    pub base: ImageComparatorConstraints,
}

impl crate::org::allbinary::media::image::comparison::ImageComparatorConstraintsInterface::ImageComparatorConstraintsInterface for NoImageComparatorConstraints {
    fn getMaxNonMatchingPixelDeltas(&mut self) -> i32 { crate::org::allbinary::input::automation::module::ImageComparatorConstraints::ImageComparatorConstraints::getMaxNonMatchingPixelDeltas(crate::org::allbinary::input::automation::module::ImageComparatorConstraints::ImageComparatorConstraintsDyn::as_ImageComparatorConstraints_mut(self)) }
    fn isCollisionWithAvoidRectangles(&mut self, p1: Option<Rc<RefCell<crate::org::allbinary::graphics::Rectangle::Rectangle>>>) -> bool { crate::org::allbinary::media::image::comparison::ImageComparatorConstraintsInterface::ImageComparatorConstraintsInterface::isCollisionWithAvoidRectangles(&mut self.base, p1) }
    fn isCollisionWithAvoidRectangles(&mut self, p1: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::GPoint::GPointDyn>>>) -> bool { crate::org::allbinary::media::image::comparison::ImageComparatorConstraintsInterface::ImageComparatorConstraintsInterface::isCollisionWithAvoidRectangles(&mut self.base, p1) }
    fn isColorAllowed(&mut self, mut frame: i32, mut point: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::GPoint::GPointDyn>>>, mut color: Option<Rc<RefCell<Color>>>) -> bool { Self::isColorAllowed(self, frame, point, color) }
    fn isImageValid(&mut self, mut bufferedImage: Option<Rc<RefCell<BufferedImage>>>) -> bool { Self::isImageValid(self, bufferedImage) }
    fn isFrameAllowed(&mut self, p1: i32) -> bool { crate::org::allbinary::input::automation::module::ImageComparatorConstraints::ImageComparatorConstraints::isFrameAllowed(crate::org::allbinary::input::automation::module::ImageComparatorConstraints::ImageComparatorConstraintsDyn::as_ImageComparatorConstraints_mut(self), p1) }
}

impl NoImageComparatorConstraints {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: ImageComparatorConstraints::new_struct(1),
        };
        // super(...) is in the struct literal above
        { let java_arguments = (i32::MAX.clone(),); this.setMaxNonMatchingPixelDeltas(java_arguments.0) };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn isColorAllowed(&mut self, mut frame: i32, mut point: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::GPoint::GPointDyn>>>, mut color: Option<Rc<RefCell<Color>>>) -> bool {
        return true;
    }
    

    pub fn isImageValid(&mut self, mut bufferedImage: Option<Rc<RefCell<BufferedImage>>>) -> bool {
        return true;
    }
}
impl std::ops::Deref for NoImageComparatorConstraints {
    type Target = ImageComparatorConstraints;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for NoImageComparatorConstraints {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::input::automation::module::ImageComparatorConstraints::ImageComparatorConstraintsDyn for NoImageComparatorConstraints {
    fn as_ImageComparatorConstraints(&self) -> &crate::org::allbinary::input::automation::module::ImageComparatorConstraints::ImageComparatorConstraints { &self.base }
    fn as_ImageComparatorConstraints_mut(&mut self) -> &mut crate::org::allbinary::input::automation::module::ImageComparatorConstraints::ImageComparatorConstraints { &mut self.base }
    fn isColorAllowed(&mut self, mut frame: i32, mut point: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::GPoint::GPointDyn>>>, mut color: Option<Rc<RefCell<Color>>>) -> bool { Self::isColorAllowed(self, frame, point, color) }
    fn isImageValid(&mut self, mut bufferedImage: Option<Rc<RefCell<BufferedImage>>>) -> bool { Self::isImageValid(self, bufferedImage) }
}


