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
// package org.allbinary.input.motion.gesture.observer
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use super::BaseMotionGestureEventListener::BaseMotionGestureEventListener;
use super::MotionGestureEvent::MotionGestureEvent;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub trait MotionGestureEventListener: BaseMotionGestureEventListener {
    

    fn onPressedMotionGestureEvent(&mut self, ev: Option<Rc<RefCell<MotionGestureEvent>>>);
    

    fn onUpMotionGestureEvent(&mut self, ev: Option<Rc<RefCell<MotionGestureEvent>>>);
    

    fn onDownMotionGestureEvent(&mut self, ev: Option<Rc<RefCell<MotionGestureEvent>>>);
    

    fn onLeftMotionGestureEvent(&mut self, ev: Option<Rc<RefCell<MotionGestureEvent>>>);
    

    fn onRightMotionGestureEvent(&mut self, ev: Option<Rc<RefCell<MotionGestureEvent>>>);
    

    fn onDiagonalDownRightMotionGestureEvent(&mut self, ev: Option<Rc<RefCell<MotionGestureEvent>>>);
    

    fn onDiagonalDownLeftMotionGestureEvent(&mut self, ev: Option<Rc<RefCell<MotionGestureEvent>>>);
    

    fn onDiagonalUpRightMotionGestureEvent(&mut self, ev: Option<Rc<RefCell<MotionGestureEvent>>>);
    

    fn onDiagonalUpLeftMotionGestureEvent(&mut self, ev: Option<Rc<RefCell<MotionGestureEvent>>>);
    

    // throws Exception
    fn released(&mut self, ev: Option<Rc<RefCell<MotionGestureEvent>>>);
}


