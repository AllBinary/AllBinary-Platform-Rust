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
// package org.allbinary.input.motion
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::input::motion::button::TouchButtonRecognizer::TouchButtonRecognizer;
use super::MotionRecognizer::MotionRecognizer;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct OnlyTouchButtonRecognizer {
    pub base: MotionRecognizer,
    pub touchButtonRecognizer: Option<Rc<RefCell<TouchButtonRecognizer>>>,
    pub touchButtonProcessing: bool,
}


impl OnlyTouchButtonRecognizer {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: MotionRecognizer::new_struct(),
            touchButtonRecognizer: None,
            touchButtonProcessing: false,
        };
        this.touchButtonProcessing = false;
        this.touchButtonRecognizer = TouchButtonRecognizer::new();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    // throws Exception
    pub fn processStartMotionEvent(&mut self, mut x: i32, mut y: i32, mut deviceId: i32, mut modifiers: i32) {
        if jcall!(self.touchButtonRecognizer).pressTouchButtonInput(x.clone(), y.clone(), deviceId.clone()) {
            self.touchButtonProcessing = true;
        }
    }
    

    // throws Exception
    pub fn processEndMotionEvent(&mut self, mut x: i32, mut y: i32, mut deviceId: i32, mut modifiers: i32) {
        if self.touchButtonProcessing {
            self.touchButtonProcessing = false;
        }
        if jcall!(self.touchButtonRecognizer).releaseTouchButtonInput(x.clone(), y.clone(), deviceId.clone()) {
            return;
        }
    }
    

    // throws Exception
    pub fn processDraggedMotionEvent(&mut self, mut x: i32, mut y: i32, mut deviceId: i32, mut modifiers: i32) {
        if self.touchButtonProcessing {
            jcall!(self.touchButtonRecognizer).pressTouchButtonInput(x.clone(), y.clone(), deviceId.clone());
        }
    }
}
impl std::ops::Deref for OnlyTouchButtonRecognizer {
    type Target = MotionRecognizer;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for OnlyTouchButtonRecognizer {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::input::motion::MotionRecognizer::MotionRecognizerDyn for OnlyTouchButtonRecognizer {
    fn as_MotionRecognizer(&self) -> &crate::org::allbinary::input::motion::MotionRecognizer::MotionRecognizer { &self.base }
    fn as_MotionRecognizer_mut(&mut self) -> &mut crate::org::allbinary::input::motion::MotionRecognizer::MotionRecognizer { &mut self.base }
    fn processStartMotionEvent(&mut self, mut x: i32, mut y: i32, mut deviceId: i32, mut modifiers: i32) { Self::processStartMotionEvent(self, x, y, deviceId, modifiers) }
    fn processEndMotionEvent(&mut self, mut x: i32, mut y: i32, mut deviceId: i32, mut modifiers: i32) { Self::processEndMotionEvent(self, x, y, deviceId, modifiers) }
    fn processDraggedMotionEvent(&mut self, mut x: i32, mut y: i32, mut deviceId: i32, mut modifiers: i32) { Self::processDraggedMotionEvent(self, x, y, deviceId, modifiers) }
}


