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
use super::*;
// use crate::java::lang::reflect::Method::Method;
// use crate::javax::servlet::jsp::JspTagException::JspTagException;
use crate::admin::taghelpers::OrderItemsHelperFactory::OrderItemsHelperFactory;
use crate::admin::taghelpers::OrderItemsRequestHelperFactory::OrderItemsRequestHelperFactory;
use crate::org::allbinary::business::user::commerce::inventory::order::OrderHistoryData::OrderHistoryData;
use crate::org::allbinary::logic::communication::http::request::AbResponseHandler::AbResponseHandler;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::system::security::licensing::LicensingException::LicensingException;
use super::TableTag::TableTag;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct OrderItemsTag {
    pub base: TableTag,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub status: String,
}


impl OrderItemsTag {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: TableTag::new_struct(),
            logUtil: None,
            status: String::new(),
        };
        this.logUtil = LogUtil::getInstance();
        this.setTagHelperFactory(OrderItemsHelperFactory::new());
        this.setTagRequestHelperFactory(OrderItemsRequestHelperFactory::new());
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn setStatus(&mut self, mut value: String) {
        self.status = value.clone();
    }
    

    // throws LicensingException
    pub fn setOrderStatus(&mut self) -> String {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = jcall!(OrderItemsRequestHelperFactory::new()).getInstance(self.getPropertiesHashMap(), self.pageContext.clone());
            if false {
                let mut helperClass: &'static str = std::any::type_name_of_val(&anyType);
                let mut method: Option<Rc<RefCell<Method>>> = jcall!(helperClass).getMethod(String::from("setOrderStatus"), None);
                let mut result: String = (jcall!(method).invoke(anyType.clone(), None)).clone() /* as String */;
                return result.clone();
            } else {
                let mut helperClass: &'static str = std::any::type_name_of_val(&anyType);
                let mut methodParams: Vec<&'static str> = vec![std::any::type_name_of_val(&self.status)];
                let mut method: Option<Rc<RefCell<Method>>> = jcall!(helperClass).getMethod(String::from("setOrderStatus"), methodParams.clone());
                let mut methodArgs: Vec<Option<Rc<RefCell<Box<dyn std::any::Any>>>>> = vec![self.status.clone()];
                let mut result: String = (jcall!(method).invoke(anyType.clone(), methodArgs.clone())).clone() /* as String */;
                return result.clone();
            }
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            panic!("{:?}", e);
            }
        }
    }
    

    // throws JspTagException
    pub fn doStartTag(&mut self) -> i32 {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            if self.isEnabled() {
                if self.getCommand().is_some() {
                    jcall!(self.getPropertiesHashMap()).put(crate::org::allbinary::business::user::commerce::inventory::order::OrderHistoryData::OrderHistoryData_STATUS.with(Clone::clone).clone(), self.status.clone());
                    if jcall!(self.getCommand()).compareTo(crate::org::allbinary::business::user::commerce::inventory::order::OrderHistoryData::OrderHistoryData_SETSTATUS.with(Clone::clone).clone()) == 0 {
                        jcall!(jcall!(self.pageContext).getOut()).print(self.setOrderStatus());
                    } else {
                        return self.base.doStartTag();
                    }
                }
            }
            return SKIP_BODY.clone();
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
impl std::ops::Deref for OrderItemsTag {
    type Target = TableTag;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for OrderItemsTag {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::admin::tags::TableTag::TableTagDyn for OrderItemsTag {
    fn as_TableTag(&self) -> &crate::admin::tags::TableTag::TableTag { &self.base }
    fn as_TableTag_mut(&mut self) -> &mut crate::admin::tags::TableTag::TableTag { &mut self.base }
    fn doStartTag(&mut self) -> i32 { Self::doStartTag(self) }
}
impl crate::tags::CommandTag::CommandTagDyn for OrderItemsTag {
    fn as_CommandTag(&self) -> &crate::tags::CommandTag::CommandTag { &self.base.base.base }
    fn as_CommandTag_mut(&mut self) -> &mut crate::tags::CommandTag::CommandTag { &mut self.base.base.base }
}
impl crate::tags::CustomTagSupport::CustomTagSupportDyn for OrderItemsTag {
    fn as_CustomTagSupport(&self) -> &crate::tags::CustomTagSupport::CustomTagSupport { &self.base.base.base.base }
    fn as_CustomTagSupport_mut(&mut self) -> &mut crate::tags::CustomTagSupport::CustomTagSupport { &mut self.base.base.base.base }
}
impl crate::tags::PropertiesTag::PropertiesTagDyn for OrderItemsTag {
    fn as_PropertiesTag(&self) -> &crate::tags::PropertiesTag::PropertiesTag { &self.base.base }
    fn as_PropertiesTag_mut(&mut self) -> &mut crate::tags::PropertiesTag::PropertiesTag { &mut self.base.base }
}


