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
use crate::org::allbinary::graphics::GPoint::GPoint;
use crate::org::allbinary::graphics::PointFactory::PointFactory;
use crate::org::allbinary::input::motion::gesture::MotionGestureInput::MotionGestureInput;
use crate::org::allbinary::input::motion::gesture::TouchMotionGestureFactory::TouchMotionGestureFactory;
use crate::org::allbinary::TsUtil::TsUtil;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObject;
use crate::org::allbinary::string::CommonLabels::CommonLabels;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

fn java_string_hash(text: &str) -> i32 {
    text.chars().fold(0i32, |hash, character| hash.wrapping_mul(31).wrapping_add(character as i32))
}

pub struct MotionGestureEvent {
    pub base: AllBinaryEventObject,
    pub motionGesture: Option<Rc<RefCell<MotionGestureInput>>>,
    pub id: i32,
    pub previousPoint: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::GPoint::GPointDyn>>>,
    pub currentPoint: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::GPoint::GPointDyn>>>,
    pub commonLabels: Option<Rc<RefCell<CommonLabels>>>,
}

thread_local! { pub static MotionGestureEvent_PREVIOUS: String = String::from(" Previous: "); }
thread_local! { pub static MotionGestureEvent_HASHCODE: String = String::from(" hashcode: "); }

impl MotionGestureEvent {
    

    pub fn new_struct(mut source: Option<Rc<RefCell<Box<dyn std::any::Any>>>>, mut id: i32, mut motionGesture: Option<Rc<RefCell<MotionGestureInput>>>) -> Self {
        let mut this = Self {
            base: AllBinaryEventObject::new_struct(source.clone()),
            motionGesture: None,
            id: 0,
            previousPoint: None,
            currentPoint: None,
            commonLabels: None,
        };
        this.motionGesture = jcall!(TouchMotionGestureFactory::getInstance()).NO_MOTION.clone();
        this.previousPoint = jcall!(PointFactory::getInstance()).ZERO_ZERO.clone();
        this.currentPoint = jcall!(PointFactory::getInstance()).ZERO_ZERO.clone();
        this.commonLabels = CommonLabels::getInstance();
        // super(...) is in the struct literal above
        this.id = id.clone();
        this.setMotionGesture(motionGesture.clone());
        this
    }

    pub fn new(mut source: Option<Rc<RefCell<Box<dyn std::any::Any>>>>, mut id: i32, mut motionGesture: Option<Rc<RefCell<MotionGestureInput>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(source, id, motionGesture))))
    }
    

    pub fn getMotionGesture(&mut self) -> Option<Rc<RefCell<MotionGestureInput>>> {
        return self.motionGesture.clone();
    }
    

    pub fn setMotionGesture(&mut self, mut motionGesture: Option<Rc<RefCell<MotionGestureInput>>>) {
        self.motionGesture = motionGesture.clone();
    }
    

    pub fn getPreviousPoint(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::graphics::GPoint::GPointDyn>>> {
        return self.previousPoint.clone();
    }
    

    pub fn setPreviousPoint(&mut self, mut previousPoint: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::GPoint::GPointDyn>>>) {
        self.previousPoint = previousPoint.clone();
    }
    

    pub fn getCurrentPoint(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::graphics::GPoint::GPointDyn>>> {
        return self.currentPoint.clone();
    }
    

    pub fn setCurrentPoint(&mut self, mut currentPoint: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::GPoint::GPointDyn>>>) {
        self.currentPoint = currentPoint.clone();
    }
    

    pub fn toString(&mut self) -> String {
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        let mut stringUtil: Option<Rc<RefCell<StringUtil>>> = StringUtil::getInstance();
        jcall!(stringBuffer).append(jcall!(stringUtil).toString(crate::java_into_object(self.motionGesture.clone())));
        jcall!(stringBuffer).append(jcall!(self.commonLabels).CURRENT.clone());
        jcall!(stringBuffer).append(jcall!(stringUtil).toString(crate::java_into_object(self.currentPoint.clone())));
        jcall!(stringBuffer).append(MotionGestureEvent_PREVIOUS.with(Clone::clone).clone());
        jcall!(stringBuffer).append(jcall!(stringUtil).toString(crate::java_into_object(self.previousPoint.clone())));
        jcall!(stringBuffer).append(MotionGestureEvent_HASHCODE.with(Clone::clone).clone());
        jcall!(stringBuffer).appendint(jcall!(TsUtil::getInstance()).hashCode(None /* Java this: a &mut self method has no Rc of itself */));
        return jcall!(stringBuffer).toString();
    }
    

    pub fn getId(&mut self) -> i32 {
        return self.id.clone();
    }
}
impl std::ops::Deref for MotionGestureEvent {
    type Target = AllBinaryEventObject;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for MotionGestureEvent {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObjectDyn for MotionGestureEvent {
    fn as_AllBinaryEventObject(&self) -> &crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObject { &self.base }
    fn as_AllBinaryEventObject_mut(&mut self) -> &mut crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObject { &mut self.base }
}


