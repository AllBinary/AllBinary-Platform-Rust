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
use crate::admin::taghelpers::PaymentGatewayHelperFactory::PaymentGatewayHelperFactory;
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

pub struct PaymentGatewayTag {
    pub base: TableTag,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
}


impl PaymentGatewayTag {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: TableTag::new_struct(),
            logUtil: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.setTagHelperFactory(PaymentGatewayHelperFactory::new());
        this.setTagRequestHelperFactory(PaymentGatewayHelperFactory::new());
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    // throws LicensingException, Exception
    pub fn process(&mut self) -> String {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = jcall!(PaymentGatewayHelperFactory::new()).getInstance(self.getPropertiesHashMap(), self.pageContext.clone());
            let mut helperClass: &'static str = std::any::type_name_of_val(&anyType);
            let mut method: Option<Rc<RefCell<Method>>> = jcall!(helperClass).getMethod(jcall!(self.base.commonStrings).PROCESS.clone(), None);
            let mut result: String = (jcall!(method).invoke(anyType.clone(), None)).clone() /* as String */;
            return result.clone();
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            jcall!(self.logUtil).put(String::from("LicensingException"), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).PROCESS.clone(), crate::java_into_object(e.clone()));
            panic!("{:?}", e);
            }
        }
    }
    

    // throws JspTagException
    pub fn doStartTag(&mut self) -> i32 {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            if self.isEnabled() {
                if jcall!(self.getCommand()).compareTo(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).globals).GLOBALS2).AUTHORIZEORDEREVALBODYONERROR.clone()) == 0 || jcall!(self.getCommand()).compareTo(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).globals).GLOBALS2).AUTHORIZEORDERANDEVALBODY.clone()) == 0 || jcall!(self.getCommand()).compareTo(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).globals).GLOBALS2).AUTHORIZEFORMEVALBODYONERROR.clone()) == 0 {
                    let mut output: String = self.process();
                    {
                        jcall!(jcall!(self.pageContext).getOut()).print(format!("{}{}", output, String::from("<br />")));
                    }
                    if jcall!(self.getCommand()).compareTo(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).globals).GLOBALS2).AUTHORIZEORDEREVALBODYONERROR.clone()) == 0 {
                        return self.SKIP_BODY.clone();
                    } else {
                        return self.EVAL_BODY_INCLUDE.clone();
                    }
                } else {
                    return self.base.doStartTag();
                }
            }
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
impl std::ops::Deref for PaymentGatewayTag {
    type Target = TableTag;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for PaymentGatewayTag {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::admin::tags::TableTag::TableTagDyn for PaymentGatewayTag {
    fn as_TableTag(&self) -> &crate::admin::tags::TableTag::TableTag { &self.base }
    fn as_TableTag_mut(&mut self) -> &mut crate::admin::tags::TableTag::TableTag { &mut self.base }
    fn doStartTag(&mut self) -> i32 { Self::doStartTag(self) }
}
impl crate::tags::CommandTag::CommandTagDyn for PaymentGatewayTag {
    fn as_CommandTag(&self) -> &crate::tags::CommandTag::CommandTag { &self.base.base.base }
    fn as_CommandTag_mut(&mut self) -> &mut crate::tags::CommandTag::CommandTag { &mut self.base.base.base }
}
impl crate::tags::CustomTagSupport::CustomTagSupportDyn for PaymentGatewayTag {
    fn as_CustomTagSupport(&self) -> &crate::tags::CustomTagSupport::CustomTagSupport { &self.base.base.base.base }
    fn as_CustomTagSupport_mut(&mut self) -> &mut crate::tags::CustomTagSupport::CustomTagSupport { &mut self.base.base.base.base }
}
impl crate::tags::PropertiesTag::PropertiesTagDyn for PaymentGatewayTag {
    fn as_PropertiesTag(&self) -> &crate::tags::PropertiesTag::PropertiesTag { &self.base.base }
    fn as_PropertiesTag_mut(&mut self) -> &mut crate::tags::PropertiesTag::PropertiesTag { &mut self.base.base }
}


