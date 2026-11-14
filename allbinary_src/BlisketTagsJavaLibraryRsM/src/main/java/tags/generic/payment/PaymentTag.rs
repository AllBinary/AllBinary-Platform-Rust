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
// package tags.generic.payment
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::javax::servlet::jsp::JspTagException::JspTagException;
use crate::org::allbinary::business::user::commerce::money::payment::PaymentData::PaymentData;
use crate::org::allbinary::logic::communication::http::request::AbResponseHandler::AbResponseHandler;
use crate::org::allbinary::logic::system::security::licensing::LicensingException::LicensingException;
use crate::tags::StoreValidationTransformTag::StoreValidationTransformTag;
use crate::java::lang::Exception::*;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct PaymentTag {
    pub base: StoreValidationTransformTag,
}


impl PaymentTag {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: StoreValidationTransformTag::new_struct(),
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    // throws JspTagException
    pub fn doStartTag(&mut self) -> i32 {
        let java_try_result = match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            self.setName(String::from("Basic Payment View"));
            self.setObjectFile(String::from("views.generic.payment.ValidationView"));
            if true {
                if { let java_value = (self.getCommand().cmp(&crate::org::allbinary::business::user::commerce::money::payment::PaymentData::PaymentData_VIEW.with(Clone::clone).clone()) as i32); java_value } == 0 || { let java_value = (self.getCommand().cmp(&crate::org::allbinary::business::user::commerce::money::payment::PaymentData::PaymentData_CHANGE.with(Clone::clone).clone()) as i32); java_value } == 0 || { let java_value = (self.getCommand().cmp(&crate::org::allbinary::business::user::commerce::money::payment::PaymentData::PaymentData_ADDNEW.with(Clone::clone).clone()) as i32); java_value } == 0 {
                } else if { let java_value = (self.getCommand().cmp(&{ let java_field = jcall!(self.commonStrings).INSERT.clone(); java_field }.clone()) as i32); java_value } == 0 {
                } else if { let java_value = (self.getCommand().cmp(&{ let java_field = jcall!(self.commonStrings).DELETE.clone(); java_field }.clone()) as i32); java_value } == 0 {
                    return Some(self.EVAL_BODY_INCLUDE.clone());
                } else if { let java_value = (self.getCommand().cmp(&crate::org::allbinary::business::user::commerce::money::payment::PaymentData::PaymentData_SELECT.with(Clone::clone).clone()) as i32); java_value } == 0 {
                    return Some(self.EVAL_BODY_INCLUDE.clone());
                } else {
                    panic!("{:?}", format!("{}: {:?}", "Exception", (format!("{}{}", String::from("No Such View Command: "), self.getCommand()))));
                }
                return Some(self.base.doStartTag());
            }
            panic!("{:?}", format!("{}: {:?}", "Exception", (String::from("Command Null"))));
        })) {
            Ok(java_return) => java_return,
            Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            AbResponseHandler::sendJspTagLicensingRedirect(self.pageContext.clone(), e.clone());
            return SKIP_BODY.clone();
            None
            }
        };if let Some(java_return) = java_try_result { return java_return; }unreachable!("try statement")
    }
}
impl std::ops::Deref for PaymentTag {
    type Target = StoreValidationTransformTag;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for PaymentTag {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::tags::ModifyTag::ModifyTagDyn for PaymentTag {
    fn as_ModifyTag(&self) -> &crate::tags::ModifyTag::ModifyTag { &self.base.base.base.base.base }
    fn as_ModifyTag_mut(&mut self) -> &mut crate::tags::ModifyTag::ModifyTag { &mut self.base.base.base.base.base }
    fn doStartTag(&mut self) -> i32 { Self::doStartTag(self) }
}
impl crate::tags::StoreValidationTransformTag::StoreValidationTransformTagDyn for PaymentTag {
    fn as_StoreValidationTransformTag(&self) -> &crate::tags::StoreValidationTransformTag::StoreValidationTransformTag { &self.base }
    fn as_StoreValidationTransformTag_mut(&mut self) -> &mut crate::tags::StoreValidationTransformTag::StoreValidationTransformTag { &mut self.base }
}
impl crate::tags::TransformTag::TransformTagDyn for PaymentTag {
    fn as_TransformTag(&self) -> &crate::tags::TransformTag::TransformTag { &self.base.base.base }
    fn as_TransformTag_mut(&mut self) -> &mut crate::tags::TransformTag::TransformTag { &mut self.base.base.base }
}
impl crate::tags::ValidationTransformTag::ValidationTransformTagDyn for PaymentTag {
    fn as_ValidationTransformTag(&self) -> &crate::tags::ValidationTransformTag::ValidationTransformTag { &self.base.base }
    fn as_ValidationTransformTag_mut(&mut self) -> &mut crate::tags::ValidationTransformTag::ValidationTransformTag { &mut self.base.base }
}
impl crate::tags::CommandTag::CommandTagDyn for PaymentTag {
    fn as_CommandTag(&self) -> &crate::tags::CommandTag::CommandTag { &self.base.base.base.base.base.base.base.base }
    fn as_CommandTag_mut(&mut self) -> &mut crate::tags::CommandTag::CommandTag { &mut self.base.base.base.base.base.base.base.base }
}
impl crate::tags::CustomTagSupport::CustomTagSupportDyn for PaymentTag {
    fn as_CustomTagSupport(&self) -> &crate::tags::CustomTagSupport::CustomTagSupport { &self.base.base.base.base.base.base.base.base.base }
    fn as_CustomTagSupport_mut(&mut self) -> &mut crate::tags::CustomTagSupport::CustomTagSupport { &mut self.base.base.base.base.base.base.base.base.base }
}
impl crate::tags::HelperTag::HelperTagDyn for PaymentTag {
    fn as_HelperTag(&self) -> &crate::tags::HelperTag::HelperTag { &self.base.base.base.base.base.base }
    fn as_HelperTag_mut(&mut self) -> &mut crate::tags::HelperTag::HelperTag { &mut self.base.base.base.base.base.base }
}
impl crate::tags::PropertiesTag::PropertiesTagDyn for PaymentTag {
    fn as_PropertiesTag(&self) -> &crate::tags::PropertiesTag::PropertiesTag { &self.base.base.base.base.base.base.base }
    fn as_PropertiesTag_mut(&mut self) -> &mut crate::tags::PropertiesTag::PropertiesTag { &mut self.base.base.base.base.base.base.base }
    fn doEndTag(&mut self) -> i32 { crate::tags::ValidationTransformTag::ValidationTransformTag::doEndTag(crate::tags::ValidationTransformTag::ValidationTransformTagDyn::as_ValidationTransformTag_mut(self)) }
}
impl crate::tags::transform::info::TransformInfoTag::TransformInfoTagDyn for PaymentTag {
    fn as_TransformInfoTag(&self) -> &crate::tags::transform::info::TransformInfoTag::TransformInfoTag { &self.base.base.base.base }
    fn as_TransformInfoTag_mut(&mut self) -> &mut crate::tags::transform::info::TransformInfoTag::TransformInfoTag { &mut self.base.base.base.base }
    fn setStoreName(&mut self, p1: String) { crate::tags::StoreValidationTransformTag::StoreValidationTransformTag::setStoreName(crate::tags::StoreValidationTransformTag::StoreValidationTransformTagDyn::as_StoreValidationTransformTag_mut(self), p1) }
}


