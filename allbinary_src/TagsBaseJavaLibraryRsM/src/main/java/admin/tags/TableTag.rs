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
// use crate::javax::servlet::http::HttpServletRequest::HttpServletRequest;
// use crate::javax::servlet::jsp::JspTagException::JspTagException;
use crate::admin::taghelpers::TagHelperFactoryInterface::TagHelperFactoryInterface;
use crate::org::allbinary::logic::communication::http::request::AbResponseHandler::AbResponseHandler;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::communication::sql::SqlStrings::SqlStrings;
use crate::org::allbinary::logic::java::bool::BooleanUtil::BooleanUtil;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::string::StringValidationUtil::StringValidationUtil;
use crate::org::allbinary::logic::system::security::licensing::LicensingException::LicensingException;
use crate::tags::PropertiesTag::PropertiesTag;
use crate::java::lang::Exception::*;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct TableTag {
    pub base: PropertiesTag,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub enabled: String,
    pub tagHelperFactoryInterface: Option<Rc<RefCell<dyn TagHelperFactoryInterface>>>,
    pub tagRequestHelperFactoryInterface: Option<Rc<RefCell<dyn TagHelperFactoryInterface>>>,
    pub requestObject: Option<Rc<RefCell<Box<dyn std::any::Any>>>>,
    pub current: Option<i32>,
    pub total: Option<i32>,
}


impl TableTag {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: PropertiesTag::new_struct(),
            logUtil: None,
            enabled: String::new(),
            tagHelperFactoryInterface: None,
            tagRequestHelperFactoryInterface: None,
            requestObject: None,
            current: None,
            total: None,
        };
        this.logUtil = LogUtil::getInstance();
        if jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypes.clone(); java_field }).LOGGING.clone(); java_field }).contains({ let java_field = jcall!(jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypeFactory.clone(); java_field }).getInstance()).JSPTAG.clone(); java_field }.clone()) {
            jcall!(this.logUtil).putF({ let java_field = jcall!(this.commonStrings).START.clone(); java_field }.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(this.commonStrings).CONSTRUCTOR.clone(); java_field }.clone());
        }
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn setIsSelected(&mut self, mut enabled: String) {
        self.enabled = enabled.clone();
    }
    

    // throws Exception
    pub fn isEnabled(&mut self) -> bool {
        if !jcall!(StringValidationUtil::getInstance()).isEmpty(self.enabled.clone()) {
            if jcall!(BooleanUtil::getInstance()).getFromString(self.enabled.clone()) || (self.enabled.cmp(&String::from("on")) as i32) == 0 {
                return true;
            }
        }
        return false;
    }
    

    pub fn setTagHelperFactory(&mut self, mut tagHelperFactoryInterface: Option<Rc<RefCell<dyn TagHelperFactoryInterface>>>) {
        self.tagHelperFactoryInterface = tagHelperFactoryInterface.clone();
    }
    

    pub fn getTagHelperFactoryInterface(&mut self) -> Option<Rc<RefCell<dyn TagHelperFactoryInterface>>> {
        return self.tagHelperFactoryInterface.clone();
    }
    

    pub fn setTagRequestHelperFactory(&mut self, mut tagHelperFactoryInterface: Option<Rc<RefCell<dyn TagHelperFactoryInterface>>>) {
        self.tagRequestHelperFactoryInterface = tagHelperFactoryInterface.clone();
    }
    

    pub fn getTagRequestHelperFactoryInterface(&mut self) -> Option<Rc<RefCell<dyn TagHelperFactoryInterface>>> {
        return self.tagRequestHelperFactoryInterface.clone();
    }
    

    pub fn setCurrent(&mut self, mut current: Option<i32>) {
        self.current = current.clone();
    }
    

    pub fn getCurrent(&mut self) -> Option<i32> {
        return self.current.clone();
    }
    

    pub fn setTotal(&mut self, mut total: Option<i32>) {
        self.total = total.clone();
    }
    

    pub fn getTotal(&mut self) -> Option<i32> {
        return self.total.clone();
    }
    

    // throws LicensingException
    pub fn drop(&mut self) -> String {
        let java_try_result = match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = jcall!(self.getTagHelperFactoryInterface()).getInstance(self.getPropertiesHashMap(), self.base.pageContext.clone());
            let mut addressHelperClass: &'static str = std::any::type_name_of_val(&anyType);
            let mut method: Option<Rc<RefCell<Method>>> = jcall!(addressHelperClass).getMethod(String::from("drop"), None);
            let mut result: String = (jcall!(method).invoke(anyType.clone(), None)).clone() /* as String */;
            return Some(result.clone());
        })) {
            Ok(java_return) => java_return,
            Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            let mut error: String = String::from("Failed to drop table");
            if jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypes.clone(); java_field }).LOGGING.clone(); java_field }).contains({ let java_field = jcall!(jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypeFactory.clone(); java_field }).getInstance()).SQLTAGSERROR.clone(); java_field }.clone()) {
                jcall!(self.logUtil).put({ let java_field = jcall!(self.commonStrings).EXCEPTION.clone(); java_field }.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(self.commonStrings).DROP.clone(); java_field }.clone(), crate::java_into_object(e.clone()));
            }
            return error.clone();
            None
            }
        };if let Some(java_return) = java_try_result { return java_return; }unreachable!("try statement")
    }
    

    // throws LicensingException
    pub fn create(&mut self) -> String {
        let java_try_result = match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = jcall!(self.getTagHelperFactoryInterface()).getInstance(self.getPropertiesHashMap(), self.base.pageContext.clone());
            let mut addressHelperClass: &'static str = std::any::type_name_of_val(&anyType);
            let mut method: Option<Rc<RefCell<Method>>> = jcall!(addressHelperClass).getMethod(String::from("create"), None);
            let mut result: String = (jcall!(method).invoke(anyType.clone(), None)).clone() /* as String */;
            return Some(result.clone());
        })) {
            Ok(java_return) => java_return,
            Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            let mut error: String = String::from("Failed to create table");
            if jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypes.clone(); java_field }).LOGGING.clone(); java_field }).contains({ let java_field = jcall!(jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypeFactory.clone(); java_field }).getInstance()).SQLTAGSERROR.clone(); java_field }.clone()) {
                jcall!(self.logUtil).put({ let java_field = jcall!(self.commonStrings).EXCEPTION.clone(); java_field }.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("create()"), crate::java_into_object(e.clone()));
            }
            return error.clone();
            None
            }
        };if let Some(java_return) = java_try_result { return java_return; }unreachable!("try statement")
    }
    

    // throws LicensingException
    pub fn restore(&mut self) -> String {
        let java_try_result = match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = jcall!(self.getTagHelperFactoryInterface()).getInstance(self.getPropertiesHashMap(), self.base.pageContext.clone());
            let mut addressHelperClass: &'static str = std::any::type_name_of_val(&anyType);
            let mut method: Option<Rc<RefCell<Method>>> = jcall!(addressHelperClass).getMethod(String::from("restore"), None);
            let mut result: String = (jcall!(method).invoke(anyType.clone(), None)).clone() /* as String */;
            return Some(result.clone());
        })) {
            Ok(java_return) => java_return,
            Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            let mut error: String = String::from("Failed to restore backup");
            if jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypes.clone(); java_field }).LOGGING.clone(); java_field }).contains({ let java_field = jcall!(jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypeFactory.clone(); java_field }).getInstance()).SQLTAGSERROR.clone(); java_field }.clone()) {
                jcall!(self.logUtil).put({ let java_field = jcall!(self.commonStrings).EXCEPTION.clone(); java_field }.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("restore()"), crate::java_into_object(e.clone()));
            }
            return error.clone();
            None
            }
        };if let Some(java_return) = java_try_result { return java_return; }unreachable!("try statement")
    }
    

    // throws LicensingException
    pub fn backup(&mut self) -> String {
        let java_try_result = match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = jcall!(self.getTagHelperFactoryInterface()).getInstance(self.getPropertiesHashMap(), self.base.pageContext.clone());
            let mut addressHelperClass: &'static str = std::any::type_name_of_val(&anyType);
            let mut method: Option<Rc<RefCell<Method>>> = jcall!(addressHelperClass).getMethod(String::from("backup"), None);
            let mut result: String = (jcall!(method).invoke(anyType.clone(), None)).clone() /* as String */;
            return Some(result.clone());
        })) {
            Ok(java_return) => java_return,
            Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            let mut error: String = String::from("Failed to make backup");
            if jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypes.clone(); java_field }).LOGGING.clone(); java_field }).contains({ let java_field = jcall!(jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypeFactory.clone(); java_field }).getInstance()).SQLTAGSERROR.clone(); java_field }.clone()) {
                jcall!(self.logUtil).put({ let java_field = jcall!(self.commonStrings).EXCEPTION.clone(); java_field }.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("backup()"), crate::java_into_object(e.clone()));
            }
            return error.clone();
            None
            }
        };if let Some(java_return) = java_try_result { return java_return; }unreachable!("try statement")
    }
    

    // throws LicensingException
    pub fn insert(&mut self) -> String {
        let java_try_result = match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut helperClass: &'static str = std::any::type_name_of_val(&self.requestObject);
            let mut method: Option<Rc<RefCell<Method>>> = jcall!(helperClass).getMethod(String::from("insert"), None);
            let mut result: String = (jcall!(method).invoke(self.requestObject.clone(), None)).clone() /* as String */;
            return Some(result.clone());
        })) {
            Ok(java_return) => java_return,
            Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            let mut error: String = String::from("Failed to insert");
            if jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypes.clone(); java_field }).LOGGING.clone(); java_field }).contains({ let java_field = jcall!(jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypeFactory.clone(); java_field }).getInstance()).SQLTAGSERROR.clone(); java_field }.clone()) {
                jcall!(self.logUtil).put({ let java_field = jcall!(self.commonStrings).EXCEPTION.clone(); java_field }.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("insert()"), crate::java_into_object(e.clone()));
            }
            return error.clone();
            None
            }
        };if let Some(java_return) = java_try_result { return java_return; }unreachable!("try statement")
    }
    

    // throws LicensingException
    pub fn delete(&mut self) -> String {
        let java_try_result = match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut helperClass: &'static str = std::any::type_name_of_val(&self.requestObject);
            let mut method: Option<Rc<RefCell<Method>>> = jcall!(helperClass).getMethod({ let java_field = jcall!(self.base.commonStrings).delete.clone(); java_field }.clone(), None);
            let mut result: String = (jcall!(method).invoke(self.requestObject.clone(), None)).clone() /* as String */;
            return Some(result.clone());
        })) {
            Ok(java_return) => java_return,
            Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            let mut error: String = String::from("Failed to delete");
            if jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypes.clone(); java_field }).LOGGING.clone(); java_field }).contains({ let java_field = jcall!(jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypeFactory.clone(); java_field }).getInstance()).SQLTAGSERROR.clone(); java_field }.clone()) {
                jcall!(self.logUtil).put({ let java_field = jcall!(self.commonStrings).EXCEPTION.clone(); java_field }.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("delete()"), crate::java_into_object(e.clone()));
            }
            return error.clone();
            None
            }
        };if let Some(java_return) = java_try_result { return java_return; }unreachable!("try statement")
    }
    

    // throws LicensingException
    pub fn update(&mut self) -> String {
        let java_try_result = match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut helperClass: &'static str = std::any::type_name_of_val(&self.requestObject);
            let mut method: Option<Rc<RefCell<Method>>> = jcall!(helperClass).getMethod(String::from("update"), None);
            let mut result: String = (jcall!(method).invoke(self.requestObject.clone(), None)).clone() /* as String */;
            return Some(result.clone());
        })) {
            Ok(java_return) => java_return,
            Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            let mut error: String = String::from("Failed to update");
            if jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypes.clone(); java_field }).LOGGING.clone(); java_field }).contains({ let java_field = jcall!(jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypeFactory.clone(); java_field }).getInstance()).SQLTAGSERROR.clone(); java_field }.clone()) {
                jcall!(self.logUtil).put({ let java_field = jcall!(self.commonStrings).EXCEPTION.clone(); java_field }.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("update()"), crate::java_into_object(e.clone()));
            }
            return error.clone();
            None
            }
        };if let Some(java_return) = java_try_result { return java_return; }unreachable!("try statement")
    }
    

    // throws JspTagException
    pub fn doStartTag(&mut self) -> i32 {
        let java_try_result = match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            if jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypes.clone(); java_field }).LOGGING.clone(); java_field }).contains({ let java_field = jcall!(jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypeFactory.clone(); java_field }).getInstance()).JSPTAG.clone(); java_field }.clone()) {
                let mut request: Option<Rc<RefCell<HttpServletRequest>>> = (jcall!(self.pageContext).getRequest()).clone() /* as HttpServletRequest */;
                let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
                jcall!(stringBuffer).append(String::from("TableTag Helper: "));
                jcall!(stringBuffer).append({ let java_argument = std::any::type_name_of_val(&self.getTagHelperFactoryInterface()).to_string(); java_argument });
                jcall!(stringBuffer).append(String::from(" Request URI: "));
                jcall!(stringBuffer).append(jcall!(request).getRequestURI());
                { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(stringBuffer).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(self.tagStrings).DO_START_TAG.clone(); java_field }.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
            }
            if self.getTagRequestHelperFactoryInterface().is_some() {
                if jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypes.clone(); java_field }).LOGGING.clone(); java_field }).contains({ let java_field = jcall!(jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypeFactory.clone(); java_field }).getInstance()).JSPTAG.clone(); java_field }.clone()) {
                    { let java_receiver = self.logUtil.clone(); let java_arguments = (format!("{}{}", String::from("TableTag RequestHelper: "), { let java_value = std::any::type_name_of_val(&self.getTagRequestHelperFactoryInterface()).to_string(); java_value }), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(self.tagStrings).DO_START_TAG.clone(); java_field }.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
                }
            } else {
                if jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypes.clone(); java_field }).LOGGING.clone(); java_field }).contains({ let java_field = jcall!(jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypeFactory.clone(); java_field }).getInstance()).JSPTAG.clone(); java_field }.clone()) {
                    jcall!(self.logUtil).putF(String::from("TableTag RequestHelper: null"), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(self.tagStrings).DO_START_TAG.clone(); java_field }.clone());
                }
            }
            if jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypes.clone(); java_field }).LOGGING.clone(); java_field }).contains({ let java_field = jcall!(jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypeFactory.clone(); java_field }).getInstance()).JSPTAG.clone(); java_field }.clone()) {
                { let java_receiver = self.logUtil.clone(); let java_arguments = (format!("{}{}", String::from("TableTag this.getCommand(): "), self.getCommand()), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(self.base.tagStrings).DO_START_TAG.clone(); java_field }.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
            }
            if self.isEnabled() {
                if true {
                    if { let java_value = (self.getCommand().cmp(&{ let java_field = jcall!(self.commonStrings).INSERT.clone(); java_field }.clone()) as i32); java_value } == 0 || { let java_value = (self.getCommand().cmp(&{ let java_field = jcall!(SqlStrings::getInstance()).INSERT_INTO.clone(); java_field }.clone()) as i32); java_value } == 0 || { let java_value = (self.getCommand().cmp(&{ let java_field = jcall!(self.base.commonStrings).DELETE.clone(); java_field }.clone()) as i32); java_value } == 0 || { let java_value = (self.getCommand().cmp(&{ let java_field = jcall!(self.base.commonStrings).UPDATE.clone(); java_field }.clone()) as i32); java_value } == 0 || { let java_value = (self.getCommand().cmp(&{ let java_field = jcall!(SqlStrings::getInstance()).UPDATE.clone(); java_field }.clone()) as i32); java_value } == 0 {
                        self.requestObject = { let java_value = (jcall!(self.getTagRequestHelperFactoryInterface()).getInstance(self.getPropertiesHashMap(), self.base.pageContext.clone())).clone(); java_value };
                    }
                    if { let java_value = (self.getCommand().cmp(&{ let java_field = jcall!(self.commonStrings).INSERT.clone(); java_field }.clone()) as i32); java_value } == 0 || { let java_value = (self.getCommand().cmp(&{ let java_field = jcall!(SqlStrings::getInstance()).INSERT_INTO.clone(); java_field }.clone()) as i32); java_value } == 0 {
                        self.insert();
                    } else if { let java_value = (self.getCommand().cmp(&{ let java_field = jcall!(self.base.commonStrings).DELETE.clone(); java_field }.clone()) as i32); java_value } == 0 {
                        self.delete();
                    } else if { let java_value = (self.getCommand().cmp(&{ let java_field = jcall!(self.base.commonStrings).UPDATE.clone(); java_field }.clone()) as i32); java_value } == 0 || { let java_value = (self.getCommand().cmp(&{ let java_field = jcall!(SqlStrings::getInstance()).UPDATE.clone(); java_field }.clone()) as i32); java_value } == 0 {
                        self.update();
                    }
                    if { let java_value = (self.getCommand().cmp(&{ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).globals.clone(); java_field }).GLOBALS2.clone(); java_field }).BACKUP.clone(); java_field }.clone()) as i32); java_value } == 0 {
                        let mut output: String = self.backup();
                        if jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypes.clone(); java_field }).LOGGING.clone(); java_field }).contains({ let java_field = jcall!(jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypeFactory.clone(); java_field }).getInstance()).JSPTAGEXTRAOUTPUT.clone(); java_field }.clone()) {
                            jcall!(jcall!(self.pageContext).getOut()).print(format!("{}{}", output, String::from("<br />")));
                        }
                        return Some(SKIP_BODY.clone());
                    } else if { let java_value = (self.getCommand().cmp(&{ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).globals.clone(); java_field }).GLOBALS2.clone(); java_field }).RESTORE.clone(); java_field }.clone()) as i32); java_value } == 0 {
                        jcall!(self.getPropertiesHashMap()).put(crate::java_into_object(String::from("current")), crate::java_into_object(self.getCurrent()));
                        jcall!(self.getPropertiesHashMap()).put(crate::java_into_object(String::from("total")), crate::java_into_object(self.getTotal()));
                        let mut output: String = self.restore();
                        if jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypes.clone(); java_field }).LOGGING.clone(); java_field }).contains({ let java_field = jcall!(jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypeFactory.clone(); java_field }).getInstance()).JSPTAGEXTRAOUTPUT.clone(); java_field }.clone()) {
                            jcall!(jcall!(self.pageContext).getOut()).print(format!("{}{}", output, String::from("<br />")));
                        }
                        return Some(SKIP_BODY.clone());
                    } else if { let java_value = (self.getCommand().cmp(&{ let java_field = jcall!(self.base.commonStrings).DROP.clone(); java_field }.clone()) as i32); java_value } == 0 {
                        let mut output: String = self.drop();
                        if jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypes.clone(); java_field }).LOGGING.clone(); java_field }).contains({ let java_field = jcall!(jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypeFactory.clone(); java_field }).getInstance()).JSPTAGEXTRAOUTPUT.clone(); java_field }.clone()) {
                            jcall!(jcall!(self.pageContext).getOut()).print(format!("{}{}", output, String::from("<br />")));
                        }
                        return Some(SKIP_BODY.clone());
                    } else if { let java_value = (self.getCommand().cmp(&{ let java_field = jcall!(self.base.commonStrings).CREATE.clone(); java_field }.clone()) as i32); java_value } == 0 {
                        let mut output: String = self.create();
                        if jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypes.clone(); java_field }).LOGGING.clone(); java_field }).contains({ let java_field = jcall!(jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypeFactory.clone(); java_field }).getInstance()).JSPTAGEXTRAOUTPUT.clone(); java_field }.clone()) {
                            jcall!(jcall!(self.pageContext).getOut()).print(format!("{}{}", output, String::from("<br />")));
                        }
                        return Some(SKIP_BODY.clone());
                    } else if { let java_value = (self.getCommand().cmp(&{ let java_field = jcall!(self.base.commonStrings).INSERT.clone(); java_field }.clone()) as i32); java_value } == 0 {
                        let mut output: String = self.insert();
                        if jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypes.clone(); java_field }).LOGGING.clone(); java_field }).contains({ let java_field = jcall!(jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypeFactory.clone(); java_field }).getInstance()).JSPTAGEXTRAOUTPUT.clone(); java_field }.clone()) {
                            jcall!(jcall!(self.pageContext).getOut()).print(format!("{}{}", output, String::from("<br />")));
                        }
                    } else if { let java_value = (self.getCommand().cmp(&{ let java_field = jcall!(self.base.commonStrings).DELETE.clone(); java_field }.clone()) as i32); java_value } == 0 {
                        let mut output: String = self.delete();
                        if jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypes.clone(); java_field }).LOGGING.clone(); java_field }).contains({ let java_field = jcall!(jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypeFactory.clone(); java_field }).getInstance()).JSPTAGEXTRAOUTPUT.clone(); java_field }.clone()) {
                            jcall!(jcall!(self.pageContext).getOut()).print(format!("{}{}", output, String::from("<br />")));
                        }
                    } else if { let java_value = (self.getCommand().cmp(&{ let java_field = jcall!(self.base.commonStrings).UPDATE.clone(); java_field }.clone()) as i32); java_value } == 0 {
                        let mut output: String = self.update();
                        if jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypes.clone(); java_field }).LOGGING.clone(); java_field }).contains({ let java_field = jcall!(jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypeFactory.clone(); java_field }).getInstance()).JSPTAGEXTRAOUTPUT.clone(); java_field }.clone()) {
                            jcall!(jcall!(self.pageContext).getOut()).print(format!("{}{}", output, String::from("<br />")));
                        }
                    }
                }
            }
            return Some(EVAL_BODY_INCLUDE.clone());
        })) {
            Ok(java_return) => java_return,
            Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            AbResponseHandler::sendJspTagLicensingRedirect(self.pageContext.clone(), e.clone());
            return SKIP_BODY.clone();
            None
            }
        };if let Some(java_return) = java_try_result { return java_return; }unreachable!("try statement")
    }
}
impl std::ops::Deref for TableTag {
    type Target = PropertiesTag;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for TableTag {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

pub trait TableTagDyn: crate::tags::PropertiesTag::PropertiesTagDyn {
    fn as_TableTag(&self) -> &TableTag;
    fn as_TableTag_mut(&mut self) -> &mut TableTag;
    fn setIsSelected(&mut self, mut enabled: String) { TableTag::setIsSelected(self.as_TableTag_mut(), enabled) }
    fn isEnabled(&mut self) -> bool { TableTag::isEnabled(self.as_TableTag_mut()) }
    fn setTagHelperFactory(&mut self, mut tagHelperFactoryInterface: Option<Rc<RefCell<dyn TagHelperFactoryInterface>>>) { TableTag::setTagHelperFactory(self.as_TableTag_mut(), tagHelperFactoryInterface) }
    fn getTagHelperFactoryInterface(&mut self) -> Option<Rc<RefCell<dyn TagHelperFactoryInterface>>> { TableTag::getTagHelperFactoryInterface(self.as_TableTag_mut()) }
    fn setTagRequestHelperFactory(&mut self, mut tagHelperFactoryInterface: Option<Rc<RefCell<dyn TagHelperFactoryInterface>>>) { TableTag::setTagRequestHelperFactory(self.as_TableTag_mut(), tagHelperFactoryInterface) }
    fn getTagRequestHelperFactoryInterface(&mut self) -> Option<Rc<RefCell<dyn TagHelperFactoryInterface>>> { TableTag::getTagRequestHelperFactoryInterface(self.as_TableTag_mut()) }
    fn setCurrent(&mut self, mut current: Option<i32>) { TableTag::setCurrent(self.as_TableTag_mut(), current) }
    fn getCurrent(&mut self) -> Option<i32> { TableTag::getCurrent(self.as_TableTag_mut()) }
    fn setTotal(&mut self, mut total: Option<i32>) { TableTag::setTotal(self.as_TableTag_mut(), total) }
    fn getTotal(&mut self) -> Option<i32> { TableTag::getTotal(self.as_TableTag_mut()) }
    fn drop(&mut self) -> String { TableTag::drop(self.as_TableTag_mut()) }
    fn create(&mut self) -> String { TableTag::create(self.as_TableTag_mut()) }
    fn restore(&mut self) -> String { TableTag::restore(self.as_TableTag_mut()) }
    fn backup(&mut self) -> String { TableTag::backup(self.as_TableTag_mut()) }
    fn insert(&mut self) -> String { TableTag::insert(self.as_TableTag_mut()) }
    fn delete(&mut self) -> String { TableTag::delete(self.as_TableTag_mut()) }
    fn update(&mut self) -> String { TableTag::update(self.as_TableTag_mut()) }
    fn doStartTag(&mut self) -> i32 { TableTag::doStartTag(self.as_TableTag_mut()) }
}
impl TableTagDyn for TableTag {
    fn as_TableTag(&self) -> &TableTag { self }
    fn as_TableTag_mut(&mut self) -> &mut TableTag { self }
}

impl crate::tags::CommandTag::CommandTagDyn for TableTag {
    fn as_CommandTag(&self) -> &crate::tags::CommandTag::CommandTag { &self.base.base }
    fn as_CommandTag_mut(&mut self) -> &mut crate::tags::CommandTag::CommandTag { &mut self.base.base }
}
impl crate::tags::CustomTagSupport::CustomTagSupportDyn for TableTag {
    fn as_CustomTagSupport(&self) -> &crate::tags::CustomTagSupport::CustomTagSupport { &self.base.base.base }
    fn as_CustomTagSupport_mut(&mut self) -> &mut crate::tags::CustomTagSupport::CustomTagSupport { &mut self.base.base.base }
}
impl crate::tags::PropertiesTag::PropertiesTagDyn for TableTag {
    fn as_PropertiesTag(&self) -> &crate::tags::PropertiesTag::PropertiesTag { &self.base }
    fn as_PropertiesTag_mut(&mut self) -> &mut crate::tags::PropertiesTag::PropertiesTag { &mut self.base }
}


