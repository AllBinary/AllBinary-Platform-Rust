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
// package org.allbinary.logic.system.security.licensing
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::java::io::InputStream::InputStream;
// use crate::java::io::OutputStream::OutputStream;
use crate::org::allbinary::business::init::LicenseInitInfoUtil::LicenseInitInfoUtil;
use crate::org::allbinary::data::resource::ResourceUtil::ResourceUtil;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::io::FileStreamFactory::FileStreamFactory;
use crate::org::allbinary::logic::io::StreamUtil::StreamUtil;
use crate::org::allbinary::logic::io::file::FileFactory::FileFactory;
use crate::org::allbinary::logic::io::file::SimpleFileUtil::SimpleFileUtil;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use crate::org::allbinary::string::CommonStrings::CommonStrings;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct LicenseServerInitFileUtil {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub commonStrings: Option<Rc<RefCell<CommonStrings>>>,
    pub NULL_OUTPUT_STREAM: Option<Rc<RefCell<OutputStream>>>,
}


impl LicenseServerInitFileUtil {

    pub fn new_struct() -> Self {
        let mut this = Self {
            logUtil: None,
            commonStrings: None,
            NULL_OUTPUT_STREAM: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.commonStrings = CommonStrings::getInstance();
        this.NULL_OUTPUT_STREAM = SimpleFileUtil::nullOutputStream();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn init(&mut self) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut filePath: String = jcall!(LicenseInitInfoUtil::getInstance()).INITFILENAME.clone();
            jcall!(LicenseInitInfoUtil::getInstance()).setFilePath(jcall!(StringUtil::getInstance()).EMPTY_STRING.clone());
            if jcall!(FileFactory::getInstance()).isFile(filePath.clone()) {
                jcall!(self.logUtil).putF(String::from("Using Existing License File"), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).INIT.clone());
            } else {
                self.write();
            }
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).INIT.clone(), crate::java_into_object(e.clone()));
            }
        }
    }
    

    pub fn write(&mut self) {
        let mut fileOutputStream: Option<Rc<RefCell<OutputStream>>> = self.NULL_OUTPUT_STREAM.clone();
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut resourceUtil: Option<Rc<RefCell<ResourceUtil>>> = ResourceUtil::getInstance();
            let mut filePath: String = jcall!(LicenseInitInfoUtil::getInstance()).INITFILENAME.clone();
            let mut inputStream: Option<Rc<RefCell<InputStream>>> = jcall!(resourceUtil).getResourceAsStream(filePath.clone());
            jcall!(self.logUtil).putF(String::from("Writing Default License File"), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).INIT.clone());
            let mut fileStreamFactory: Option<Rc<RefCell<FileStreamFactory>>> = FileStreamFactory::getInstance();
            fileOutputStream = jcall!(fileStreamFactory).getFileOutputStreamInstance(jcall!(StringUtil::getInstance()).EMPTY_STRING.clone(), filePath.clone());
            let mut b: i32 = 0;
            let mut index: i32 = 0;
            while true {
                b = jcall!(inputStream).read();
                if b == -1 {
                    break;
                }
                jcall!(fileOutputStream).write(b.clone());
                index += 1;
            }
            jcall!(self.logUtil).putF(format!("{}{}", String::from("Wrote Bytes: "), index), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).INIT.clone());
            jcall!(fileOutputStream).flush();
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).INIT.clone(), crate::java_into_object(e.clone()));
            }
        }
        { // finally
            jcall!(StreamUtil::getInstance()).close(fileOutputStream.clone());
        }
    }
}

