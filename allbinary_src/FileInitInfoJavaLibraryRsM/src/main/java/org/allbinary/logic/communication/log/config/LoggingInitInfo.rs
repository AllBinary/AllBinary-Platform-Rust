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
// package org.allbinary.logic.communication.log.config
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::data::tree::dom::DomSearchHelper::DomSearchHelper;
use crate::org::allbinary::data::tree::dom::document::DomDocumentHelper::DomDocumentHelper;
use crate::org::allbinary::data::tree::dom::document::XmlDocumentHelper::XmlDocumentHelper;
use crate::org::allbinary::globals::PATH_GLOBALS::PATH_GLOBALS;
use crate::org::allbinary::globals::URLGLOBALS::URLGLOBALS;
use crate::org::allbinary::logic::io::AbFileLocalInputStream::AbFileLocalInputStream;
use crate::org::allbinary::logic::io::file::AbFile::AbFile;
use crate::org::allbinary::logic::io::path::AbPath::AbPath;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
use crate::org::w3c::dom::Document::Document;
use crate::org::w3c::dom::Node::Node;
use crate::org::w3c::dom::NodeList::NodeList;
use super::LogConfig::LogConfig;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct LoggingInitInfo {
}

thread_local! { pub static LoggingInitInfo_INITFILENAME: String = String::from("logConfig.xml"); }
thread_local! { pub static LoggingInitInfo_PACKAGE: String = jcall!(PATH_GLOBALS::getInstance()).INIT_PATH.clone(); }
pub static mut LoggingInitInfo_logConfigInfoList: std::cell::LazyCell<Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>> = std::cell::LazyCell::new(|| None);
pub static mut LoggingInitInfo_hasRead: std::cell::LazyCell<bool> = std::cell::LazyCell::new(|| false);

impl LoggingInitInfo {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    // throws Exception
    // synchronized
    pub fn getDoc() -> Option<Rc<RefCell<Document>>> {
        let mut path: String = format!("{}{}", URLGLOBALS::getWebappPath(), LoggingInitInfo_PACKAGE.with(Clone::clone));
        let mut FILEABPATH: Option<Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>> = AbPath::new(path.clone(), LoggingInitInfo_INITFILENAME.with(Clone::clone).clone()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>);
        let mut file: Option<Rc<RefCell<AbFile>>> = AbFile::createAbFileFromAbPath(FILEABPATH.clone());
        let mut document: Option<Rc<RefCell<Document>>> = DomDocumentHelper::create1(AbFileLocalInputStream::new(file.clone()));
        return document.clone();
    }
    

    // throws Exception
    // synchronized
    pub fn write() {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            unsafe { *LoggingInitInfo_hasRead = false };
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            panic!("{:?}", e);
            }
        }
    }
    

    // throws Exception
    // synchronized
    pub fn read() {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut document: Option<Rc<RefCell<Document>>> = LoggingInitInfo::getDoc();
            let mut logConfigsNodeList: Option<Rc<RefCell<NodeList>>> = jcall!(document).getElementsByTagName(jcall!(LogConfigsData::LogConfigsData::getInstance()).NAME.clone());
            let mut logConfigsNode: Option<Rc<RefCell<Node>>> = jcall!(logConfigsNodeList).item(0);
            let mut logConfigNodeVector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = DomSearchHelper::getAllNodes(jcall!(LogConfigData::LogConfigData::getInstance()).NAME.clone(), jcall!(logConfigsNode).getChildNodes());
            unsafe { *LoggingInitInfo_logConfigInfoList = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayListD::BasicArrayListDDyn>>) };
            let mut size: i32 = jcall!(logConfigNodeVector).size();
            for i in (0..size) {
                let mut node: Option<Rc<RefCell<Node>>> = (jcall!(logConfigNodeVector).get(i.clone())).clone() /* as Node */;
                let mut logConfigInfo: Option<Rc<RefCell<LogConfig>>> = LogConfig::new1(node.clone());
                jcall!(unsafe { LoggingInitInfo_logConfigInfoList.clone() }).add(crate::java_into_object(logConfigInfo.clone()));
            }
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            panic!("{:?}", e);
            }
        }
    }
    

    // synchronized
    pub fn setHasRead(mut value: bool) {
        unsafe { *LoggingInitInfo_hasRead = value.clone() };
    }
    

    // throws Exception
    // synchronized
    pub fn updateIfNeeded() {
        if !unsafe { LoggingInitInfo_hasRead.clone() } {
            LoggingInitInfo::read();
            unsafe { *LoggingInitInfo_hasRead = true };
            if unsafe { LoggingInitInfo_logConfigInfoList.clone() }.is_none() {
                panic!("{:?}", format!("{}: {:?}", "Exception", (String::from("Read Failed"))));
            }
        }
    }
    

    pub fn isValid(&mut self) -> bool {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            return true;
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            return false;
            }
        }
    }
    

    // synchronized
    pub fn set(mut logConfigInfoVector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) {
        unsafe { *LoggingInitInfo_logConfigInfoList = logConfigInfoVector.clone() };
    }
    

    // throws Exception
    pub fn get() -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        LoggingInitInfo::updateIfNeeded();
        return unsafe { LoggingInitInfo_logConfigInfoList.clone() }.clone();
    }
    

    // throws Exception
    pub fn getTypeNameList() -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        LoggingInitInfo::updateIfNeeded();
        let mut allLogTypeVector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        let mut size: i32 = jcall!(unsafe { LoggingInitInfo_logConfigInfoList.clone() }).size();
        for index in (0..size) {
            let mut logConfigInfo: Option<Rc<RefCell<LogConfig>>> = { let java_object = (jcall!(unsafe { LoggingInitInfo_logConfigInfoList.clone() }).as_BasicArrayList_mut().objectArray[(index) as usize]).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<LogConfig>>>>().expect("ClassCastException").clone()) };
            let mut logTypeVector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = jcall!(logConfigInfo).getTypeVector();
            if jcall!(logConfigInfo).isEnabled() {
                jcall!(allLogTypeVector).addAllList(logTypeVector.clone());
            }
        }
        return allLogTypeVector.clone();
    }
    

    // throws Exception
    pub fn getTypeList() -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        LoggingInitInfo::updateIfNeeded();
        let mut allLogTypeVector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        let mut size: i32 = jcall!(unsafe { LoggingInitInfo_logConfigInfoList.clone() }).size();
        for index in (0..size) {
            let mut logConfigInfo: Option<Rc<RefCell<LogConfig>>> = { let java_object = (jcall!(unsafe { LoggingInitInfo_logConfigInfoList.clone() }).as_BasicArrayList_mut().objectArray[(index) as usize]).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<LogConfig>>>>().expect("ClassCastException").clone()) };
            let mut logTypeVector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = jcall!(logConfigInfo).getTypeVector();
            if jcall!(logConfigInfo).isEnabled() {
                jcall!(allLogTypeVector).addAllList(logTypeVector.clone());
            }
        }
        return allLogTypeVector.clone();
    }
    

    // throws Exception
    pub fn getNumberOfLogConfigs(&mut self) -> i32 {
        return jcall!(unsafe { LoggingInitInfo_logConfigInfoList.clone() }).size();
    }
    

    pub fn toString(&mut self) -> String {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut document: Option<Rc<RefCell<Document>>> = LoggingInitInfo::getDoc();
            return XmlDocumentHelper::toString(document.clone());
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            return String::from("Error");
            }
        }
    }
}

