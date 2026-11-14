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
use crate::java::lang::Exception::*;

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
        if jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypes.clone(); java_field }).LOGGING.clone(); java_field }).contains({ let java_field = jcall!(jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypeFactory.clone(); java_field }).getInstance()).JSPTAG.clone(); java_field }.clone()) {
            jcall!(this.logUtil).putF({ let java_field = jcall!(this.commonStrings).START.clone(); java_field }.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(this.commonStrings).CONSTRUCTOR.clone(); java_field }.clone());
        }
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    // throws JspTagException
    pub fn doStartTag(&mut self) -> i32 {
        let java_try_result = match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            if jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypes.clone(); java_field }).LOGGING.clone(); java_field }).contains({ let java_field = jcall!(jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypeFactory.clone(); java_field }).getInstance()).JSPTAG.clone(); java_field }.clone()) {
                jcall!(self.logUtil).putF({ let java_field = jcall!(self.commonStrings).START.clone(); java_field }.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(self.base.tagStrings).DO_START_TAG.clone(); java_field }.clone());
            }
            jcall!(self.getPropertiesHashMap()).put(crate::java_into_object(crate::org::allbinary::business::DynamicObjectData::DynamicObjectData_NAME.with(Clone::clone).clone()), crate::java_into_object(String::from("workflows.template.data.BasicStoreWorkFlow")));
            return Some(self.base.doStartTag());
        })) {
            Ok(java_return) => java_return,
            Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            AbResponseHandler::sendJspTagRedirect(self.pageContext.clone(), e.clone());
            return SKIP_BODY.clone();
            None
            }
        };if let Some(java_return) = java_try_result { return java_return; }unreachable!("try statement")
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
impl crate::tags::CommandTag::CommandTagDyn for BasicWorkFlowTag {
    fn as_CommandTag(&self) -> &crate::tags::CommandTag::CommandTag { &self.base.base.base.base }
    fn as_CommandTag_mut(&mut self) -> &mut crate::tags::CommandTag::CommandTag { &mut self.base.base.base.base }
}
impl crate::tags::CustomTagSupport::CustomTagSupportDyn for BasicWorkFlowTag {
    fn as_CustomTagSupport(&self) -> &crate::tags::CustomTagSupport::CustomTagSupport { &self.base.base.base.base.base }
    fn as_CustomTagSupport_mut(&mut self) -> &mut crate::tags::CustomTagSupport::CustomTagSupport { &mut self.base.base.base.base.base }
}
impl crate::tags::HelperTag::HelperTagDyn for BasicWorkFlowTag {
    fn as_HelperTag(&self) -> &crate::tags::HelperTag::HelperTag { &self.base.base }
    fn as_HelperTag_mut(&mut self) -> &mut crate::tags::HelperTag::HelperTag { &mut self.base.base }
}
impl crate::tags::PropertiesTag::PropertiesTagDyn for BasicWorkFlowTag {
    fn as_PropertiesTag(&self) -> &crate::tags::PropertiesTag::PropertiesTag { &self.base.base.base }
    fn as_PropertiesTag_mut(&mut self) -> &mut crate::tags::PropertiesTag::PropertiesTag { &mut self.base.base.base }
    fn doEndTag(&mut self) -> i32 { crate::tags::HelperTag::HelperTag::doEndTag(crate::tags::HelperTag::HelperTagDyn::as_HelperTag_mut(self)) }
}


