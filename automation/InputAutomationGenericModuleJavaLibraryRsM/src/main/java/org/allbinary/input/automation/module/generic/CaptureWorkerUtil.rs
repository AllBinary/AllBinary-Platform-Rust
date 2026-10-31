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
// package org.allbinary.input.automation.module.generic
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
use crate::org::allbinary::input::automation::actions::script::condition::ProfileActionScriptConditionInterface::ProfileActionScriptConditionInterface;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::string::CommonStrings::CommonStrings;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct CaptureWorkerUtil {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
}


impl CaptureWorkerUtil {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            logUtil: None,
        };
        this.logUtil = LogUtil::getInstance();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    // throws Exception
    pub fn processProfileActionConditions(mut vector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>, mut frame: Option<i64>) {
        let mut logUtil: Option<Rc<RefCell<LogUtil>>> = LogUtil::getInstance();
        let mut commonStrings: Option<Rc<RefCell<CommonStrings>>> = CommonStrings::getInstance();
        jcall!(logUtil).putF(jcall!(commonStrings).START.clone(), crate::java_into_object(String::from("CaptureWorkerUtil")), String::from("processProfileActionConditions"));
        let mut size: i32 = jcall!(vector).size();
        for index in (0..size) {
            let mut profileActionConditionInterface: Option<Rc<RefCell<dyn ProfileActionScriptConditionInterface>>> = { let java_object = jcall!(vector).get(index.clone()); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn ProfileActionScriptConditionInterface>>>>().expect("ClassCastException").clone()) };
            if jcall!(profileActionConditionInterface).shouldProcess(frame.clone()) {
                jcall!(profileActionConditionInterface).process(frame.clone());
            } else {
            }
        }
    }
}

