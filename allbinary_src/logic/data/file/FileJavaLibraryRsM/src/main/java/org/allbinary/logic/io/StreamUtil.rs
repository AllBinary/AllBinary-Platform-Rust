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
// use crate::java::io::ByteArrayOutputStream::ByteArrayOutputStream;
use crate::java::io::Closeable::Closeable;
use crate::java::io::InputStream::InputStream;
use crate::java::io::OutputStream::OutputStream;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::java::lang::Exception::*;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct StreamUtil {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
}

thread_local! { pub static StreamUtil_instance: Option<Rc<RefCell<StreamUtil>>> = StreamUtil::new(); }

impl StreamUtil {

    pub fn toString(&mut self) -> String {
        String::from(std::any::type_name::<Self>())
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<StreamUtil>>> {
        return StreamUtil_instance.with(Clone::clone).clone();
    }
    

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
    pub fn get(&mut self, mut inputStream: Option<Rc<RefCell<dyn crate::java::io::InputStream::InputStreamDyn>>>, mut outputStream: Option<Rc<RefCell<dyn crate::java::io::OutputStream::OutputStreamDyn>>>, mut buffer: Vec<i8>) -> Option<Rc<RefCell<dyn crate::java::io::OutputStream::OutputStreamDyn>>> {
        let mut len: i32 = 0;
        while true {
            len = { let java_value = (jcall!(inputStream).as_InputStream_mut().read3(buffer.clone(), 0, (buffer.len() as i32).clone())).clone(); java_value };
            if len != -1 {
            } else {
                break;
            }
            jcall!(outputStream).as_OutputStream_mut().write3(buffer.clone(), 0, len.clone());
        }
        return outputStream.clone();
    }
    

    // throws Exception
    pub fn getByteArray(&mut self, mut inputStream: Option<Rc<RefCell<dyn crate::java::io::InputStream::InputStreamDyn>>>, mut outputStream2: Option<Rc<RefCell<crate::java::io::OutputStream::OutputStream>>>, mut byteArray: Vec<i8>) -> Vec<i8> {
        let mut outputStream: Option<Rc<RefCell<crate::java::io::OutputStream::OutputStream>>> = outputStream2.clone();
        let java_try_result = match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            self.get(inputStream.clone(), outputStream.clone().map(|reference| reference as Rc<RefCell<dyn crate::java::io::OutputStream::OutputStreamDyn>>), byteArray.clone());
            return Some(jcall!(outputStream).toByteArray());
        })) {
            Ok(java_return) => java_return,
            Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            panic!("{:?}", e);
            None
            }
        };
        { // finally
            self.close(outputStream.clone().map(|reference| reference as Rc<RefCell<dyn Closeable>>));
        }if let Some(java_return) = java_try_result { return java_return; }unreachable!("try statement")
    }
    

    pub fn close(&mut self, mut closeable: Option<Rc<RefCell<dyn Closeable>>>) -> bool {
        let java_try_result = match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            if closeable.is_some() {
                jcall!(closeable).close();
            }
            return Some(true);
        })) {
            Ok(java_return) => java_return,
            Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            let mut commonStrings: Option<Rc<RefCell<CommonStrings>>> = CommonStrings::getInstance();
            jcall!(self.logUtil).put({ let java_field = jcall!(commonStrings).EXCEPTION.clone(); java_field }.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(commonStrings).CLOSE.clone(); java_field }.clone(), crate::java_into_object(e.clone()));
            return false;
            None
            }
        };if let Some(java_return) = java_try_result { return java_return; }unreachable!("try statement")
    }
}

