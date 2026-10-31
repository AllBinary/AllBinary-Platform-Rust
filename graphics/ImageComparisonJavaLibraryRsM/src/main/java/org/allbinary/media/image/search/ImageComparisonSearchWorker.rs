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
// package org.allbinary.media.image.search
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::java::awt::image::BufferedImage::BufferedImage;
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
use super::ImageComparisonSearchConstraintsInterface::ImageComparisonSearchConstraintsInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct ImageComparisonSearchWorker {
    pub base: BasicEventHandler,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub commonStrings: Option<Rc<RefCell<CommonStrings>>>,
    pub imageComparisonInfoVector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>,
    pub index: i64,
    pub running: bool,
    pub imageSearchConstraintsInterface: Option<Rc<RefCell<dyn ImageComparisonSearchConstraintsInterface>>>,
}

// impl ImageComparisonResultsListener for ImageComparisonSearchWorker with the methods of the trait

impl ImageComparisonSearchWorker {
    

    pub fn new_struct(mut imageSearchConstraintsInterface: Option<Rc<RefCell<dyn ImageComparisonSearchConstraintsInterface>>>) -> Self {
        let mut this = Self {
            base: BasicEventHandler::new_struct(),
            logUtil: None,
            commonStrings: None,
            imageComparisonInfoVector: None,
            index: 0_i64,
            running: false,
            imageSearchConstraintsInterface: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.commonStrings = CommonStrings::getInstance();
        this.imageComparisonInfoVector = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        this.imageSearchConstraintsInterface = imageSearchConstraintsInterface.clone();
        this
    }

    pub fn new(mut imageSearchConstraintsInterface: Option<Rc<RefCell<dyn ImageComparisonSearchConstraintsInterface>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(imageSearchConstraintsInterface))))
    }
    

    pub fn onImageComparisonResultsEvent(&mut self, mut imageComparisonResultsEvent: Option<Rc<RefCell<ImageComparisonResultsEvent>>>) {
        { let java_receiver = self.imageComparisonInfoVector.clone(); let java_arguments = (crate::java_into_object(jcall!(imageComparisonResultsEvent).getImageComparisonResult()),); jcall!(java_receiver).add(java_arguments.0); };
        self.run();
    }
    

    pub fn onEvent(&mut self, mut allBinaryEventObject: Option<Rc<RefCell<dyn crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObjectDyn>>>) {
        self.onImageComparisonResultsEvent((allBinaryEventObject).clone() /* as ImageComparisonResultsEvent */);
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
            jcall!(self.logUtil).putF(jcall!(self.commonStrings).START.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).RUN.clone());
            self.setRunning(true);
            let mut timeHelper: Option<Rc<RefCell<dyn crate::org::allbinary::time::TimeDelayHelper::TimeDelayHelperDyn>>> = TimeDelayHelper::new(1000).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::time::TimeDelayHelper::TimeDelayHelperDyn>>);
            jcall!(timeHelper).setStartTimeTNT();
            let mut imageComparisonInfo: Option<Rc<RefCell<ImageComparisonResult>>> = { let java_object = (jcall!(self.imageComparisonInfoVector).get(0)).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<ImageComparisonResult>>>>().expect("ClassCastException").clone()) };
            { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(imageComparisonInfo).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).RUN.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
            let mut latestBufferedImage: Option<Rc<RefCell<BufferedImage>>> = jcall!(imageComparisonInfo).getBufferedImages()[(1) as usize].clone();
            jcall!(self.imageComparisonInfoVector).remove(crate::java_into_object(imageComparisonInfo.clone()));
            self.index += 1;
            { let java_receiver = self.logUtil.clone(); let java_arguments = (format!("{}{}", jcall!(CommonLabels::getInstance()).ELAPSED, jcall!(timeHelper).getElapsedTNT()), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).RUN.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
            self.setRunning(false);
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
impl std::ops::Deref for ImageComparisonSearchWorker {
    type Target = BasicEventHandler;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for ImageComparisonSearchWorker {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::logic::util::event::handler::BasicEventHandler::BasicEventHandlerDyn for ImageComparisonSearchWorker {
    fn as_BasicEventHandler(&self) -> &crate::org::allbinary::logic::util::event::handler::BasicEventHandler::BasicEventHandler { &self.base }
    fn as_BasicEventHandler_mut(&mut self) -> &mut crate::org::allbinary::logic::util::event::handler::BasicEventHandler::BasicEventHandler { &mut self.base }
}
impl crate::org::allbinary::logic::util::event::handler::BasicEventHandlerInterface::BasicEventHandlerInterface for ImageComparisonSearchWorker {
    fn addListenerInterface(&mut self, p1: Option<Rc<RefCell<dyn crate::org::allbinary::logic::util::event::EventListenerInterface::EventListenerInterface>>>) { crate::org::allbinary::logic::util::event::handler::BasicEventHandler::BasicEventHandler::addListenerInterface(crate::org::allbinary::logic::util::event::handler::BasicEventHandler::BasicEventHandlerDyn::as_BasicEventHandler_mut(self), p1) }
    fn addListeners(&mut self, p1: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) { crate::org::allbinary::logic::util::event::handler::BasicEventHandler::BasicEventHandler::addListeners(crate::org::allbinary::logic::util::event::handler::BasicEventHandler::BasicEventHandlerDyn::as_BasicEventHandler_mut(self), p1) }
    fn fireEvent(&mut self, p1: Option<Rc<RefCell<dyn crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObjectDyn>>>) { crate::org::allbinary::logic::util::event::handler::BasicEventHandler::BasicEventHandler::fireEvent(crate::org::allbinary::logic::util::event::handler::BasicEventHandler::BasicEventHandlerDyn::as_BasicEventHandler_mut(self), p1) }
    fn removeAllListeners(&mut self) { crate::org::allbinary::logic::util::event::handler::BasicEventHandler::BasicEventHandler::removeAllListeners(crate::org::allbinary::logic::util::event::handler::BasicEventHandler::BasicEventHandlerDyn::as_BasicEventHandler_mut(self)) }
    fn removeListener(&mut self, p1: Option<Rc<RefCell<dyn crate::org::allbinary::logic::util::event::EventListenerInterface::EventListenerInterface>>>) { crate::org::allbinary::logic::util::event::handler::BasicEventHandler::BasicEventHandler::removeListener(crate::org::allbinary::logic::util::event::handler::BasicEventHandler::BasicEventHandlerDyn::as_BasicEventHandler_mut(self), p1) }
}


