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
use crate::org::allbinary::logic::StdUtil::StdUtil;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObject;
use crate::org::allbinary::string::CommonLabels::CommonLabels;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::org::allbinary::time::TimeDelayHelper::TimeDelayHelper;
use crate::java::lang::Exception::*;
use super::ImageComparisonResultsListener::ImageComparisonResultsListener;
use super::ImageComparisonResultsEvent::ImageComparisonResultsEvent;
use super::ImageComparisonResult::ImageComparisonResult;
use super::ComparisonImageInputOutput::ComparisonImageInputOutput;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct SaveImageComparisonWorker {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub commonStrings: Option<Rc<RefCell<CommonStrings>>>,
    pub running: bool,
    pub imageComparisonInfoVector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>,
}

impl ImageComparisonResultsListener for SaveImageComparisonWorker {
    fn onImageComparisonResultsEvent(&mut self, mut imageComparisonResultsEvent: Option<Rc<RefCell<ImageComparisonResultsEvent>>>) { Self::onImageComparisonResultsEvent(self, imageComparisonResultsEvent) }
}
impl crate::org::allbinary::logic::util::event::BasicEventListenerInterface::BasicEventListenerInterface for SaveImageComparisonWorker {
}
impl crate::org::allbinary::logic::util::event::EventListenerInterface::EventListenerInterface for SaveImageComparisonWorker {
    fn onEvent(&mut self, mut allBinaryEventObject: Option<Rc<RefCell<dyn crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObjectDyn>>>) { Self::onEvent(self, allBinaryEventObject) }
}

impl SaveImageComparisonWorker {

    pub fn toString(&mut self) -> String {
        String::from(std::any::type_name::<Self>())
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            logUtil: None,
            commonStrings: None,
            running: false,
            imageComparisonInfoVector: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.commonStrings = CommonStrings::getInstance();
        this.imageComparisonInfoVector = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
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
            let mut imageComparisonInfo: Option<Rc<RefCell<ImageComparisonResult>>> = jcall!(imageComparisonResultsEvent).getImageComparisonResult();
            { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(imageComparisonInfo).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(self.commonStrings).RUN.clone(); java_field }.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
            jcall!(ComparisonImageInputOutput::new()).save2(imageComparisonInfo.clone(), jcall!(imageComparisonInfo).getFrameTwo());
            jcall!(self.imageComparisonInfoVector).remove(crate::java_into_object(imageComparisonInfo.clone()));
            { let java_receiver = self.logUtil.clone(); let java_arguments = (format!("{}{}", { let java_field = jcall!(CommonLabels::getInstance()).ELAPSED.clone(); java_field }, jcall!(timeHelper).getElapsedTNT()), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(self.commonStrings).RUN.clone(); java_field }.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
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

