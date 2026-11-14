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
use crate::org::allbinary::input::media::image::capture::ScreenCaptureImagesWorker::ScreenCaptureImagesWorker;
use crate::org::allbinary::media::image::comparison::ImageComparisonWorker::ImageComparisonWorker;
use crate::org::allbinary::media::image::comparison::SaveImageComparisonWorker::SaveImageComparisonWorker;
use crate::org::allbinary::media::image::comparison::motion::MotionRectanglesWorker::MotionRectanglesWorker;
use crate::org::allbinary::media::image::comparison::motion::SaveMotionRectanglesResultsWorker::SaveMotionRectanglesResultsWorker;
use crate::org::allbinary::media::image::comparison::motion::TestInputForMotionRectanglesResultsWorker::TestInputForMotionRectanglesResultsWorker;
use crate::java::lang::Exception::*;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct TestInputAutomationWorker {
    pub captureWorker: Option<Rc<RefCell<ScreenCaptureImagesWorker>>>,
    pub imageComparisonWorker: Option<Rc<RefCell<dyn crate::org::allbinary::media::image::comparison::ImageComparisonWorker::ImageComparisonWorkerDyn>>>,
    pub motionRectanglesWorker: Option<Rc<RefCell<MotionRectanglesWorker>>>,
    pub changedPixelsImageComparisonResultsWorker: Option<Rc<RefCell<SaveImageComparisonWorker>>>,
    pub saveMotionRectanglesAsImagesWorker: Option<Rc<RefCell<SaveMotionRectanglesResultsWorker>>>,
    pub moveMouseToFirstMotionRectanglesResultsWorker: Option<Rc<RefCell<TestInputForMotionRectanglesResultsWorker>>>,
}


impl TestInputAutomationWorker {

    pub fn toString(&mut self) -> String {
        String::from(std::any::type_name::<Self>())
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            captureWorker: None,
            imageComparisonWorker: None,
            motionRectanglesWorker: None,
            changedPixelsImageComparisonResultsWorker: None,
            saveMotionRectanglesAsImagesWorker: None,
            moveMouseToFirstMotionRectanglesResultsWorker: None,
        };
        this.captureWorker = ScreenCaptureImagesWorker::new();
        this.changedPixelsImageComparisonResultsWorker = SaveImageComparisonWorker::new();
        this.saveMotionRectanglesAsImagesWorker = SaveMotionRectanglesResultsWorker::new();
        this.moveMouseToFirstMotionRectanglesResultsWorker = TestInputForMotionRectanglesResultsWorker::new();
        jcall!(this.motionRectanglesWorker).addListenerInterface(this.saveMotionRectanglesAsImagesWorker.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::util::event::EventListenerInterface::EventListenerInterface>>));
        jcall!(this.motionRectanglesWorker).addListenerInterface(this.moveMouseToFirstMotionRectanglesResultsWorker.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::util::event::EventListenerInterface::EventListenerInterface>>));
        jcall!(this.imageComparisonWorker).as_ImageComparisonWorker_mut().addListenerInterface(this.changedPixelsImageComparisonResultsWorker.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::util::event::EventListenerInterface::EventListenerInterface>>));
        jcall!(this.imageComparisonWorker).as_ImageComparisonWorker_mut().addListenerInterface(this.motionRectanglesWorker.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::util::event::EventListenerInterface::EventListenerInterface>>));
        jcall!(this.captureWorker).addListenerInterface(this.imageComparisonWorker.clone().map(|_java_reference| -> Rc<RefCell<dyn crate::org::allbinary::logic::util::event::EventListenerInterface::EventListenerInterface>> { panic!("ClassCastException: the Rust trait of ImageComparisonWorker does not have EventListenerInterface as a base trait") }));
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
}

