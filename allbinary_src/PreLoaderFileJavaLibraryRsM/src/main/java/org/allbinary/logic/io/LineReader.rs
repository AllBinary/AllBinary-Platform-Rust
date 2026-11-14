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
// package org.allbinary.logic.io
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::java::io::DataInputStream::DataInputStream;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::string::CommonSeps::CommonSeps;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::java::lang::Exception::*;
use super::AbFileInputStream::AbFileInputStream;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct LineReader {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub commonStrings: Option<Rc<RefCell<CommonStrings>>>,
    pub fileName: String,
    pub string: String,
    pub index: i32,
    pub idFile: Option<Rc<RefCell<dyn crate::org::allbinary::logic::io::AbFileInputStream::AbFileInputStreamDyn>>>,
    pub idData: Option<Rc<RefCell<DataInputStream>>>,
}


impl LineReader {

    pub fn toString(&mut self) -> String {
        String::from(std::any::type_name::<Self>())
    }
    

    pub fn new_struct(mut fileName: String) -> Self {
        let mut this = Self {
            logUtil: None,
            commonStrings: None,
            fileName: String::new(),
            string: String::new(),
            index: 0,
            idFile: None,
            idData: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.commonStrings = CommonStrings::getInstance();
        this.index = 0;
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut bytes: Vec<i8> = vec![0; (100000) as usize];
            this.fileName = fileName.clone();
            this.idFile = AbFileInputStream::new(fileName.clone()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::io::AbFileInputStream::AbFileInputStreamDyn>>);
            jcall!(this.idFile).as_AbFileInputStream_mut().read1(bytes.clone());
            this.string = String::from_utf8(bytes.clone().iter().map(|value| *value as u8).collect()).unwrap();
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            jcall!(this.logUtil).put(format!("{}{}", String::from("File: "), fileName), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("LineReader"), crate::java_into_object(e.clone()));
            {
            }
            }
        }
        this
    }

    pub fn new(mut fileName: String) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(fileName))))
    }
    

    pub fn hasNext(&mut self) -> bool {
        let java_try_result = match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut nextIndex: i32 = { let java_from = (self.index).max(0) as usize; self.string.get(java_from..).and_then(|java_rest| java_rest.find(&{ let java_field = jcall!(CommonSeps::getInstance()).NEW_LINE.clone(); java_field })).map(|index| (index + java_from) as i32).unwrap_or(-1) };
            if nextIndex == -1 {
                return Some(false);
            }
            return Some(true);
        })) {
            Ok(java_return) => java_return,
            Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            jcall!(self.logUtil).put({ let java_field = jcall!(self.commonStrings).EXCEPTION.clone(); java_field }.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("hasNext"), crate::java_into_object(e.clone()));
            {
            }
            return false;
            None
            }
        };if let Some(java_return) = java_try_result { return java_return; }unreachable!("try statement")
    }
    

    // throws Exception
    pub fn next(&mut self) -> String {
        let java_try_result = match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut nextIndex: i32 = { let java_from = (self.index).max(0) as usize; self.string.get(java_from..).and_then(|java_rest| java_rest.find(&{ let java_field = jcall!(CommonSeps::getInstance()).NEW_LINE.clone(); java_field })).map(|index| (index + java_from) as i32).unwrap_or(-1) };
            if nextIndex == -1 {
                panic!("{:?}", format!("{}: {:?}", "Exception", (String::from("next() should have been called first"))));
            }
            let mut temp: String = self.string[(self.index) as usize..(nextIndex) as usize].to_string();
            self.index = nextIndex + 1;
            return Some(temp.clone());
        })) {
            Ok(java_return) => java_return,
            Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            jcall!(self.logUtil).put({ let java_field = jcall!(self.commonStrings).EXCEPTION.clone(); java_field }.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("next"), crate::java_into_object(e.clone()));
            {
            }
            panic!("{:?}", e);
            None
            }
        };if let Some(java_return) = java_try_result { return java_return; }unreachable!("try statement")
    }
}

