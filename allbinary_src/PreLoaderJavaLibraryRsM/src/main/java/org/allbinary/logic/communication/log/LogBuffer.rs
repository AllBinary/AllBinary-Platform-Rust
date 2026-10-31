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
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
use crate::org::allbinary::logic::StdUtil::StdUtil;
use super::LogUtil::LogUtil;
use super::Log::Log;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct LogBuffer {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub logEntryVector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>,
}


impl LogBuffer {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            logUtil: None,
            logEntryVector: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.logEntryVector = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn add(&mut self, mut log: Option<Rc<RefCell<Log>>>) {
        jcall!(self.logEntryVector).add(crate::java_into_object(log.clone()));
    }
    

    pub fn logAll(&mut self) {
        let mut size: i32 = jcall!(self.logEntryVector).size();
        for index in (0..size) {
            let mut log: Option<Rc<RefCell<Log>>> = { let java_object = (jcall!(self.logEntryVector).get(index.clone())).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<Log>>>>().expect("ClassCastException").clone()) };
            jcall!(self.logUtil).putL(log.clone());
        }
    }
}

