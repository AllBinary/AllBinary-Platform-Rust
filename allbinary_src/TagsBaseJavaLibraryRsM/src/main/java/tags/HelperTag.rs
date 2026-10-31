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
use crate::admin::taghelpers::TagHelperFactoryInterface::TagHelperFactoryInterface;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use super::PropertiesTag::PropertiesTag;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct HelperTag {
    pub base: PropertiesTag,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub tagHelperFactoryInterface: Option<Rc<RefCell<dyn TagHelperFactoryInterface>>>,
    pub anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>>,
}


impl HelperTag {
    

    pub fn new_struct(mut tagHelperFactoryInterface: Option<Rc<RefCell<dyn TagHelperFactoryInterface>>>) -> Self {
        let mut this = Self {
            base: PropertiesTag::new_struct(),
            logUtil: None,
            tagHelperFactoryInterface: None,
            anyType: None,
        };
        this.logUtil = LogUtil::getInstance();
        if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).JSPTAG.clone()) {
            jcall!(this.logUtil).putF(jcall!(this.commonStrings).START.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(this.commonStrings).CONSTRUCTOR.clone());
        }
        this.tagHelperFactoryInterface = tagHelperFactoryInterface.clone();
        this
    }

    pub fn new(mut tagHelperFactoryInterface: Option<Rc<RefCell<dyn TagHelperFactoryInterface>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(tagHelperFactoryInterface))))
    }
    

    // throws Exception
    pub fn setHelper(&mut self) {
        if self.anyType.is_none() {
            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).JSPTAG.clone()) {
                { let java_receiver = self.logUtil.clone(); let java_arguments = (format!("{}{}", String::from("Creating TagHelper with: \n"), jcall!(self.getPropertiesHashMap()).toString()), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("doStartTag"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
            }
            self.anyType = { let java_receiver = self.tagHelperFactoryInterface.clone(); let java_arguments = (self.getPropertiesHashMap(), self.pageContext.clone(),); let java_result = jcall!(java_receiver).getInstance(java_arguments.0, java_arguments.1); java_result };
        }
    }
    

    pub fn getHelper(&mut self) -> Option<Rc<RefCell<Box<dyn std::any::Any>>>> {
        return self.anyType.clone();
    }
    

    pub fn doEndTag(&mut self) -> i32 {
        if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).JSPTAG.clone()) {
            jcall!(self.logUtil).putF(String::from("Tag Ended"), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("doEndTag"));
        }
        self.anyType = None;
        return self.base.doEndTag();
    }
}
impl std::ops::Deref for HelperTag {
    type Target = PropertiesTag;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for HelperTag {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

pub trait HelperTagDyn: crate::tags::PropertiesTag::PropertiesTagDyn {
    fn as_HelperTag(&self) -> &HelperTag;
    fn as_HelperTag_mut(&mut self) -> &mut HelperTag;
    fn setHelper(&mut self) { HelperTag::setHelper(self.as_HelperTag_mut()) }
    fn getHelper(&mut self) -> Option<Rc<RefCell<Box<dyn std::any::Any>>>> { HelperTag::getHelper(self.as_HelperTag_mut()) }
}
impl HelperTagDyn for HelperTag {
    fn as_HelperTag(&self) -> &HelperTag { self }
    fn as_HelperTag_mut(&mut self) -> &mut HelperTag { self }
}

impl crate::tags::CommandTag::CommandTagDyn for HelperTag {
    fn as_CommandTag(&self) -> &crate::tags::CommandTag::CommandTag { &self.base.base }
    fn as_CommandTag_mut(&mut self) -> &mut crate::tags::CommandTag::CommandTag { &mut self.base.base }
}
impl crate::tags::CustomTagSupport::CustomTagSupportDyn for HelperTag {
    fn as_CustomTagSupport(&self) -> &crate::tags::CustomTagSupport::CustomTagSupport { &self.base.base.base }
    fn as_CustomTagSupport_mut(&mut self) -> &mut crate::tags::CustomTagSupport::CustomTagSupport { &mut self.base.base.base }
}
impl crate::tags::PropertiesTag::PropertiesTagDyn for HelperTag {
    fn as_PropertiesTag(&self) -> &crate::tags::PropertiesTag::PropertiesTag { &self.base }
    fn as_PropertiesTag_mut(&mut self) -> &mut crate::tags::PropertiesTag::PropertiesTag { &mut self.base }
    fn doEndTag(&mut self) -> i32 { Self::doEndTag(self) }
}


