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
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::util::cache::CacheableInterface::CacheableInterface;
use super::MotionRectangles::MotionRectangles;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct MotionRectanglesResultsFrameCacheable {
    pub motionRectangles: Option<Rc<RefCell<dyn crate::org::allbinary::media::image::comparison::motion::MotionRectangles::MotionRectanglesDyn>>>,
    pub frame: Option<i64>,
}

impl CacheableInterface for MotionRectanglesResultsFrameCacheable {
    fn getKey(&mut self) -> Option<Rc<RefCell<Box<dyn std::any::Any>>>> { Self::getKey(self) }
}

impl MotionRectanglesResultsFrameCacheable {
    

    pub fn new_struct(mut motionRectangles: Option<Rc<RefCell<dyn crate::org::allbinary::media::image::comparison::motion::MotionRectangles::MotionRectanglesDyn>>>, mut frame: Option<i64>) -> Self {
        let mut this = Self {
            motionRectangles: None,
            frame: None,
        };
        this.frame = frame.clone();
        this.setMotionRectangles(motionRectangles.clone());
        this
    }

    pub fn new(mut motionRectangles: Option<Rc<RefCell<dyn crate::org::allbinary::media::image::comparison::motion::MotionRectangles::MotionRectanglesDyn>>>, mut frame: Option<i64>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(motionRectangles, frame))))
    }
    

    pub fn getKey(&mut self) -> Option<Rc<RefCell<Box<dyn std::any::Any>>>> {
        return self.frame.clone();
    }
    

    pub fn getMotionRectangles(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::media::image::comparison::motion::MotionRectangles::MotionRectanglesDyn>>> {
        return self.motionRectangles.clone();
    }
    

    pub fn setMotionRectangles(&mut self, mut motionRectangles: Option<Rc<RefCell<dyn crate::org::allbinary::media::image::comparison::motion::MotionRectangles::MotionRectanglesDyn>>>) {
        self.motionRectangles = motionRectangles.clone();
    }
    

    pub fn toString(&mut self) -> String {
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        jcall!(stringBuffer).append(std::any::type_name_of_val(&self).to_string());
        jcall!(stringBuffer).append(String::from(" Frame: "));
        jcall!(stringBuffer).append(jcall!(self.getKey()).toString());
        return jcall!(stringBuffer).toString();
    }
}

