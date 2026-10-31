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
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
use crate::org::allbinary::media::image::comparison::ImageComparisonResult::ImageComparisonResult;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct MotionRectangles {
    pub name: String,
    pub rectangleVector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>,
    pub imageComparisonResult: Option<Rc<RefCell<ImageComparisonResult>>>,
    pub MAX_PIXEL_DISTANCE_THRESHOLD: i32,
}


impl MotionRectangles {
    

    pub fn new_struct(mut name: String, mut imageComparisonResult: Option<Rc<RefCell<ImageComparisonResult>>>) -> Self {
        let mut this = Self {
            name: String::new(),
            rectangleVector: None,
            imageComparisonResult: None,
            MAX_PIXEL_DISTANCE_THRESHOLD: 0,
        };
        this.MAX_PIXEL_DISTANCE_THRESHOLD = 2;
        this.name = name.clone();
        this.setImageComparisonResult(imageComparisonResult.clone());
        this.setVector(BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayListD::BasicArrayListDDyn>>).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>));
        this
    }

    pub fn new(mut name: String, mut imageComparisonResult: Option<Rc<RefCell<ImageComparisonResult>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(name, imageComparisonResult))))
    }
    

    pub fn getVector(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        return self.rectangleVector.clone();
    }
    

    pub fn setVector(&mut self, mut rectangleVector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) {
        self.rectangleVector = rectangleVector.clone();
    }
    

    pub fn getName(&mut self) -> String {
        return self.name.clone();
    }
    

    pub fn getImageComparisonResult(&mut self) -> Option<Rc<RefCell<ImageComparisonResult>>> {
        return self.imageComparisonResult.clone();
    }
    

    pub fn setImageComparisonResult(&mut self, mut imageComparisonResult: Option<Rc<RefCell<ImageComparisonResult>>>) {
        self.imageComparisonResult = imageComparisonResult.clone();
    }
}
pub trait MotionRectanglesDyn {
    fn as_MotionRectangles(&self) -> &MotionRectangles;
    fn as_MotionRectangles_mut(&mut self) -> &mut MotionRectangles;
    fn getVector(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> { MotionRectangles::getVector(self.as_MotionRectangles_mut()) }
    fn setVector(&mut self, mut rectangleVector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) { MotionRectangles::setVector(self.as_MotionRectangles_mut(), rectangleVector) }
    fn getName(&mut self) -> String { MotionRectangles::getName(self.as_MotionRectangles_mut()) }
    fn getImageComparisonResult(&mut self) -> Option<Rc<RefCell<ImageComparisonResult>>> { MotionRectangles::getImageComparisonResult(self.as_MotionRectangles_mut()) }
    fn setImageComparisonResult(&mut self, mut imageComparisonResult: Option<Rc<RefCell<ImageComparisonResult>>>) { MotionRectangles::setImageComparisonResult(self.as_MotionRectangles_mut(), imageComparisonResult) }
}
impl MotionRectanglesDyn for MotionRectangles {
    fn as_MotionRectangles(&self) -> &MotionRectangles { self }
    fn as_MotionRectangles_mut(&mut self) -> &mut MotionRectangles { self }
}


