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
// use crate::javax::servlet::http::HttpServletRequest::HttpServletRequest;
// use crate::javax::servlet::jsp::JspTagException::JspTagException;
use crate::org::allbinary::logic::communication::http::request::AbResponseHandler::AbResponseHandler;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::system::security::licensing::LicensingException::LicensingException;
use crate::taghelpers::ValidatedViewHelperFactory::ValidatedViewHelperFactory;
use super::TransformTag::TransformTag;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct ValidationTransformTag {
    pub base: TransformTag,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub xslFile: String,
    pub logic: bool,
}


impl ValidationTransformTag {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: TransformTag::new_struct1(ValidatedViewHelperFactory::new()),
            logUtil: None,
            xslFile: String::new(),
            logic: false,
        };
        this.logUtil = LogUtil::getInstance();
        // super(...) is in the struct literal above
        this.logic = true;
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn setXsl(&mut self, mut value: String) {
        self.xslFile = value.clone();
        self.setTemplateFile(self.xslFile.clone());
    }
    

    pub fn setLogic(&mut self, mut logic: bool) {
        self.logic = logic.clone();
    }
    

    // throws Exception
    pub fn isValid(&mut self) -> bool {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut helperClass: &'static str = std::any::type_name_of_val(&self.getHelper());
            let mut method: Option<Rc<RefCell<Method>>> = jcall!(helperClass).getMethod(jcall!(self.base.commonStrings).IS_VALID.clone(), None);
            let mut result: Option<bool> = (jcall!(method).invoke(self.getHelper(), None)).clone() /* as Boolean */;
            return result.unwrap();
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).SQLTAGSERROR.clone()) {
                jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).IS_VALID.clone(), crate::java_into_object(e.clone()));
            }
            panic!("{:?}", e);
            }
        }
    }
    

    // throws Exception
    pub fn validationInfo(&mut self) -> String {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut addressHelperClass: &'static str = std::any::type_name_of_val(&self.getHelper());
            let mut method: Option<Rc<RefCell<Method>>> = jcall!(addressHelperClass).getMethod(String::from("validationInfo"), None);
            let mut result: String = (jcall!(method).invoke(self.getHelper(), None)).clone() /* as String */;
            return result.clone();
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).SQLTAGSERROR.clone()) {
                jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("validationInfo()"), crate::java_into_object(e.clone()));
            }
            panic!("{:?}", e);
            }
        }
    }
    

    // throws JspTagException
    pub fn doStartTag(&mut self) -> i32 {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).JSPTAG.clone()) {
                let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
                let mut request: Option<Rc<RefCell<HttpServletRequest>>> = (jcall!(self.pageContext).getRequest()).clone() /* as HttpServletRequest */;
                jcall!(stringBuffer).append(String::from("ValidationViewTag Start For: "));
                jcall!(stringBuffer).append(self.getName());
                jcall!(stringBuffer).append(String::from("\nView FIle: "));
                jcall!(stringBuffer).append(self.getObjectFile());
                jcall!(stringBuffer).append(String::from("\nRequest URI: "));
                jcall!(stringBuffer).append(jcall!(request).getRequestURI());
                { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(stringBuffer).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.base.tagStrings).DO_START_TAG.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
            }
            self.setHelper();
            if self.isValid() {
                self.base.doStartTag();
                if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).JSPTAG.clone()) {
                    let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
                    jcall!(stringBuffer).append(String::from("View File: "));
                    jcall!(stringBuffer).append(self.getObjectFile());
                    jcall!(stringBuffer).append(String::from("\nLogic includes body if true="));
                    jcall!(stringBuffer).appendboolean(self.logic.clone());
                    { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(stringBuffer).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.base.tagStrings).DO_START_TAG.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
                }
                if self.logic {
                    return EVAL_BODY_INCLUDE.clone();
                } else {
                    return SKIP_BODY.clone();
                }
            } else {
                jcall!(jcall!(self.base.pageContext).getOut()).print(self.validationInfo());
                if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).JSPTAG.clone()) {
                    let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
                    jcall!(stringBuffer).append(String::from("View File: "));
                    jcall!(stringBuffer).append(self.getObjectFile());
                    jcall!(stringBuffer).append(String::from("\nisValid()=false"));
                    jcall!(stringBuffer).append(String::from("\nLogic skips body if true="));
                    jcall!(stringBuffer).appendboolean(self.logic.clone());
                    { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(stringBuffer).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.base.tagStrings).DO_START_TAG.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
                }
                if self.logic {
                    return SKIP_BODY.clone();
                } else {
                    return EVAL_BODY_INCLUDE.clone();
                }
            }
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            AbResponseHandler::sendJspTagLicensingRedirect(self.pageContext.clone(), e.clone());
            return SKIP_BODY.clone();
            }
        }
    }
    

    pub fn doEndTag(&mut self) -> i32 {
        if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).JSPTAG.clone()) {
            jcall!(self.logUtil).putF(String::from("Tag Ended"), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("doEndTag"));
        }
        self.logic = true;
        return self.base.doEndTag();
    }
}
impl std::ops::Deref for ValidationTransformTag {
    type Target = TransformTag;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for ValidationTransformTag {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

pub trait ValidationTransformTagDyn: crate::tags::TransformTag::TransformTagDyn {
    fn as_ValidationTransformTag(&self) -> &ValidationTransformTag;
    fn as_ValidationTransformTag_mut(&mut self) -> &mut ValidationTransformTag;
    fn setXsl(&mut self, mut value: String) { ValidationTransformTag::setXsl(self.as_ValidationTransformTag_mut(), value) }
    fn setLogic(&mut self, mut logic: bool) { ValidationTransformTag::setLogic(self.as_ValidationTransformTag_mut(), logic) }
    fn isValid(&mut self) -> bool { ValidationTransformTag::isValid(self.as_ValidationTransformTag_mut()) }
    fn validationInfo(&mut self) -> String { ValidationTransformTag::validationInfo(self.as_ValidationTransformTag_mut()) }
}
impl ValidationTransformTagDyn for ValidationTransformTag {
    fn as_ValidationTransformTag(&self) -> &ValidationTransformTag { self }
    fn as_ValidationTransformTag_mut(&mut self) -> &mut ValidationTransformTag { self }
}

impl crate::tags::ModifyTag::ModifyTagDyn for ValidationTransformTag {
    fn as_ModifyTag(&self) -> &crate::tags::ModifyTag::ModifyTag { &self.base.base.base }
    fn as_ModifyTag_mut(&mut self) -> &mut crate::tags::ModifyTag::ModifyTag { &mut self.base.base.base }
    fn doStartTag(&mut self) -> i32 { Self::doStartTag(self) }
}
impl crate::tags::TransformTag::TransformTagDyn for ValidationTransformTag {
    fn as_TransformTag(&self) -> &crate::tags::TransformTag::TransformTag { &self.base }
    fn as_TransformTag_mut(&mut self) -> &mut crate::tags::TransformTag::TransformTag { &mut self.base }
}
impl crate::tags::CommandTag::CommandTagDyn for ValidationTransformTag {
    fn as_CommandTag(&self) -> &crate::tags::CommandTag::CommandTag { &self.base.base.base.base.base.base }
    fn as_CommandTag_mut(&mut self) -> &mut crate::tags::CommandTag::CommandTag { &mut self.base.base.base.base.base.base }
}
impl crate::tags::CustomTagSupport::CustomTagSupportDyn for ValidationTransformTag {
    fn as_CustomTagSupport(&self) -> &crate::tags::CustomTagSupport::CustomTagSupport { &self.base.base.base.base.base.base.base }
    fn as_CustomTagSupport_mut(&mut self) -> &mut crate::tags::CustomTagSupport::CustomTagSupport { &mut self.base.base.base.base.base.base.base }
}
impl crate::tags::HelperTag::HelperTagDyn for ValidationTransformTag {
    fn as_HelperTag(&self) -> &crate::tags::HelperTag::HelperTag { &self.base.base.base.base }
    fn as_HelperTag_mut(&mut self) -> &mut crate::tags::HelperTag::HelperTag { &mut self.base.base.base.base }
}
impl crate::tags::PropertiesTag::PropertiesTagDyn for ValidationTransformTag {
    fn as_PropertiesTag(&self) -> &crate::tags::PropertiesTag::PropertiesTag { &self.base.base.base.base.base }
    fn as_PropertiesTag_mut(&mut self) -> &mut crate::tags::PropertiesTag::PropertiesTag { &mut self.base.base.base.base.base }
    fn doEndTag(&mut self) -> i32 { Self::doEndTag(self) }
}
impl crate::tags::transform::info::TransformInfoTag::TransformInfoTagDyn for ValidationTransformTag {
    fn as_TransformInfoTag(&self) -> &crate::tags::transform::info::TransformInfoTag::TransformInfoTag { &self.base.base }
    fn as_TransformInfoTag_mut(&mut self) -> &mut crate::tags::transform::info::TransformInfoTag::TransformInfoTag { &mut self.base.base }
}


