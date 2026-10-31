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
// package org.allbinary.input.motion.gyro.action
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::game::input::PlatformInputMappingFactory::PlatformInputMappingFactory;
use crate::org::allbinary::game::input::motion::action::GameKeyCompleteMotionGestureInputEvent::GameKeyCompleteMotionGestureInputEvent;
use crate::org::allbinary::input::gyro::OrientationMotionGestureFactory::OrientationMotionGestureFactory;
use crate::org::allbinary::logic::NullUtil::NullUtil;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct RightOrientationInputToGameKeyEventAction {
    pub base: GameKeyCompleteMotionGestureInputEvent,
}

pub static mut RightOrientationInputToGameKeyEventAction_instance: std::cell::LazyCell<Option<Rc<RefCell<Box<dyn std::any::Any>>>>> = std::cell::LazyCell::new(|| jcall!(NullUtil::getInstance()).NULL_OBJECT.clone());

impl RightOrientationInputToGameKeyEventAction {
    

    pub fn getInstance() -> Option<Rc<RefCell<GameKeyCompleteMotionGestureInputEvent>>> {
        if match (&unsafe { RightOrientationInputToGameKeyEventAction_instance.clone() }, &jcall!(NullUtil::getInstance()).NULL_OBJECT) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            unsafe { *RightOrientationInputToGameKeyEventAction_instance = RightOrientationInputToGameKeyEventAction::new() };
        }
        return (unsafe { RightOrientationInputToGameKeyEventAction_instance.clone() }).clone() /* as GameKeyCompleteMotionGestureInputEvent */;
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: GameKeyCompleteMotionGestureInputEvent::new_struct3(String::from("Right Action"), jcall!(OrientationMotionGestureFactory::getInstance()).RIGHT.clone(), jcall!(jcall!(PlatformInputMappingFactory::getInstance()).getPersistentInputMappingInstance()).getInputMapping()),
        };
        // super(...) is in the struct literal above
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
}
impl std::ops::Deref for RightOrientationInputToGameKeyEventAction {
    type Target = GameKeyCompleteMotionGestureInputEvent;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for RightOrientationInputToGameKeyEventAction {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}



