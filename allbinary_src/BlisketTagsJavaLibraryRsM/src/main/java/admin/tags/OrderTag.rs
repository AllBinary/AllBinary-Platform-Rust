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
// package admin.tags
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;
use super::*;
// use crate::java::lang::reflect::Method::Method;
// use crate::java::util::HashMap::HashMap;
// use crate::javax::servlet::jsp::JspTagException::JspTagException;
// use crate::javax::servlet::jsp::tagext::TagSupport::TagSupport;
use crate::admin::taghelpers::OrderHelperFactory::OrderHelperFactory;
use crate::org::allbinary::business::context::modules::storefront::StoreFrontData::StoreFrontData;
use crate::org::allbinary::business::user::commerce::money::payment::gateway::PaymentGatewayData::PaymentGatewayData;
use crate::org::allbinary::logic::StdUtil::StdUtil;
use crate::org::allbinary::logic::communication::http::request::AbResponseHandler::AbResponseHandler;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::system::security::licensing::LicensingException::LicensingException;
use crate::tags::CustomTagSupport::CustomTagSupport;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct OrderTag {
    pub base: CustomTagSupport,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub command: String,
    pub storeName: String,
    pub propertiesHashMap: Option<Rc<RefCell<HashMap>>>,
}


impl OrderTag {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: CustomTagSupport::new_struct(),
            logUtil: None,
            command: String::new(),
            storeName: String::new(),
            propertiesHashMap: None,
        };
        this.logUtil = LogUtil::getInstance();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn setCommand(&mut self, mut command: String) {
        self.command = command.clone();
    }
    

    pub fn setStoreName(&mut self, mut value: String) {
        self.storeName = value.clone();
    }
    

    // throws LicensingException
    pub fn process(&mut self) -> String {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = jcall!(OrderHelperFactory::new()).getInstance(self.propertiesHashMap.clone(), self.pageContext.clone());
            let mut helperClass: &'static str = std::any::type_name_of_val(&anyType);
            let mut method: Option<Rc<RefCell<Method>>> = jcall!(helperClass).getMethod(jcall!(self.base.commonStrings).PROCESS.clone(), None);
            let mut result: String = (jcall!(method).invoke(anyType.clone(), None)).clone() /* as String */;
            return result.clone();
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            panic!("{:?}", e);
            }
        }
    }
    

    // throws LicensingException
    pub fn setPaymentGateway(&mut self) -> Option<bool> {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = jcall!(OrderHelperFactory::new()).getInstance(self.propertiesHashMap.clone(), self.pageContext.clone());
            let mut addressHelperClass: &'static str = std::any::type_name_of_val(&anyType);
            let mut method: Option<Rc<RefCell<Method>>> = jcall!(addressHelperClass).getMethod(String::from("setPaymentGateway"), None);
            let mut result: Option<bool> = (jcall!(method).invoke(anyType.clone(), None)).clone() /* as Boolean */;
            return result.clone();
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            panic!("{:?}", e);
            }
        }
    }
    

    // throws JspTagException
    pub fn doStartTag(&mut self) -> i32 {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            if true {
                self.propertiesHashMap = jcall!(StdUtil::getInstance()).createHashMap();
                { let java_receiver = self.propertiesHashMap.clone(); let java_arguments = (jcall!(StoreFrontData::getInstance()).NAME.clone(), self.storeName.clone(),); jcall!(java_receiver).put(java_arguments.0, java_arguments.1); };
                if (self.command.cmp(&jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).globals).GLOBALS2).PROCESS.clone()) as i32) == 0 {
                    self.process();
                } else if (self.command.cmp(&jcall!(crate::org::allbinary::business::user::commerce::money::payment::gateway::PaymentGatewayData::PaymentGatewayData_NAME.with(Clone::clone)).toString()) as i32) == 0 {
                    if match (&self.setPaymentGateway(), &true) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                        return unsafe { crate::javax::servlet::jsp::tagext::TagSupport::TagSupport_EVAL_BODY_INCLUDE.clone() }.clone();
                    }
                }
            }
            return unsafe { crate::javax::servlet::jsp::tagext::TagSupport::TagSupport_SKIP_BODY.clone() }.clone();
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            AbResponseHandler::sendJspTagLicensingRedirect(self.pageContext.clone(), e.clone());
            return SKIP_BODY.clone();
            }
        }
    }
}
impl std::ops::Deref for OrderTag {
    type Target = CustomTagSupport;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for OrderTag {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::tags::CustomTagSupport::CustomTagSupportDyn for OrderTag {
    fn as_CustomTagSupport(&self) -> &crate::tags::CustomTagSupport::CustomTagSupport { &self.base }
    fn as_CustomTagSupport_mut(&mut self) -> &mut crate::tags::CustomTagSupport::CustomTagSupport { &mut self.base }
}


