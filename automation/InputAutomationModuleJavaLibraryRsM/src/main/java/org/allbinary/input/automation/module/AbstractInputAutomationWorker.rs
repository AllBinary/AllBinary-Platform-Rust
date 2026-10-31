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
use crate::Runnable;
use super::*;
use crate::org::allbinary::input::media::image::capture::CaptureWorkerInterface::CaptureWorkerInterface;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::media::image::comparison::ImageComparisonWorker::ImageComparisonWorker;
use crate::org::allbinary::media::image::comparison::motion::MotionRectanglesWorker::MotionRectanglesWorker;
use crate::org::allbinary::string::CommonLabels::CommonLabels;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::org::allbinary::thread::RunnableInterface::RunnableInterface;
use crate::org::allbinary::time::TimeDelayHelper::TimeDelayHelper;
use crate::java::lang::Thread::*;
use super::InputAutomationActionInterface::InputAutomationActionInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct AbstractInputAutomationWorker {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub commonStrings: Option<Rc<RefCell<CommonStrings>>>,
    pub index: i64,
    pub running: bool,
    pub inputAutomationActionInterface: Option<Rc<RefCell<dyn InputAutomationActionInterface>>>,
    pub captureWorkerInterface: Option<Rc<RefCell<dyn CaptureWorkerInterface>>>,
    pub imageComparisonWorker: Option<Rc<RefCell<dyn crate::org::allbinary::media::image::comparison::ImageComparisonWorker::ImageComparisonWorkerDyn>>>,
    pub motionRectanglesWorker: Option<Rc<RefCell<MotionRectanglesWorker>>>,
    pub captureThread: Option<Rc<RefCell<Thread>>>,
}

impl RunnableInterface for AbstractInputAutomationWorker {
    fn isRunning(&mut self) -> bool { Self::isRunning(self) }
    fn setRunning(&mut self, mut running: bool) { Self::setRunning(self, running) }
    fn setThread(&mut self, mut thread: Option<Rc<RefCell<Thread>>>) { Self::setThread(self, thread) }
}
impl crate::Runnable for AbstractInputAutomationWorker {
    fn run(&mut self) { Self::run(self) }
}

impl AbstractInputAutomationWorker {
    

    pub fn new_struct(mut inputAutomationActionInterface: Option<Rc<RefCell<dyn InputAutomationActionInterface>>>) -> Self {
        let mut this = Self {
            logUtil: None,
            commonStrings: None,
            index: 0_i64,
            running: false,
            inputAutomationActionInterface: None,
            captureWorkerInterface: None,
            imageComparisonWorker: None,
            motionRectanglesWorker: None,
            captureThread: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.commonStrings = CommonStrings::getInstance();
        this
    }

    pub fn new(mut inputAutomationActionInterface: Option<Rc<RefCell<dyn InputAutomationActionInterface>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(inputAutomationActionInterface))))
    }
    

    // throws Exception
    pub fn setThread(&mut self, mut thread: Option<Rc<RefCell<Thread>>>) {
    }
    

    // synchronized
    pub fn isRunning(&mut self) -> bool {
        return self.running.clone();
    }
    

    // synchronized
    pub fn setRunning(&mut self, mut running: bool) {
        self.running = running.clone();
    }
    

    // synchronized
    pub fn isAnyDataWorkerRunning(&mut self) -> bool {
        if self.captureThread.is_some() && (jcall!(self.captureThread).isAlive() || jcall!(self.getCaptureWorker()).isRunning() || jcall!(self.getMotionRectanglesWorker()).isRunning() || jcall!(self.getImageComparisonWorker()).isRunning()) {
            return true;
        }
        return false;
    }
    

    // throws Exception
    // synchronized
    pub fn startDataWorkers(&mut self) {
        if !self.isAnyDataWorkerRunning() {
            self.captureThread = Thread::new1(self.getCaptureWorker());
            { let java_receiver = self.logUtil.clone(); let java_arguments = (format!("{}{}", String::from("Starting CaptureWorkers - Need more images - Thread State: "), jcall!(jcall!(self.captureThread).getState()).toString()), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("startCaptureWorkers"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
            jcall!(self.captureThread).start();
        }
    }
    

    // throws Exception
    // synchronized
    pub fn waitForDataWorkers(&mut self) {
        while self.isAnyDataWorkerRunning() {
            jcall!(self.logUtil).putF(String::from("Waiting"), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).RUN.clone());
            std::thread::sleep(std::time::Duration::from_millis((250) as u64));
        }
    }
    

    // throws Exception
    pub fn stopDataWorkers(&mut self) {
        jcall!(self.getCaptureWorker()).setRunning(false);
    }
    

    // throws Exception
    pub fn process(&mut self) {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    pub fn run(&mut self) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            jcall!(self.logUtil).putF(jcall!(self.commonStrings).START.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).RUN.clone());
            self.setRunning(true);
            let mut timeHelper: Option<Rc<RefCell<dyn crate::org::allbinary::time::TimeDelayHelper::TimeDelayHelperDyn>>> = TimeDelayHelper::new(1000).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::time::TimeDelayHelper::TimeDelayHelperDyn>>);
            while self.isRunning() {
                jcall!(timeHelper).setStartTimeTNT();
                self.process();
                self.index += 1;
                { let java_receiver = self.logUtil.clone(); let java_arguments = (format!("{}{}", format!("{}{}", format!("{}{}", jcall!(CommonLabels::getInstance()).ELAPSED, jcall!(timeHelper).getElapsedTNT()), String::from(" Index: ")), self.index), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).RUN.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
            }
            self.stopDataWorkers();
            self.waitForDataWorkers();
            jcall!(self.logUtil).putF(jcall!(self.commonStrings).END.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).RUN.clone());
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).RUN.clone(), crate::java_into_object(e.clone()));
            }
        }
    }
    

    pub fn getInputAutomationActionInterface(&mut self) -> Option<Rc<RefCell<dyn InputAutomationActionInterface>>> {
        return self.inputAutomationActionInterface.clone();
    }
    

    pub fn setInputAutomationActionInterface(&mut self, mut inputAutomationActionInterface: Option<Rc<RefCell<dyn InputAutomationActionInterface>>>) {
        self.inputAutomationActionInterface = inputAutomationActionInterface.clone();
    }
    

    pub fn getCaptureWorker(&mut self) -> Option<Rc<RefCell<dyn CaptureWorkerInterface>>> {
        return self.captureWorkerInterface.clone();
    }
    

    pub fn setCaptureWorker(&mut self, mut captureWorkerInterface: Option<Rc<RefCell<dyn CaptureWorkerInterface>>>) {
        self.captureWorkerInterface = captureWorkerInterface.clone();
    }
    

    pub fn getImageComparisonWorker(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::media::image::comparison::ImageComparisonWorker::ImageComparisonWorkerDyn>>> {
        return self.imageComparisonWorker.clone();
    }
    

    pub fn setImageComparisonWorker(&mut self, mut imageComparisonWorker: Option<Rc<RefCell<dyn crate::org::allbinary::media::image::comparison::ImageComparisonWorker::ImageComparisonWorkerDyn>>>) {
        self.imageComparisonWorker = imageComparisonWorker.clone();
    }
    

    pub fn getMotionRectanglesWorker(&mut self) -> Option<Rc<RefCell<MotionRectanglesWorker>>> {
        return self.motionRectanglesWorker.clone();
    }
    

    pub fn setMotionRectanglesWorker(&mut self, mut motionRectanglesWorker: Option<Rc<RefCell<MotionRectanglesWorker>>>) {
        self.motionRectanglesWorker = motionRectanglesWorker.clone();
    }
}
pub trait AbstractInputAutomationWorkerDyn {
    fn as_AbstractInputAutomationWorker(&self) -> &AbstractInputAutomationWorker;
    fn as_AbstractInputAutomationWorker_mut(&mut self) -> &mut AbstractInputAutomationWorker;
    fn setThread(&mut self, mut thread: Option<Rc<RefCell<Thread>>>) { AbstractInputAutomationWorker::setThread(self.as_AbstractInputAutomationWorker_mut(), thread) }
    fn isRunning(&mut self) -> bool { AbstractInputAutomationWorker::isRunning(self.as_AbstractInputAutomationWorker_mut()) }
    fn setRunning(&mut self, mut running: bool) { AbstractInputAutomationWorker::setRunning(self.as_AbstractInputAutomationWorker_mut(), running) }
    fn isAnyDataWorkerRunning(&mut self) -> bool { AbstractInputAutomationWorker::isAnyDataWorkerRunning(self.as_AbstractInputAutomationWorker_mut()) }
    fn startDataWorkers(&mut self) { AbstractInputAutomationWorker::startDataWorkers(self.as_AbstractInputAutomationWorker_mut()) }
    fn waitForDataWorkers(&mut self) { AbstractInputAutomationWorker::waitForDataWorkers(self.as_AbstractInputAutomationWorker_mut()) }
    fn stopDataWorkers(&mut self) { AbstractInputAutomationWorker::stopDataWorkers(self.as_AbstractInputAutomationWorker_mut()) }
    fn process(&mut self) { AbstractInputAutomationWorker::process(self.as_AbstractInputAutomationWorker_mut()) }
    fn run(&mut self) { AbstractInputAutomationWorker::run(self.as_AbstractInputAutomationWorker_mut()) }
    fn getInputAutomationActionInterface(&mut self) -> Option<Rc<RefCell<dyn InputAutomationActionInterface>>> { AbstractInputAutomationWorker::getInputAutomationActionInterface(self.as_AbstractInputAutomationWorker_mut()) }
    fn setInputAutomationActionInterface(&mut self, mut inputAutomationActionInterface: Option<Rc<RefCell<dyn InputAutomationActionInterface>>>) { AbstractInputAutomationWorker::setInputAutomationActionInterface(self.as_AbstractInputAutomationWorker_mut(), inputAutomationActionInterface) }
    fn getCaptureWorker(&mut self) -> Option<Rc<RefCell<dyn CaptureWorkerInterface>>> { AbstractInputAutomationWorker::getCaptureWorker(self.as_AbstractInputAutomationWorker_mut()) }
    fn setCaptureWorker(&mut self, mut captureWorkerInterface: Option<Rc<RefCell<dyn CaptureWorkerInterface>>>) { AbstractInputAutomationWorker::setCaptureWorker(self.as_AbstractInputAutomationWorker_mut(), captureWorkerInterface) }
    fn getImageComparisonWorker(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::media::image::comparison::ImageComparisonWorker::ImageComparisonWorkerDyn>>> { AbstractInputAutomationWorker::getImageComparisonWorker(self.as_AbstractInputAutomationWorker_mut()) }
    fn setImageComparisonWorker(&mut self, mut imageComparisonWorker: Option<Rc<RefCell<dyn crate::org::allbinary::media::image::comparison::ImageComparisonWorker::ImageComparisonWorkerDyn>>>) { AbstractInputAutomationWorker::setImageComparisonWorker(self.as_AbstractInputAutomationWorker_mut(), imageComparisonWorker) }
    fn getMotionRectanglesWorker(&mut self) -> Option<Rc<RefCell<MotionRectanglesWorker>>> { AbstractInputAutomationWorker::getMotionRectanglesWorker(self.as_AbstractInputAutomationWorker_mut()) }
    fn setMotionRectanglesWorker(&mut self, mut motionRectanglesWorker: Option<Rc<RefCell<MotionRectanglesWorker>>>) { AbstractInputAutomationWorker::setMotionRectanglesWorker(self.as_AbstractInputAutomationWorker_mut(), motionRectanglesWorker) }
}
impl AbstractInputAutomationWorkerDyn for AbstractInputAutomationWorker {
    fn as_AbstractInputAutomationWorker(&self) -> &AbstractInputAutomationWorker { self }
    fn as_AbstractInputAutomationWorker_mut(&mut self) -> &mut AbstractInputAutomationWorker { self }
}


