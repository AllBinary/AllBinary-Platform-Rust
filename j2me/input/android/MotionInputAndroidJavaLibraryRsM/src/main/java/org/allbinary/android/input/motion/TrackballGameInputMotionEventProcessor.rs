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
// package org.allbinary.android.input.motion
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::android::view::MotionEvent::MotionEvent;
use crate::org::allbinary::game::input::motion::action::SingleKeyPressGameKeyFromMotionGestureEventListener::SingleKeyPressGameKeyFromMotionGestureEventListener;
use crate::org::allbinary::input::motion::trackball::action::DownTrackballInputToGameKeyEventAction::DownTrackballInputToGameKeyEventAction;
use crate::org::allbinary::input::motion::trackball::action::LeftTrackballInputToGameKeyEventAction::LeftTrackballInputToGameKeyEventAction;
use crate::org::allbinary::input::motion::trackball::action::RightTrackballInputToGameKeyEventAction::RightTrackballInputToGameKeyEventAction;
use crate::org::allbinary::input::motion::trackball::action::UpTrackballInputToGameKeyEventAction::UpTrackballInputToGameKeyEventAction;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use super::MotionEventProcessor::MotionEventProcessor;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct TrackballGameInputMotionEventProcessor {
    pub base: MotionEventProcessor,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub listener: Option<Rc<RefCell<SingleKeyPressGameKeyFromMotionGestureEventListener>>>,
    pub commonStrings: Option<Rc<RefCell<CommonStrings>>>,
}

pub static mut TrackballGameInputMotionEventProcessor_PROCESSOR: std::cell::LazyCell<Option<Rc<RefCell<TrackballGameInputMotionEventProcessor>>>> = std::cell::LazyCell::new(|| TrackballGameInputMotionEventProcessor::new());

impl TrackballGameInputMotionEventProcessor {
    

    pub fn getInstance() -> Option<Rc<RefCell<TrackballGameInputMotionEventProcessor>>> {
        return unsafe { TrackballGameInputMotionEventProcessor_PROCESSOR.clone() }.clone();
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: MotionEventProcessor::new_struct(),
            logUtil: None,
            listener: None,
            commonStrings: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.listener = SingleKeyPressGameKeyFromMotionGestureEventListener::new();
        this.commonStrings = CommonStrings::getInstance();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn process(&mut self, mut motionEvent: Option<Rc<RefCell<MotionEvent>>>) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut x: f32 = jcall!(motionEvent).getX();
            let mut y: f32 = jcall!(motionEvent).getY();
            if x < 0.0_f32 {
                { let java_receiver = self.listener.clone(); let java_arguments = (LeftTrackballInputToGameKeyEventAction::getInstance().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::input::CompleteMotionGestureInputEvent::CompleteMotionGestureInputEventDyn>>),); jcall!(java_receiver).onCompleteMotionGestureInputEvent(java_arguments.0); };
            } else if x > 0.0_f32 {
                { let java_receiver = self.listener.clone(); let java_arguments = (RightTrackballInputToGameKeyEventAction::getInstance().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::input::CompleteMotionGestureInputEvent::CompleteMotionGestureInputEventDyn>>),); jcall!(java_receiver).onCompleteMotionGestureInputEvent(java_arguments.0); };
            }
            if y < 0.0_f32 {
                { let java_receiver = self.listener.clone(); let java_arguments = (UpTrackballInputToGameKeyEventAction::getInstance().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::input::CompleteMotionGestureInputEvent::CompleteMotionGestureInputEventDyn>>),); jcall!(java_receiver).onCompleteMotionGestureInputEvent(java_arguments.0); };
            } else if y > 0.0_f32 {
                { let java_receiver = self.listener.clone(); let java_arguments = (DownTrackballInputToGameKeyEventAction::getInstance().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::input::CompleteMotionGestureInputEvent::CompleteMotionGestureInputEventDyn>>),); jcall!(java_receiver).onCompleteMotionGestureInputEvent(java_arguments.0); };
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
impl std::ops::Deref for TrackballGameInputMotionEventProcessor {
    type Target = MotionEventProcessor;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for TrackballGameInputMotionEventProcessor {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::android::input::motion::MotionEventProcessor::MotionEventProcessorDyn for TrackballGameInputMotionEventProcessor {
    fn as_MotionEventProcessor(&self) -> &crate::org::allbinary::android::input::motion::MotionEventProcessor::MotionEventProcessor { &self.base }
    fn as_MotionEventProcessor_mut(&mut self) -> &mut crate::org::allbinary::android::input::motion::MotionEventProcessor::MotionEventProcessor { &mut self.base }
    fn process(&mut self, mut motionEvent: Option<Rc<RefCell<MotionEvent>>>) { Self::process(self, motionEvent) }
}


