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
// package org.allbinary.data.resource
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::java::io::ByteArrayInputStream::ByteArrayInputStream;
// use crate::java::io::ByteArrayOutputStream::ByteArrayOutputStream;
use crate::java::io::FileInputStream::FileInputStream;
use crate::java::io::InputStream::InputStream;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::io::StreamUtil::StreamUtil;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use crate::java::lang::Exception::*;
use crate::java::lang::ClassLoader::*;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

//J2SE - PC
pub struct ResourceUtil {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub path: String,
    pub ext: String,
}

thread_local! { pub static ResourceUtil_instance: Option<Rc<RefCell<ResourceUtil>>> = ResourceUtil::new(); }

impl ResourceUtil {

    pub fn toString(&mut self) -> String {
        String::from(std::any::type_name::<Self>())
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<ResourceUtil>>> {
        return ResourceUtil_instance.with(Clone::clone).clone();
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            logUtil: None,
            path: String::new(),
            ext: String::new(),
        };
        this.logUtil = LogUtil::getInstance();
        this.path = { let java_field = jcall!(StringUtil::getInstance()).EMPTY_STRING.clone(); java_field }.clone();
        this.ext = { let java_field = jcall!(StringUtil::getInstance()).EMPTY_STRING.clone(); java_field }.clone();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn setLoadingPaths(&mut self, mut path: String, mut ext: String) {
        self.path = path.clone();
        self.ext = ext.clone();
    }
    

    pub fn setClassLoader(&mut self, mut classLoader: Option<Rc<RefCell<ClassLoader>>>) {
    }
    

    // throws Exception
    pub fn getResourceAsStream(&mut self, mut resource: String) -> Option<Rc<RefCell<dyn crate::java::io::InputStream::InputStreamDyn>>> {
        let mut inputStream: Option<Rc<RefCell<dyn crate::java::io::InputStream::InputStreamDyn>>> = self.getResourceAsStreamAtStart(resource.clone(), 2);
        if inputStream.is_none() {
            inputStream = self.getResourceAsStreamAtStart(resource.clone(), 1);
            if inputStream.is_none() {
                panic!("{:?}", format!("{}: {:?}", "Exception", (jcall!(StringMaker::new()).append(String::from("Unable to obtain: ")).append(resource.clone()).toString())));
            }
        }
        return inputStream.clone();
    }
    

    // throws Exception
    pub fn getResourceAsStreamAtStart(&mut self, mut resource: String, mut startIndex: i32) -> Option<Rc<RefCell<dyn crate::java::io::InputStream::InputStreamDyn>>> {
        let mut inputStream: Option<Rc<RefCell<dyn crate::java::io::InputStream::InputStreamDyn>>> = FileInputStream::new(jcall!(StringMaker::new()).append(self.path.clone()).append(resource.clone()).append(self.ext.clone()).toString()).map(|reference| reference as Rc<RefCell<dyn crate::java::io::InputStream::InputStreamDyn>>).map(|reference| reference as Rc<RefCell<dyn crate::java::io::InputStream::InputStreamDyn>>);
        if inputStream.is_some() {
            let mut byteArray: Vec<i8> = vec![0; (jcall!(inputStream).as_InputStream_mut().available()) as usize];
            jcall!(StreamUtil::getInstance()).getByteArray(inputStream.clone(), crate::java::io::OutputStream::OutputStream::new(), byteArray.clone());
            return crate::java::io::InputStream::InputStream::new1(byteArray.clone()).map(|reference| reference as Rc<RefCell<dyn crate::java::io::InputStream::InputStreamDyn>>).map(|reference| reference as Rc<RefCell<dyn crate::java::io::InputStream::InputStreamDyn>>);
        }
        return inputStream.clone();
    }
    

    pub fn addResource(&mut self, mut resource: String, mut value: Option<i32>) {
    }
}

