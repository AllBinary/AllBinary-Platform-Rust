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
// use crate::javax::servlet::jsp::JspTagException::JspTagException;
use crate::org::allbinary::business::context::modules::storefront::StoreFrontData::StoreFrontData;
use crate::org::allbinary::logic::communication::http::request::AbResponseHandler::AbResponseHandler;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use super::ValidationTransformTag::ValidationTransformTag;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct StoreValidationTransformTag {
    pub base: ValidationTransformTag,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub storeName: String,
}


impl StoreValidationTransformTag {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: ValidationTransformTag::new_struct(),
            logUtil: None,
            storeName: String::new(),
        };
        this.logUtil = LogUtil::getInstance();
        // super(...) is in the struct literal above
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn setStoreName(&mut self, mut value: String) {
        self.storeName = value.clone();
        jcall!(self.getPropertiesHashMap()).put(jcall!(StoreFrontData::getInstance()).NAME.clone(), self.storeName.clone());
    }
    

    // throws JspTagException
    pub fn doStartTag(&mut self) -> i32 {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).JSPTAG.clone()) {
                jcall!(self.logUtil).putF(String::from("Tag Start"), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.base.tagStrings).DO_START_TAG.clone());
            }
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
impl std::ops::Deref for StoreValidationTransformTag {
    type Target = ValidationTransformTag;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for StoreValidationTransformTag {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

pub trait StoreValidationTransformTagDyn: crate::tags::ValidationTransformTag::ValidationTransformTagDyn {
    fn as_StoreValidationTransformTag(&self) -> &StoreValidationTransformTag;
    fn as_StoreValidationTransformTag_mut(&mut self) -> &mut StoreValidationTransformTag;
}
impl StoreValidationTransformTagDyn for StoreValidationTransformTag {
    fn as_StoreValidationTransformTag(&self) -> &StoreValidationTransformTag { self }
    fn as_StoreValidationTransformTag_mut(&mut self) -> &mut StoreValidationTransformTag { self }
}

impl crate::tags::ModifyTag::ModifyTagDyn for StoreValidationTransformTag {
    fn as_ModifyTag(&self) -> &crate::tags::ModifyTag::ModifyTag { &self.base.base.base.base }
    fn as_ModifyTag_mut(&mut self) -> &mut crate::tags::ModifyTag::ModifyTag { &mut self.base.base.base.base }
    fn doStartTag(&mut self) -> i32 { Self::doStartTag(self) }
}
impl crate::tags::TransformTag::TransformTagDyn for StoreValidationTransformTag {
    fn as_TransformTag(&self) -> &crate::tags::TransformTag::TransformTag { &self.base.base }
    fn as_TransformTag_mut(&mut self) -> &mut crate::tags::TransformTag::TransformTag { &mut self.base.base }
}
impl crate::tags::ValidationTransformTag::ValidationTransformTagDyn for StoreValidationTransformTag {
    fn as_ValidationTransformTag(&self) -> &crate::tags::ValidationTransformTag::ValidationTransformTag { &self.base }
    fn as_ValidationTransformTag_mut(&mut self) -> &mut crate::tags::ValidationTransformTag::ValidationTransformTag { &mut self.base }
}
impl crate::tags::CommandTag::CommandTagDyn for StoreValidationTransformTag {
    fn as_CommandTag(&self) -> &crate::tags::CommandTag::CommandTag { &self.base.base.base.base.base.base.base }
    fn as_CommandTag_mut(&mut self) -> &mut crate::tags::CommandTag::CommandTag { &mut self.base.base.base.base.base.base.base }
}
impl crate::tags::CustomTagSupport::CustomTagSupportDyn for StoreValidationTransformTag {
    fn as_CustomTagSupport(&self) -> &crate::tags::CustomTagSupport::CustomTagSupport { &self.base.base.base.base.base.base.base.base }
    fn as_CustomTagSupport_mut(&mut self) -> &mut crate::tags::CustomTagSupport::CustomTagSupport { &mut self.base.base.base.base.base.base.base.base }
}
impl crate::tags::HelperTag::HelperTagDyn for StoreValidationTransformTag {
    fn as_HelperTag(&self) -> &crate::tags::HelperTag::HelperTag { &self.base.base.base.base.base }
    fn as_HelperTag_mut(&mut self) -> &mut crate::tags::HelperTag::HelperTag { &mut self.base.base.base.base.base }
}
impl crate::tags::PropertiesTag::PropertiesTagDyn for StoreValidationTransformTag {
    fn as_PropertiesTag(&self) -> &crate::tags::PropertiesTag::PropertiesTag { &self.base.base.base.base.base.base }
    fn as_PropertiesTag_mut(&mut self) -> &mut crate::tags::PropertiesTag::PropertiesTag { &mut self.base.base.base.base.base.base }
    fn doEndTag(&mut self) -> i32 { crate::tags::ValidationTransformTag::ValidationTransformTag::doEndTag(crate::tags::ValidationTransformTag::ValidationTransformTagDyn::as_ValidationTransformTag_mut(self)) }
}
impl crate::tags::transform::info::TransformInfoTag::TransformInfoTagDyn for StoreValidationTransformTag {
    fn as_TransformInfoTag(&self) -> &crate::tags::transform::info::TransformInfoTag::TransformInfoTag { &self.base.base.base }
    fn as_TransformInfoTag_mut(&mut self) -> &mut crate::tags::transform::info::TransformInfoTag::TransformInfoTag { &mut self.base.base.base }
    fn setStoreName(&mut self, mut value: String) { Self::setStoreName(self, value) }
}


