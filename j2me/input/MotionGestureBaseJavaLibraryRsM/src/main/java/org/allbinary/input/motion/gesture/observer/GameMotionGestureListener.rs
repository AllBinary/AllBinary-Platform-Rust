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
use crate::org::allbinary::input::motion::gesture::MotionGestureInput::MotionGestureInput;
use crate::org::allbinary::input::motion::gesture::TouchMotionGestureFactory::TouchMotionGestureFactory;
use crate::org::allbinary::logic::communication::log::ForcedLogUtil::ForcedLogUtil;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObject;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
use super::MotionGestureEventListener::MotionGestureEventListener;
use super::CompleteMotionGestureListenerInterface::CompleteMotionGestureListenerInterface;
use super::MotionGestureEvent::MotionGestureEvent;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct GameMotionGestureListener {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub commonStrings: Option<Rc<RefCell<CommonStrings>>>,
    pub motionGestureCollection: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>,
    pub touchGestureCollection: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>,
    pub signed: Option<Rc<RefCell<dyn CompleteMotionGestureListenerInterface>>>,
}

// impl MotionGestureEventListener for GameMotionGestureListener with the methods of the trait

impl GameMotionGestureListener {
    

    pub fn new_struct(mut signed: Option<Rc<RefCell<dyn CompleteMotionGestureListenerInterface>>>) -> Self {
        let mut this = Self {
            logUtil: None,
            commonStrings: None,
            motionGestureCollection: None,
            touchGestureCollection: None,
            signed: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.commonStrings = CommonStrings::getInstance();
        this.motionGestureCollection = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        this.touchGestureCollection = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        jcall!(this.logUtil).putF(jcall!(this.commonStrings).START.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(this.commonStrings).CONSTRUCTOR.clone());
        this.signed = signed.clone();
        this
    }

    pub fn new(mut signed: Option<Rc<RefCell<dyn CompleteMotionGestureListenerInterface>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(signed))))
    }
    

    pub fn onEvent(&mut self, mut eventObject: Option<Rc<RefCell<dyn crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObjectDyn>>>) {
        ForcedLogUtil::log(jcall!(self.commonStrings).NOT_IMPLEMENTED.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())));
    }
    

    pub fn onUpMotionGestureEvent(&mut self, mut ev: Option<Rc<RefCell<MotionGestureEvent>>>) {
        self.onMotionGestureEvent(ev.clone());
    }
    

    pub fn onDownMotionGestureEvent(&mut self, mut ev: Option<Rc<RefCell<MotionGestureEvent>>>) {
        self.onMotionGestureEvent(ev.clone());
    }
    

    pub fn onLeftMotionGestureEvent(&mut self, mut ev: Option<Rc<RefCell<MotionGestureEvent>>>) {
        self.onMotionGestureEvent(ev.clone());
    }
    

    pub fn onRightMotionGestureEvent(&mut self, mut ev: Option<Rc<RefCell<MotionGestureEvent>>>) {
        self.onMotionGestureEvent(ev.clone());
    }
    

    pub fn onDiagonalDownRightMotionGestureEvent(&mut self, mut ev: Option<Rc<RefCell<MotionGestureEvent>>>) {
        self.onMotionGestureEvent(ev.clone());
    }
    

    pub fn onDiagonalDownLeftMotionGestureEvent(&mut self, mut ev: Option<Rc<RefCell<MotionGestureEvent>>>) {
        self.onMotionGestureEvent(ev.clone());
    }
    

    pub fn onDiagonalUpRightMotionGestureEvent(&mut self, mut ev: Option<Rc<RefCell<MotionGestureEvent>>>) {
        self.onMotionGestureEvent(ev.clone());
    }
    

    pub fn onDiagonalUpLeftMotionGestureEvent(&mut self, mut ev: Option<Rc<RefCell<MotionGestureEvent>>>) {
        self.onMotionGestureEvent(ev.clone());
    }
    

    pub fn onPressedMotionGestureEvent(&mut self, mut ev: Option<Rc<RefCell<MotionGestureEvent>>>) {
        { let java_receiver = self.touchGestureCollection.clone(); let java_arguments = (crate::java_into_object(jcall!(ev).getMotionGesture()),); jcall!(java_receiver).add(java_arguments.0); };
    }
    

    pub fn released(&mut self, mut ev: Option<Rc<RefCell<MotionGestureEvent>>>) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            { let java_receiver = self.touchGestureCollection.clone(); let java_arguments = (crate::java_into_object(jcall!(ev).getMotionGesture()),); jcall!(java_receiver).add(java_arguments.0); };
            jcall!(self.signed).onMotionGestureCompleted(self.touchGestureCollection.clone());
            jcall!(self.touchGestureCollection).clear();
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
            jcall!(stringBuffer).append(jcall!(self.commonStrings).EXCEPTION_LABEL.clone());
            jcall!(stringBuffer).append(jcall!(StringUtil::getInstance()).toString(crate::java_into_object(jcall!(ev).getMotionGesture())));
            { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(stringBuffer).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("release"), crate::java_into_object(e.clone()),); jcall!(java_receiver).put(java_arguments.0, java_arguments.1, java_arguments.2, java_arguments.3); };
            }
        }
    }
    

    pub fn onScrolledMotionGestureEvent(&mut self, mut motionGestureEvent: Option<Rc<RefCell<MotionGestureEvent>>>) {
    }
    

    pub fn onMotionGestureEvent(&mut self, mut ev: Option<Rc<RefCell<MotionGestureEvent>>>) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut motionGestureInput: Option<Rc<RefCell<MotionGestureInput>>> = jcall!(ev).getMotionGesture();
            if match (&motionGestureInput, &jcall!(TouchMotionGestureFactory::getInstance()).PRESSED) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                self.onPressedMotionGestureEvent(ev.clone());
            } else if match (&motionGestureInput, &jcall!(TouchMotionGestureFactory::getInstance()).RELEASED) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                self.released(ev.clone());
            } else {
                jcall!(self.motionGestureCollection).add(crate::java_into_object(motionGestureInput.clone()));
                jcall!(self.signed).onMotionGestureCompleted(self.motionGestureCollection.clone());
                jcall!(self.motionGestureCollection).clear();
                jcall!(self.touchGestureCollection).clear();
            }
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
            jcall!(stringBuffer).append(jcall!(self.commonStrings).EXCEPTION_LABEL.clone());
            jcall!(stringBuffer).append(jcall!(StringUtil::getInstance()).toString(crate::java_into_object(jcall!(ev).getMotionGesture())));
            { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(stringBuffer).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("onMotionGestureEvent"), crate::java_into_object(e.clone()),); jcall!(java_receiver).put(java_arguments.0, java_arguments.1, java_arguments.2, java_arguments.3); };
            }
        }
    }
}

