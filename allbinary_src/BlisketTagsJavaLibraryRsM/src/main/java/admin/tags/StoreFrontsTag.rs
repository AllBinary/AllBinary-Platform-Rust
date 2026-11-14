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
// use crate::javax::servlet::jsp::tagext::TagSupport::TagSupport;
use crate::admin::taghelpers::StoreFrontsHelperFactory::StoreFrontsHelperFactory;
use crate::admin::taghelpers::StoreFrontsRequestHelperFactory::StoreFrontsRequestHelperFactory;
use crate::org::allbinary::business::context::modules::storefront::StoreFrontData::StoreFrontData;
use crate::org::allbinary::logic::communication::http::request::AbResponseHandler::AbResponseHandler;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::system::security::licensing::LicensingException::LicensingException;
use crate::java::lang::Exception::*;
use super::TableTag::TableTag;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct StoreFrontsTag {
    pub base: TableTag,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
}


impl StoreFrontsTag {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: TableTag::new_struct(),
            logUtil: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.setTagHelperFactory(StoreFrontsHelperFactory::new().map(|reference| reference as Rc<RefCell<dyn crate::admin::taghelpers::TagHelperFactoryInterface::TagHelperFactoryInterface>>));
        this.setTagRequestHelperFactory(StoreFrontsRequestHelperFactory::new().map(|reference| reference as Rc<RefCell<dyn crate::admin::taghelpers::TagHelperFactoryInterface::TagHelperFactoryInterface>>));
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    // throws LicensingException
    pub fn generateSelect(&mut self) -> String {
        let java_try_result = match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = jcall!(self.getTagHelperFactoryInterface()).getInstance(self.getPropertiesHashMap(), self.base.pageContext.clone());
            let mut method: Option<Rc<RefCell<Method>>> = jcall!(std::any::type_name_of_val(&anyType)).getMethod(String::from("generateSelect"), None);
            let mut result: String = (jcall!(method).invoke(anyType.clone(), None)).clone() /* as String */;
            return Some(result.clone());
        })) {
            Ok(java_return) => java_return,
            Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            panic!("{:?}", e);
            None
            }
        };if let Some(java_return) = java_try_result { return java_return; }unreachable!("try statement")
    }
    

    // throws LicensingException
    pub fn install(&mut self) -> String {
        let java_try_result = match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = jcall!(self.getTagRequestHelperFactoryInterface()).getInstance(self.getPropertiesHashMap(), self.base.pageContext.clone());
            let mut method: Option<Rc<RefCell<Method>>> = jcall!(std::any::type_name_of_val(&anyType)).getMethod(String::from("install"), None);
            let mut result: String = (jcall!(method).invoke(anyType.clone(), None)).clone() /* as String */;
            return Some(result.clone());
        })) {
            Ok(java_return) => java_return,
            Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            panic!("{:?}", e);
            None
            }
        };if let Some(java_return) = java_try_result { return java_return; }unreachable!("try statement")
    }
    

    // throws LicensingException
    pub fn sendStoreCreatedEmails(&mut self) -> String {
        let java_try_result = match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = jcall!(self.getTagRequestHelperFactoryInterface()).getInstance(self.getPropertiesHashMap(), self.base.pageContext.clone());
            let mut method: Option<Rc<RefCell<Method>>> = jcall!(std::any::type_name_of_val(&anyType)).getMethod(String::from("sendStoreCreatedEmails"), None);
            let mut result: String = (jcall!(method).invoke(anyType.clone(), None)).clone() /* as String */;
            return Some(result.clone());
        })) {
            Ok(java_return) => java_return,
            Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            panic!("{:?}", e);
            None
            }
        };if let Some(java_return) = java_try_result { return java_return; }unreachable!("try statement")
    }
    

    // throws JspTagException
    pub fn doStartTag(&mut self) -> i32 {
        let java_try_result = match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            if self.isEnabled() {
                if true {
                    let mut storeFrontData: Option<Rc<RefCell<StoreFrontData>>> = StoreFrontData::getInstance();
                    if { let java_value = (self.getCommand().cmp(&{ let java_field = jcall!(storeFrontData).SELECT.clone(); java_field }.clone()) as i32); java_value } == 0 {
                        jcall!(jcall!(self.pageContext).getOut()).print(self.generateSelect());
                    } else if { let java_value = (self.getCommand().cmp(&{ let java_field = jcall!(storeFrontData).INSTALL.clone(); java_field }.clone()) as i32); java_value } == 0 {
                        jcall!(self.getPropertiesHashMap()).put(crate::java_into_object(String::from("current")), crate::java_into_object(self.getCurrent()));
                        jcall!(self.getPropertiesHashMap()).put(crate::java_into_object(String::from("total")), crate::java_into_object(self.getTotal()));
                        if { let java_value = ((self.getCurrent()).unwrap() as i32); java_value } == { let java_value = ((self.getTotal()).unwrap() as i32); java_value } {
                            return Some(unsafe { crate::javax::servlet::jsp::tagext::TagSupport::TagSupport_EVAL_BODY_INCLUDE.clone() }.clone());
                        }
                        self.install();
                    } else if { let java_value = (self.getCommand().cmp(&{ let java_field = jcall!(storeFrontData).INSTALL_COMPLETE.clone(); java_field }.clone()) as i32); java_value } == 0 {
                        self.sendStoreCreatedEmails();
                    } else {
                        return Some(self.base.doStartTag());
                    }
                }
            }
            return Some(SKIP_BODY.clone());
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
impl std::ops::Deref for StoreFrontsTag {
    type Target = TableTag;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for StoreFrontsTag {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::admin::tags::TableTag::TableTagDyn for StoreFrontsTag {
    fn as_TableTag(&self) -> &crate::admin::tags::TableTag::TableTag { &self.base }
    fn as_TableTag_mut(&mut self) -> &mut crate::admin::tags::TableTag::TableTag { &mut self.base }
    fn doStartTag(&mut self) -> i32 { Self::doStartTag(self) }
}
impl crate::tags::CommandTag::CommandTagDyn for StoreFrontsTag {
    fn as_CommandTag(&self) -> &crate::tags::CommandTag::CommandTag { &self.base.base.base }
    fn as_CommandTag_mut(&mut self) -> &mut crate::tags::CommandTag::CommandTag { &mut self.base.base.base }
}
impl crate::tags::CustomTagSupport::CustomTagSupportDyn for StoreFrontsTag {
    fn as_CustomTagSupport(&self) -> &crate::tags::CustomTagSupport::CustomTagSupport { &self.base.base.base.base }
    fn as_CustomTagSupport_mut(&mut self) -> &mut crate::tags::CustomTagSupport::CustomTagSupport { &mut self.base.base.base.base }
}
impl crate::tags::PropertiesTag::PropertiesTagDyn for StoreFrontsTag {
    fn as_PropertiesTag(&self) -> &crate::tags::PropertiesTag::PropertiesTag { &self.base.base }
    fn as_PropertiesTag_mut(&mut self) -> &mut crate::tags::PropertiesTag::PropertiesTag { &mut self.base.base }
}


