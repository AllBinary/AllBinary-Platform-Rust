//
// AllBinary Open License Version 1
// Copyright (c) 2009 AllBinary
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
// package org.allbinary.android.input.motion.api1
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::android::view::MotionEvent::MotionEvent;
use crate::org::allbinary::android::input::motion::BaseGameInputMotionEventProcessor::BaseGameInputMotionEventProcessor;
use crate::org::allbinary::input::motion::AllMotionRecognizer::AllMotionRecognizer;
use crate::org::allbinary::input::motion::MotionRecognizer::MotionRecognizer;
use crate::org::allbinary::logic::NullUtil::NullUtil;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct GameInputMotionEventProcessorAPI1 {
    pub base: BaseGameInputMotionEventProcessor,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub motionRecognizer: Option<Rc<RefCell<dyn crate::org::allbinary::input::motion::MotionRecognizer::MotionRecognizerDyn>>>,
}

pub static mut GameInputMotionEventProcessorAPI1_instance: std::cell::LazyCell<Option<Rc<RefCell<Box<dyn std::any::Any>>>>> = std::cell::LazyCell::new(|| jcall!(NullUtil::getInstance()).NULL_OBJECT.clone());

impl GameInputMotionEventProcessorAPI1 {
    

    pub fn getInstance() -> Option<Rc<RefCell<GameInputMotionEventProcessorAPI1>>> {
        if match (&unsafe { GameInputMotionEventProcessorAPI1_instance.clone() }, &jcall!(NullUtil::getInstance()).NULL_OBJECT) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            unsafe { *GameInputMotionEventProcessorAPI1_instance = GameInputMotionEventProcessorAPI1::new() };
        }
        return (unsafe { GameInputMotionEventProcessorAPI1_instance.clone() }).clone() /* as GameInputMotionEventProcessorAPI1 */;
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: BaseGameInputMotionEventProcessor::new_struct(),
            logUtil: None,
            motionRecognizer: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.motionRecognizer = AllMotionRecognizer::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::input::motion::MotionRecognizer::MotionRecognizerDyn>>);
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn process(&mut self, mut motionEvent: Option<Rc<RefCell<MotionEvent>>>) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut x: i32 = ((jcall!(motionEvent).getX()) as i32);
            let mut y: i32 = ((jcall!(motionEvent).getY()) as i32);
            let mut action: i32 = jcall!(motionEvent).getAction();
            if action == unsafe { crate::android::view::MotionEvent::MotionEvent_ACTION_DOWN.clone() } {
                { let java_receiver = self.motionRecognizer.clone(); let java_arguments = (x.clone(), y.clone(), jcall!(motionEvent).getDeviceId(), 0,); jcall!(java_receiver).processStartMotionEvent(java_arguments.0, java_arguments.1, java_arguments.2, java_arguments.3); };
            } else if action == unsafe { crate::android::view::MotionEvent::MotionEvent_ACTION_UP.clone() } || action == unsafe { crate::android::view::MotionEvent::MotionEvent_ACTION_CANCEL.clone() } {
                { let java_receiver = self.motionRecognizer.clone(); let java_arguments = (x.clone(), y.clone(), jcall!(motionEvent).getDeviceId(), 0,); jcall!(java_receiver).processEndMotionEvent(java_arguments.0, java_arguments.1, java_arguments.2, java_arguments.3); };
            } else if action == unsafe { crate::android::view::MotionEvent::MotionEvent_ACTION_MOVE.clone() } {
                { let java_receiver = self.motionRecognizer.clone(); let java_arguments = (x.clone(), y.clone(), jcall!(motionEvent).getDeviceId(), 0,); jcall!(java_receiver).processDraggedMotionEvent(java_arguments.0, java_arguments.1, java_arguments.2, java_arguments.3); };
            }
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).PROCESS.clone(), crate::java_into_object(e.clone()));
            }
        }
    }
}
impl std::ops::Deref for GameInputMotionEventProcessorAPI1 {
    type Target = BaseGameInputMotionEventProcessor;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for GameInputMotionEventProcessorAPI1 {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::android::input::motion::BaseGameInputMotionEventProcessor::BaseGameInputMotionEventProcessorDyn for GameInputMotionEventProcessorAPI1 {
    fn as_BaseGameInputMotionEventProcessor(&self) -> &crate::org::allbinary::android::input::motion::BaseGameInputMotionEventProcessor::BaseGameInputMotionEventProcessor { &self.base }
    fn as_BaseGameInputMotionEventProcessor_mut(&mut self) -> &mut crate::org::allbinary::android::input::motion::BaseGameInputMotionEventProcessor::BaseGameInputMotionEventProcessor { &mut self.base }
}
impl crate::org::allbinary::android::input::motion::MotionEventProcessor::MotionEventProcessorDyn for GameInputMotionEventProcessorAPI1 {
    fn as_MotionEventProcessor(&self) -> &crate::org::allbinary::android::input::motion::MotionEventProcessor::MotionEventProcessor { &self.base.base }
    fn as_MotionEventProcessor_mut(&mut self) -> &mut crate::org::allbinary::android::input::motion::MotionEventProcessor::MotionEventProcessor { &mut self.base.base }
    fn process(&mut self, mut motionEvent: Option<Rc<RefCell<MotionEvent>>>) { Self::process(self, motionEvent) }
}


