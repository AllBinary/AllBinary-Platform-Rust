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
// package org.allbinary.input.media.image
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::input::media::image::capture::CapturedImageInputOutput::CapturedImageInputOutput;
use crate::org::allbinary::media::image::comparison::ComparisonImageInputOutput::ComparisonImageInputOutput;
use crate::org::allbinary::media::image::comparison::motion::MotionRectanglesImageInputOutput::MotionRectanglesImageInputOutput;
use crate::org::allbinary::media::image::io::ImageIOInterface::ImageIOInterface;
use super::InputImageType::InputImageType;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct InputImageIOInterfaceFactory {
}


impl InputImageIOInterfaceFactory {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    // throws Exception
    pub fn getInstance(mut imageType: Option<Rc<RefCell<InputImageType>>>) -> Option<Rc<RefCell<dyn ImageIOInterface>>> {
        if match (&imageType, &unsafe { InputImageType_CAPTURE.clone() }) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            return CapturedImageInputOutput::new().map(|reference| reference as Rc<RefCell<dyn ImageIOInterface>>);
        } else if match (&imageType, &unsafe { InputImageType_COMPARISON.clone() }) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            return ComparisonImageInputOutput::new().map(|reference| reference as Rc<RefCell<dyn ImageIOInterface>>);
        } else if match (&imageType, &unsafe { InputImageType_MOTION.clone() }) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            return MotionRectanglesImageInputOutput::new().map(|reference| reference as Rc<RefCell<dyn ImageIOInterface>>);
        } else {
            panic!("{:?}", format!("{}: {:?}", "Exception", (format!("{}{}", String::from("No ImageIOInterface for: "), jcall!(imageType).toString()))));
        }
    }
}

