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
use std::collections::HashMap;
use super::*;
// use crate::java::util::HashMap::HashMap;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
// use crate::javax::servlet::jsp::JspTagException::JspTagException;
// use crate::javax::servlet::jsp::tagext::Tag::Tag;
use crate::admin::taghelpers::MultipartRequestParamForwardHelper::MultipartRequestParamForwardHelper;
use crate::org::allbinary::logic::StdUtil::StdUtil;
use crate::org::allbinary::logic::communication::http::request::AbResponseHandler::AbResponseHandler;
use crate::org::allbinary::logic::system::security::licensing::LicensingException::LicensingException;
use crate::tags::CustomTagSupport::CustomTagSupport;
use crate::tags::HelperTag::HelperTag;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct MultipartRequestParamForwardTag {
    pub base: CustomTagSupport,
    pub page: String,
    pub command: String,
    pub paramVector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>,
}


impl MultipartRequestParamForwardTag {

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: CustomTagSupport::new_struct(),
            page: String::new(),
            command: String::new(),
            paramVector: None,
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn setPage(&mut self, mut page: String) {
        self.page = page.clone();
    }
    

    pub fn setCommand(&mut self, mut command: String) {
        self.command = command.clone();
    }
    

    pub fn setParamVector(&mut self, mut paramVector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) {
        self.paramVector = paramVector.clone();
    }
    

    // throws JspTagException
    pub fn doStartTag(&mut self) -> i32 {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut parentTag: Option<Rc<RefCell<dyn crate::tags::HelperTag::HelperTagDyn>>> = (self.getParent()).clone() /* as HelperTag */;
            jcall!(ParentInventoryTagHelper::ParentInventoryTagHelper::getInstance()).isValid(None /* Java this: a &mut self method has no Rc of itself */, parentTag.clone());
            let mut hashMap: Option<Rc<RefCell<HashMap>>> = jcall!(StdUtil::getInstance()).createHashMap();
            jcall!(hashMap).put(AbTagData_PARENT.with(Clone::clone).clone(), parentTag.clone());
            let mut multipartRequestParamForwardHelper: Option<Rc<RefCell<MultipartRequestParamForwardHelper>>> = MultipartRequestParamForwardHelper::new(parentTag.clone(), self.page.clone(), self.command.clone(), self.paramVector.clone(), self.pageContext.clone());
            jcall!(multipartRequestParamForwardHelper).forward();
            return unsafe { crate::javax::servlet::jsp::tagext::Tag::Tag_SKIP_BODY.clone() }.clone();
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            AbResponseHandler::sendJspTagLicensingRedirect(self.pageContext.clone(), e.clone());
            return unsafe { crate::javax::servlet::jsp::tagext::Tag::Tag_SKIP_BODY.clone() }.clone();
            }
        }
    }
}
impl std::ops::Deref for MultipartRequestParamForwardTag {
    type Target = CustomTagSupport;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for MultipartRequestParamForwardTag {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::tags::CustomTagSupport::CustomTagSupportDyn for MultipartRequestParamForwardTag {
    fn as_CustomTagSupport(&self) -> &crate::tags::CustomTagSupport::CustomTagSupport { &self.base }
    fn as_CustomTagSupport_mut(&mut self) -> &mut crate::tags::CustomTagSupport::CustomTagSupport { &mut self.base }
}


