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
use crate::java::io::DataOutputStream::DataOutputStream;
use crate::java::io::FileOutputStream::FileOutputStream;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::io::file::AbFileSystem::AbFileSystem;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::java::lang::Exception::*;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct SmallDelete {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub commonStrings: Option<Rc<RefCell<CommonStrings>>>,
    pub fileName: String,
    pub string: String,
}


impl SmallDelete {

    pub fn toString(&mut self) -> String {
        String::from(std::any::type_name::<Self>())
    }
    

    pub fn new_struct(mut fileName: String) -> Self {
        let mut this = Self {
            logUtil: None,
            commonStrings: None,
            fileName: String::new(),
            string: String::new(),
        };
        this.logUtil = LogUtil::getInstance();
        this.commonStrings = CommonStrings::getInstance();
        this.fileName = fileName.clone();
        this.string = { let java_value = (jcall!(AbFileSystem::getInstance()).readAsString(fileName.clone())).clone(); java_value };
        this
    }

    pub fn new(mut fileName: String) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(fileName))))
    }
    

    pub fn deleteAtStart(&mut self, mut start: String, mut end: String) -> bool {
        let java_try_result = match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut beginIndex: i32 = 0;
            let mut endIndex: i32 = 0;
            beginIndex = { let java_value = (self.string.find(&start.clone()).map(|index| index as i32).unwrap_or(-1)).clone(); java_value };
            endIndex = { let java_value = (self.string.find(&end.clone()).map(|index| index as i32).unwrap_or(-1)).clone(); java_value };
            if endIndex > beginIndex && beginIndex >= 0 && endIndex >= 0 && beginIndex < 10 && endIndex < 525 {
                let mut text: String = self.string[(endIndex) as usize..((self.string.chars().count() as i32)) as usize].to_string();
                let mut idFile: Option<Rc<RefCell<dyn crate::java::io::FileOutputStream::FileOutputStreamDyn>>> = FileOutputStream::new1(self.fileName.clone()).map(|reference| reference as Rc<RefCell<dyn crate::java::io::FileOutputStream::FileOutputStreamDyn>>);
                let mut idOutData: Option<Rc<RefCell<DataOutputStream>>> = DataOutputStream::new(idFile.clone());
                jcall!(idOutData).writeBytes(text.clone());
                return Some(true);
            }
            return Some(false);
        })) {
            Ok(java_return) => java_return,
            Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            if jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypes.clone(); java_field }).LOGGING.clone(); java_field }).contains({ let java_field = jcall!(jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypeFactory.clone(); java_field }).getInstance()).IDLOGGING.clone(); java_field }.clone()) {
                jcall!(self.logUtil).put({ let java_field = jcall!(self.commonStrings).EXCEPTION.clone(); java_field }.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("deleteAtStart"), crate::java_into_object(e.clone()));
            }
            return false;
            None
            }
        };if let Some(java_return) = java_try_result { return java_return; }unreachable!("try statement")
    }
    

    pub fn deleteBetween(&mut self, mut start: String, mut end: String, mut newText: String) -> bool {
        let java_try_result = match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut beginIndex: i32 = 0;
            let mut endIndex: i32 = 0;
            beginIndex = { let java_value = (self.string.find(&start.clone()).map(|index| index as i32).unwrap_or(-1)).clone(); java_value };
            endIndex = { let java_value = (self.string.find(&end.clone()).map(|index| index as i32).unwrap_or(-1)).clone(); java_value };
            if endIndex > beginIndex && beginIndex >= 0 && endIndex >= 0 && beginIndex < 50 && endIndex < 425 {
                let mut newStart: String = self.string[(0) as usize..(beginIndex) as usize].to_string();
                let mut text: String = self.string[(endIndex + (end.chars().count() as i32)) as usize..((self.string.chars().count() as i32)) as usize].to_string();
                let mut idFile: Option<Rc<RefCell<dyn crate::java::io::FileOutputStream::FileOutputStreamDyn>>> = FileOutputStream::new1(self.fileName.clone()).map(|reference| reference as Rc<RefCell<dyn crate::java::io::FileOutputStream::FileOutputStreamDyn>>);
                let mut idOutData: Option<Rc<RefCell<DataOutputStream>>> = DataOutputStream::new(idFile.clone());
                jcall!(idOutData).writeBytes(format!("{}{}", format!("{}{}", newStart, newText), text));
                return Some(true);
            }
            return Some(false);
        })) {
            Ok(java_return) => java_return,
            Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            if jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypes.clone(); java_field }).LOGGING.clone(); java_field }).contains({ let java_field = jcall!(jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypeFactory.clone(); java_field }).getInstance()).IDLOGGING.clone(); java_field }.clone()) {
                jcall!(self.logUtil).put({ let java_field = jcall!(self.commonStrings).EXCEPTION.clone(); java_field }.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("deleteAtStart"), crate::java_into_object(e.clone()));
            }
            return false;
            None
            }
        };if let Some(java_return) = java_try_result { return java_return; }unreachable!("try statement")
    }
}

