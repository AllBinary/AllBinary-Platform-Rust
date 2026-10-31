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
// package org.allbinary.logic.communication.log
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::logic::NullUtil::NullUtil;
use crate::org::allbinary::string::CommonStrings::CommonStrings;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

//ActualPlatform
pub struct PreLogUtil {
}

thread_local! { pub static PreLogUtil_LOG_SUCCESS: String = String::from("org.allbinary: "); }

impl PreLogUtil {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn put(mut specialMessage: String, mut anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>>, mut functionName: String) {
        PreLogUtil::putOE(specialMessage.clone(), anyType.clone(), functionName.clone(), jcall!(NullUtil::getInstance()).NULL_OBJECT.clone());
    }
    

    pub fn putOE(mut specialMessage: String, mut anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>>, mut functionName: String, mut exception: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) {
        let mut className: String = jcall!(CommonStrings::getInstance()).EMPTY.clone();
        if true {
            className = std::any::type_name_of_val(&anyType).to_string().clone();
        }
        let mut message: String = jcall!(LogFormatUtil::LogFormatUtil::getInstance()).get(className.clone(), functionName.clone(), specialMessage.clone(), crate::java_into_object(exception.clone()));
        print!("{}", PreLogUtil_LOG_SUCCESS.with(Clone::clone).clone());
        println!("{}", message.clone());
    }
    

    pub fn putS(mut specialMessage: String, mut className: String, mut functionName: String) {
        PreLogUtil::putSE(specialMessage.clone(), className.clone(), functionName.clone(), jcall!(NullUtil::getInstance()).NULL_OBJECT.clone());
    }
    

    pub fn putSE(mut specialMessage: String, mut className: String, mut functionName: String, mut exception: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) {
        let mut message: String = jcall!(LogFormatUtil::LogFormatUtil::getInstance()).get(className.clone(), functionName.clone(), specialMessage.clone(), crate::java_into_object(exception.clone()));
        print!("{}", PreLogUtil_LOG_SUCCESS.with(Clone::clone).clone());
        println!("{}", message.clone());
    }
}

