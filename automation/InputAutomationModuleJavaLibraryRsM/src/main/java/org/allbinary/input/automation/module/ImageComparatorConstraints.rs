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
// use crate::java::awt::Color::Color;
// use crate::java::awt::Rectangle::Rectangle;
// use crate::java::awt::image::BufferedImage::BufferedImage;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
use crate::org::allbinary::collision::RectangleCollisionUtil::RectangleCollisionUtil;
use crate::org::allbinary::graphics::GPoint::GPoint;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::media::image::comparison::ImageComparatorConstraintsInterface::ImageComparatorConstraintsInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct ImageComparatorConstraints {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub avoidVector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>,
    pub maxNonMatchingPixelDeltas: i32,
    pub doImageComparisonEveryNthFrame: i32,
}

// impl ImageComparatorConstraintsInterface for ImageComparatorConstraints with the methods of the trait

impl ImageComparatorConstraints {
    

    pub fn new_struct(mut doImageComparisonEveryNthFrame: i32) -> Self {
        let mut this = Self {
            logUtil: None,
            avoidVector: None,
            maxNonMatchingPixelDeltas: 0,
            doImageComparisonEveryNthFrame: 0,
        };
        this.logUtil = LogUtil::getInstance();
        this.doImageComparisonEveryNthFrame = doImageComparisonEveryNthFrame.clone();
        this.setAvoidVector(BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayListD::BasicArrayListDDyn>>).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>));
        this
    }

    pub fn new(mut doImageComparisonEveryNthFrame: i32) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(doImageComparisonEveryNthFrame))))
    }
    

    pub fn getAvoidVector(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        return self.avoidVector.clone();
    }
    

    pub fn isColorAllowed(&mut self, mut frame: i32, mut point: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::GPoint::GPointDyn>>>, mut color: Option<Rc<RefCell<Color>>>) -> bool {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    pub fn isCollisionWithAvoidRectangles(&mut self, mut rectangle: Option<Rc<RefCell<Rectangle>>>) -> bool {
        let mut isCollsionWithAvoidRectangles: bool = false;
        let mut avoidVector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = self.getAvoidVector();
        let mut size: i32 = jcall!(avoidVector).size();
        for index in (0..size) {
            let mut avoidRectangle: Option<Rc<RefCell<Rectangle>>> = (jcall!(avoidVector).get(index.clone())).clone() /* as Rectangle */;
            if RectangleCollisionUtil::isCollision(avoidRectangle.clone(), rectangle.clone()) {
                isCollsionWithAvoidRectangles = true;
            }
        }
        return isCollsionWithAvoidRectangles.clone();
    }
    

    // Java overloaded method: Rust needs a different name
    pub fn isCollisionWithAvoidRectangles(&mut self, mut point: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::GPoint::GPointDyn>>>) -> bool {
        let mut isCollsionWithAvoidRectangles: bool = false;
        let mut avoidVector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = self.getAvoidVector();
        let mut size: i32 = jcall!(avoidVector).size();
        for index in (0..size) {
            let mut avoidRectangle: Option<Rc<RefCell<Rectangle>>> = (jcall!(avoidVector).get(index.clone())).clone() /* as Rectangle */;
            if RectangleCollisionUtil::isCollision(avoidRectangle.clone(), point.clone()) {
                isCollsionWithAvoidRectangles = true;
                break;
            }
        }
        return isCollsionWithAvoidRectangles.clone();
    }
    

    pub fn setAvoidVector(&mut self, mut avoidVector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) {
        self.avoidVector = avoidVector.clone();
    }
    

    pub fn getMaxNonMatchingPixelDeltas(&mut self) -> i32 {
        return self.maxNonMatchingPixelDeltas.clone();
    }
    

    pub fn setMaxNonMatchingPixelDeltas(&mut self, mut maxNonMatchingPixelDeltas: i32) {
        self.maxNonMatchingPixelDeltas = maxNonMatchingPixelDeltas.clone();
    }
    

    pub fn isFrameAllowed(&mut self, mut frame: i32) -> bool {
        let mut remainder: i32 = ((frame + 1) % self.doImageComparisonEveryNthFrame);
        jcall!(self.logUtil).putF(format!("{}{}", format!("{}{}", format!("{}{}", format!("{}{}", format!("{}{}", String::from(" Frame: "), frame), String::from(" remainder: ")), remainder), String::from(" this.doImageComparisonEveryNthFrame: ")), self.doImageComparisonEveryNthFrame), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("isCollisionWithAvoidRectangles"));
        if remainder == 0 {
            return true;
        } else {
            return false;
        }
    }
    

    // throws Exception
    pub fn isImageValid(&mut self, mut bufferedImage: Option<Rc<RefCell<BufferedImage>>>) -> bool {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    pub fn log(&mut self) {
        let mut avoidVector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = self.getAvoidVector();
        let mut size: i32 = jcall!(avoidVector).size();
        for index in (0..size) {
            let mut avoidRectangle: Option<Rc<RefCell<Rectangle>>> = (jcall!(avoidVector).get(index.clone())).clone() /* as Rectangle */;
            jcall!(self.logUtil).putF(format!("{}{}", String::from("Avoid Rectangle: "), avoidRectangle), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("log"));
        }
    }
}
pub trait ImageComparatorConstraintsDyn {
    fn as_ImageComparatorConstraints(&self) -> &ImageComparatorConstraints;
    fn as_ImageComparatorConstraints_mut(&mut self) -> &mut ImageComparatorConstraints;
    fn getAvoidVector(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> { ImageComparatorConstraints::getAvoidVector(self.as_ImageComparatorConstraints_mut()) }
    fn isColorAllowed(&mut self, mut frame: i32, mut point: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::GPoint::GPointDyn>>>, mut color: Option<Rc<RefCell<Color>>>) -> bool { ImageComparatorConstraints::isColorAllowed(self.as_ImageComparatorConstraints_mut(), frame, point, color) }
    fn setAvoidVector(&mut self, mut avoidVector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) { ImageComparatorConstraints::setAvoidVector(self.as_ImageComparatorConstraints_mut(), avoidVector) }
    fn getMaxNonMatchingPixelDeltas(&mut self) -> i32 { ImageComparatorConstraints::getMaxNonMatchingPixelDeltas(self.as_ImageComparatorConstraints_mut()) }
    fn setMaxNonMatchingPixelDeltas(&mut self, mut maxNonMatchingPixelDeltas: i32) { ImageComparatorConstraints::setMaxNonMatchingPixelDeltas(self.as_ImageComparatorConstraints_mut(), maxNonMatchingPixelDeltas) }
    fn isFrameAllowed(&mut self, mut frame: i32) -> bool { ImageComparatorConstraints::isFrameAllowed(self.as_ImageComparatorConstraints_mut(), frame) }
    fn isImageValid(&mut self, mut bufferedImage: Option<Rc<RefCell<BufferedImage>>>) -> bool { ImageComparatorConstraints::isImageValid(self.as_ImageComparatorConstraints_mut(), bufferedImage) }
    fn log(&mut self) { ImageComparatorConstraints::log(self.as_ImageComparatorConstraints_mut()) }
}
impl ImageComparatorConstraintsDyn for ImageComparatorConstraints {
    fn as_ImageComparatorConstraints(&self) -> &ImageComparatorConstraints { self }
    fn as_ImageComparatorConstraints_mut(&mut self) -> &mut ImageComparatorConstraints { self }
}


