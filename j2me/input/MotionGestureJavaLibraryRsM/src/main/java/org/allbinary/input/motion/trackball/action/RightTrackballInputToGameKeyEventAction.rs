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
// package org.allbinary.input.motion.trackball.action
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::game::input::PlatformInputMappingFactory::PlatformInputMappingFactory;
use crate::org::allbinary::game::input::motion::action::GameKeyCompleteMotionGestureInputEvent::GameKeyCompleteMotionGestureInputEvent;
use crate::org::allbinary::input::motion::gesture::TrackballMotionGestureFactory::TrackballMotionGestureFactory;
use crate::org::allbinary::logic::NullUtil::NullUtil;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct RightTrackballInputToGameKeyEventAction {
    pub base: GameKeyCompleteMotionGestureInputEvent,
}

pub static mut RightTrackballInputToGameKeyEventAction_instance: std::cell::LazyCell<Option<Rc<RefCell<Box<dyn std::any::Any>>>>> = std::cell::LazyCell::new(|| jcall!(NullUtil::getInstance()).NULL_OBJECT.clone());
impl crate::org::allbinary::game::input::CompleteMotionGestureInputInterface::CompleteMotionGestureInputInterface for RightTrackballInputToGameKeyEventAction {
}

impl RightTrackballInputToGameKeyEventAction {
    

    pub fn getInstance() -> Option<Rc<RefCell<dyn crate::org::allbinary::game::input::motion::action::GameKeyCompleteMotionGestureInputEvent::GameKeyCompleteMotionGestureInputEventDyn>>> {
        if match (&unsafe { RightTrackballInputToGameKeyEventAction_instance.clone() }, &jcall!(NullUtil::getInstance()).NULL_OBJECT) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            unsafe { *RightTrackballInputToGameKeyEventAction_instance = RightTrackballInputToGameKeyEventAction::new() };
        }
        return (unsafe { RightTrackballInputToGameKeyEventAction_instance.clone() }).clone() /* as GameKeyCompleteMotionGestureInputEvent */;
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: GameKeyCompleteMotionGestureInputEvent::new_struct(String::from("Right Action"), jcall!(TrackballMotionGestureFactory::getInstance()).RIGHT.clone(), jcall!(jcall!(PlatformInputMappingFactory::getInstance()).getPersistentInputMappingInstance()).getInputMapping()),
        };
        // super(...) is in the struct literal above
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
}
impl std::ops::Deref for RightTrackballInputToGameKeyEventAction {
    type Target = GameKeyCompleteMotionGestureInputEvent;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for RightTrackballInputToGameKeyEventAction {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::game::input::motion::action::GameKeyCompleteMotionGestureInputEvent::GameKeyCompleteMotionGestureInputEventDyn for RightTrackballInputToGameKeyEventAction {
    fn as_GameKeyCompleteMotionGestureInputEvent(&self) -> &crate::org::allbinary::game::input::motion::action::GameKeyCompleteMotionGestureInputEvent::GameKeyCompleteMotionGestureInputEvent { &self.base }
    fn as_GameKeyCompleteMotionGestureInputEvent_mut(&mut self) -> &mut crate::org::allbinary::game::input::motion::action::GameKeyCompleteMotionGestureInputEvent::GameKeyCompleteMotionGestureInputEvent { &mut self.base }
}
impl crate::org::allbinary::game::input::CompleteMotionGestureInputEvent::CompleteMotionGestureInputEventDyn for RightTrackballInputToGameKeyEventAction {
    fn as_CompleteMotionGestureInputEvent(&self) -> &crate::org::allbinary::game::input::CompleteMotionGestureInputEvent::CompleteMotionGestureInputEvent { &self.base.base }
    fn as_CompleteMotionGestureInputEvent_mut(&mut self) -> &mut crate::org::allbinary::game::input::CompleteMotionGestureInputEvent::CompleteMotionGestureInputEvent { &mut self.base.base }
}
impl crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObjectDyn for RightTrackballInputToGameKeyEventAction {
    fn as_AllBinaryEventObject(&self) -> &crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObject { &self.base.base.base }
    fn as_AllBinaryEventObject_mut(&mut self) -> &mut crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObject { &mut self.base.base.base }
}
impl crate::org::allbinary::game::input::GameKeyEventSourceInterface::GameKeyEventSourceInterface for RightTrackballInputToGameKeyEventAction {
    fn getSourceId(&mut self) -> i32 { crate::org::allbinary::game::input::motion::action::GameKeyCompleteMotionGestureInputEvent::GameKeyCompleteMotionGestureInputEvent::getSourceId(crate::org::allbinary::game::input::motion::action::GameKeyCompleteMotionGestureInputEvent::GameKeyCompleteMotionGestureInputEventDyn::as_GameKeyCompleteMotionGestureInputEvent_mut(self)) }
}


