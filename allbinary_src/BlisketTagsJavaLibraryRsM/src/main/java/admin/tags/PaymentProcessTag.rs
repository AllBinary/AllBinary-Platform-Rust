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
use crate::admin::taghelpers::PaymentProcessHelperFactory::PaymentProcessHelperFactory;
use crate::org::allbinary::logic::communication::http::request::AbResponseHandler::AbResponseHandler;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::system::security::licensing::LicensingException::LicensingException;
use crate::tags::StoreValidationTransformTag::StoreValidationTransformTag;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct PaymentProcessTag {
    pub base: StoreValidationTransformTag,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
}


impl PaymentProcessTag {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: StoreValidationTransformTag::new_struct(),
            logUtil: None,
        };
        this.logUtil = LogUtil::getInstance();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    // throws LicensingException, Exception
    pub fn process(&mut self) -> String {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = jcall!(PaymentProcessHelperFactory::new()).getInstance(self.getPropertiesHashMap(), self.pageContext.clone());
            let mut helperClass: &'static str = std::any::type_name_of_val(&anyType);
            let mut method: Option<Rc<RefCell<Method>>> = jcall!(helperClass).getMethod(jcall!(self.base.commonStrings).PROCESS.clone(), None);
            let mut result: String = (jcall!(method).invoke(anyType.clone(), None)).clone() /* as String */;
            return result.clone();
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).LICENSINGERROR.clone()) {
                jcall!(self.logUtil).put(String::from("LicensingException"), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).PROCESS.clone(), crate::java_into_object(e.clone()));
            }
            panic!("{:?}", e);
            }
        }
    }
    

    // throws JspTagException
    pub fn doStartTag(&mut self) -> i32 {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            if jcall!(self.getCommand()).compareTo(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).globals).GLOBALS2).AUTHORIZEORDEREVALBODYONERROR.clone()) == 0 || jcall!(self.getCommand()).compareTo(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).globals).GLOBALS2).AUTHORIZEORDERANDEVALBODY.clone()) == 0 || jcall!(self.getCommand()).compareTo(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).globals).GLOBALS2).AUTHORIZEFORMEVALBODYONERROR.clone()) == 0 {
                jcall!(jcall!(self.pageContext).getOut()).print(format!("{}{}", self.process(), String::from("<br />")));
                if jcall!(self.getCommand()).compareTo(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).globals).GLOBALS2).AUTHORIZEORDEREVALBODYONERROR.clone()) == 0 {
                    return self.SKIP_BODY.clone();
                } else {
                    return self.EVAL_BODY_INCLUDE.clone();
                }
            } else {
                panic!("{:?}", format!("{}: {:?}", "Exception", (String::from("No Such Payment Processing Command"))));
            }
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            AbResponseHandler::sendJspTagRedirect(self.pageContext.clone(), e.clone());
            return SKIP_BODY.clone();
            }
        }
    }
}
impl std::ops::Deref for PaymentProcessTag {
    type Target = StoreValidationTransformTag;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for PaymentProcessTag {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::tags::CommandTag::CommandTagDyn for PaymentProcessTag {
    fn as_CommandTag(&self) -> &crate::tags::CommandTag::CommandTag { &self.base.base.base.base.base.base.base.base }
    fn as_CommandTag_mut(&mut self) -> &mut crate::tags::CommandTag::CommandTag { &mut self.base.base.base.base.base.base.base.base }
}
impl crate::tags::CustomTagSupport::CustomTagSupportDyn for PaymentProcessTag {
    fn as_CustomTagSupport(&self) -> &crate::tags::CustomTagSupport::CustomTagSupport { &self.base.base.base.base.base.base.base.base.base }
    fn as_CustomTagSupport_mut(&mut self) -> &mut crate::tags::CustomTagSupport::CustomTagSupport { &mut self.base.base.base.base.base.base.base.base.base }
}
impl crate::tags::HelperTag::HelperTagDyn for PaymentProcessTag {
    fn as_HelperTag(&self) -> &crate::tags::HelperTag::HelperTag { &self.base.base.base.base.base.base }
    fn as_HelperTag_mut(&mut self) -> &mut crate::tags::HelperTag::HelperTag { &mut self.base.base.base.base.base.base }
}
impl crate::tags::PropertiesTag::PropertiesTagDyn for PaymentProcessTag {
    fn as_PropertiesTag(&self) -> &crate::tags::PropertiesTag::PropertiesTag { &self.base.base.base.base.base.base.base }
    fn as_PropertiesTag_mut(&mut self) -> &mut crate::tags::PropertiesTag::PropertiesTag { &mut self.base.base.base.base.base.base.base }
    fn doEndTag(&mut self) -> i32 { crate::tags::ValidationTransformTag::ValidationTransformTag::doEndTag(crate::tags::ValidationTransformTag::ValidationTransformTagDyn::as_ValidationTransformTag_mut(self)) }
}
impl crate::tags::ModifyTag::ModifyTagDyn for PaymentProcessTag {
    fn as_ModifyTag(&self) -> &crate::tags::ModifyTag::ModifyTag { &self.base.base.base.base.base }
    fn as_ModifyTag_mut(&mut self) -> &mut crate::tags::ModifyTag::ModifyTag { &mut self.base.base.base.base.base }
    fn doStartTag(&mut self) -> i32 { Self::doStartTag(self) }
}
impl crate::tags::StoreValidationTransformTag::StoreValidationTransformTagDyn for PaymentProcessTag {
    fn as_StoreValidationTransformTag(&self) -> &crate::tags::StoreValidationTransformTag::StoreValidationTransformTag { &self.base }
    fn as_StoreValidationTransformTag_mut(&mut self) -> &mut crate::tags::StoreValidationTransformTag::StoreValidationTransformTag { &mut self.base }
}
impl crate::tags::TransformTag::TransformTagDyn for PaymentProcessTag {
    fn as_TransformTag(&self) -> &crate::tags::TransformTag::TransformTag { &self.base.base.base }
    fn as_TransformTag_mut(&mut self) -> &mut crate::tags::TransformTag::TransformTag { &mut self.base.base.base }
}
impl crate::tags::ValidationTransformTag::ValidationTransformTagDyn for PaymentProcessTag {
    fn as_ValidationTransformTag(&self) -> &crate::tags::ValidationTransformTag::ValidationTransformTag { &self.base.base }
    fn as_ValidationTransformTag_mut(&mut self) -> &mut crate::tags::ValidationTransformTag::ValidationTransformTag { &mut self.base.base }
}
impl crate::tags::transform::info::TransformInfoTag::TransformInfoTagDyn for PaymentProcessTag {
    fn as_TransformInfoTag(&self) -> &crate::tags::transform::info::TransformInfoTag::TransformInfoTag { &self.base.base.base.base }
    fn as_TransformInfoTag_mut(&mut self) -> &mut crate::tags::transform::info::TransformInfoTag::TransformInfoTag { &mut self.base.base.base.base }
    fn setStoreName(&mut self, p1: String) { crate::tags::StoreValidationTransformTag::StoreValidationTransformTag::setStoreName(crate::tags::StoreValidationTransformTag::StoreValidationTransformTagDyn::as_StoreValidationTransformTag_mut(self), p1) }
}


