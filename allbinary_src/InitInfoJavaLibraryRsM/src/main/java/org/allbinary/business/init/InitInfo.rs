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
// package org.allbinary.business.init
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::java::util::HashMap::HashMap;
use crate::org::allbinary::logic::StdUtil::StdUtil;
use crate::org::allbinary::logic::communication::log::PreLogUtil::PreLogUtil;
use crate::org::allbinary::logic::communication::log::config::r#type::LogConfigTypeFactory::LogConfigTypeFactory;
use crate::org::allbinary::logic::communication::log::config::r#type::LogConfigTypes::LogConfigTypes;
use crate::org::allbinary::logic::io::path::AbPath::AbPath;
use crate::org::allbinary::logic::java::bool::BooleanUtil::BooleanUtil;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use crate::java::lang::Exception::*;
use super::InitInfoEntity::InitInfoEntity;
use super::InitException::InitException;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct InitInfo {
    pub TESTING: String,
    pub MAINPATH: String,
    pub TESTHTMLPATH: String,
    pub testing: String,
    pub mainPath: Option<Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>>,
    pub testHtmlPath: Option<Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>>,
    pub hasRead: bool,
}

thread_local! { pub static InitInfo_instance: Option<Rc<RefCell<InitInfo>>> = InitInfo::new(); }

impl InitInfo {

    pub fn toString(&mut self) -> String {
        String::from(std::any::type_name::<Self>())
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<InitInfo>>> {
        return InitInfo_instance.with(Clone::clone).clone();
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            TESTING: String::new(),
            MAINPATH: String::new(),
            TESTHTMLPATH: String::new(),
            testing: String::new(),
            mainPath: None,
            testHtmlPath: None,
            hasRead: false,
        };
        this.TESTING = String::from("TESTING");
        this.MAINPATH = String::from("MAINPATH");
        this.TESTHTMLPATH = String::from("TESTHTMLPATH");
        this.testing = String::new();
        this.mainPath = None;
        this.testHtmlPath = None;
        this.hasRead = false;
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    // throws InitException
    // synchronized
    pub fn set(&mut self) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut initInfoEntity: Option<Rc<RefCell<InitInfoEntity>>> = InitInfoEntity::new();
            if jcall!(unsafe { crate::org::allbinary::logic::communication::log::config::r#type::LogConfigTypes::LogConfigTypes_LOGGING.clone() }).contains(crate::java_into_object({ let java_field = jcall!(LogConfigTypeFactory::getInstance()).PRELOADER.clone(); java_field }.clone())) {
                PreLogUtil::put(String::from("created entity"), crate::java_into_object(String::from("InitInfo")), String::from("set()"));
            }
            if !jcall!(initInfoEntity).is() {
                if jcall!(unsafe { crate::org::allbinary::logic::communication::log::config::r#type::LogConfigTypes::LogConfigTypes_LOGGING.clone() }).contains(crate::java_into_object({ let java_field = jcall!(LogConfigTypeFactory::getInstance()).PRELOADER.clone(); java_field }.clone())) {
                    PreLogUtil::put(String::from("adding"), crate::java_into_object(String::from("InitInfo")), String::from("set()"));
                }
                jcall!(initInfoEntity).add();
            } else {
                if jcall!(unsafe { crate::org::allbinary::logic::communication::log::config::r#type::LogConfigTypes::LogConfigTypes_LOGGING.clone() }).contains(crate::java_into_object({ let java_field = jcall!(LogConfigTypeFactory::getInstance()).PRELOADER.clone(); java_field }.clone())) {
                    PreLogUtil::put(String::from("update"), crate::java_into_object(String::from("InitInfo")), String::from("set()"));
                }
                jcall!(initInfoEntity).update();
            }
            self.hasRead = false;
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            if jcall!(unsafe { crate::org::allbinary::logic::communication::log::config::r#type::LogConfigTypes::LogConfigTypes_LOGGING.clone() }).contains(crate::java_into_object({ let java_field = jcall!(LogConfigTypeFactory::getInstance()).PRELOADERERROR.clone(); java_field }.clone())) {
                PreLogUtil::putOE(String::from("error"), crate::java_into_object(String::from("InitInfo")), String::from("set()"), crate::java_into_object(e.clone()));
            }
            }
        }
    }
    

    // Java overloaded method: Rust needs a different name
    // synchronized
    pub fn set1(&mut self, mut hashMap: Option<Rc<RefCell<HashMap<crate::JavaObject, crate::JavaObject>>>>) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            self.testing = { let java_value = ({ let java_object = (jcall!(hashMap).get(crate::java_into_object(self.TESTING.clone()))).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() }).clone(); java_value };
            self.mainPath = { let java_value = (AbPath::new({ let java_object = (jcall!(hashMap).get(crate::java_into_object(self.MAINPATH.clone()))).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() }, { let java_field = jcall!(StringUtil::getInstance()).EMPTY_STRING.clone(); java_field }.clone()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>)).clone(); java_value };
            self.testHtmlPath = { let java_value = (AbPath::new({ let java_object = (jcall!(hashMap).get(crate::java_into_object(self.TESTHTMLPATH.clone()))).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() }, { let java_field = jcall!(StringUtil::getInstance()).EMPTY_STRING.clone(); java_field }.clone()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>)).clone(); java_value };
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            if jcall!(unsafe { crate::org::allbinary::logic::communication::log::config::r#type::LogConfigTypes::LogConfigTypes_LOGGING.clone() }).contains(crate::java_into_object({ let java_field = jcall!(LogConfigTypeFactory::getInstance()).PRELOADERERROR.clone(); java_field }.clone())) {
                PreLogUtil::putOE(String::from("error"), crate::java_into_object(String::from("InitInfo")), String::from("set()"), crate::java_into_object(e.clone()));
            }
            }
        }
    }
    

    // synchronized
    pub fn get(&mut self) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            self.testing = None;
            self.mainPath = None;
            self.testHtmlPath = None;
            let mut initInfoEntity: Option<Rc<RefCell<InitInfoEntity>>> = InitInfoEntity::new();
            jcall!(initInfoEntity).get();
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            if jcall!(unsafe { crate::org::allbinary::logic::communication::log::config::r#type::LogConfigTypes::LogConfigTypes_LOGGING.clone() }).contains(crate::java_into_object({ let java_field = jcall!(LogConfigTypeFactory::getInstance()).PRELOADERERROR.clone(); java_field }.clone())) {
                PreLogUtil::putOE(String::from("error"), crate::java_into_object(String::from("InitInfo")), String::from("set()"), crate::java_into_object(e.clone()));
            }
            return;
            }
        }
    }
    

    // synchronized
    pub fn setHasRead(&mut self, mut value: bool) {
        self.hasRead = value.clone();
    }
    

    // synchronized
    pub fn updateIfNeeded(&mut self) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            if !self.hasRead {
                self.hasRead = true;
                self.get();
            }
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            if jcall!(unsafe { crate::org::allbinary::logic::communication::log::config::r#type::LogConfigTypes::LogConfigTypes_LOGGING.clone() }).contains(crate::java_into_object({ let java_field = jcall!(LogConfigTypeFactory::getInstance()).PRELOADERERROR.clone(); java_field }.clone())) {
                PreLogUtil::putOE(String::from("error"), crate::java_into_object(String::from("InitInfo")), String::from("updateIfNeeded()"), crate::java_into_object(e.clone()));
            }
            return;
            }
        }
    }
    

    pub fn isTesting(&mut self) -> bool {
        let java_try_result = match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            self.updateIfNeeded();
            return Some(jcall!(BooleanUtil::getInstance()).getFromString(self.testing.clone()));
        })) {
            Ok(java_return) => java_return,
            Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            if jcall!(unsafe { crate::org::allbinary::logic::communication::log::config::r#type::LogConfigTypes::LogConfigTypes_LOGGING.clone() }).contains(crate::java_into_object({ let java_field = jcall!(LogConfigTypeFactory::getInstance()).PRELOADERERROR.clone(); java_field }.clone())) {
                PreLogUtil::putOE(String::from("error"), crate::java_into_object(String::from("InitInfo")), String::from("updateIfNeeded()"), crate::java_into_object(e.clone()));
            }
            return false;
            None
            }
        };if let Some(java_return) = java_try_result { return java_return; }unreachable!("try statement")
    }
    

    pub fn getTesting(&mut self) -> String {
        self.updateIfNeeded();
        return self.testing.clone();
    }
    

    pub fn getTestHtmlPath(&mut self) -> String {
        self.updateIfNeeded();
        if self.testHtmlPath.is_some() {
            return jcall!(self.testHtmlPath).toString();
        } else {
            return None;
        }
    }
    

    pub fn getMainPath(&mut self) -> String {
        self.updateIfNeeded();
        if self.mainPath.is_some() {
            return jcall!(self.mainPath).toString();
        } else {
            return None;
        }
    }
    

    pub fn setTesting(&mut self, mut value: String) {
        self.testing = value.clone();
    }
    

    pub fn setTestHtmlPath(&mut self, mut value: Option<Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>>) {
        self.testHtmlPath = value.clone();
    }
    

    pub fn setMainPath(&mut self, mut value: Option<Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>>) {
        self.mainPath = value.clone();
    }
    

    pub fn isMainPathValid(&mut self, mut abPath: Option<Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>>) -> bool {
        return true;
    }
    

    pub fn isTestHtmlPathValid(&mut self, mut abPath: Option<Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>>) -> bool {
        return true;
    }
    

    // throws Exception
    pub fn isTestingValid(&mut self, mut testing: String) -> bool {
        return jcall!(BooleanUtil::getInstance()).isStringBoolean(testing.clone());
    }
    

    pub fn toHashMap(&mut self) -> Option<Rc<RefCell<HashMap<crate::JavaObject, crate::JavaObject>>>> {
        let mut hashMap: Option<Rc<RefCell<HashMap<crate::JavaObject, crate::JavaObject>>>> = jcall!(StdUtil::getInstance()).createHashMap();
        jcall!(hashMap).put(crate::java_into_object(self.TESTING.clone()), crate::java_into_object(self.getTesting()));
        jcall!(hashMap).put(crate::java_into_object(self.MAINPATH.clone()), crate::java_into_object(self.getMainPath()));
        jcall!(hashMap).put(crate::java_into_object(self.TESTHTMLPATH.clone()), crate::java_into_object(self.getTestHtmlPath()));
        return hashMap.clone();
    }
}

