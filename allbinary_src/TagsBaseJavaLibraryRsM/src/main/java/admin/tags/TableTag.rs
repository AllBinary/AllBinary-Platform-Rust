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
        if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).JSPTAG.clone()) {
            jcall!(this.logUtil).putF(jcall!(this.commonStrings).START.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(this.commonStrings).CONSTRUCTOR.clone());
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
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = jcall!(self.getTagHelperFactoryInterface()).getInstance(self.getPropertiesHashMap(), self.base.pageContext.clone());
            let mut addressHelperClass: &'static str = std::any::type_name_of_val(&anyType);
            let mut method: Option<Rc<RefCell<Method>>> = jcall!(addressHelperClass).getMethod(String::from("drop"), None);
            let mut result: String = (jcall!(method).invoke(anyType.clone(), None)).clone() /* as String */;
            return result.clone();
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            let mut error: String = String::from("Failed to drop table");
            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).SQLTAGSERROR.clone()) {
                jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).DROP.clone(), crate::java_into_object(e.clone()));
            }
            return error.clone();
            }
        }
    }
    

    // throws LicensingException
    pub fn create(&mut self) -> String {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = jcall!(self.getTagHelperFactoryInterface()).getInstance(self.getPropertiesHashMap(), self.base.pageContext.clone());
            let mut addressHelperClass: &'static str = std::any::type_name_of_val(&anyType);
            let mut method: Option<Rc<RefCell<Method>>> = jcall!(addressHelperClass).getMethod(String::from("create"), None);
            let mut result: String = (jcall!(method).invoke(anyType.clone(), None)).clone() /* as String */;
            return result.clone();
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            let mut error: String = String::from("Failed to create table");
            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).SQLTAGSERROR.clone()) {
                jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("create()"), crate::java_into_object(e.clone()));
            }
            return error.clone();
            }
        }
    }
    

    // throws LicensingException
    pub fn restore(&mut self) -> String {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = jcall!(self.getTagHelperFactoryInterface()).getInstance(self.getPropertiesHashMap(), self.base.pageContext.clone());
            let mut addressHelperClass: &'static str = std::any::type_name_of_val(&anyType);
            let mut method: Option<Rc<RefCell<Method>>> = jcall!(addressHelperClass).getMethod(String::from("restore"), None);
            let mut result: String = (jcall!(method).invoke(anyType.clone(), None)).clone() /* as String */;
            return result.clone();
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            let mut error: String = String::from("Failed to restore backup");
            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).SQLTAGSERROR.clone()) {
                jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("restore()"), crate::java_into_object(e.clone()));
            }
            return error.clone();
            }
        }
    }
    

    // throws LicensingException
    pub fn backup(&mut self) -> String {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = jcall!(self.getTagHelperFactoryInterface()).getInstance(self.getPropertiesHashMap(), self.base.pageContext.clone());
            let mut addressHelperClass: &'static str = std::any::type_name_of_val(&anyType);
            let mut method: Option<Rc<RefCell<Method>>> = jcall!(addressHelperClass).getMethod(String::from("backup"), None);
            let mut result: String = (jcall!(method).invoke(anyType.clone(), None)).clone() /* as String */;
            return result.clone();
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            let mut error: String = String::from("Failed to make backup");
            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).SQLTAGSERROR.clone()) {
                jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("backup()"), crate::java_into_object(e.clone()));
            }
            return error.clone();
            }
        }
    }
    

    // throws LicensingException
    pub fn insert(&mut self) -> String {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut helperClass: &'static str = std::any::type_name_of_val(&self.requestObject);
            let mut method: Option<Rc<RefCell<Method>>> = jcall!(helperClass).getMethod(String::from("insert"), None);
            let mut result: String = (jcall!(method).invoke(self.requestObject.clone(), None)).clone() /* as String */;
            return result.clone();
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            let mut error: String = String::from("Failed to insert");
            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).SQLTAGSERROR.clone()) {
                jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("insert()"), crate::java_into_object(e.clone()));
            }
            return error.clone();
            }
        }
    }
    

    // throws LicensingException
    pub fn delete(&mut self) -> String {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut helperClass: &'static str = std::any::type_name_of_val(&self.requestObject);
            let mut method: Option<Rc<RefCell<Method>>> = jcall!(helperClass).getMethod(jcall!(self.base.commonStrings).delete.clone(), None);
            let mut result: String = (jcall!(method).invoke(self.requestObject.clone(), None)).clone() /* as String */;
            return result.clone();
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            let mut error: String = String::from("Failed to delete");
            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).SQLTAGSERROR.clone()) {
                jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("delete()"), crate::java_into_object(e.clone()));
            }
            return error.clone();
            }
        }
    }
    

    // throws LicensingException
    pub fn update(&mut self) -> String {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut helperClass: &'static str = std::any::type_name_of_val(&self.requestObject);
            let mut method: Option<Rc<RefCell<Method>>> = jcall!(helperClass).getMethod(String::from("update"), None);
            let mut result: String = (jcall!(method).invoke(self.requestObject.clone(), None)).clone() /* as String */;
            return result.clone();
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            let mut error: String = String::from("Failed to update");
            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).SQLTAGSERROR.clone()) {
                jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("update()"), crate::java_into_object(e.clone()));
            }
            return error.clone();
            }
        }
    }
    

    // throws JspTagException
    pub fn doStartTag(&mut self) -> i32 {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).JSPTAG.clone()) {
                let mut request: Option<Rc<RefCell<HttpServletRequest>>> = (jcall!(self.pageContext).getRequest()).clone() /* as HttpServletRequest */;
                let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
                jcall!(stringBuffer).append(String::from("TableTag Helper: "));
                jcall!(stringBuffer).append(std::any::type_name_of_val(&self.getTagHelperFactoryInterface()).to_string());
                jcall!(stringBuffer).append(String::from(" Request URI: "));
                jcall!(stringBuffer).append(jcall!(request).getRequestURI());
                { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(stringBuffer).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.tagStrings).DO_START_TAG.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
            }
            if self.getTagRequestHelperFactoryInterface().is_some() {
                if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).JSPTAG.clone()) {
                    { let java_receiver = self.logUtil.clone(); let java_arguments = (format!("{}{}", String::from("TableTag RequestHelper: "), std::any::type_name_of_val(&self.getTagRequestHelperFactoryInterface()).to_string()), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.tagStrings).DO_START_TAG.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
                }
            } else {
                if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).JSPTAG.clone()) {
                    jcall!(self.logUtil).putF(String::from("TableTag RequestHelper: null"), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.tagStrings).DO_START_TAG.clone());
                }
            }
            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).JSPTAG.clone()) {
                { let java_receiver = self.logUtil.clone(); let java_arguments = (format!("{}{}", String::from("TableTag this.getCommand(): "), self.getCommand()), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.base.tagStrings).DO_START_TAG.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
            }
            if self.isEnabled() {
                if self.getCommand().is_some() {
                    if jcall!(self.getCommand()).compareTo(jcall!(self.commonStrings).INSERT.clone()) == 0 || jcall!(self.getCommand()).compareTo(jcall!(SqlStrings::getInstance()).INSERT_INTO.clone()) == 0 || jcall!(self.getCommand()).compareTo(jcall!(self.base.commonStrings).DELETE.clone()) == 0 || jcall!(self.getCommand()).compareTo(jcall!(self.base.commonStrings).UPDATE.clone()) == 0 || jcall!(self.getCommand()).compareTo(jcall!(SqlStrings::getInstance()).UPDATE.clone()) == 0 {
                        self.requestObject = jcall!(self.getTagRequestHelperFactoryInterface()).getInstance(self.getPropertiesHashMap(), self.base.pageContext.clone());
                    }
                    if jcall!(self.getCommand()).compareTo(jcall!(self.commonStrings).INSERT.clone()) == 0 || jcall!(self.getCommand()).compareTo(jcall!(SqlStrings::getInstance()).INSERT_INTO.clone()) == 0 {
                        self.insert();
                    } else if jcall!(self.getCommand()).compareTo(jcall!(self.base.commonStrings).DELETE.clone()) == 0 {
                        self.delete();
                    } else if jcall!(self.getCommand()).compareTo(jcall!(self.base.commonStrings).UPDATE.clone()) == 0 || jcall!(self.getCommand()).compareTo(jcall!(SqlStrings::getInstance()).UPDATE.clone()) == 0 {
                        self.update();
                    }
                    if jcall!(self.getCommand()).compareTo(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).globals).GLOBALS2).BACKUP.clone()) == 0 {
                        let mut output: String = self.backup();
                        if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).JSPTAGEXTRAOUTPUT.clone()) {
                            jcall!(jcall!(self.pageContext).getOut()).print(format!("{}{}", output, String::from("<br />")));
                        }
                        return SKIP_BODY.clone();
                    } else if jcall!(self.getCommand()).compareTo(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).globals).GLOBALS2).RESTORE.clone()) == 0 {
                        jcall!(self.getPropertiesHashMap()).put(String::from("current"), self.getCurrent());
                        jcall!(self.getPropertiesHashMap()).put(String::from("total"), self.getTotal());
                        let mut output: String = self.restore();
                        if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).JSPTAGEXTRAOUTPUT.clone()) {
                            jcall!(jcall!(self.pageContext).getOut()).print(format!("{}{}", output, String::from("<br />")));
                        }
                        return SKIP_BODY.clone();
                    } else if jcall!(self.getCommand()).compareTo(jcall!(self.base.commonStrings).DROP.clone()) == 0 {
                        let mut output: String = self.drop();
                        if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).JSPTAGEXTRAOUTPUT.clone()) {
                            jcall!(jcall!(self.pageContext).getOut()).print(format!("{}{}", output, String::from("<br />")));
                        }
                        return SKIP_BODY.clone();
                    } else if jcall!(self.getCommand()).compareTo(jcall!(self.base.commonStrings).CREATE.clone()) == 0 {
                        let mut output: String = self.create();
                        if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).JSPTAGEXTRAOUTPUT.clone()) {
                            jcall!(jcall!(self.pageContext).getOut()).print(format!("{}{}", output, String::from("<br />")));
                        }
                        return SKIP_BODY.clone();
                    } else if jcall!(self.getCommand()).compareTo(jcall!(self.base.commonStrings).INSERT.clone()) == 0 {
                        let mut output: String = self.insert();
                        if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).JSPTAGEXTRAOUTPUT.clone()) {
                            jcall!(jcall!(self.pageContext).getOut()).print(format!("{}{}", output, String::from("<br />")));
                        }
                    } else if jcall!(self.getCommand()).compareTo(jcall!(self.base.commonStrings).DELETE.clone()) == 0 {
                        let mut output: String = self.delete();
                        if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).JSPTAGEXTRAOUTPUT.clone()) {
                            jcall!(jcall!(self.pageContext).getOut()).print(format!("{}{}", output, String::from("<br />")));
                        }
                    } else if jcall!(self.getCommand()).compareTo(jcall!(self.base.commonStrings).UPDATE.clone()) == 0 {
                        let mut output: String = self.update();
                        if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).JSPTAGEXTRAOUTPUT.clone()) {
                            jcall!(jcall!(self.pageContext).getOut()).print(format!("{}{}", output, String::from("<br />")));
                        }
                    }
                }
            }
            return EVAL_BODY_INCLUDE.clone();
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


