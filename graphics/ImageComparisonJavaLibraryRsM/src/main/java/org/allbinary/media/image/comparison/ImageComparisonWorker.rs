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
// package org.allbinary.media.image.comparison
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
use crate::org::allbinary::input::media::image::capture::CapturedImageWorkerResultsEvent::CapturedImageWorkerResultsEvent;
use crate::org::allbinary::input::media::image::capture::CapturedImageWorkerResultsListener::CapturedImageWorkerResultsListener;
use crate::org::allbinary::logic::StdUtil::StdUtil;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObject;
use crate::org::allbinary::logic::util::event::handler::BasicEventHandler::BasicEventHandler;
use crate::org::allbinary::string::CommonLabels::CommonLabels;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::org::allbinary::time::TimeDelayHelper::TimeDelayHelper;
use crate::java::lang::Exception::*;
use super::ImageComparatorConstraintsInterface::ImageComparatorConstraintsInterface;
use super::ImageComparator::ImageComparator;
use super::ImageComparisonResult::ImageComparisonResult;
use super::ImageComparisonResultFrameCacheable::ImageComparisonResultFrameCacheable;
use super::ImageComparisonResultsEvent::ImageComparisonResultsEvent;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct ImageComparisonWorker {
    pub base: BasicEventHandler,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub commonStrings: Option<Rc<RefCell<CommonStrings>>>,
    pub bufferedImageVector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>,
    pub imageComparatorConstraintsInterface: Option<Rc<RefCell<dyn ImageComparatorConstraintsInterface>>>,
    pub imageComparator: Option<Rc<RefCell<ImageComparator>>>,
    pub running: bool,
    pub index2: i32,
}

impl CapturedImageWorkerResultsListener for ImageComparisonWorker {
    fn onCaptureEvent(&mut self, mut capturedImageWorkerResultsEvent: Option<Rc<RefCell<CapturedImageWorkerResultsEvent>>>) { Self::onCaptureEvent(self, capturedImageWorkerResultsEvent) }
}
impl crate::org::allbinary::logic::util::event::BasicEventListenerInterface::BasicEventListenerInterface for ImageComparisonWorker {
}
impl crate::org::allbinary::logic::util::event::EventListenerInterface::EventListenerInterface for ImageComparisonWorker {
    fn onEvent(&mut self, mut allBinaryEventObject: Option<Rc<RefCell<dyn crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObjectDyn>>>) { Self::onEvent(self, allBinaryEventObject) }
}

impl ImageComparisonWorker {
    

    pub fn new_struct(mut imageComparatorConstraintsInterface: Option<Rc<RefCell<dyn ImageComparatorConstraintsInterface>>>) -> Self {
        let mut this = Self {
            base: BasicEventHandler::new_struct(),
            logUtil: None,
            commonStrings: None,
            bufferedImageVector: None,
            imageComparatorConstraintsInterface: None,
            imageComparator: None,
            running: false,
            index2: 0,
        };
        this.logUtil = LogUtil::getInstance();
        this.commonStrings = CommonStrings::getInstance();
        this.bufferedImageVector = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        this.imageComparatorConstraintsInterface = imageComparatorConstraintsInterface.clone();
        this.imageComparator = ImageComparator::new(imageComparatorConstraintsInterface.clone());
        this
    }

    pub fn new(mut imageComparatorConstraintsInterface: Option<Rc<RefCell<dyn ImageComparatorConstraintsInterface>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(imageComparatorConstraintsInterface))))
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
    pub fn onCaptureEvent(&mut self, mut capturedImageWorkerResultsEvent: Option<Rc<RefCell<CapturedImageWorkerResultsEvent>>>) {
        jcall!(self.bufferedImageVector).add(crate::java_into_object(capturedImageWorkerResultsEvent.clone()));
        if jcall!(self.bufferedImageVector).size() > 1 {
            self.run();
        }
    }
    

    // synchronized
    pub fn onEvent(&mut self, mut allBinaryEventObject: Option<Rc<RefCell<dyn crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObjectDyn>>>) {
        self.onCaptureEvent((allBinaryEventObject).clone().map(|reference| { assert!({ let java_borrowed = reference.borrow(); let java_any: &dyn std::any::Any = &*java_borrowed; java_any.is::<CapturedImageWorkerResultsEvent>() }, "ClassCastException: not a CapturedImageWorkerResultsEvent"); unsafe { Rc::from_raw(Rc::into_raw(reference) as *const RefCell<CapturedImageWorkerResultsEvent>) } }));
    }
    

    pub fn run(&mut self) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            jcall!(self.logUtil).putF({ let java_field = jcall!(self.commonStrings).START.clone(); java_field }.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(self.commonStrings).RUN.clone(); java_field }.clone());
            self.setRunning(true);
            let mut timeHelper: Option<Rc<RefCell<dyn crate::org::allbinary::time::TimeDelayHelper::TimeDelayHelperDyn>>> = TimeDelayHelper::new(1000).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::time::TimeDelayHelper::TimeDelayHelperDyn>>);
            jcall!(timeHelper).setStartTimeTNT();
            if jcall!(self.imageComparatorConstraintsInterface).isFrameAllowed(self.index2.clone()) {
                let mut capturedImageWorkerResultsEvent: Vec<Option<Rc<RefCell<CapturedImageWorkerResultsEvent>>>> = vec![None; (2) as usize];
                capturedImageWorkerResultsEvent[(0) as usize] = { let java_object = (jcall!(self.bufferedImageVector).get(0)).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<CapturedImageWorkerResultsEvent>>>>().expect("ClassCastException").clone()) };
                capturedImageWorkerResultsEvent[(1) as usize] = { let java_object = (jcall!(self.bufferedImageVector).get(1)).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<CapturedImageWorkerResultsEvent>>>>().expect("ClassCastException").clone()) };
                if { let java_receiver = self.imageComparatorConstraintsInterface.clone(); let java_arguments = (jcall!(capturedImageWorkerResultsEvent[(0) as usize]).getBufferedImage(),); let java_result = jcall!(java_receiver).isImageValid(java_arguments.0); java_result } && { let java_receiver = self.imageComparatorConstraintsInterface.clone(); let java_arguments = (jcall!(capturedImageWorkerResultsEvent[(1) as usize]).getBufferedImage(),); let java_result = jcall!(java_receiver).isImageValid(java_arguments.0); java_result } {
                    let mut imageComparisonResult: Option<Rc<RefCell<ImageComparisonResult>>> = { let java_receiver = self.imageComparator.clone(); let java_arguments = (jcall!(capturedImageWorkerResultsEvent[(0) as usize]).getBufferedImage(), jcall!(capturedImageWorkerResultsEvent[(1) as usize]).getBufferedImage(), jcall!(capturedImageWorkerResultsEvent[(0) as usize]).getFrame(), jcall!(capturedImageWorkerResultsEvent[(1) as usize]).getFrame(), 0,); let java_result = jcall!(java_receiver).compare(java_arguments.0, java_arguments.1, java_arguments.2, java_arguments.3, java_arguments.4); java_result };
                    let mut frame: Option<i64> = jcall!(capturedImageWorkerResultsEvent[(1) as usize]).getFrame();
                    let mut imageComparisonResultFrameCacheable: Option<Rc<RefCell<ImageComparisonResultFrameCacheable>>> = ImageComparisonResultFrameCacheable::new(imageComparisonResult.clone(), frame.clone());
                    jcall!(ImageComparisonResultCacheSingleton::ImageComparisonResultCacheSingleton::getInstance()).add(imageComparisonResultFrameCacheable.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::util::cache::CacheableInterface::CacheableInterface>>));
                    self.fireEvent(ImageComparisonResultsEvent::new(None /* Java this: a &mut self method has no Rc of itself */, imageComparisonResult.clone()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObjectDyn>>));
                    { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(StringMaker::new()).append(String::from("Image Comparison Result: ")).append(jcall!(imageComparisonResult).toString()).append(String::from(" for frame: ")).appendlong(frame.clone()).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(self.commonStrings).RUN.clone(); java_field }.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
                } else {
                    jcall!(self.logUtil).putF(String::from("An Image Was Not Valid: Image Worker Event Processing terminated"), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(self.commonStrings).RUN.clone(); java_field }.clone());
                }
            }
            jcall!(self.bufferedImageVector).remove(crate::java_into_object(0));
            self.index2 += 1;
            let mut message: String = jcall!(StringMaker::new()).append(String::from("Frame: ")).appendint(self.index2.clone()).append({ let java_field = jcall!(CommonLabels::getInstance()).ELAPSED.clone(); java_field }.clone()).appendlong(jcall!(timeHelper).getElapsedTNT()).toString();
            jcall!(self.logUtil).putF(message.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(self.commonStrings).RUN.clone(); java_field }.clone());
            self.setRunning(false);
            jcall!(self.logUtil).putF({ let java_field = jcall!(self.commonStrings).END.clone(); java_field }.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(self.commonStrings).RUN.clone(); java_field }.clone());
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            jcall!(self.logUtil).put({ let java_field = jcall!(self.commonStrings).EXCEPTION.clone(); java_field }.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(self.commonStrings).RUN.clone(); java_field }.clone(), crate::java_into_object(e.clone()));
            }
        }
    }
}
impl std::ops::Deref for ImageComparisonWorker {
    type Target = BasicEventHandler;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for ImageComparisonWorker {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

pub trait ImageComparisonWorkerDyn: crate::org::allbinary::logic::util::event::handler::BasicEventHandler::BasicEventHandlerDyn {
    fn as_ImageComparisonWorker(&self) -> &ImageComparisonWorker;
    fn as_ImageComparisonWorker_mut(&mut self) -> &mut ImageComparisonWorker;
    fn isRunning(&mut self) -> bool { ImageComparisonWorker::isRunning(self.as_ImageComparisonWorker_mut()) }
    fn setRunning(&mut self, mut running: bool) { ImageComparisonWorker::setRunning(self.as_ImageComparisonWorker_mut(), running) }
    fn onCaptureEvent(&mut self, mut capturedImageWorkerResultsEvent: Option<Rc<RefCell<CapturedImageWorkerResultsEvent>>>) { ImageComparisonWorker::onCaptureEvent(self.as_ImageComparisonWorker_mut(), capturedImageWorkerResultsEvent) }
    fn onEvent(&mut self, mut allBinaryEventObject: Option<Rc<RefCell<dyn crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObjectDyn>>>) { ImageComparisonWorker::onEvent(self.as_ImageComparisonWorker_mut(), allBinaryEventObject) }
    fn run(&mut self) { ImageComparisonWorker::run(self.as_ImageComparisonWorker_mut()) }
}
impl ImageComparisonWorkerDyn for ImageComparisonWorker {
    fn as_ImageComparisonWorker(&self) -> &ImageComparisonWorker { self }
    fn as_ImageComparisonWorker_mut(&mut self) -> &mut ImageComparisonWorker { self }
}

impl crate::org::allbinary::logic::util::event::handler::BasicEventHandler::BasicEventHandlerDyn for ImageComparisonWorker {
    fn as_BasicEventHandler(&self) -> &crate::org::allbinary::logic::util::event::handler::BasicEventHandler::BasicEventHandler { &self.base }
    fn as_BasicEventHandler_mut(&mut self) -> &mut crate::org::allbinary::logic::util::event::handler::BasicEventHandler::BasicEventHandler { &mut self.base }
}
impl crate::org::allbinary::logic::util::event::handler::BasicEventHandlerInterface::BasicEventHandlerInterface for ImageComparisonWorker {
    fn addListenerInterface(&mut self, p1: Option<Rc<RefCell<dyn crate::org::allbinary::logic::util::event::EventListenerInterface::EventListenerInterface>>>) { crate::org::allbinary::logic::util::event::handler::BasicEventHandler::BasicEventHandler::addListenerInterface(crate::org::allbinary::logic::util::event::handler::BasicEventHandler::BasicEventHandlerDyn::as_BasicEventHandler_mut(self), p1) }
    fn addListeners(&mut self, p1: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) { crate::org::allbinary::logic::util::event::handler::BasicEventHandler::BasicEventHandler::addListeners(crate::org::allbinary::logic::util::event::handler::BasicEventHandler::BasicEventHandlerDyn::as_BasicEventHandler_mut(self), p1) }
    fn fireEvent(&mut self, p1: Option<Rc<RefCell<dyn crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObjectDyn>>>) { crate::org::allbinary::logic::util::event::handler::BasicEventHandler::BasicEventHandler::fireEvent(crate::org::allbinary::logic::util::event::handler::BasicEventHandler::BasicEventHandlerDyn::as_BasicEventHandler_mut(self), p1) }
    fn removeAllListeners(&mut self) { crate::org::allbinary::logic::util::event::handler::BasicEventHandler::BasicEventHandler::removeAllListeners(crate::org::allbinary::logic::util::event::handler::BasicEventHandler::BasicEventHandlerDyn::as_BasicEventHandler_mut(self)) }
    fn removeListener(&mut self, p1: Option<Rc<RefCell<dyn crate::org::allbinary::logic::util::event::EventListenerInterface::EventListenerInterface>>>) { crate::org::allbinary::logic::util::event::handler::BasicEventHandler::BasicEventHandler::removeListener(crate::org::allbinary::logic::util::event::handler::BasicEventHandler::BasicEventHandlerDyn::as_BasicEventHandler_mut(self), p1) }
}


