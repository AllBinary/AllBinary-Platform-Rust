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
// package org.allbinary.input.media.image.capture
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::java::awt::image::BufferedImage::BufferedImage;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::util::event::handler::BasicEventHandler::BasicEventHandler;
use crate::org::allbinary::media::image::cache::BufferedImageFrameCacheable::BufferedImageFrameCacheable;
use crate::org::allbinary::string::CommonLabels::CommonLabels;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::org::allbinary::time::TimeDelayHelper::TimeDelayHelper;
use crate::java::lang::Thread::*;
use super::CaptureWorkerInterface::CaptureWorkerInterface;
use super::ScreenScavangerRobot::ScreenScavangerRobot;
use super::CapturedImageWorkerResultsEvent::CapturedImageWorkerResultsEvent;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct ScreenCaptureImagesWorker {
    pub base: BasicEventHandler,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub commonStrings: Option<Rc<RefCell<CommonStrings>>>,
    pub running: bool,
    pub screenScavangerRobot: Option<Rc<RefCell<ScreenScavangerRobot>>>,
}

pub static mut ScreenCaptureImagesWorker_index: std::cell::LazyCell<i64> = std::cell::LazyCell::new(|| 0_i64);
// impl CaptureWorkerInterface for ScreenCaptureImagesWorker with the methods of the trait

impl ScreenCaptureImagesWorker {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: BasicEventHandler::new_struct(),
            logUtil: None,
            commonStrings: None,
            running: false,
            screenScavangerRobot: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.commonStrings = CommonStrings::getInstance();
        this.screenScavangerRobot = ScreenScavangerRobot::new();
        unsafe { *ScreenCaptureImagesWorker_index = ProcessingFrameIndexFactory::ProcessingFrameIndexFactory::next() };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    // synchronized
    pub fn isRunning(&mut self) -> bool {
        return self.running.clone();
    }
    

    // synchronized
    pub fn setRunning(&mut self, mut running: bool) {
        self.running = running.clone();
    }
    

    // throws Exception
    pub fn setThread(&mut self, mut thread: Option<Rc<RefCell<Thread>>>) {
    }
    

    pub fn run(&mut self) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            jcall!(self.logUtil).putF(jcall!(self.commonStrings).START.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).RUN.clone());
            self.setRunning(true);
            let mut timeHelper: Option<Rc<RefCell<dyn crate::org::allbinary::time::TimeDelayHelper::TimeDelayHelperDyn>>> = TimeDelayHelper::new(1000).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::time::TimeDelayHelper::TimeDelayHelperDyn>>);
            while self.isRunning() {
                jcall!(timeHelper).setStartTimeTNT();
                let mut bufferedImage: Option<Rc<RefCell<BufferedImage>>> = jcall!(self.screenScavangerRobot).getScreenAsBufferedImages()[(0) as usize].clone();
                let mut frame: Option<i64> = Some((unsafe { ScreenCaptureImagesWorker_index.clone() }.clone()) as i64);
                unsafe { *ScreenCaptureImagesWorker_index += 1 };
                jcall!(CapturedBufferedImagesCacheSingleton::CapturedBufferedImagesCacheSingleton::getInstance()).add(BufferedImageFrameCacheable::new(bufferedImage.clone(), frame.clone()));
                let mut capturedImageEvent: Option<Rc<RefCell<CapturedImageWorkerResultsEvent>>> = CapturedImageWorkerResultsEvent::new(None /* Java this: a &mut self method has no Rc of itself */, frame.clone(), bufferedImage.clone());
                self.fireEvent(capturedImageEvent.clone());
                { let java_receiver = self.logUtil.clone(); let java_arguments = (format!("{}{}", jcall!(CommonLabels::getInstance()).ELAPSED, jcall!(timeHelper).getElapsedTNT()), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).RUN.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
                self.setRunning(false);
            }
            jcall!(self.logUtil).putF(jcall!(self.commonStrings).END.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).RUN.clone());
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).RUN.clone(), crate::java_into_object(e.clone()));
            }
        }
    }
}
impl std::ops::Deref for ScreenCaptureImagesWorker {
    type Target = BasicEventHandler;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for ScreenCaptureImagesWorker {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::logic::util::event::handler::BasicEventHandler::BasicEventHandlerDyn for ScreenCaptureImagesWorker {
    fn as_BasicEventHandler(&self) -> &crate::org::allbinary::logic::util::event::handler::BasicEventHandler::BasicEventHandler { &self.base }
    fn as_BasicEventHandler_mut(&mut self) -> &mut crate::org::allbinary::logic::util::event::handler::BasicEventHandler::BasicEventHandler { &mut self.base }
}


