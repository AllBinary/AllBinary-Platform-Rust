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
// package org.allbinary.input.automation.module.game.test
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;
use super::*;
// use crate::java::util::HashMap::HashMap;
// use crate::java::util::Set::Set;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
use crate::org::allbinary::input::automation::module::AbstractInputAutomationWorker::AbstractInputAutomationWorker;
use crate::org::allbinary::input::automation::module::InputAutomationActionInterface::InputAutomationActionInterface;
use crate::org::allbinary::input::automation::module::generic::configuration::profile::actions::GenericProfileAction::GenericProfileAction;
use crate::org::allbinary::input::automation::module::generic::configuration::profile::actions::GenericProfileActions::GenericProfileActions;
use crate::org::allbinary::input::automation::module::generic::configuration::profile::actions::script::GenericProfileActionScript::GenericProfileActionScript;
use crate::org::allbinary::input::media::image::capture::CapturedBufferedImagesCacheSingleton::CapturedBufferedImagesCacheSingleton;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::util::cache::J2SECacheInterface::J2SECacheInterface;
use crate::org::allbinary::media::image::comparison::ImageComparatorConstraintsInterface::ImageComparatorConstraintsInterface;
use crate::org::allbinary::media::image::comparison::motion::MotionRectangleConstraintsInterface::MotionRectangleConstraintsInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct TestInputAutomationCaptureWorker {
    pub base: AbstractInputAutomationWorker,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub genericProfileActions: Option<Rc<RefCell<GenericProfileActions>>>,
    pub MESSAGE: String,
    pub PROCESS_DATA_WORKER_RESULTS: String,
    pub IMAGE_AVAILABLE_SO_PROCESSING: String,
    pub IMAGE_NOT_AVAILABLE: String,
}

impl crate::Runnable for TestInputAutomationCaptureWorker {
    fn run(&mut self) { self.base.run() }
}

impl TestInputAutomationCaptureWorker {
    

    pub fn new_struct(mut inputAutomationActionInterface: Option<Rc<RefCell<dyn InputAutomationActionInterface>>>, mut genericProfileActions: Option<Rc<RefCell<GenericProfileActions>>>, mut imageComparatorConstraintsInterface: Option<Rc<RefCell<dyn ImageComparatorConstraintsInterface>>>, mut motionRectangleConstraintsInterface: Option<Rc<RefCell<dyn MotionRectangleConstraintsInterface>>>) -> Self {
        let mut this = Self {
            base: AbstractInputAutomationWorker::new_struct(inputAutomationActionInterface.clone()),
            logUtil: None,
            genericProfileActions: None,
            MESSAGE: String::new(),
            PROCESS_DATA_WORKER_RESULTS: String::new(),
            IMAGE_AVAILABLE_SO_PROCESSING: String::new(),
            IMAGE_NOT_AVAILABLE: String::new(),
        };
        this.logUtil = LogUtil::getInstance();
        this.MESSAGE = String::from("GenericInputAutomationCaptureWorker");
        this.PROCESS_DATA_WORKER_RESULTS = String::from("processDataWorkerResults");
        this.IMAGE_AVAILABLE_SO_PROCESSING = String::from("Image Available so processing");
        this.IMAGE_NOT_AVAILABLE = String::from("Image Not Available");
        // super(...) is in the struct literal above
        jcall!(this.logUtil).putF(this.MESSAGE.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(this.commonStrings).CONSTRUCTOR.clone());
        this.setGenericProfileActions(genericProfileActions.clone());
        this
    }

    pub fn new(mut inputAutomationActionInterface: Option<Rc<RefCell<dyn InputAutomationActionInterface>>>, mut genericProfileActions: Option<Rc<RefCell<GenericProfileActions>>>, mut imageComparatorConstraintsInterface: Option<Rc<RefCell<dyn ImageComparatorConstraintsInterface>>>, mut motionRectangleConstraintsInterface: Option<Rc<RefCell<dyn MotionRectangleConstraintsInterface>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(inputAutomationActionInterface, genericProfileActions, imageComparatorConstraintsInterface, motionRectangleConstraintsInterface))))
    }
    

    // throws Exception
    pub fn processDataWorkerResults(&mut self) {
        let mut cacheInterface: Option<Rc<RefCell<dyn J2SECacheInterface>>> = (CapturedBufferedImagesCacheSingleton::getInstance()).clone() /* as J2SECacheInterface */;
        if jcall!(jcall!(cacheInterface).keySet()).size() > 0 {
            jcall!(self.logUtil).putF(self.IMAGE_AVAILABLE_SO_PROCESSING.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), self.PROCESS_DATA_WORKER_RESULTS.clone());
            let mut anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = jcall!(jcall!(cacheInterface).keySet()).toArray()[(0) as usize].clone();
            let mut hashMap: Option<Rc<RefCell<HashMap>>> = jcall!(self.getGenericProfileActions()).getHashMap();
            let mut set: Option<Rc<RefCell<dyn Set>>> = jcall!(hashMap).keySet();
            let mut actionNameArray: Vec<Option<Rc<RefCell<Box<dyn std::any::Any>>>>> = jcall!(set).toArray();
            let mut size: i32 = (actionNameArray.len() as i32).clone();
            for index in (0..size) {
                let mut actionNameString: String = (actionNameArray[(index) as usize]).clone() /* as String */;
                let mut genericProfileAction: Option<Rc<RefCell<GenericProfileAction>>> = (jcall!(hashMap).get(actionNameString.clone())).clone() /* as GenericProfileAction */;
                let mut genericProfileActionScript: Option<Rc<RefCell<GenericProfileActionScript>>> = jcall!(genericProfileAction).getGenericProfileActionScript();
                let mut vector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = jcall!(genericProfileActionScript).getProfileActionConditionInterfaceVector();
            }
            jcall!(cacheInterface).get(anyType.clone());
        } else {
            jcall!(self.logUtil).putF(self.IMAGE_NOT_AVAILABLE.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), self.PROCESS_DATA_WORKER_RESULTS.clone());
        }
    }
    

    // throws Exception
    pub fn process(&mut self) {
        jcall!(self.logUtil).putF(jcall!(self.commonStrings).START.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).PROCESS.clone());
        self.startDataWorkers();
        self.processDataWorkerResults();
    }
    

    pub fn getGenericProfileActions(&mut self) -> Option<Rc<RefCell<GenericProfileActions>>> {
        return self.genericProfileActions.clone();
    }
    

    pub fn setGenericProfileActions(&mut self, mut genericProfileActions: Option<Rc<RefCell<GenericProfileActions>>>) {
        self.genericProfileActions = genericProfileActions.clone();
    }
}
impl std::ops::Deref for TestInputAutomationCaptureWorker {
    type Target = AbstractInputAutomationWorker;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for TestInputAutomationCaptureWorker {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::input::automation::module::AbstractInputAutomationWorker::AbstractInputAutomationWorkerDyn for TestInputAutomationCaptureWorker {
    fn as_AbstractInputAutomationWorker(&self) -> &crate::org::allbinary::input::automation::module::AbstractInputAutomationWorker::AbstractInputAutomationWorker { &self.base }
    fn as_AbstractInputAutomationWorker_mut(&mut self) -> &mut crate::org::allbinary::input::automation::module::AbstractInputAutomationWorker::AbstractInputAutomationWorker { &mut self.base }
    fn process(&mut self) { Self::process(self) }
}


