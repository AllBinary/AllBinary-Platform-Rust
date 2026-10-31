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
// use crate::java::io::InputStream::InputStream;
use crate::org::allbinary::logic::NullUtil::NullUtil;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::string::CommonSeps::CommonSeps;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::java::lang::Thread::*;
use super::ClassLoader::ClassLoader;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

//J2SE - J2ME
pub struct ResourceUtil {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
}

pub static mut ResourceUtil_classLoader: std::cell::LazyCell<Option<Rc<RefCell<Box<dyn std::any::Any>>>>> = std::cell::LazyCell::new(|| jcall!(NullUtil::getInstance()).NULL_OBJECT.clone());
thread_local! { pub static ResourceUtil_instance: Option<Rc<RefCell<ResourceUtil>>> = ResourceUtil::new(); }

impl ResourceUtil {
    

    pub fn getInstance() -> Option<Rc<RefCell<ResourceUtil>>> {
        return ResourceUtil_instance.with(Clone::clone).clone();
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
    

    pub fn setLoadingPaths(&mut self, mut path: String, mut ext: String) {
        { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(CommonStrings::getInstance()).NOT_IMPLEMENTED.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("setLoadingPaths"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
    }
    

    pub fn setClassLoader(&mut self, mut classLoader: Option<Rc<RefCell<ClassLoader>>>) {
        { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(StringMaker::new()).append(String::from("Resource Loader: ")).append(std::any::type_name_of_val(&classLoader).to_string()).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("setClassLoader"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        unsafe { *ResourceUtil_classLoader = classLoader.clone() };
    }
    

    // throws Exception
    pub fn getResourceAsStream(&mut self, mut resource: String) -> Option<Rc<RefCell<InputStream>>> {
        let mut inputStream: Option<Rc<RefCell<InputStream>>> = self.getResourceAsStreamAtStart(resource.clone(), 2);
        if inputStream.is_none() {
            inputStream = self.getResourceAsStreamAtStart(resource.clone(), 1);
            if inputStream.is_none() {
                panic!("{:?}", format!("{}: {:?}", "Exception", (jcall!(StringMaker::new()).append(String::from("Unable to obtain: ")).append(resource.clone()).toString())));
            }
        }
        return inputStream.clone();
    }
    

    // throws Exception
    pub fn getResourceAsStreamAtStart(&mut self, mut resource: String, mut startIndex: i32) -> Option<Rc<RefCell<InputStream>>> {
        let mut commonSeps: Option<Rc<RefCell<CommonSeps>>> = CommonSeps::getInstance();
        let mut index: i32 = resource.find(&jcall!(commonSeps).COLON.clone()).map(|index| index as i32).unwrap_or(-1);
        let mut resourcePath: String = resource[(index + startIndex) as usize..].to_string();
        let mut inputStream: Option<Rc<RefCell<InputStream>>> = jcall!(std::any::type_name_of_val(&resource)).getResourceAsStream(resourcePath.clone());
        if inputStream.is_some() {
            return inputStream.clone();
        }
        let mut classLoader: Option<Rc<RefCell<ClassLoader>>> = (unsafe { ResourceUtil_classLoader.clone() }).clone() /* as ClassLoader */;
        inputStream = jcall!(classLoader).getResourceAsStream(resourcePath.clone());
        if inputStream.is_some() {
            return inputStream.clone();
        }
        inputStream = jcall!(jcall!(Thread::currentThread()).getContextClassLoader()).getResourceAsStream(resourcePath.clone());
        if inputStream.is_some() {
            return inputStream.clone();
        }
        return inputStream.clone();
    }
    

    pub fn addResource(&mut self, mut resource: String, mut value: Option<i32>) {
    }
}

