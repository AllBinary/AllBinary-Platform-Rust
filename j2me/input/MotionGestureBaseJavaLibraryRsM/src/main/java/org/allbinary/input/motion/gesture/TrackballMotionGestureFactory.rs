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
// package org.allbinary.input.motion.gesture
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::game::input::InputFactory::InputFactory;
use crate::org::allbinary::logic::NullUtil::NullUtil;
use super::MotionGestureInput::MotionGestureInput;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct TrackballMotionGestureFactory {
    pub LEFT: Option<Rc<RefCell<MotionGestureInput>>>,
    pub RIGHT: Option<Rc<RefCell<MotionGestureInput>>>,
    pub UP: Option<Rc<RefCell<MotionGestureInput>>>,
    pub DOWN: Option<Rc<RefCell<MotionGestureInput>>>,
}

pub static mut TrackballMotionGestureFactory_instance: std::cell::LazyCell<Option<Rc<RefCell<Box<dyn std::any::Any>>>>> = std::cell::LazyCell::new(|| jcall!(NullUtil::getInstance()).NULL_OBJECT.clone());

impl TrackballMotionGestureFactory {
    

    pub fn getInstance() -> Option<Rc<RefCell<TrackballMotionGestureFactory>>> {
        if match (&unsafe { TrackballMotionGestureFactory_instance.clone() }, &jcall!(NullUtil::getInstance()).NULL_OBJECT) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            unsafe { *TrackballMotionGestureFactory_instance = TrackballMotionGestureFactory::new() };
        }
        return (unsafe { TrackballMotionGestureFactory_instance.clone() }).clone() /* as TrackballMotionGestureFactory */;
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            LEFT: None,
            RIGHT: None,
            UP: None,
            DOWN: None,
        };
        let mut MAX: i32 = jcall!(InputFactory::getInstance()).MAX.clone();
        this.LEFT = MotionGestureInput::new(MAX - 26, String::from("L Trackball"));
        this.RIGHT = MotionGestureInput::new(MAX - 27, String::from("R Trackball"));
        this.UP = MotionGestureInput::new(MAX - 28, String::from("Up Trackball"));
        this.DOWN = MotionGestureInput::new(MAX - 29, String::from("D Trackball"));
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
}

