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
// package org.allbinary.android.input.motion.api5
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::android::view::MotionEvent::MotionEvent;
use crate::org::allbinary::android::input::motion::BaseGameInputMotionEventProcessor::BaseGameInputMotionEventProcessor;
use crate::org::allbinary::graphics::displayable::DisplayInfoSingleton::DisplayInfoSingleton;
use crate::org::allbinary::graphics::displayable::ScalableListener::ScalableListener;
use crate::org::allbinary::input::motion::AllMotionRecognizer::AllMotionRecognizer;
use crate::org::allbinary::input::motion::MotionRecognizer::MotionRecognizer;
use crate::org::allbinary::logic::NullUtil::NullUtil;
use crate::org::allbinary::logic::communication::log::ForcedLogUtil::ForcedLogUtil;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct GameInputMotionEventProcessorAPI5 {
    pub base: BaseGameInputMotionEventProcessor,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub displayInfo: Option<Rc<RefCell<DisplayInfoSingleton>>>,
    pub motionRecognizerArray: Vec<Option<Rc<RefCell<dyn crate::org::allbinary::input::motion::MotionRecognizer::MotionRecognizerDyn>>>>,
    pub TOTAL_MINUS_ONE: i32,
}

pub static mut GameInputMotionEventProcessorAPI5_instance: std::cell::LazyCell<Option<Rc<RefCell<Box<dyn std::any::Any>>>>> = std::cell::LazyCell::new(|| jcall!(NullUtil::getInstance()).NULL_OBJECT.clone());

impl GameInputMotionEventProcessorAPI5 {
    

    pub fn getInstance() -> Option<Rc<RefCell<GameInputMotionEventProcessorAPI5>>> {
        if match (&unsafe { GameInputMotionEventProcessorAPI5_instance.clone() }, &jcall!(NullUtil::getInstance()).NULL_OBJECT) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            unsafe { *GameInputMotionEventProcessorAPI5_instance = GameInputMotionEventProcessorAPI5::new() };
        }
        return (unsafe { GameInputMotionEventProcessorAPI5_instance.clone() }).clone() /* as GameInputMotionEventProcessorAPI5 */;
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: BaseGameInputMotionEventProcessor::new_struct(),
            logUtil: None,
            displayInfo: None,
            motionRecognizerArray: Vec::new(),
            TOTAL_MINUS_ONE: 0,
        };
        this.logUtil = LogUtil::getInstance();
        this.displayInfo = DisplayInfoSingleton::getInstance();
        let mut size: i32 = jcall!(GameInputMotionInfoAPI5::GameInputMotionInfoAPI5::getInstance()).MAX_POINTERS.clone();
        this.motionRecognizerArray = vec![None; (size) as usize];
        this.TOTAL_MINUS_ONE = size.clone();
        for index in (0..size) {
            this.motionRecognizerArray[(index) as usize] = AllMotionRecognizer::new();
        }
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn isValid(&mut self, mut pointerId: i32) -> bool {
        if pointerId > self.TOTAL_MINUS_ONE {
            let mut message: String = String::from("Log that the pointer Ids are not what would be expected or Android has more than what I set my stuff to support pointerId: ");
            ForcedLogUtil::log(format!("{}{}", message, pointerId), crate::java_into_object(String::from(std::any::type_name::<Self>())));
            return false;
        } else {
            return true;
        }
    }
    

    pub fn process(&mut self, mut motionEvent: Option<Rc<RefCell<MotionEvent>>>) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut action: i32 = jcall!(motionEvent).getAction();
            let mut total: i32 = (self.motionRecognizerArray.len() as i32).clone();
            if total > jcall!(motionEvent).getPointerCount() {
                total = jcall!(motionEvent).getPointerCount();
            } else {
                let mut message: String = String::from("Log that Android has more pointers than I have expected so increase it motionEvent.getPointerCount(): ");
                ForcedLogUtil::log(format!("{}{}", message, jcall!(motionEvent).getPointerCount()), crate::java_into_object(String::from(std::any::type_name::<Self>())));
            }
            let mut totalMinusOne: i32 = total - 1;
            if action == unsafe { crate::android::view::MotionEvent::MotionEvent_ACTION_DOWN.clone() } {
                for pointerIndex in (0..=totalMinusOne).rev() {
                    let mut pointerId: i32 = jcall!(motionEvent).getPointerId(pointerIndex.clone());
                    if self.isValid(pointerId.clone()) {
                        let mut scalableListener: Option<Rc<RefCell<dyn ScalableListener>>> = jcall!(self.displayInfo).getScalableListener();
                        let mut x: i32 = ((((jcall!(motionEvent).getX(pointerIndex.clone()) - jcall!(self.displayInfo).getLeft()) / jcall!(scalableListener).getRatio(jcall!(self.displayInfo).getRatio()))) as i32);
                        let mut y: i32 = ((((jcall!(motionEvent).getY(pointerIndex.clone()) - jcall!(self.displayInfo).getTop()) / jcall!(scalableListener).getRatio(jcall!(self.displayInfo).getRatio()))) as i32);
                        jcall!(self.motionRecognizerArray[(pointerId) as usize]).processStartMotionEvent(x.clone(), y.clone(), jcall!(motionEvent).getDeviceId(), 0);
                    }
                }
            } else if action == unsafe { crate::android::view::MotionEvent::MotionEvent_ACTION_UP.clone() } || action == unsafe { crate::android::view::MotionEvent::MotionEvent_ACTION_CANCEL.clone() } {
                for pointerIndex in (0..=totalMinusOne).rev() {
                    let mut pointerId: i32 = jcall!(motionEvent).getPointerId(pointerIndex.clone());
                    if self.isValid(pointerId.clone()) {
                        let mut scalableListener: Option<Rc<RefCell<dyn ScalableListener>>> = jcall!(self.displayInfo).getScalableListener();
                        let mut x: i32 = ((((jcall!(motionEvent).getX(pointerIndex.clone()) - jcall!(self.displayInfo).getLeft()) / jcall!(scalableListener).getRatio(jcall!(self.displayInfo).getRatio()))) as i32);
                        let mut y: i32 = ((((jcall!(motionEvent).getY(pointerIndex.clone()) - jcall!(self.displayInfo).getTop()) / jcall!(scalableListener).getRatio(jcall!(self.displayInfo).getRatio()))) as i32);
                        jcall!(self.motionRecognizerArray[(pointerId) as usize]).processEndMotionEvent(x.clone(), y.clone(), jcall!(motionEvent).getDeviceId(), 0);
                    }
                }
            } else if action == unsafe { crate::android::view::MotionEvent::MotionEvent_ACTION_MOVE.clone() } {
                for pointerIndex in (0..=totalMinusOne).rev() {
                    let mut pointerId: i32 = jcall!(motionEvent).getPointerId(pointerIndex.clone());
                    if self.isValid(pointerId.clone()) {
                        let mut scalableListener: Option<Rc<RefCell<dyn ScalableListener>>> = jcall!(self.displayInfo).getScalableListener();
                        let mut x: i32 = ((((jcall!(motionEvent).getX(pointerIndex.clone()) - jcall!(self.displayInfo).getLeft()) / jcall!(scalableListener).getRatio(jcall!(self.displayInfo).getRatio()))) as i32);
                        let mut y: i32 = ((((jcall!(motionEvent).getY(pointerIndex.clone()) - jcall!(self.displayInfo).getTop()) / jcall!(scalableListener).getRatio(jcall!(self.displayInfo).getRatio()))) as i32);
                        jcall!(self.motionRecognizerArray[(pointerId) as usize]).processDraggedMotionEvent(x.clone(), y.clone(), jcall!(motionEvent).getDeviceId(), 0);
                    }
                }
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
impl std::ops::Deref for GameInputMotionEventProcessorAPI5 {
    type Target = BaseGameInputMotionEventProcessor;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for GameInputMotionEventProcessorAPI5 {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::android::input::motion::BaseGameInputMotionEventProcessor::BaseGameInputMotionEventProcessorDyn for GameInputMotionEventProcessorAPI5 {
    fn as_BaseGameInputMotionEventProcessor(&self) -> &crate::org::allbinary::android::input::motion::BaseGameInputMotionEventProcessor::BaseGameInputMotionEventProcessor { &self.base }
    fn as_BaseGameInputMotionEventProcessor_mut(&mut self) -> &mut crate::org::allbinary::android::input::motion::BaseGameInputMotionEventProcessor::BaseGameInputMotionEventProcessor { &mut self.base }
}
impl crate::org::allbinary::android::input::motion::MotionEventProcessor::MotionEventProcessorDyn for GameInputMotionEventProcessorAPI5 {
    fn as_MotionEventProcessor(&self) -> &crate::org::allbinary::android::input::motion::MotionEventProcessor::MotionEventProcessor { &self.base.base }
    fn as_MotionEventProcessor_mut(&mut self) -> &mut crate::org::allbinary::android::input::motion::MotionEventProcessor::MotionEventProcessor { &mut self.base.base }
    fn process(&mut self, mut motionEvent: Option<Rc<RefCell<MotionEvent>>>) { Self::process(self, motionEvent) }
}


