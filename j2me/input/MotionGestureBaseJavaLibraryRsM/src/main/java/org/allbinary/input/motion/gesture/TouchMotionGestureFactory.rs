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

pub struct TouchMotionGestureFactory {
    pub LEFT: Option<Rc<RefCell<MotionGestureInput>>>,
    pub RIGHT: Option<Rc<RefCell<MotionGestureInput>>>,
    pub UP: Option<Rc<RefCell<MotionGestureInput>>>,
    pub DOWN: Option<Rc<RefCell<MotionGestureInput>>>,
    pub PRESSED: Option<Rc<RefCell<MotionGestureInput>>>,
    pub RELEASED: Option<Rc<RefCell<MotionGestureInput>>>,
    pub DIAGONAL_DOWN_RIGHT: Option<Rc<RefCell<MotionGestureInput>>>,
    pub DIAGONAL_DOWN_LEFT: Option<Rc<RefCell<MotionGestureInput>>>,
    pub DIAGONAL_UP_RIGHT: Option<Rc<RefCell<MotionGestureInput>>>,
    pub DIAGONAL_UP_LEFT: Option<Rc<RefCell<MotionGestureInput>>>,
    pub SCROLL_UP: Option<Rc<RefCell<MotionGestureInput>>>,
    pub SCROLL_DOWN: Option<Rc<RefCell<MotionGestureInput>>>,
    pub NO_MOTION: Option<Rc<RefCell<MotionGestureInput>>>,
    pub LAST_MOTION: Option<Rc<RefCell<MotionGestureInput>>>,
}

pub static mut TouchMotionGestureFactory_instance: std::cell::LazyCell<Option<Rc<RefCell<Box<dyn std::any::Any>>>>> = std::cell::LazyCell::new(|| jcall!(NullUtil::getInstance()).NULL_OBJECT.clone());

impl TouchMotionGestureFactory {
    

    pub fn getInstance() -> Option<Rc<RefCell<TouchMotionGestureFactory>>> {
        if match (&unsafe { TouchMotionGestureFactory_instance.clone() }, &jcall!(NullUtil::getInstance()).NULL_OBJECT) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            unsafe { *TouchMotionGestureFactory_instance = TouchMotionGestureFactory::new() };
        }
        return (unsafe { TouchMotionGestureFactory_instance.clone() }).clone() /* as TouchMotionGestureFactory */;
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            LEFT: None,
            RIGHT: None,
            UP: None,
            DOWN: None,
            PRESSED: None,
            RELEASED: None,
            DIAGONAL_DOWN_RIGHT: None,
            DIAGONAL_DOWN_LEFT: None,
            DIAGONAL_UP_RIGHT: None,
            DIAGONAL_UP_LEFT: None,
            SCROLL_UP: None,
            SCROLL_DOWN: None,
            NO_MOTION: None,
            LAST_MOTION: None,
        };
        let mut MAX: i32 = jcall!(InputFactory::getInstance()).MAX.clone();
        this.LEFT = MotionGestureInput::new(MAX - 3, String::from("Left Touch"));
        this.RIGHT = MotionGestureInput::new(MAX - 4, String::from("Right Touch"));
        this.UP = MotionGestureInput::new(MAX - 5, String::from("Up Touch"));
        this.DOWN = MotionGestureInput::new(MAX - 6, String::from("Down Touch"));
        this.PRESSED = MotionGestureInput::new(MAX - 7, String::from("Screen Press"));
        this.RELEASED = MotionGestureInput::new(MAX - 8, String::from("Released"));
        this.DIAGONAL_DOWN_RIGHT = MotionGestureInput::new(MAX - 9, String::from("Diagonal Dn R"));
        this.DIAGONAL_DOWN_LEFT = MotionGestureInput::new(MAX - 10, String::from("Diagonal Dn L"));
        this.DIAGONAL_UP_RIGHT = MotionGestureInput::new(MAX - 11, String::from("Diagonal Up R"));
        this.DIAGONAL_UP_LEFT = MotionGestureInput::new(MAX - 12, String::from("Diagonal Up L"));
        this.SCROLL_UP = MotionGestureInput::new(MAX - 13, String::from("Scroll Up"));
        this.SCROLL_DOWN = MotionGestureInput::new(MAX - 14, String::from("Scroll Down"));
        this.NO_MOTION = MotionGestureInput::new(MAX - 2, String::from("No Motion"));
        this.LAST_MOTION = MotionGestureInput::new(MAX - 19, String::from("Last Motion"));
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
}

