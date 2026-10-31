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
// package org.allbinary.input.gyro
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::game::input::InputFactory::InputFactory;
use crate::org::allbinary::input::motion::gesture::MotionGestureInput::MotionGestureInput;
use crate::org::allbinary::logic::NullUtil::NullUtil;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct OrientationMotionGestureFactory {
    pub LEFT: Option<Rc<RefCell<MotionGestureInput>>>,
    pub RIGHT: Option<Rc<RefCell<MotionGestureInput>>>,
    pub UP: Option<Rc<RefCell<MotionGestureInput>>>,
    pub DOWN: Option<Rc<RefCell<MotionGestureInput>>>,
    pub ROLL_LEFT: Option<Rc<RefCell<MotionGestureInput>>>,
    pub ROLL_RIGHT: Option<Rc<RefCell<MotionGestureInput>>>,
}

pub static mut OrientationMotionGestureFactory_instance: std::cell::LazyCell<Option<Rc<RefCell<Box<dyn std::any::Any>>>>> = std::cell::LazyCell::new(|| jcall!(NullUtil::getInstance()).NULL_OBJECT.clone());

impl OrientationMotionGestureFactory {
    

    pub fn getInstance() -> Option<Rc<RefCell<OrientationMotionGestureFactory>>> {
        if match (&unsafe { OrientationMotionGestureFactory_instance.clone() }, &jcall!(NullUtil::getInstance()).NULL_OBJECT) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            unsafe { *OrientationMotionGestureFactory_instance = OrientationMotionGestureFactory::new() };
        }
        return (unsafe { OrientationMotionGestureFactory_instance.clone() }).clone() /* as OrientationMotionGestureFactory */;
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            LEFT: None,
            RIGHT: None,
            UP: None,
            DOWN: None,
            ROLL_LEFT: None,
            ROLL_RIGHT: None,
        };
        let mut MAX: i32 = jcall!(InputFactory::getInstance()).MAX.clone();
        this.LEFT = MotionGestureInput::new(MAX - 20, String::from("Left Orient"));
        this.RIGHT = MotionGestureInput::new(MAX - 21, String::from("Right Orient"));
        this.UP = MotionGestureInput::new(MAX - 22, String::from("Up Orient"));
        this.DOWN = MotionGestureInput::new(MAX - 23, String::from("Down Orient"));
        this.ROLL_LEFT = MotionGestureInput::new(MAX - 24, String::from("Roll Left"));
        this.ROLL_RIGHT = MotionGestureInput::new(MAX - 25, String::from("Roll Right"));
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
}

