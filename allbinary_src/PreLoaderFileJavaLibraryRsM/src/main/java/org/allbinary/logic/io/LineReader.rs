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
// use crate::java::io::DataInputStream::DataInputStream;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::string::CommonSeps::CommonSeps;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
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
            this.idFile = AbFileInputStream::new1(fileName.clone()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::io::AbFileInputStream::AbFileInputStreamDyn>>);
            jcall!(this.idFile).as_AbFileInputStream_mut().read(bytes.clone());
            this.string = String::from_utf8(bytes.clone().iter().map(|value| *value as u8).collect()).unwrap();
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

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
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut nextIndex: i32 = { let java_receiver = self.string.clone(); let java_arguments = (jcall!(CommonSeps::getInstance()).NEW_LINE.clone(), self.index.clone(),); let java_result = jcall!(java_receiver).indexOf(java_arguments.0, java_arguments.1); java_result };
            if nextIndex == -1 {
                return false;
            }
            return true;
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("hasNext"), crate::java_into_object(e.clone()));
            {
            }
            return false;
            }
        }
    }
    

    // throws Exception
    pub fn next(&mut self) -> String {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut nextIndex: i32 = { let java_receiver = self.string.clone(); let java_arguments = (jcall!(CommonSeps::getInstance()).NEW_LINE.clone(), self.index.clone(),); let java_result = jcall!(java_receiver).indexOf(java_arguments.0, java_arguments.1); java_result };
            if nextIndex == -1 {
                panic!("{:?}", format!("{}: {:?}", "Exception", (String::from("next() should have been called first"))));
            }
            let mut temp: String = self.string[(self.index) as usize..(nextIndex) as usize].to_string();
            self.index = nextIndex + 1;
            return temp.clone();
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("next"), crate::java_into_object(e.clone()));
            {
            }
            panic!("{:?}", e);
            }
        }
    }
}

