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
// use crate::java::util::Set::Set;
use crate::org::allbinary::input::automation::robot::InputRobotFactory::InputRobotFactory;
use crate::org::allbinary::input::automation::robot::InputRobotInterface::InputRobotInterface;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::media::image::comparison::ImageComparatorConstraintsInterface::ImageComparatorConstraintsInterface;
use crate::org::allbinary::media::image::comparison::motion::MotionRectangleConstraintsInterface::MotionRectangleConstraintsInterface;
use crate::org::allbinary::util::ABHashtable::ABHashtable;
use crate::java::lang::Thread::*;
use super::AbstractInputAutomationWorker::AbstractInputAutomationWorker;
use super::InputAutomationActionInterface::InputAutomationActionInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct TestInputAutomationActionWorker {
    pub base: AbstractInputAutomationWorker,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub sleep: i32,
}

impl crate::Runnable for TestInputAutomationActionWorker {
    fn run(&mut self) { self.base.run() }
}

impl TestInputAutomationActionWorker {
    

    pub fn new_struct(mut sleep: i32, mut inputAutomationActionInterface: Option<Rc<RefCell<dyn InputAutomationActionInterface>>>, mut imageComparatorConstraintsInterface: Option<Rc<RefCell<dyn ImageComparatorConstraintsInterface>>>, mut motionRectangleConstraintsInterface: Option<Rc<RefCell<dyn MotionRectangleConstraintsInterface>>>) -> Self {
        let mut this = Self {
            base: AbstractInputAutomationWorker::new_struct(inputAutomationActionInterface.clone()),
            logUtil: None,
            sleep: 0,
        };
        this.logUtil = LogUtil::getInstance();
        // super(...) is in the struct literal above
        jcall!(this.logUtil).putF(String::from("BasicTestAttackWorder"), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(this.commonStrings).CONSTRUCTOR.clone());
        this.sleep = sleep.clone();
        this
    }

    pub fn new(mut sleep: i32, mut inputAutomationActionInterface: Option<Rc<RefCell<dyn InputAutomationActionInterface>>>, mut imageComparatorConstraintsInterface: Option<Rc<RefCell<dyn ImageComparatorConstraintsInterface>>>, mut motionRectangleConstraintsInterface: Option<Rc<RefCell<dyn MotionRectangleConstraintsInterface>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(sleep, inputAutomationActionInterface, imageComparatorConstraintsInterface, motionRectangleConstraintsInterface))))
    }
    

    // throws Exception
    pub fn processBetweenWorkers(&mut self) {
        std::thread::sleep(std::time::Duration::from_millis((2000) as u64));
    }
    

    // throws Exception
    pub fn process(&mut self) {
        jcall!(self.logUtil).putF(jcall!(self.commonStrings).START.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).PROCESS.clone());
        let mut hashtable: Option<Rc<RefCell<ABHashtable<Option<Rc<RefCell<Box<dyn std::any::Any>>>>, Option<Rc<RefCell<Box<dyn std::any::Any>>>>>>>> = jcall!(InputRobotFactory::getInstance()).get();
        let mut set: Option<Rc<RefCell<dyn Set>>> = jcall!(hashtable).keySet();
        let mut inputTypeNameArray: Vec<Option<Rc<RefCell<Box<dyn std::any::Any>>>>> = jcall!(set).toArray();
        let mut size: i32 = (inputTypeNameArray.len() as i32).clone();
        for index in (0..size) {
            let mut inputTypeNameString: String = (inputTypeNameArray[(index) as usize]).clone() /* as String */;
            let mut robot: Option<Rc<RefCell<dyn InputRobotInterface>>> = { let java_object = jcall!(hashtable).get(crate::java_into_object(inputTypeNameString.clone())); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn InputRobotInterface>>>>().expect("ClassCastException").clone()) };
            if self.sleep > 0 {
                std::thread::sleep(std::time::Duration::from_millis((self.sleep.clone()) as u64));
            }
        }
    }
}
impl std::ops::Deref for TestInputAutomationActionWorker {
    type Target = AbstractInputAutomationWorker;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for TestInputAutomationActionWorker {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::input::automation::module::AbstractInputAutomationWorker::AbstractInputAutomationWorkerDyn for TestInputAutomationActionWorker {
    fn as_AbstractInputAutomationWorker(&self) -> &crate::org::allbinary::input::automation::module::AbstractInputAutomationWorker::AbstractInputAutomationWorker { &self.base }
    fn as_AbstractInputAutomationWorker_mut(&mut self) -> &mut crate::org::allbinary::input::automation::module::AbstractInputAutomationWorker::AbstractInputAutomationWorker { &mut self.base }
    fn process(&mut self) { Self::process(self) }
}


