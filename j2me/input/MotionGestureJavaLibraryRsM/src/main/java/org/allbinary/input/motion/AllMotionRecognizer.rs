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
use crate::org::allbinary::graphics::CustomGPoint::CustomGPoint;
use crate::org::allbinary::graphics::GPointCircularPool::GPointCircularPool;
use crate::org::allbinary::input::motion::button::TouchButtonRecognizer::TouchButtonRecognizer;
use crate::org::allbinary::input::motion::gesture::MotionGestureRecognizer::MotionGestureRecognizer;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use super::MotionRecognizer::MotionRecognizer;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct AllMotionRecognizer {
    pub base: MotionRecognizer,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub motionGestureRecognizer: Option<Rc<RefCell<MotionGestureRecognizer>>>,
    pub touchButtonProcessing: bool,
    pub touchButtonRecognizer: Option<Rc<RefCell<TouchButtonRecognizer>>>,
    pub id: i32,
    pub lastX: i32,
    pub lastY: i32,
}

pub static mut AllMotionRecognizer_index: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| 0);
thread_local! { pub static AllMotionRecognizer_pointCircularPool: Option<Rc<RefCell<GPointCircularPool>>> = GPointCircularPool::new(40); }

impl AllMotionRecognizer {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: MotionRecognizer::new_struct(),
            logUtil: None,
            motionGestureRecognizer: None,
            touchButtonProcessing: false,
            touchButtonRecognizer: None,
            id: 0,
            lastX: 0,
            lastY: 0,
        };
        this.logUtil = LogUtil::getInstance();
        this.touchButtonProcessing = false;
        this.id = unsafe { let value = AllMotionRecognizer_index.clone(); *AllMotionRecognizer_index += 1; value };
        this.motionGestureRecognizer = MotionGestureRecognizer::new(this.id.clone());
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
        if !self.touchButtonProcessing {
            let mut point: Option<Rc<RefCell<CustomGPoint>>> = { let java_object = (jcall!(AllMotionRecognizer_pointCircularPool.with(Clone::clone)).getNextInstance()).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<CustomGPoint>>>>().expect("ClassCastException").clone()) };
            jcall!(point).setX(x.clone());
            jcall!(point).setY(y.clone());
            jcall!(self.motionGestureRecognizer).processPressedMotionEvent(point.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::graphics::GPoint::GPointDyn>>), deviceId.clone(), modifiers.clone());
        }
    }
    

    // throws Exception
    pub fn processEndMotionEvent(&mut self, mut x: i32, mut y: i32, mut deviceId: i32, mut modifiers: i32) {
        if jcall!(self.touchButtonRecognizer).releaseTouchButtonInput(x.clone(), y.clone(), deviceId.clone()) {
        }
        if !self.touchButtonProcessing {
            let mut point: Option<Rc<RefCell<CustomGPoint>>> = { let java_object = (jcall!(AllMotionRecognizer_pointCircularPool.with(Clone::clone)).getNextInstance()).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<CustomGPoint>>>>().expect("ClassCastException").clone()) };
            jcall!(point).setX(x.clone());
            jcall!(point).setY(y.clone());
            jcall!(self.motionGestureRecognizer).processReleasedMotionEvent(point.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::graphics::GPoint::GPointDyn>>), deviceId.clone(), modifiers.clone());
        }
        if self.touchButtonProcessing {
            self.touchButtonProcessing = false;
        }
    }
    

    // throws Exception
    pub fn processDraggedMotionEvent(&mut self, mut x: i32, mut y: i32, mut deviceId: i32, mut modifiers: i32) {
        if self.touchButtonProcessing {
            jcall!(self.touchButtonRecognizer).pressTouchButtonInput(x.clone(), y.clone(), deviceId.clone());
        }
        if !self.touchButtonProcessing {
            let mut point: Option<Rc<RefCell<CustomGPoint>>> = { let java_object = (jcall!(AllMotionRecognizer_pointCircularPool.with(Clone::clone)).getNextInstance()).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<CustomGPoint>>>>().expect("ClassCastException").clone()) };
            jcall!(point).setX(x.clone());
            jcall!(point).setY(y.clone());
            jcall!(self.motionGestureRecognizer).processDraggedMotionEvent(point.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::graphics::GPoint::GPointDyn>>), deviceId.clone(), modifiers.clone());
        }
    }
    

    // throws Exception
    pub fn processMovedMotionEvent(&mut self, mut x: i32, mut y: i32, mut deviceId: i32, mut modifiers: i32) {
        if x != self.lastX || y != self.lastY {
            self.lastX = x.clone();
            self.lastY = y.clone();
            let mut point: Option<Rc<RefCell<CustomGPoint>>> = { let java_object = (jcall!(AllMotionRecognizer_pointCircularPool.with(Clone::clone)).getNextInstance()).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<CustomGPoint>>>>().expect("ClassCastException").clone()) };
            jcall!(point).setX(x.clone());
            jcall!(point).setY(y.clone());
            jcall!(self.motionGestureRecognizer).processMovedMotionEvent(point.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::graphics::GPoint::GPointDyn>>), deviceId.clone(), modifiers.clone());
        }
    }
    

    // throws Exception
    pub fn processScrolledMotionEvent(&mut self, mut x: i32, mut y: i32, mut deviceId: i32, mut modifiers: i32) {
        let mut point: Option<Rc<RefCell<CustomGPoint>>> = { let java_object = (jcall!(AllMotionRecognizer_pointCircularPool.with(Clone::clone)).getNextInstance()).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<CustomGPoint>>>>().expect("ClassCastException").clone()) };
        jcall!(point).setX(x.clone());
        jcall!(point).setY(y.clone());
        jcall!(self.motionGestureRecognizer).processScrolledMotionEvent(point.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::graphics::GPoint::GPointDyn>>), deviceId.clone(), modifiers.clone());
    }
    

    pub fn getMotionGestureRecognizer(&mut self) -> Option<Rc<RefCell<MotionGestureRecognizer>>> {
        return self.motionGestureRecognizer.clone();
    }
}
impl std::ops::Deref for AllMotionRecognizer {
    type Target = MotionRecognizer;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for AllMotionRecognizer {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::input::motion::MotionRecognizer::MotionRecognizerDyn for AllMotionRecognizer {
    fn as_MotionRecognizer(&self) -> &crate::org::allbinary::input::motion::MotionRecognizer::MotionRecognizer { &self.base }
    fn as_MotionRecognizer_mut(&mut self) -> &mut crate::org::allbinary::input::motion::MotionRecognizer::MotionRecognizer { &mut self.base }
    fn processStartMotionEvent(&mut self, mut x: i32, mut y: i32, mut deviceId: i32, mut modifiers: i32) { Self::processStartMotionEvent(self, x, y, deviceId, modifiers) }
    fn processEndMotionEvent(&mut self, mut x: i32, mut y: i32, mut deviceId: i32, mut modifiers: i32) { Self::processEndMotionEvent(self, x, y, deviceId, modifiers) }
    fn processDraggedMotionEvent(&mut self, mut x: i32, mut y: i32, mut deviceId: i32, mut modifiers: i32) { Self::processDraggedMotionEvent(self, x, y, deviceId, modifiers) }
}


