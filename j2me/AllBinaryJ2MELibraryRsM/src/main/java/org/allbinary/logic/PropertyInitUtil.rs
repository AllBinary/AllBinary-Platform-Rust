// 
// AllBinary Open License Version 1 
// Copyright (c) 2026 AllBinary 
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
// package org.allbinary.logic
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::string::CommonStrings::CommonStrings;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct PropertyInitUtil {
}

thread_local! { pub static PropertyInitUtil_instance: Option<Rc<RefCell<PropertyInitUtil>>> = PropertyInitUtil::new(); }

impl PropertyInitUtil {

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance(mut value: i32) -> Option<Rc<RefCell<PropertyInitUtil>>> {
        let mut commonStrings: Option<Rc<RefCell<CommonStrings>>> = CommonStrings::getInstance();
        let mut logUtil: Option<Rc<RefCell<LogUtil>>> = LogUtil::getInstance();
        jcall!(logUtil).putF(jcall!(StringMaker::new()).append(String::from("PropertyInitUtil: ")).appendint(value.clone()).toString(), crate::java_into_object(logUtil.clone()), jcall!(commonStrings).CONSTRUCTOR.clone());
        return PropertyInitUtil_instance.with(Clone::clone).clone();
    }
}

