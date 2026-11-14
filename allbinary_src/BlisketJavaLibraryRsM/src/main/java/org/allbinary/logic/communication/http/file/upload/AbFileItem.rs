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
// package org.allbinary.logic.communication.http.file.upload
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::java::io::File::File;
use crate::java::io::IOException::IOException;
use crate::java::io::InputStream::InputStream;
use crate::java::io::OutputStream::OutputStream;
// use crate::java::io::UnsupportedEncodingException::UnsupportedEncodingException;
use crate::org::allbinary::logic::communication::log::ForcedLogUtil::ForcedLogUtil;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::io::file::AbFile::AbFile;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use crate::org::allbinary::logic::string::StringValidationUtil::StringValidationUtil;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::org::apache::commons::fileupload::FileItem::FileItem;
use crate::org::apache::commons::fileupload::FileItemHeaders::FileItemHeaders;
use crate::org::apache::commons::fileupload::FileItemUtil::FileItemUtil;
use crate::java::lang::Exception::*;
use crate::java::lang::RuntimeException::*;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct AbFileItem {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub commonStrings: Option<Rc<RefCell<CommonStrings>>>,
    pub name: String,
    pub fieldName: String,
    pub byteArray: Vec<i8>,
}

// impl FileItem for AbFileItem with the methods of the trait

impl AbFileItem {

    pub fn toString(&mut self) -> String {
        String::from(std::any::type_name::<Self>())
    }
    

    pub fn new_struct(mut name: String, mut fieldName: String, mut byteArray: Vec<i8>) -> Self {
        let mut this = Self {
            logUtil: None,
            commonStrings: None,
            name: String::new(),
            fieldName: String::new(),
            byteArray: Vec::new(),
        };
        this.logUtil = LogUtil::getInstance();
        this.commonStrings = CommonStrings::getInstance();
        this.name = name.clone();
        this.fieldName = fieldName.clone();
        this.byteArray = byteArray.clone();
        this
    }

    pub fn new(mut name: String, mut fieldName: String, mut byteArray: Vec<i8>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(name, fieldName, byteArray))))
    }
    

    // throws IOException
    pub fn getInputStream(&mut self) -> Option<Rc<RefCell<dyn crate::java::io::InputStream::InputStreamDyn>>> {
        panic!("{:?}", format!("{}: {:?}", "IOException", ({ let java_field = jcall!(self.commonStrings).NOT_IMPLEMENTED.clone(); java_field }.clone())));
    }
    

    pub fn getContentType(&mut self) -> String {
        return None;
    }
    

    pub fn getName(&mut self) -> String {
        return self.name.clone();
    }
    

    pub fn isInMemory(&mut self) -> bool {
        return true;
    }
    

    pub fn getSize(&mut self) -> i64 {
        return (((self.byteArray.len() as i32)) as i64).clone();
    }
    

    pub fn get(&mut self) -> Vec<i8> {
        return self.byteArray.clone();
    }
    

    // throws UnsupportedEncodingException
    pub fn getString1(&mut self, mut encoding: String) -> String {
        return String::new(self.byteArray.clone(), encoding.clone());
    }
    

    // Java overloaded method: Rust needs a different name
    pub fn getString(&mut self) -> String {
        let java_try_result = match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            return Some(jcall!(FileItemUtil::getInstance()).getString(self.byteArray.clone()));
        })) {
            Ok(java_return) => java_return,
            Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            if jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypes.clone(); java_field }).LOGGING.clone(); java_field }).contains({ let java_field = jcall!(jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypeFactory.clone(); java_field }).getInstance()).HTTPERROR.clone(); java_field }.clone()) {
                jcall!(self.logUtil).put({ let java_field = jcall!(self.commonStrings).EXCEPTION.clone(); java_field }.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("getString"), crate::java_into_object(e.clone()));
            }
            return { let java_field = jcall!(StringUtil::getInstance()).EMPTY_STRING.clone(); java_field }.clone();
            None
            }
        };if let Some(java_return) = java_try_result { return java_return; }unreachable!("try statement")
    }
    

    // throws Exception
    pub fn write(&mut self, mut file: Option<Rc<RefCell<File>>>) {
        panic!("{:?}", format!("{}: {:?}", "Exception", ({ let java_field = jcall!(self.commonStrings).NOT_IMPLEMENTED.clone(); java_field }.clone())));
    }
    

    // throws Exception
    // Java overloaded method: Rust needs a different name
    pub fn write_AbFile(&mut self, mut file: Option<Rc<RefCell<AbFile>>>) {
        panic!("{:?}", format!("{}: {:?}", "Exception", ({ let java_field = jcall!(self.commonStrings).NOT_IMPLEMENTED.clone(); java_field }.clone())));
    }
    

    pub fn delete(&mut self) {
        ForcedLogUtil::log({ let java_field = jcall!(self.commonStrings).NOT_IMPLEMENTED.clone(); java_field }.clone(), crate::java_into_object(String::from("delete()")));
    }
    

    pub fn getFieldName(&mut self) -> String {
        return self.fieldName.clone();
    }
    

    pub fn setFieldName(&mut self, mut name: String) {
        self.fieldName = name.clone();
    }
    

    pub fn isFormField(&mut self) -> bool {
        if jcall!(StringValidationUtil::getInstance()).isEmpty(self.fieldName.clone()) {
            return false;
        } else {
            return true;
        }
    }
    

    pub fn setFormField(&mut self, mut state: bool) {
    }
    

    // throws IOException
    pub fn getOutputStream(&mut self) -> Option<Rc<RefCell<dyn crate::java::io::OutputStream::OutputStreamDyn>>> {
        panic!("{:?}", format!("{}: {:?}", "IOException", ({ let java_field = jcall!(self.commonStrings).NOT_IMPLEMENTED.clone(); java_field }.clone())));
    }
    

    pub fn getHeaders(&mut self) -> Option<Rc<RefCell<FileItemHeaders>>> {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ({ let java_field = jcall!(self.commonStrings).NOT_IMPLEMENTED.clone(); java_field }.clone())));
    }
    

    pub fn setHeaders(&mut self, mut fih: Option<Rc<RefCell<FileItemHeaders>>>) {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ({ let java_field = jcall!(self.commonStrings).NOT_IMPLEMENTED.clone(); java_field }.clone())));
    }
}

