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
use crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObject;
use super::MotionRectangles::MotionRectangles;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct MotionRectanglesResultsEvent {
    pub base: AllBinaryEventObject,
    pub frame: Option<i64>,
    pub motionRectangles: Option<Rc<RefCell<dyn crate::org::allbinary::media::image::comparison::motion::MotionRectangles::MotionRectanglesDyn>>>,
}


impl MotionRectanglesResultsEvent {
    

    pub fn new_struct(mut anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>>, mut frame: Option<i64>, mut motionRectangles: Option<Rc<RefCell<dyn crate::org::allbinary::media::image::comparison::motion::MotionRectangles::MotionRectanglesDyn>>>) -> Self {
        let mut this = Self {
            base: AllBinaryEventObject::new_struct(anyType.clone()),
            frame: None,
            motionRectangles: None,
        };
        // super(...) is in the struct literal above
        this.frame = frame.clone();
        this.setMotionRectangles(motionRectangles.clone());
        this
    }

    pub fn new(mut anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>>, mut frame: Option<i64>, mut motionRectangles: Option<Rc<RefCell<dyn crate::org::allbinary::media::image::comparison::motion::MotionRectangles::MotionRectanglesDyn>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(anyType, frame, motionRectangles))))
    }
    

    pub fn getMotionRectangles(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::media::image::comparison::motion::MotionRectangles::MotionRectanglesDyn>>> {
        return self.motionRectangles.clone();
    }
    

    pub fn setMotionRectangles(&mut self, mut motionRectangles: Option<Rc<RefCell<dyn crate::org::allbinary::media::image::comparison::motion::MotionRectangles::MotionRectanglesDyn>>>) {
        self.motionRectangles = motionRectangles.clone();
    }
    

    pub fn getFrame(&mut self) -> Option<i64> {
        return self.frame.clone();
    }
}
impl std::ops::Deref for MotionRectanglesResultsEvent {
    type Target = AllBinaryEventObject;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for MotionRectanglesResultsEvent {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObjectDyn for MotionRectanglesResultsEvent {
    fn as_AllBinaryEventObject(&self) -> &crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObject { &self.base }
    fn as_AllBinaryEventObject_mut(&mut self) -> &mut crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObject { &mut self.base }
}


