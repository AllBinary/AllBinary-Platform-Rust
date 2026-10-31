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
// package org.allbinary.logic.communication.log.config.type
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::data::tree::dom::DomNodeHelper::DomNodeHelper;
use crate::org::allbinary::data::tree::dom::DomSearchHelper::DomSearchHelper;
use crate::org::allbinary::logic::communication::log::PreLogUtil::PreLogUtil;
use crate::org::allbinary::logic::communication::log::config::LoggingInitInfo::LoggingInitInfo;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
use crate::org::w3c::dom::Node::Node;
use super::LogConfigType::LogConfigType;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct LogConfigTypes {
}

thread_local! { pub static LogConfigTypes_JSP_ERROR_NO_FUNCTION: String = String::from("in JSP Not In Function"); }
pub static mut LogConfigTypes_LOGGING: std::cell::LazyCell<Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>> = std::cell::LazyCell::new(|| BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>));

impl LogConfigTypes {
    

    // Java static initializer: call this once at startup
    pub fn static_init() {
        LogConfigTypes::init();
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    // synchronized
    pub fn init() {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            PreLogUtil::put(String::from("Initialize LogconfigTypes"), String::from("LogConfigTypes"), String::from("init()"));
            LogConfigTypeFactory::LogConfigTypeFactory::getInstance();
            let mut loggingInitInfo: Option<Rc<RefCell<LoggingInitInfo>>> = LoggingInitInfo::new();
            let mut logConfigTypeVector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = jcall!(loggingInitInfo).getTypeList();
            PreLogUtil::put(format!("{}{}", String::from("Number Of Log Configs: "), jcall!(loggingInitInfo).getNumberOfLogConfigs()), String::from("LogConfigTypes"), String::from("init()"));
            PreLogUtil::put(format!("{}{}", String::from("Number Of Log Config Type Names: "), jcall!(logConfigTypeVector).size()), String::from("LogConfigTypes"), String::from("init()"));
            jcall!(unsafe { LogConfigTypes_LOGGING.clone() }).addAllList(logConfigTypeVector.clone());
            jcall!(unsafe { LogConfigTypes_LOGGING.clone() }).add(crate::java_into_object(jcall!(LogConfigTypeFactory::LogConfigTypeFactory::getInstance()).NETBEANS_MODULE.clone()));
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            PreLogUtil::putSE(String::from("Unable to initialize LogConfigTypes"), String::from("LogConfigTypes"), String::from("init()"), e.clone());
            }
        }
    }
    

    // throws Exception
    pub fn getInstance(mut node: Option<Rc<RefCell<Node>>>) -> Option<Rc<RefCell<LogConfigType>>> {
        let mut nameValueNode: Option<Rc<RefCell<Node>>> = DomSearchHelper::getNode(jcall!(LogConfigTypeData::LogConfigTypeData::getInstance()).NAME.clone(), jcall!(node).getChildNodes());
        let mut name: String = DomNodeHelper::getTextNodeValue(nameValueNode.clone());
        let mut descriptionValueNode: Option<Rc<RefCell<Node>>> = DomSearchHelper::getNodeNoThrow(jcall!(LogConfigTypeData::LogConfigTypeData::getInstance()).DESCRIPTION.clone(), jcall!(node).getChildNodes());
        if descriptionValueNode.is_some() {
            let mut description: String = DomNodeHelper::getTextNodeValue(descriptionValueNode.clone());
        }
        let mut availableLogConfigTypes: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = unsafe { LogConfigType_availableLogConfigTypes.clone() }.clone();
        let mut size: i32 = jcall!(availableLogConfigTypes).size();
        let mut logConfigType: Option<Rc<RefCell<LogConfigType>>> = None;
        for index in (0..size) {
            logConfigType = { let java_object = (jcall!(availableLogConfigTypes).get(index.clone())).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<LogConfigType>>>>().expect("ClassCastException").clone()) };
            if jcall!(jcall!(logConfigType).getName()).compareTo(name.clone()) == 0 {
                return logConfigType.clone();
            }
        }
        panic!("{:?}", format!("{}: {:?}", "Exception", (format!("{}{}", String::from("No Such LogConfigType: "), name))));
    }
}

