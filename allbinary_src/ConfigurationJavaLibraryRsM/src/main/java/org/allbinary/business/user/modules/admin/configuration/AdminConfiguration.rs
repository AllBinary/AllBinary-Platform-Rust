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
// package org.allbinary.business.user.modules.admin.configuration
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;
use super::*;
// use crate::java::util::HashMap::HashMap;
// use crate::javax::servlet::http::HttpServletRequest::HttpServletRequest;
use crate::org::allbinary::business::context::configuration::ContextConfiguration::ContextConfiguration;
use crate::org::allbinary::business::context::configuration::ContextConfigurationData::ContextConfigurationData;
use crate::org::allbinary::business::context::configuration::ContextConfigurationDomDocumentMapping::ContextConfigurationDomDocumentMapping;
use crate::org::allbinary::business::context::configuration::ContextConfigurationInterface::ContextConfigurationInterface;
use crate::org::allbinary::business::context::configuration::ContextConfigurationInterfaceFactory::ContextConfigurationInterfaceFactory;
use crate::org::allbinary::business::context::configuration::ContextConfigurationPathUtil::ContextConfigurationPathUtil;
use crate::org::allbinary::logic::communication::http::request::RequestParams::RequestParams;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::control::crypt::file::CryptFileWriter::CryptFileWriter;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use super::AdminConfigurationInterface::AdminConfigurationInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct AdminConfiguration {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub commonStrings: Option<Rc<RefCell<CommonStrings>>>,
    pub contextConfigurationInterface: Option<Rc<RefCell<dyn ContextConfigurationInterface>>>,
}

impl AdminConfigurationInterface for AdminConfiguration {
    fn write(&mut self) { Self::write(self) }
    fn getContextConfigurationInterface(&mut self) -> Option<Rc<RefCell<dyn ContextConfigurationInterface>>> { Self::getContextConfigurationInterface(self) }
    fn setContextConfigurationInterface(&mut self, mut contextConfigurationInterface: Option<Rc<RefCell<dyn ContextConfigurationInterface>>>) { Self::setContextConfigurationInterface(self, contextConfigurationInterface) }
}

impl AdminConfiguration {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            logUtil: None,
            commonStrings: None,
            contextConfigurationInterface: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.commonStrings = CommonStrings::getInstance();
        jcall!(this.logUtil).putF(jcall!(this.commonStrings).START.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(this.commonStrings).CONSTRUCTOR.clone());
        this.contextConfigurationInterface = jcall!(ContextConfigurationInterfaceFactory::getInstance()).getInstance1(AdminConfigurationData_CONTEXTNAME.with(Clone::clone).clone());
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn new_struct1(mut request: Option<Rc<RefCell<HttpServletRequest>>>) -> Self {
        let mut this = Self {
            logUtil: None,
            commonStrings: None,
            contextConfigurationInterface: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.commonStrings = CommonStrings::getInstance();
        jcall!(this.logUtil).putF(jcall!(this.commonStrings).START.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(this.commonStrings).CONSTRUCTOR.clone());
        { let java_arguments = (jcall!(RequestParams::new1(request.clone()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::communication::http::request::RequestParams::RequestParamsDyn>>)).toHashMap(),); this.getFormData(java_arguments.0) };
        this
    }

    pub fn new1(mut request: Option<Rc<RefCell<HttpServletRequest>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct1(request))))
    }
    

    pub fn new_struct1(mut storeHashMap: Option<Rc<RefCell<HashMap>>>) -> Self {
        let mut this = Self {
            logUtil: None,
            commonStrings: None,
            contextConfigurationInterface: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.commonStrings = CommonStrings::getInstance();
        jcall!(this.logUtil).putF(jcall!(this.commonStrings).START.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(this.commonStrings).CONSTRUCTOR.clone());
        this.getFormData(storeHashMap.clone());
        this
    }

    pub fn new1(mut storeHashMap: Option<Rc<RefCell<HashMap>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct1(storeHashMap))))
    }
    

    // throws Exception
    pub fn getFormData(&mut self, mut storeHashMap: Option<Rc<RefCell<HashMap>>>) {
        jcall!(self.logUtil).putF(jcall!(self.commonStrings).START.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("getFormData"));
        self.setContextConfigurationInterface((ContextConfiguration::new1(storeHashMap.clone())).clone() /* as ContextConfigurationInterface */);
    }
    

    // throws Exception
    // synchronized
    pub fn write(&mut self) {
        let mut contextConfigurationDomDocumentMapping: Option<Rc<RefCell<ContextConfigurationDomDocumentMapping>>> = ContextConfigurationDomDocumentMapping::new(self.getContextConfigurationInterface());
        let mut cryptFileWriter: Option<Rc<RefCell<CryptFileWriter>>> = CryptFileWriter::new(jcall!(ContextConfigurationData::getInstance()).UNCRYPTED_EXTENSION.clone(), jcall!(ContextConfigurationData::getInstance()).ENCRYPTED_EXTENSION.clone());
        jcall!(cryptFileWriter).write(ContextConfigurationPathUtil::getAbPath(AdminConfigurationData_CONTEXTNAME.with(Clone::clone).clone()), jcall!(contextConfigurationDomDocumentMapping).toXmlDoc());
    }
    

    pub fn getContextConfigurationInterface(&mut self) -> Option<Rc<RefCell<dyn ContextConfigurationInterface>>> {
        return self.contextConfigurationInterface.clone();
    }
    

    pub fn setContextConfigurationInterface(&mut self, mut contextConfigurationInterface: Option<Rc<RefCell<dyn ContextConfigurationInterface>>>) {
        self.contextConfigurationInterface = contextConfigurationInterface.clone();
    }
}

