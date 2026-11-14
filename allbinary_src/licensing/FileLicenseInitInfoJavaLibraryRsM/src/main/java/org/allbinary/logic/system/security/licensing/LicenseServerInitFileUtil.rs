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
use crate::java::io::InputStream::InputStream;
use crate::java::io::OutputStream::OutputStream;
use crate::org::allbinary::business::init::LicenseInitInfoUtil::LicenseInitInfoUtil;
use crate::org::allbinary::data::resource::ResourceUtil::ResourceUtil;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::io::FileStreamFactory::FileStreamFactory;
use crate::org::allbinary::logic::io::StreamUtil::StreamUtil;
use crate::org::allbinary::logic::io::file::FileFactory::FileFactory;
use crate::org::allbinary::logic::io::file::SimpleFileUtil::SimpleFileUtil;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::java::lang::Exception::*;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct LicenseServerInitFileUtil {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub commonStrings: Option<Rc<RefCell<CommonStrings>>>,
    pub NULL_OUTPUT_STREAM: Option<Rc<RefCell<dyn crate::java::io::OutputStream::OutputStreamDyn>>>,
}


impl LicenseServerInitFileUtil {

    pub fn toString(&mut self) -> String {
        String::from(std::any::type_name::<Self>())
    }

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
            let mut filePath: String = { let java_field = jcall!(LicenseInitInfoUtil::getInstance()).INITFILENAME.clone(); java_field }.clone();
            jcall!(LicenseInitInfoUtil::getInstance()).setFilePath({ let java_field = jcall!(StringUtil::getInstance()).EMPTY_STRING.clone(); java_field }.clone());
            if jcall!(FileFactory::getInstance()).isFile(filePath.clone()) {
                jcall!(self.logUtil).putF(String::from("Using Existing License File"), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(self.commonStrings).INIT.clone(); java_field }.clone());
            } else {
                self.write();
            }
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            jcall!(self.logUtil).put({ let java_field = jcall!(self.commonStrings).EXCEPTION.clone(); java_field }.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(self.commonStrings).INIT.clone(); java_field }.clone(), crate::java_into_object(e.clone()));
            }
        }
    }
    

    pub fn write(&mut self) {
        let mut fileOutputStream: Option<Rc<RefCell<dyn crate::java::io::OutputStream::OutputStreamDyn>>> = self.NULL_OUTPUT_STREAM.clone();
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut resourceUtil: Option<Rc<RefCell<ResourceUtil>>> = ResourceUtil::getInstance();
            let mut filePath: String = { let java_field = jcall!(LicenseInitInfoUtil::getInstance()).INITFILENAME.clone(); java_field }.clone();
            let mut inputStream: Option<Rc<RefCell<dyn crate::java::io::InputStream::InputStreamDyn>>> = jcall!(resourceUtil).getResourceAsStream(filePath.clone());
            jcall!(self.logUtil).putF(String::from("Writing Default License File"), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(self.commonStrings).INIT.clone(); java_field }.clone());
            let mut fileStreamFactory: Option<Rc<RefCell<FileStreamFactory>>> = FileStreamFactory::getInstance();
            fileOutputStream = { let java_value = (jcall!(fileStreamFactory).getFileOutputStreamInstance({ let java_field = jcall!(StringUtil::getInstance()).EMPTY_STRING.clone(); java_field }.clone(), filePath.clone())).clone(); java_value }.map(|reference| reference as Rc<RefCell<dyn crate::java::io::OutputStream::OutputStreamDyn>>);
            let mut b: i32 = 0;
            let mut index: i32 = 0;
            while true {
                b = { let java_value = (jcall!(inputStream).as_InputStream_mut().read()).clone(); java_value };
                if b == -1 {
                    break;
                }
                jcall!(fileOutputStream).as_OutputStream_mut().write(b.clone());
                index += 1;
            }
            jcall!(self.logUtil).putF(format!("{}{}", String::from("Wrote Bytes: "), index), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(self.commonStrings).INIT.clone(); java_field }.clone());
            jcall!(fileOutputStream).as_OutputStream_mut().flush();
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            jcall!(self.logUtil).put({ let java_field = jcall!(self.commonStrings).EXCEPTION.clone(); java_field }.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(self.commonStrings).INIT.clone(); java_field }.clone(), crate::java_into_object(e.clone()));
            }
        }
        { // finally
            jcall!(StreamUtil::getInstance()).close(fileOutputStream.clone().map(|_java_reference| -> Rc<RefCell<dyn crate::java::io::Closeable::Closeable>> { panic!("ClassCastException: the Rust trait of OutputStream does not have Closeable as a base trait") }));
        }
    }
}

