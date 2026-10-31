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
use crate::org::allbinary::logic::communication::log::PreLogUtil::PreLogUtil;
use crate::org::allbinary::string::CommonStrings::CommonStrings;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct CompleteMotionGestureInputToGameMotionGestureInput {
    pub commonStrings: Option<Rc<RefCell<CommonStrings>>>,
}

thread_local! { pub static CompleteMotionGestureInputToGameMotionGestureInput_instance: Option<Rc<RefCell<CompleteMotionGestureInputToGameMotionGestureInput>>> = CompleteMotionGestureInputToGameMotionGestureInput::new(); }

impl CompleteMotionGestureInputToGameMotionGestureInput {

    pub fn new_struct() -> Self {
        let mut this = Self {
            commonStrings: None,
        };
        this.commonStrings = CommonStrings::getInstance();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<CompleteMotionGestureInputToGameMotionGestureInput>>> {
        return CompleteMotionGestureInputToGameMotionGestureInput_instance.with(Clone::clone).clone();
    }
    

    pub fn init(&mut self) {
        PreLogUtil::put(String::from("No Touch Button = No Compound Motion Gestures"), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).INIT.clone());
    }
}

