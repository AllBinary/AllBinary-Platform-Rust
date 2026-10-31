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
// use crate::java::io::DataOutputStream::DataOutputStream;
// use crate::java::io::FileOutputStream::FileOutputStream;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::io::file::AbFileSystem::AbFileSystem;
use crate::org::allbinary::string::CommonStrings::CommonStrings;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct SmallInsert {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub commonStrings: Option<Rc<RefCell<CommonStrings>>>,
    pub fileName: String,
    pub string: String,
}


impl SmallInsert {
    

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
        this.string = jcall!(AbFileSystem::getInstance()).readAsString(fileName.clone());
        this
    }

    pub fn new(mut fileName: String) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(fileName))))
    }
    

    pub fn atBeginning(&mut self, mut text: String) -> bool {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut idFile: Option<Rc<RefCell<FileOutputStream>>> = FileOutputStream::new(self.fileName.clone());
            let mut idOutData: Option<Rc<RefCell<DataOutputStream>>> = DataOutputStream::new(idFile.clone());
            jcall!(idOutData).writeBytes(format!("{}{}", text, self.string));
            return true;
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).IDLOGGING.clone()) {
                jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("insertAtBeginning"), crate::java_into_object(e.clone()));
            }
            return false;
            }
        }
    }
    

    pub fn after(&mut self, mut text: String, mut after: String) -> bool {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut index: i32 = self.string.find(&after.clone()).map(|index| index as i32).unwrap_or(-1);
            if index < 0 {
                println!("{}", format!("{}{}", format!("{}{}", format!("{}{}", String::from("No such start: "), after), String::from(" in: ")), self.string));
                return false;
            }
            index = index + (after.chars().count() as i32);
            let mut start: String = self.string[(0) as usize..(index) as usize].to_string();
            let mut end: String = self.string[(index + 1) as usize..((self.string.chars().count() as i32)) as usize].to_string();
            let mut idFile: Option<Rc<RefCell<FileOutputStream>>> = FileOutputStream::new(self.fileName.clone());
            let mut idOutData: Option<Rc<RefCell<DataOutputStream>>> = DataOutputStream::new(idFile.clone());
            jcall!(idOutData).writeBytes(format!("{}{}", format!("{}{}", start, text), end));
            jcall!(idOutData).close();
            jcall!(idFile).close();
            return true;
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).IDLOGGING.clone()) {
                jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("after"), crate::java_into_object(e.clone()));
            }
            return false;
            }
        }
    }
    

    pub fn atEnd(&mut self, mut text: String) -> bool {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut idFile: Option<Rc<RefCell<FileOutputStream>>> = FileOutputStream::new(self.fileName.clone());
            let mut idOutData: Option<Rc<RefCell<DataOutputStream>>> = DataOutputStream::new(idFile.clone());
            jcall!(idOutData).writeBytes(format!("{}{}", self.string, text));
            return true;
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).IDLOGGING.clone()) {
                jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("insertAtEnd"), crate::java_into_object(e.clone()));
            }
            return false;
            }
        }
    }
}

