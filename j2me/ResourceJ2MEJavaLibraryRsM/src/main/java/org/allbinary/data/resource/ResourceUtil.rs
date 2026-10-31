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
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::string::CommonSeps::CommonSeps;
use crate::org::allbinary::string::CommonStrings::CommonStrings;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

//J2ME
pub struct ResourceUtil {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
}

pub static mut ResourceUtil_instance: std::cell::LazyCell<Option<Rc<RefCell<ResourceUtil>>>> = std::cell::LazyCell::new(|| ResourceUtil::new());

impl ResourceUtil {
    

    pub fn getInstance() -> Option<Rc<RefCell<ResourceUtil>>> {
        return unsafe { ResourceUtil_instance.clone() }.clone();
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
    

    // throws Exception
    pub fn getResourceAsStream(&mut self, mut resource: String) -> Option<Rc<RefCell<InputStream>>> {
        let mut inputStream: Option<Rc<RefCell<InputStream>>> = jcall!(std::any::type_name_of_val(&resource)).getResourceAsStream(resource.clone());
        if inputStream.is_none() {
            inputStream = self.getResourceAsStreamAtStart(resource.clone(), 2);
            if inputStream.is_none() {
                inputStream = self.getResourceAsStreamAtStart(resource.clone(), 1);
                if inputStream.is_none() {
                    let mut stringMaker: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
                    let mut index: i32 = resource.rfind('/').map(|index| index as i32).unwrap_or(-1);
                    let mut resourcePath: String = resource[(index + 1) as usize..].to_string();
                    inputStream = jcall!(std::any::type_name_of_val(&resource)).getResourceAsStream(resourcePath.clone());
                    if inputStream.is_none() {
                        resourcePath = resource[(index.clone()) as usize..].to_string();
                        inputStream = jcall!(std::any::type_name_of_val(&resource)).getResourceAsStream(resourcePath.clone());
                        if inputStream.is_none() {
                            let mut RES: String = String::from("res");
                            resourcePath = jcall!(stringMaker).append(RES.clone()).append(resource[(index.clone()) as usize..].to_string()).toString();
                            inputStream = jcall!(std::any::type_name_of_val(&resource)).getResourceAsStream(resourcePath.clone());
                            if inputStream.is_none() {
                                jcall!(stringMaker).delete(0, jcall!(stringMaker).length());
                                resourcePath = jcall!(stringMaker).append(String::from("/")).append(RES.clone()).append(resource[(index.clone()) as usize..].to_string()).toString();
                                inputStream = jcall!(std::any::type_name_of_val(&resource)).getResourceAsStream(resourcePath.clone());
                                let mut COLON: String = jcall!(CommonSeps::getInstance()).COLON.clone();
                                if inputStream.is_none() {
                                    jcall!(stringMaker).delete(0, jcall!(stringMaker).length());
                                    resourcePath = jcall!(stringMaker).append(RES.clone()).append(COLON.clone()).append(resource[(index.clone()) as usize..].to_string()).toString();
                                    inputStream = jcall!(std::any::type_name_of_val(&resource)).getResourceAsStream(resourcePath.clone());
                                    if inputStream.is_none() {
                                        jcall!(stringMaker).delete(0, jcall!(stringMaker).length());
                                        resourcePath = jcall!(stringMaker).append(RES.clone()).append(COLON.clone()).append(resource[(index + 1) as usize..].to_string()).toString();
                                        inputStream = jcall!(std::any::type_name_of_val(&resource)).getResourceAsStream(resourcePath.clone());
                                        if inputStream.is_none() {
                                            let mut RESOURCE_STRING: String = String::from("resource");
                                            jcall!(stringMaker).delete(0, jcall!(stringMaker).length());
                                            resourcePath = jcall!(stringMaker).append(RESOURCE_STRING.clone()).append(COLON.clone()).append(resource[(index.clone()) as usize..].to_string()).toString();
                                            inputStream = jcall!(std::any::type_name_of_val(&resource)).getResourceAsStream(resourcePath.clone());
                                            if inputStream.is_none() {
                                                jcall!(stringMaker).delete(0, jcall!(stringMaker).length());
                                                resourcePath = jcall!(stringMaker).append(RESOURCE_STRING.clone()).append(COLON.clone()).append(resource[(index + 1) as usize..].to_string()).toString();
                                                inputStream = jcall!(std::any::type_name_of_val(&resource)).getResourceAsStream(resourcePath.clone());
                                                if inputStream.is_none() {
                                                    jcall!(stringMaker).delete(0, jcall!(stringMaker).length());
                                                    panic!("{:?}", format!("{}: {:?}", "Exception", (jcall!(stringMaker).append(String::from("Unable to obtain: ")).append(resource.clone()).toString())));
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        return inputStream.clone();
    }
    

    // throws Exception
    pub fn getResourceAsStreamAtStart(&mut self, mut resource: String, mut startIndex: i32) -> Option<Rc<RefCell<InputStream>>> {
        let mut stringMaker: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        let mut index: i32 = resource.find(&jcall!(CommonSeps::getInstance()).COLON.clone()).map(|index| index as i32).unwrap_or(-1);
        let mut resourcePath: String = resource[(index + startIndex) as usize..].to_string();
        let mut inputStream: Option<Rc<RefCell<InputStream>>> = jcall!(std::any::type_name_of_val(&resource)).getResourceAsStream(resourcePath.clone());
        if inputStream.is_some() {
            return inputStream.clone();
        }
        return inputStream.clone();
    }
    

    pub fn addResource(&mut self, mut resource: String, mut value: Option<i32>) {
    }
}

