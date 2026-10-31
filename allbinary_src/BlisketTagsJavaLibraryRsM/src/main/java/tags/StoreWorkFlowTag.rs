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
// package tags
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::java::lang::reflect::Method::Method;
// use crate::javax::servlet::jsp::JspTagException::JspTagException;
use crate::org::allbinary::logic::communication::http::request::AbResponseHandler::AbResponseHandler;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::system::security::licensing::LicensingException::LicensingException;
use crate::org::allbinary::logic::visual::transform::info::TransformInfoData::TransformInfoData;
use crate::taghelpers::StoreTagWorkFlowHelperFactory::StoreTagWorkFlowHelperFactory;
use super::HelperTag::HelperTag;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

//and hidden field removal
pub struct StoreWorkFlowTag {
    pub base: HelperTag,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub viewName: String,
    pub viewFile: String,
}


impl StoreWorkFlowTag {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: HelperTag::new_struct(StoreTagWorkFlowHelperFactory::new()),
            logUtil: None,
            viewName: String::new(),
            viewFile: String::new(),
        };
        this.logUtil = LogUtil::getInstance();
        // super(...) is in the struct literal above
        if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).JSPTAG.clone()) {
            jcall!(this.logUtil).putF(jcall!(this.commonStrings).START.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(this.commonStrings).CONSTRUCTOR.clone());
        }
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn setName(&mut self, mut value: String) {
        self.viewName = value.clone();
        jcall!(self.getPropertiesHashMap()).put(jcall!(TransformInfoData::getInstance()).NAME.clone(), self.viewName.clone());
    }
    

    // throws Exception
    pub fn process(&mut self) -> i32 {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).JSPTAG.clone()) {
                jcall!(self.logUtil).putF(jcall!(self.commonStrings).START.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).PROCESS.clone());
            }
            let mut helperClass: &'static str = std::any::type_name_of_val(&self.getHelper());
            let mut method: Option<Rc<RefCell<Method>>> = jcall!(helperClass).getMethod(jcall!(self.base.commonStrings).PROCESS.clone(), None);
            let mut result: Option<i32> = (jcall!(method).invoke(self.getHelper(), None)).clone() /* as Integer */;
            return ((result).unwrap() as i32);
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).SQLTAGSERROR.clone()) {
                jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).PROCESS.clone(), crate::java_into_object(e.clone()));
            }
            panic!("{:?}", e);
            }
        }
    }
    

    // throws JspTagException
    pub fn doStartTag(&mut self) -> i32 {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).JSPTAG.clone()) {
                jcall!(self.logUtil).putF(jcall!(self.commonStrings).START.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.base.tagStrings).DO_START_TAG.clone());
            }
            self.setHelper();
            return self.process();
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
impl std::ops::Deref for StoreWorkFlowTag {
    type Target = HelperTag;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for StoreWorkFlowTag {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

pub trait StoreWorkFlowTagDyn: crate::tags::HelperTag::HelperTagDyn {
    fn as_StoreWorkFlowTag(&self) -> &StoreWorkFlowTag;
    fn as_StoreWorkFlowTag_mut(&mut self) -> &mut StoreWorkFlowTag;
    fn setName(&mut self, mut value: String) { StoreWorkFlowTag::setName(self.as_StoreWorkFlowTag_mut(), value) }
    fn process(&mut self) -> i32 { StoreWorkFlowTag::process(self.as_StoreWorkFlowTag_mut()) }
    fn doStartTag(&mut self) -> i32 { StoreWorkFlowTag::doStartTag(self.as_StoreWorkFlowTag_mut()) }
}
impl StoreWorkFlowTagDyn for StoreWorkFlowTag {
    fn as_StoreWorkFlowTag(&self) -> &StoreWorkFlowTag { self }
    fn as_StoreWorkFlowTag_mut(&mut self) -> &mut StoreWorkFlowTag { self }
}

impl crate::tags::CommandTag::CommandTagDyn for StoreWorkFlowTag {
    fn as_CommandTag(&self) -> &crate::tags::CommandTag::CommandTag { &self.base.base.base }
    fn as_CommandTag_mut(&mut self) -> &mut crate::tags::CommandTag::CommandTag { &mut self.base.base.base }
}
impl crate::tags::CustomTagSupport::CustomTagSupportDyn for StoreWorkFlowTag {
    fn as_CustomTagSupport(&self) -> &crate::tags::CustomTagSupport::CustomTagSupport { &self.base.base.base.base }
    fn as_CustomTagSupport_mut(&mut self) -> &mut crate::tags::CustomTagSupport::CustomTagSupport { &mut self.base.base.base.base }
}
impl crate::tags::HelperTag::HelperTagDyn for StoreWorkFlowTag {
    fn as_HelperTag(&self) -> &crate::tags::HelperTag::HelperTag { &self.base }
    fn as_HelperTag_mut(&mut self) -> &mut crate::tags::HelperTag::HelperTag { &mut self.base }
}
impl crate::tags::PropertiesTag::PropertiesTagDyn for StoreWorkFlowTag {
    fn as_PropertiesTag(&self) -> &crate::tags::PropertiesTag::PropertiesTag { &self.base.base }
    fn as_PropertiesTag_mut(&mut self) -> &mut crate::tags::PropertiesTag::PropertiesTag { &mut self.base.base }
    fn doEndTag(&mut self) -> i32 { crate::tags::HelperTag::HelperTag::doEndTag(crate::tags::HelperTag::HelperTagDyn::as_HelperTag_mut(self)) }
}


