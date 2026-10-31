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
// package org.allbinary.logic.communication.http
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use super::AcceptableResponseUtil::AcceptableResponseUtil;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct AcceptableResponse {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub commonStrings: Option<Rc<RefCell<CommonStrings>>>,
}


impl AcceptableResponse {
    

    pub fn new_struct(mut candidate: String, mut tagName: String, mut index: i32) -> Self {
        let mut this = Self {
            logUtil: None,
            commonStrings: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.commonStrings = CommonStrings::getInstance();
        jcall!(this.logUtil).putF(tagName.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(this.commonStrings).CONSTRUCTOR.clone());
        let mut acceptableResponseUtil: Option<Rc<RefCell<AcceptableResponseUtil>>> = AcceptableResponseUtil::getInstance();
        jcall!(jcall!(acceptableResponseUtil).candidateVector).addAt(index.clone(), crate::java_into_object(candidate.clone()));
        jcall!(jcall!(acceptableResponseUtil).tagNameVector).addAt(index.clone(), crate::java_into_object(tagName.clone()));
        this
    }

    pub fn new(mut candidate: String, mut tagName: String, mut index: i32) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(candidate, tagName, index))))
    }
}

