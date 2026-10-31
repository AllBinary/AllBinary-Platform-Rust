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
// package tags.workflow.processing.basic
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::javax::servlet::jsp::JspTagException::JspTagException;
use crate::org::allbinary::business::DynamicObjectData::DynamicObjectData;
use crate::org::allbinary::logic::communication::http::request::AbResponseHandler::AbResponseHandler;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::tags::StoreWorkFlowTag::StoreWorkFlowTag;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

//and remove the hidden fields
pub struct BasicWorkFlowTag {
    pub base: StoreWorkFlowTag,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
}


impl BasicWorkFlowTag {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: StoreWorkFlowTag::new_struct(),
            logUtil: None,
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
    

    // throws JspTagException
    pub fn doStartTag(&mut self) -> i32 {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).JSPTAG.clone()) {
                jcall!(self.logUtil).putF(jcall!(self.commonStrings).START.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.base.tagStrings).DO_START_TAG.clone());
            }
            jcall!(self.getPropertiesHashMap()).put(crate::org::allbinary::business::DynamicObjectData::DynamicObjectData_NAME.with(Clone::clone).clone(), String::from("workflows.template.data.BasicStoreWorkFlow"));
            return self.base.doStartTag();
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
impl std::ops::Deref for BasicWorkFlowTag {
    type Target = StoreWorkFlowTag;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for BasicWorkFlowTag {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::tags::StoreWorkFlowTag::StoreWorkFlowTagDyn for BasicWorkFlowTag {
    fn as_StoreWorkFlowTag(&self) -> &crate::tags::StoreWorkFlowTag::StoreWorkFlowTag { &self.base }
    fn as_StoreWorkFlowTag_mut(&mut self) -> &mut crate::tags::StoreWorkFlowTag::StoreWorkFlowTag { &mut self.base }
    fn doStartTag(&mut self) -> i32 { Self::doStartTag(self) }
}


