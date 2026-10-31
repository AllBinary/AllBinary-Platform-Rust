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
use crate::jsinterop::annotations::JsType;
use crate::org::allbinary::string::CommonSeps::CommonSeps;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use crate::playn::core::PlayN::PlayN;

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
        let mut className: String = PreLogUtil::getClassName(anyType.clone());
        if false {
            className = jcall!(CommonStrings::getInstance()).EMPTY.clone();
        }
        className = jcall!(StringMaker::new()).append(className.clone()).append(jcall!(CommonSeps::getInstance()).FORWARD_SLASH.clone()).append(jcall!(StringUtil::getInstance()).toString(crate::java_into_object(anyType.clone()))).toString();
        let mut message: String = jcall!(LogFormatUtil::LogFormatUtil::getInstance()).getS(className.clone(), functionName.clone(), specialMessage.clone());
        jcall!(jcall!(PlayN::getInstance()).log()).debug(format!("{}{}", PreLogUtil_LOG_SUCCESS.with(Clone::clone), message));
    }
    

    pub fn putOE(mut specialMessage: String, mut anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>>, mut functionName: String, mut exception: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) {
        let mut className: String = PreLogUtil::getClassName(anyType.clone());
        if false {
            className = jcall!(CommonStrings::getInstance()).EMPTY.clone();
        }
        className = jcall!(StringMaker::new()).append(className.clone()).append(jcall!(CommonSeps::getInstance()).FORWARD_SLASH.clone()).append(jcall!(StringUtil::getInstance()).toString(crate::java_into_object(anyType.clone()))).toString();
        let mut message: String = jcall!(LogFormatUtil::LogFormatUtil::getInstance()).get(className.clone(), functionName.clone(), specialMessage.clone(), crate::java_into_object(exception.clone()));
        jcall!(jcall!(PlayN::getInstance()).log()).error(format!("{}{}", PreLogUtil_LOG_SUCCESS.with(Clone::clone), message), { let reference = exception.as_ref().expect("null reference").borrow(); reference.as_ref().downcast_ref::<String>().cloned().unwrap_or_else(|| format!("{:?}", reference.as_ref().type_id())) });
    }
    

    pub fn putS(mut specialMessage: String, mut className: String, mut functionName: String) {
        let mut message: String = jcall!(LogFormatUtil::LogFormatUtil::getInstance()).getS(className.clone(), functionName.clone(), specialMessage.clone());
        jcall!(jcall!(PlayN::getInstance()).log()).debug(format!("{}{}", PreLogUtil_LOG_SUCCESS.with(Clone::clone), message));
    }
    

    pub fn putSE(mut specialMessage: String, mut className: String, mut functionName: String, mut exception: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) {
        let mut message: String = jcall!(LogFormatUtil::LogFormatUtil::getInstance()).get(className.clone(), functionName.clone(), specialMessage.clone(), crate::java_into_object(exception.clone()));
        jcall!(jcall!(PlayN::getInstance()).log()).error(format!("{}{}", PreLogUtil_LOG_SUCCESS.with(Clone::clone), message), { let reference = exception.as_ref().expect("null reference").borrow(); reference.as_ref().downcast_ref::<String>().cloned().unwrap_or_else(|| format!("{:?}", reference.as_ref().type_id())) });
    }
    

    pub fn getClassName(mut anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) -> String {
        unimplemented!("abstract method getClassName")
    }
}

