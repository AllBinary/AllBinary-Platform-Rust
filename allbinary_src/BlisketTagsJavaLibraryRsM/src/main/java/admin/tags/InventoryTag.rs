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
// use crate::javax::servlet::jsp::JspTagException::JspTagException;
// use crate::javax::servlet::jsp::tagext::Tag::Tag;
use crate::admin::taghelpers::InventoryHelperFactory::InventoryHelperFactory;
use crate::admin::taghelpers::InventoryRequestHelperFactory::InventoryRequestHelperFactory;
use crate::org::allbinary::business::context::modules::storefront::StoreFrontData::StoreFrontData;
use crate::org::allbinary::logic::communication::http::request::AbResponseHandler::AbResponseHandler;
use crate::java::lang::Exception::*;
use super::TableTag::TableTag;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct InventoryTag {
    pub base: TableTag,
    pub storeName: String,
}


impl InventoryTag {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: TableTag::new_struct(),
            storeName: String::new(),
        };
        // super(...) is in the struct literal above
        this.setTagHelperFactory(InventoryHelperFactory::new().map(|reference| reference as Rc<RefCell<dyn crate::admin::taghelpers::TagHelperFactoryInterface::TagHelperFactoryInterface>>));
        this.setTagRequestHelperFactory(InventoryRequestHelperFactory::new().map(|reference| reference as Rc<RefCell<dyn crate::admin::taghelpers::TagHelperFactoryInterface::TagHelperFactoryInterface>>));
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn setStoreName(&mut self, mut value: String) {
        self.storeName = value.clone();
    }
    

    // throws JspTagException
    pub fn doStartTag(&mut self) -> i32 {
        let java_try_result = match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            if self.isEnabled() {
                if true {
                    if { let java_value = (self.getCommand().cmp(&{ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).globals.clone(); java_field }).GLOBALS2.clone(); java_field }).BACKUP.clone(); java_field }.clone()) as i32); java_value } != 0 && { let java_value = (self.getCommand().cmp(&{ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).globals.clone(); java_field }).GLOBALS2.clone(); java_field }).RESTORE.clone(); java_field }.clone()) as i32); java_value } != 0 && { let java_value = (self.getCommand().cmp(&{ let java_field = jcall!(self.commonStrings).CREATE.clone(); java_field }.clone()) as i32); java_value } != 0 && { let java_value = (self.getCommand().cmp(&{ let java_field = jcall!(self.commonStrings).DROP.clone(); java_field }.clone()) as i32); java_value } != 0 {
                        let mut parentTag: Option<Rc<RefCell<Tag>>> = self.getParent();
                        jcall!(ParentInventoryTagHelper::ParentInventoryTagHelper::getInstance()).isValid(None /* Java this: a &mut self method has no Rc of itself */, parentTag.clone());
                        jcall!(self.getPropertiesHashMap()).put(crate::java_into_object(AbTagData_PARENT.with(Clone::clone).clone()), crate::java_into_object(parentTag.clone()));
                        jcall!(self.getPropertiesHashMap()).put(crate::java_into_object({ let java_field = jcall!(StoreFrontData::getInstance()).NAME.clone(); java_field }.clone()), crate::java_into_object(self.storeName.clone()));
                    }
                    return Some(self.base.doStartTag());
                }
            }
            return Some(SKIP_BODY.clone());
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
impl std::ops::Deref for InventoryTag {
    type Target = TableTag;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for InventoryTag {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::admin::tags::TableTag::TableTagDyn for InventoryTag {
    fn as_TableTag(&self) -> &crate::admin::tags::TableTag::TableTag { &self.base }
    fn as_TableTag_mut(&mut self) -> &mut crate::admin::tags::TableTag::TableTag { &mut self.base }
    fn doStartTag(&mut self) -> i32 { Self::doStartTag(self) }
}
impl crate::tags::CommandTag::CommandTagDyn for InventoryTag {
    fn as_CommandTag(&self) -> &crate::tags::CommandTag::CommandTag { &self.base.base.base }
    fn as_CommandTag_mut(&mut self) -> &mut crate::tags::CommandTag::CommandTag { &mut self.base.base.base }
}
impl crate::tags::CustomTagSupport::CustomTagSupportDyn for InventoryTag {
    fn as_CustomTagSupport(&self) -> &crate::tags::CustomTagSupport::CustomTagSupport { &self.base.base.base.base }
    fn as_CustomTagSupport_mut(&mut self) -> &mut crate::tags::CustomTagSupport::CustomTagSupport { &mut self.base.base.base.base }
}
impl crate::tags::PropertiesTag::PropertiesTagDyn for InventoryTag {
    fn as_PropertiesTag(&self) -> &crate::tags::PropertiesTag::PropertiesTag { &self.base.base }
    fn as_PropertiesTag_mut(&mut self) -> &mut crate::tags::PropertiesTag::PropertiesTag { &mut self.base.base }
}


