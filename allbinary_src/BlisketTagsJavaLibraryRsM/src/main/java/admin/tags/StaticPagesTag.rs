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
use crate::admin::taghelpers::StaticPagesHelperFactory::StaticPagesHelperFactory;
use crate::admin::taghelpers::StaticPagesRequestHelperFactory::StaticPagesRequestHelperFactory;
use crate::org::allbinary::logic::communication::http::request::AbResponseHandler::AbResponseHandler;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::control::search::SearchData::SearchData;
use crate::org::allbinary::logic::system::security::licensing::LicensingException::LicensingException;
use crate::org::allbinary::logic::visual::transform::info::TransformInfoData::TransformInfoData;
use super::TableTag::TableTag;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct StaticPagesTag {
    pub base: TableTag,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub xslFile: String,
}


impl StaticPagesTag {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: TableTag::new_struct(),
            logUtil: None,
            xslFile: String::new(),
        };
        this.logUtil = LogUtil::getInstance();
        this.setTagHelperFactory(StaticPagesHelperFactory::new());
        this.setTagRequestHelperFactory(StaticPagesRequestHelperFactory::new());
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn setXsl(&mut self, mut value: String) {
        self.xslFile = value.clone();
    }
    

    // throws LicensingException
    pub fn generateStaticPages(&mut self) -> String {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = jcall!(StaticPagesRequestHelperFactory::new()).getInstance(self.getPropertiesHashMap(), self.pageContext.clone());
            let mut helperClass: &'static str = std::any::type_name_of_val(&anyType);
            let mut method: Option<Rc<RefCell<Method>>> = jcall!(helperClass).getMethod(String::from("generateStaticPages"), None);
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
    pub fn makePublic(&mut self) -> String {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = jcall!(StaticPagesRequestHelperFactory::new()).getInstance(self.getPropertiesHashMap(), self.pageContext.clone());
            let mut helperClass: &'static str = std::any::type_name_of_val(&anyType);
            let mut method: Option<Rc<RefCell<Method>>> = jcall!(helperClass).getMethod(String::from("makePublic"), None);
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
    

    // throws JspTagException
    pub fn doStartTag(&mut self) -> i32 {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            if self.isEnabled() {
                if self.getCommand().is_some() {
                    if jcall!(self.getCommand()).compareTo(crate::org::allbinary::logic::control::search::SearchData::SearchData_GENERATESTATICPAGES.with(Clone::clone).clone()) == 0 {
                        jcall!(self.getPropertiesHashMap()).put(jcall!(TransformInfoData::getInstance()).TEMPLATEFILENAME.clone(), self.xslFile.clone());
                        let mut output: String = self.generateStaticPages();
                        if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).JSPTAGEXTRAOUTPUT.clone()) {
                            jcall!(jcall!(self.pageContext).getOut()).print(output.clone());
                        }
                    } else if jcall!(self.getCommand()).compareTo(crate::org::allbinary::logic::control::search::SearchData::SearchData_MAKEPUBLIC.with(Clone::clone).clone()) == 0 {
                        let mut output: String = self.makePublic();
                        if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).JSPTAGEXTRAOUTPUT.clone()) {
                            jcall!(jcall!(self.pageContext).getOut()).print(output.clone());
                        }
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
impl std::ops::Deref for StaticPagesTag {
    type Target = TableTag;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for StaticPagesTag {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::admin::tags::TableTag::TableTagDyn for StaticPagesTag {
    fn as_TableTag(&self) -> &crate::admin::tags::TableTag::TableTag { &self.base }
    fn as_TableTag_mut(&mut self) -> &mut crate::admin::tags::TableTag::TableTag { &mut self.base }
    fn doStartTag(&mut self) -> i32 { Self::doStartTag(self) }
}
impl crate::tags::CommandTag::CommandTagDyn for StaticPagesTag {
    fn as_CommandTag(&self) -> &crate::tags::CommandTag::CommandTag { &self.base.base.base }
    fn as_CommandTag_mut(&mut self) -> &mut crate::tags::CommandTag::CommandTag { &mut self.base.base.base }
}
impl crate::tags::CustomTagSupport::CustomTagSupportDyn for StaticPagesTag {
    fn as_CustomTagSupport(&self) -> &crate::tags::CustomTagSupport::CustomTagSupport { &self.base.base.base.base }
    fn as_CustomTagSupport_mut(&mut self) -> &mut crate::tags::CustomTagSupport::CustomTagSupport { &mut self.base.base.base.base }
}
impl crate::tags::PropertiesTag::PropertiesTagDyn for StaticPagesTag {
    fn as_PropertiesTag(&self) -> &crate::tags::PropertiesTag::PropertiesTag { &self.base.base }
    fn as_PropertiesTag_mut(&mut self) -> &mut crate::tags::PropertiesTag::PropertiesTag { &mut self.base.base }
}


