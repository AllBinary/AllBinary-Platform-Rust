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
use crate::android::app::Activity::Activity;
use crate::android::content::Context::Context;
use crate::android::content::res::Resources::Resources;
use crate::org::allbinary::debug::DebugFactory::DebugFactory;
use crate::org::allbinary::debug::NoDebug::NoDebug;
use crate::org::allbinary::logic::NullUtil::NullUtil;
use crate::org::allbinary::logic::StdUtil::StdUtil;
use crate::org::allbinary::logic::communication::log::ForcedLogUtil::ForcedLogUtil;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::communication::log::PreLogUtil::PreLogUtil;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::string::CommonSeps::CommonSeps;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::org::allbinary::util::ABHashtable::ABHashtable;
use crate::org::allbinary::util::HashtableUtil::HashtableUtil;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

//Android
pub struct ResourceUtil {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub context: Option<Rc<RefCell<Box<dyn std::any::Any>>>>,
    pub resources: Option<Rc<RefCell<Box<dyn std::any::Any>>>>,
    pub hashMap: Option<Rc<RefCell<ABHashtable<Option<Rc<RefCell<Box<dyn std::any::Any>>>>, Option<Rc<RefCell<Box<dyn std::any::Any>>>>>>>>,
}

thread_local! { pub static ResourceUtil_instance: Option<Rc<RefCell<ResourceUtil>>> = ResourceUtil::new(); }

impl ResourceUtil {
    

    pub fn getInstance() -> Option<Rc<RefCell<ResourceUtil>>> {
        return ResourceUtil_instance.with(Clone::clone).clone();
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            logUtil: None,
            context: None,
            resources: None,
            hashMap: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.context = jcall!(NullUtil::getInstance()).NULL_OBJECT.clone();
        this.resources = jcall!(NullUtil::getInstance()).NULL_OBJECT.clone();
        this.hashMap = jcall!(StdUtil::getInstance()).createHashtable();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn setLoadingPaths(&mut self, mut path: String, mut ext: String) {
        { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(CommonStrings::getInstance()).NOT_IMPLEMENTED.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("setLoadingPaths"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
    }
    

    pub fn getContext(&mut self) -> Option<Rc<RefCell<Context>>> {
        return (self.context).clone() /* as Context */;
    }
    

    pub fn setContextFromActivity(&mut self, mut activity: Option<Rc<RefCell<Activity>>>) {
        self.context = activity.clone();
    }
    

    pub fn setContext(&mut self, mut aContext: Option<Rc<RefCell<Context>>>) {
        self.context = aContext.clone();
    }
    

    pub fn getResourceId(&mut self, mut resource: String) -> Option<i32> {
        let mut value: Option<i32> = { let java_object = jcall!(self.hashMap).get(crate::java_into_object(resource.clone())); java_object.as_ref().map(|object| { let object = object.borrow(); object.downcast_ref::<i32>().copied().or_else(|| object.downcast_ref::<Option<i32>>().copied().flatten()).expect("ClassCastException") }) };
        if !match (&DebugFactory::getInstance(), &NoDebug::getInstance()) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            PreLogUtil::put(jcall!(StringMaker::new()).append(resource.clone()).append(jcall!(CommonSeps::getInstance()).COLON.clone()).append((value).expect("NullPointerException").to_string()).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("getResourceId"));
        }
        return value.clone();
    }
    

    pub fn addResource(&mut self, mut resource: String, mut value: Option<i32>) {
        if !match (&DebugFactory::getInstance(), &NoDebug::getInstance()) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            PreLogUtil::put(jcall!(StringMaker::new()).append(resource.clone()).append(jcall!(CommonSeps::getInstance()).COLON.clone()).append((value).expect("NullPointerException").to_string()).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("addResource"));
            if self.containsDuplicate(resource.clone(), value.clone()) {
                ForcedLogUtil::log(jcall!(StringMaker::new()).append(String::from("Found Duplicate Resource: ")).append(resource.clone()).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())));
            }
        }
        jcall!(self.hashMap).put(crate::java_into_object(resource.clone()), crate::java_into_object((value.clone()).expect("NullPointerException")));
    }
    

    pub fn containsDuplicate(&mut self, mut resource: String, mut value: Option<i32>) -> bool {
        let mut objectArray: Vec<Option<Rc<RefCell<Box<dyn std::any::Any>>>>> = jcall!(HashtableUtil::getInstance()).getKeysAsArray(self.hashMap.clone());
        {
            let mut index: i32 = (objectArray.len() as i32).clone();
            while { index -= 1; index } >= 0 {
            let mut integer: Option<i32> = { let java_object = jcall!(self.hashMap).get(crate::java_into_object((objectArray[(index) as usize]).clone() /* as Any */)); java_object.as_ref().map(|object| { let object = object.borrow(); object.downcast_ref::<i32>().copied().or_else(|| object.downcast_ref::<Option<i32>>().copied().flatten()).expect("ClassCastException") }) };
            if resource != objectArray[(index) as usize] {
                if !match (&value, &integer) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                    if ((value).unwrap() as i32) == ((integer).unwrap() as i32) {
                        return true;
                    }
                }
            }
            }
        }
        return false;
    }
    

    pub fn setResources(&mut self, mut resources: Option<Rc<RefCell<Resources>>>) {
        { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(StringMaker::new()).append(String::from("Resource Loader: ")).append(std::any::type_name_of_val(&resources).to_string()).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("setResources"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        self.resources = resources.clone();
    }
    

    // throws Exception
    pub fn getResourceAsStream(&mut self, mut resource: String) -> Option<Rc<RefCell<InputStream>>> {
        let mut integer: Option<i32> = { let java_object = jcall!(self.hashMap).get(crate::java_into_object(resource.clone())); java_object.as_ref().map(|object| { let object = object.borrow(); object.downcast_ref::<i32>().copied().or_else(|| object.downcast_ref::<Option<i32>>().copied().flatten()).expect("ClassCastException") }) };
        let mut id: i32 = ((integer).unwrap() as i32);
        let mut resources: Option<Rc<RefCell<Resources>>> = ((self.resources).clone() /* as Resources */);
        let mut inputStream: Option<Rc<RefCell<InputStream>>> = jcall!(resources).openRawResource(id.clone());
        if inputStream.is_some() {
            return inputStream.clone();
        }
        return inputStream.clone();
    }
}

