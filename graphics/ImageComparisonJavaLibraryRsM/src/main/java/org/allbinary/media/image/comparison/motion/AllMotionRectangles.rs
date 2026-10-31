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
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
use crate::org::allbinary::media::image::comparison::ImageComparisonResult::ImageComparisonResult;
use crate::org::allbinary::media::image::comparison::pixel::PixelDelta::PixelDelta;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct AllMotionRectangles {
    pub base: MotionRectangles,
}

thread_local! { pub static AllMotionRectangles_NAME: String = String::from("allMotionRectangles_"); }

impl AllMotionRectangles {
    

    pub fn new_struct(mut imageComparisonResult: Option<Rc<RefCell<ImageComparisonResult>>>) -> Self {
        let mut this = Self {
            base: MotionRectangles::new_struct(AllMotionRectangles_NAME.with(Clone::clone).clone(), imageComparisonResult.clone()),
        };
        // super(...) is in the struct literal above
        this.createMotionRectangles(imageComparisonResult.clone());
        this
    }

    pub fn new(mut imageComparisonResult: Option<Rc<RefCell<ImageComparisonResult>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(imageComparisonResult))))
    }
    

    // throws Exception
    pub fn createMotionRectangles(&mut self, mut imageComparisonInfo: Option<Rc<RefCell<ImageComparisonResult>>>) {
        let mut vector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = jcall!(imageComparisonInfo).getNonMatchingPixelVector();
        let mut size: i32 = jcall!(vector).size();
        for index in (0..size) {
            let mut pixelDelta: Option<Rc<RefCell<PixelDelta>>> = { let java_object = (jcall!(vector).get(index.clone())).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<PixelDelta>>>>().expect("ClassCastException").clone()) };
            self.addPixelDeltaToAMotionRectangle(pixelDelta.clone());
        }
    }
    

    // throws Exception
    pub fn addPixelDeltaToExistingMotionRectangle(&mut self, mut pixelDelta: Option<Rc<RefCell<PixelDelta>>>) -> bool {
        let mut vector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = self.getVector();
        let mut size: i32 = jcall!(vector).size();
        for index in (0..size) {
            let mut motionRectangle: Option<Rc<RefCell<MotionRectangle>>> = { let java_object = (jcall!(vector).get(index.clone())).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<MotionRectangle>>>>().expect("ClassCastException").clone()) };
            if self.shouldPixelDeltaPartOfMotionRectangle(motionRectangle.clone(), pixelDelta.clone()) {
                let mut newWidth: i32 = jcall!(jcall!(pixelDelta).getPoint()).getX() - jcall!(jcall!(motionRectangle).getRectangle()).x;
                let mut newHeight: i32 = jcall!(jcall!(pixelDelta).getPoint()).getY() - jcall!(jcall!(motionRectangle).getRectangle()).y;
                if newWidth < jcall!(jcall!(motionRectangle).getRectangle()).width {
                    newWidth = jcall!(jcall!(motionRectangle).getRectangle()).width.clone();
                }
                if newHeight < jcall!(jcall!(motionRectangle).getRectangle()).height {
                    newHeight = jcall!(jcall!(motionRectangle).getRectangle()).height.clone();
                }
                jcall!(jcall!(motionRectangle).getRectangle()).setSize(newWidth.clone(), newHeight.clone());
                return true;
            }
        }
        return false;
    }
    

    // throws Exception
    pub fn addPixelDeltaToAMotionRectangle(&mut self, mut pixelDelta: Option<Rc<RefCell<PixelDelta>>>) {
        if !self.addPixelDeltaToExistingMotionRectangle(pixelDelta.clone()) {
            let mut newRectangle: Option<Rc<RefCell<Rectangle>>> = Rectangle::new(jcall!(jcall!(pixelDelta).getPoint()).getX(), jcall!(jcall!(pixelDelta).getPoint()).getY(), 1, 1);
            let mut motionRectangle: Option<Rc<RefCell<MotionRectangle>>> = MotionRectangle::new(newRectangle.clone());
            jcall!(jcall!(motionRectangle).getPixelDeltaVector()).add(pixelDelta.clone());
            jcall!(self.getVector()).add(motionRectangle.clone());
        }
    }
    

    // throws Exception
    pub fn shouldPixelDeltaPartOfMotionRectangle(&mut self, mut motionRectangle: Option<Rc<RefCell<MotionRectangle>>>, mut pixelDelta: Option<Rc<RefCell<PixelDelta>>>) -> bool {
        let mut isPixelLeftOfTheRectangle: bool = false;
        if jcall!(jcall!(motionRectangle).getRectangle()).y > jcall!(jcall!(pixelDelta).getPoint()).getY() {
            panic!("{:?}", format!("{}: {:?}", "Exception", (String::from("PixelAboveTheRectangle"))));
        }
        if jcall!(jcall!(motionRectangle).getRectangle()).x > jcall!(jcall!(pixelDelta).getPoint()).getX() {
            isPixelLeftOfTheRectangle = true;
        }
        if !isPixelLeftOfTheRectangle {
            return self.isPixelDeltaLikelyPartOfMotionRectangle(motionRectangle.clone(), pixelDelta.clone());
        }
        return false;
    }
    

    // throws Exception
    pub fn isPixelDeltaLikelyPartOfMotionRectangle(&mut self, mut motionRectangle: Option<Rc<RefCell<MotionRectangle>>>, mut pixelDelta: Option<Rc<RefCell<PixelDelta>>>) -> bool {
        let mut maxX: i32 = jcall!(jcall!(motionRectangle).getRectangle()).x + jcall!(jcall!(motionRectangle).getRectangle()).width;
        let mut maxY: i32 = jcall!(jcall!(motionRectangle).getRectangle()).y + jcall!(jcall!(motionRectangle).getRectangle()).height;
        let mut horizontalDistance: i32 = jcall!(jcall!(pixelDelta).getPoint()).getX() - maxX;
        let mut verticalDistance: i32 = jcall!(jcall!(pixelDelta).getPoint()).getY() - maxY;
        if horizontalDistance < self.MAX_PIXEL_DISTANCE_THRESHOLD && verticalDistance < self.MAX_PIXEL_DISTANCE_THRESHOLD {
            return true;
        }
        return false;
    }
}
impl std::ops::Deref for AllMotionRectangles {
    type Target = MotionRectangles;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for AllMotionRectangles {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::media::image::comparison::motion::MotionRectangles::MotionRectanglesDyn for AllMotionRectangles {
    fn as_MotionRectangles(&self) -> &crate::org::allbinary::media::image::comparison::motion::MotionRectangles::MotionRectangles { &self.base }
    fn as_MotionRectangles_mut(&mut self) -> &mut crate::org::allbinary::media::image::comparison::motion::MotionRectangles::MotionRectangles { &mut self.base }
}


