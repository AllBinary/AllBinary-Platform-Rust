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
use crate::admin::taghelpers::BasicTextEmailHelperFactory::BasicTextEmailHelperFactory;
use crate::org::allbinary::logic::StdUtil::StdUtil;
use crate::org::allbinary::logic::communication::http::request::AbResponseHandler::AbResponseHandler;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::tags::CustomTagSupport::CustomTagSupport;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct BasicTextEmailTag {
    pub base: CustomTagSupport,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub subject: String,
    pub body: String,
}


impl BasicTextEmailTag {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: CustomTagSupport::new_struct(),
            logUtil: None,
            subject: String::new(),
            body: String::new(),
        };
        this.logUtil = LogUtil::getInstance();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn setSubject(&mut self, mut value: String) {
        self.subject = value.clone();
    }
    

    pub fn setBody(&mut self, mut value: String) {
        self.body = value.clone();
    }
    

    pub fn send(&mut self) -> String {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut propertiesHashMap: Option<Rc<RefCell<HashMap>>> = jcall!(StdUtil::getInstance()).createHashMap();
            jcall!(propertiesHashMap).put(String::from("Subject"), self.subject.clone());
            jcall!(propertiesHashMap).put(String::from("Body"), self.body.clone());
            let mut anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = jcall!(BasicTextEmailHelperFactory::new()).getInstance(propertiesHashMap.clone(), self.pageContext.clone());
            let mut method: Option<Rc<RefCell<Method>>> = jcall!(std::any::type_name_of_val(&anyType)).getMethod(String::from("send"), None);
            jcall!(method).invoke(anyType.clone(), None);
            return String::from("Email Sent");
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            let mut error: String = String::from("Failed to Send Email.");
            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).SQLTAGSERROR.clone()) {
                jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("send()"), crate::java_into_object(e.clone()));
            }
            return error.clone();
            }
        }
    }
    

    // throws JspTagException
    pub fn doStartTag(&mut self) -> i32 {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            jcall!(jcall!(self.pageContext).getOut()).print(self.send());
            return SKIP_BODY.clone();
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            AbResponseHandler::sendJspTagRedirect(self.pageContext.clone(), e.clone());
            return SKIP_BODY.clone();
            }
        }
    }
}
impl std::ops::Deref for BasicTextEmailTag {
    type Target = CustomTagSupport;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for BasicTextEmailTag {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::tags::CustomTagSupport::CustomTagSupportDyn for BasicTextEmailTag {
    fn as_CustomTagSupport(&self) -> &crate::tags::CustomTagSupport::CustomTagSupport { &self.base }
    fn as_CustomTagSupport_mut(&mut self) -> &mut crate::tags::CustomTagSupport::CustomTagSupport { &mut self.base }
}


