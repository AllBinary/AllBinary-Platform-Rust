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
// use crate::java::awt::Rectangle::Rectangle;
// use crate::java::awt::event::InputEvent::InputEvent;
use crate::java::awt::event::KeyEvent::KeyEvent;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
use crate::org::allbinary::input::automation::robot::InputRobotFactory::InputRobotFactory;
use crate::org::allbinary::input::automation::robot::InputRobotInterface::InputRobotInterface;
use crate::org::allbinary::input::automation::robot::TempInputRobotNames::TempInputRobotNames;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObject;
use crate::org::allbinary::string::CommonLabels::CommonLabels;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::org::allbinary::time::TimeDelayHelper::TimeDelayHelper;
use crate::org::allbinary::util::ABHashtable::ABHashtable;
use crate::java::lang::Thread::*;
use crate::java::lang::Exception::*;
use super::MotionRectanglesResultsListener::MotionRectanglesResultsListener;
use super::MotionRectanglesResultsEvent::MotionRectanglesResultsEvent;
use super::MotionRectangles::MotionRectangles;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct TestInputForMotionRectanglesResultsWorker {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub commonStrings: Option<Rc<RefCell<CommonStrings>>>,
    pub index: i64,
    pub running: bool,
    pub motionRectanglesVector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>,
}

impl MotionRectanglesResultsListener for TestInputForMotionRectanglesResultsWorker {
    fn onMotionRectanglesImageComparisonResultsEvent(&mut self, mut motionRectanglesImageComparisonResultsEvent: Option<Rc<RefCell<MotionRectanglesResultsEvent>>>) { Self::onMotionRectanglesImageComparisonResultsEvent(self, motionRectanglesImageComparisonResultsEvent) }
}
impl crate::org::allbinary::logic::util::event::BasicEventListenerInterface::BasicEventListenerInterface for TestInputForMotionRectanglesResultsWorker {
}
impl crate::org::allbinary::logic::util::event::EventListenerInterface::EventListenerInterface for TestInputForMotionRectanglesResultsWorker {
    fn onEvent(&mut self, mut allBinaryEventObject: Option<Rc<RefCell<dyn crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObjectDyn>>>) { Self::onEvent(self, allBinaryEventObject) }
}

impl TestInputForMotionRectanglesResultsWorker {

    pub fn toString(&mut self) -> String {
        String::from(std::any::type_name::<Self>())
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            logUtil: None,
            commonStrings: None,
            index: 0_i64,
            running: false,
            motionRectanglesVector: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.commonStrings = CommonStrings::getInstance();
        this.motionRectanglesVector = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getMotionRectanglesVector(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        return self.motionRectanglesVector.clone();
    }
    

    pub fn onMotionRectanglesImageComparisonResultsEvent(&mut self, mut motionRectanglesImageComparisonResultsEvent: Option<Rc<RefCell<MotionRectanglesResultsEvent>>>) {
        jcall!(self.getMotionRectanglesVector()).add(crate::java_into_object(jcall!(motionRectanglesImageComparisonResultsEvent).getMotionRectangles()));
        self.run();
    }
    

    pub fn onEvent(&mut self, mut allBinaryEventObject: Option<Rc<RefCell<dyn crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObjectDyn>>>) {
        self.onMotionRectanglesImageComparisonResultsEvent((allBinaryEventObject).clone().map(|reference| { assert!({ let java_borrowed = reference.borrow(); let java_any: &dyn std::any::Any = &*java_borrowed; java_any.is::<MotionRectanglesResultsEvent>() }, "ClassCastException: not a MotionRectanglesResultsEvent"); unsafe { Rc::from_raw(Rc::into_raw(reference) as *const RefCell<MotionRectanglesResultsEvent>) } }));
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
            let mut motionRectangles: Option<Rc<RefCell<dyn crate::org::allbinary::media::image::comparison::motion::MotionRectangles::MotionRectanglesDyn>>> = { let java_object = (jcall!(self.getMotionRectanglesVector()).get(0)).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn crate::org::allbinary::media::image::comparison::motion::MotionRectangles::MotionRectanglesDyn>>>>().expect("ClassCastException").clone()) };
            let mut motionRectangleVector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = jcall!(motionRectangles).getVector();
            if jcall!(motionRectangleVector).size() > 0 {
                let mut rectangle: Option<Rc<RefCell<Rectangle>>> = { let java_object = (jcall!(motionRectangleVector).get(0)).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<Rectangle>>>>().expect("ClassCastException").clone()) };
                let mut robotHashtable: Option<Rc<RefCell<ABHashtable<crate::JavaObject, crate::JavaObject>>>> = jcall!(InputRobotFactory::getInstance()).get();
                let mut inputTypeNameArray: Vec<Option<Rc<RefCell<Box<dyn std::any::Any>>>>> = jcall!(jcall!(robotHashtable).keySet()).toArray();
                let mut size: i32 = (inputTypeNameArray.len() as i32).clone();
                for index in (0..size) {
                    let mut inputTypeNameString: String = { let java_object = (inputTypeNameArray[(index) as usize]).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() };
                    let mut inputRobotInterface: Option<Rc<RefCell<dyn InputRobotInterface>>> = { let java_object = (jcall!(robotHashtable).get(crate::java_into_object(inputTypeNameString.clone()))).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn InputRobotInterface>>>>().expect("ClassCastException").clone()) };
                    jcall!(inputRobotInterface).mouseMoveToTarget(rectangle.clone());
                    if (jcall!(inputRobotInterface).getName().cmp(&crate::org::allbinary::input::automation::robot::TempInputRobotNames::TempInputRobotNames_DX_NAME.with(Clone::clone).clone()) as i32) == 0 {
                        jcall!(inputRobotInterface).mousePress(unsafe { crate::java::awt::event::InputEvent::InputEvent_BUTTON1_MASK.clone() }.clone());
                        std::thread::sleep(std::time::Duration::from_millis((300) as u64));
                        jcall!(inputRobotInterface).mouseRelease(unsafe { crate::java::awt::event::InputEvent::InputEvent_BUTTON1_MASK.clone() }.clone());
                        jcall!(inputRobotInterface).mousePress(unsafe { crate::java::awt::event::InputEvent::InputEvent_BUTTON2_MASK.clone() }.clone());
                        std::thread::sleep(std::time::Duration::from_millis((300) as u64));
                        jcall!(inputRobotInterface).mouseRelease(unsafe { crate::java::awt::event::InputEvent::InputEvent_BUTTON2_MASK.clone() }.clone());
                        jcall!(inputRobotInterface).mousePress(unsafe { crate::java::awt::event::InputEvent::InputEvent_BUTTON3_MASK.clone() }.clone());
                        std::thread::sleep(std::time::Duration::from_millis((300) as u64));
                        jcall!(inputRobotInterface).mouseRelease(unsafe { crate::java::awt::event::InputEvent::InputEvent_BUTTON3_MASK.clone() }.clone());
                        jcall!(inputRobotInterface).keyPress(unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_1.clone() }.clone());
                        std::thread::sleep(std::time::Duration::from_millis((300) as u64));
                        jcall!(inputRobotInterface).keyRelease(unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_1.clone() }.clone());
                        jcall!(inputRobotInterface).keyPress(unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_2.clone() }.clone());
                        std::thread::sleep(std::time::Duration::from_millis((300) as u64));
                        jcall!(inputRobotInterface).keyRelease(unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_2.clone() }.clone());
                        jcall!(inputRobotInterface).keyPress(unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_3.clone() }.clone());
                        std::thread::sleep(std::time::Duration::from_millis((300) as u64));
                        jcall!(inputRobotInterface).keyRelease(unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_3.clone() }.clone());
                        jcall!(inputRobotInterface).keyPress(unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_4.clone() }.clone());
                        std::thread::sleep(std::time::Duration::from_millis((300) as u64));
                        jcall!(inputRobotInterface).keyRelease(unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_4.clone() }.clone());
                        jcall!(inputRobotInterface).keyPress(unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_F1.clone() }.clone());
                        std::thread::sleep(std::time::Duration::from_millis((300) as u64));
                        jcall!(inputRobotInterface).keyRelease(unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_F1.clone() }.clone());
                    }
                }
            }
            self.index += 1;
            jcall!(self.getMotionRectanglesVector()).remove(crate::java_into_object(motionRectangles.clone()));
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

