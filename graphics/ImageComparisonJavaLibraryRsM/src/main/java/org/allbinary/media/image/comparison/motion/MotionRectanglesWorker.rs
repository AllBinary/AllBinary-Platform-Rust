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
use crate::org::allbinary::logic::StdUtil::StdUtil;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObject;
use crate::org::allbinary::logic::util::event::handler::BasicEventHandler::BasicEventHandler;
use crate::org::allbinary::media::image::comparison::ImageComparisonResult::ImageComparisonResult;
use crate::org::allbinary::media::image::comparison::ImageComparisonResultsEvent::ImageComparisonResultsEvent;
use crate::org::allbinary::media::image::comparison::ImageComparisonResultsListener::ImageComparisonResultsListener;
use crate::org::allbinary::string::CommonLabels::CommonLabels;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::org::allbinary::time::TimeDelayHelper::TimeDelayHelper;
use crate::java::lang::Exception::*;
use super::MotionRectangleConstraintsInterface::MotionRectangleConstraintsInterface;
use super::AllMotionRectangles::AllMotionRectangles;
use super::MotionRectanglesResultsFrameCacheable::MotionRectanglesResultsFrameCacheable;
use super::ConsolidateMotionRectangles::ConsolidateMotionRectangles;
use super::ConstrainedMotionRectangles::ConstrainedMotionRectangles;
use super::MotionRectanglesResultsEvent::MotionRectanglesResultsEvent;
use super::MotionRectangles::MotionRectangles;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct MotionRectanglesWorker {
    pub base: BasicEventHandler,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub commonStrings: Option<Rc<RefCell<CommonStrings>>>,
    pub imageComparisonInfoVector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>,
    pub motionRectangleConstraintsInterface: Option<Rc<RefCell<dyn MotionRectangleConstraintsInterface>>>,
    pub index: i64,
    pub running: bool,
}

impl ImageComparisonResultsListener for MotionRectanglesWorker {
    fn onImageComparisonResultsEvent(&mut self, mut imageComparisonResultsEvent: Option<Rc<RefCell<ImageComparisonResultsEvent>>>) { Self::onImageComparisonResultsEvent(self, imageComparisonResultsEvent) }
}
impl crate::org::allbinary::logic::util::event::BasicEventListenerInterface::BasicEventListenerInterface for MotionRectanglesWorker {
}
impl crate::org::allbinary::logic::util::event::EventListenerInterface::EventListenerInterface for MotionRectanglesWorker {
    fn onEvent(&mut self, mut allBinaryEventObject: Option<Rc<RefCell<dyn crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObjectDyn>>>) { Self::onEvent(self, allBinaryEventObject) }
}

impl MotionRectanglesWorker {
    

    pub fn new_struct(mut motionRectangleConstraintsInterface: Option<Rc<RefCell<dyn MotionRectangleConstraintsInterface>>>) -> Self {
        let mut this = Self {
            base: BasicEventHandler::new_struct(),
            logUtil: None,
            commonStrings: None,
            imageComparisonInfoVector: None,
            motionRectangleConstraintsInterface: None,
            index: 0_i64,
            running: false,
        };
        this.logUtil = LogUtil::getInstance();
        this.commonStrings = CommonStrings::getInstance();
        this.imageComparisonInfoVector = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        this.motionRectangleConstraintsInterface = motionRectangleConstraintsInterface.clone();
        this
    }

    pub fn new(mut motionRectangleConstraintsInterface: Option<Rc<RefCell<dyn MotionRectangleConstraintsInterface>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(motionRectangleConstraintsInterface))))
    }
    

    pub fn onImageComparisonResultsEvent(&mut self, mut imageComparisonResultsEvent: Option<Rc<RefCell<ImageComparisonResultsEvent>>>) {
        jcall!(self.imageComparisonInfoVector).add(crate::java_into_object(imageComparisonResultsEvent.clone()));
        self.run();
    }
    

    pub fn onEvent(&mut self, mut allBinaryEventObject: Option<Rc<RefCell<dyn crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObjectDyn>>>) {
        self.onImageComparisonResultsEvent((allBinaryEventObject).clone().map(|reference| { assert!({ let java_borrowed = reference.borrow(); let java_any: &dyn std::any::Any = &*java_borrowed; java_any.is::<ImageComparisonResultsEvent>() }, "ClassCastException: not a ImageComparisonResultsEvent"); unsafe { Rc::from_raw(Rc::into_raw(reference) as *const RefCell<ImageComparisonResultsEvent>) } }));
    }
    

    // synchronized
    pub fn isRunning(&mut self) -> bool {
        return self.running.clone();
    }
    

    // synchronized
    pub fn setRunning(&mut self, mut running: bool) {
        self.running = running.clone();
    }
    

    pub fn run(&mut self) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            jcall!(self.logUtil).putF({ let java_field = jcall!(self.commonStrings).START.clone(); java_field }.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(self.commonStrings).RUN.clone(); java_field }.clone());
            self.setRunning(true);
            let mut timeHelper: Option<Rc<RefCell<dyn crate::org::allbinary::time::TimeDelayHelper::TimeDelayHelperDyn>>> = TimeDelayHelper::new(1000).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::time::TimeDelayHelper::TimeDelayHelperDyn>>);
            jcall!(timeHelper).setStartTimeTNT();
            let mut imageComparisonResultsEvent: Option<Rc<RefCell<ImageComparisonResultsEvent>>> = { let java_object = (jcall!(self.imageComparisonInfoVector).get(0)).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<ImageComparisonResultsEvent>>>>().expect("ClassCastException").clone()) };
            let mut imageComparisonInfo: Option<Rc<RefCell<ImageComparisonResult>>> = (jcall!(imageComparisonResultsEvent).getImageComparisonResult()).clone();
            { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(imageComparisonInfo).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(self.commonStrings).RUN.clone(); java_field }.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
            let mut allMotionRectangles: Option<Rc<RefCell<AllMotionRectangles>>> = AllMotionRectangles::new(imageComparisonInfo.clone());
            jcall!(AllMotionRectanglesResultsCacheSingleton::AllMotionRectanglesResultsCacheSingleton::getInstance()).add(MotionRectanglesResultsFrameCacheable::new(allMotionRectangles.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::media::image::comparison::motion::MotionRectangles::MotionRectanglesDyn>>), jcall!(imageComparisonInfo).getFrameTwo()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::util::cache::CacheableInterface::CacheableInterface>>));
            let mut consolidatedMotionRectangles: Option<Rc<RefCell<ConsolidateMotionRectangles>>> = ConsolidateMotionRectangles::new(allMotionRectangles.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::media::image::comparison::motion::MotionRectangles::MotionRectanglesDyn>>));
            jcall!(ConsolidatedMotionRectanglesResultsCacheSingleton::ConsolidatedMotionRectanglesResultsCacheSingleton::getInstance()).add(MotionRectanglesResultsFrameCacheable::new(consolidatedMotionRectangles.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::media::image::comparison::motion::MotionRectangles::MotionRectanglesDyn>>), jcall!(imageComparisonInfo).getFrameTwo()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::util::cache::CacheableInterface::CacheableInterface>>));
            let mut constrainedMotionRectangles: Option<Rc<RefCell<ConstrainedMotionRectangles>>> = ConstrainedMotionRectangles::new(self.motionRectangleConstraintsInterface.clone(), consolidatedMotionRectangles.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::media::image::comparison::motion::MotionRectangles::MotionRectanglesDyn>>));
            jcall!(constrainedMotionRectangles).applyMotionRectangleConstraints(consolidatedMotionRectangles.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::media::image::comparison::motion::MotionRectangles::MotionRectanglesDyn>>));
            jcall!(ConstrainedMotionRectanglesResultsCacheSingleton::ConstrainedMotionRectanglesResultsCacheSingleton::getInstance()).add(MotionRectanglesResultsFrameCacheable::new(constrainedMotionRectangles.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::media::image::comparison::motion::MotionRectangles::MotionRectanglesDyn>>), jcall!(imageComparisonInfo).getFrameTwo()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::util::cache::CacheableInterface::CacheableInterface>>));
            { let java_arguments = (MotionRectanglesResultsEvent::new(None /* Java this: a &mut self method has no Rc of itself */, jcall!(imageComparisonInfo).getFrameTwo(), (constrainedMotionRectangles).clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::media::image::comparison::motion::MotionRectangles::MotionRectanglesDyn>>)).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObjectDyn>>),); self.fireEvent(java_arguments.0) };
            jcall!(self.imageComparisonInfoVector).remove(crate::java_into_object(imageComparisonResultsEvent.clone()));
            self.index += 1;
            { let java_receiver = self.logUtil.clone(); let java_arguments = (format!("{}{}", { let java_field = jcall!(CommonLabels::getInstance()).ELAPSED.clone(); java_field }, jcall!(timeHelper).getElapsedTNT()), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(self.commonStrings).RUN.clone(); java_field }.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
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
impl std::ops::Deref for MotionRectanglesWorker {
    type Target = BasicEventHandler;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for MotionRectanglesWorker {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::logic::util::event::handler::BasicEventHandler::BasicEventHandlerDyn for MotionRectanglesWorker {
    fn as_BasicEventHandler(&self) -> &crate::org::allbinary::logic::util::event::handler::BasicEventHandler::BasicEventHandler { &self.base }
    fn as_BasicEventHandler_mut(&mut self) -> &mut crate::org::allbinary::logic::util::event::handler::BasicEventHandler::BasicEventHandler { &mut self.base }
}
impl crate::org::allbinary::logic::util::event::handler::BasicEventHandlerInterface::BasicEventHandlerInterface for MotionRectanglesWorker {
    fn addListenerInterface(&mut self, p1: Option<Rc<RefCell<dyn crate::org::allbinary::logic::util::event::EventListenerInterface::EventListenerInterface>>>) { crate::org::allbinary::logic::util::event::handler::BasicEventHandler::BasicEventHandler::addListenerInterface(crate::org::allbinary::logic::util::event::handler::BasicEventHandler::BasicEventHandlerDyn::as_BasicEventHandler_mut(self), p1) }
    fn addListeners(&mut self, p1: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) { crate::org::allbinary::logic::util::event::handler::BasicEventHandler::BasicEventHandler::addListeners(crate::org::allbinary::logic::util::event::handler::BasicEventHandler::BasicEventHandlerDyn::as_BasicEventHandler_mut(self), p1) }
    fn fireEvent(&mut self, p1: Option<Rc<RefCell<dyn crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObjectDyn>>>) { crate::org::allbinary::logic::util::event::handler::BasicEventHandler::BasicEventHandler::fireEvent(crate::org::allbinary::logic::util::event::handler::BasicEventHandler::BasicEventHandlerDyn::as_BasicEventHandler_mut(self), p1) }
    fn removeAllListeners(&mut self) { crate::org::allbinary::logic::util::event::handler::BasicEventHandler::BasicEventHandler::removeAllListeners(crate::org::allbinary::logic::util::event::handler::BasicEventHandler::BasicEventHandlerDyn::as_BasicEventHandler_mut(self)) }
    fn removeListener(&mut self, p1: Option<Rc<RefCell<dyn crate::org::allbinary::logic::util::event::EventListenerInterface::EventListenerInterface>>>) { crate::org::allbinary::logic::util::event::handler::BasicEventHandler::BasicEventHandler::removeListener(crate::org::allbinary::logic::util::event::handler::BasicEventHandler::BasicEventHandlerDyn::as_BasicEventHandler_mut(self), p1) }
}


