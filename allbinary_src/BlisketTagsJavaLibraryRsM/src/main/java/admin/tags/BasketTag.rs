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
// use crate::java::lang::reflect::Method::Method;
// use crate::java::util::HashMap::HashMap;
// use crate::javax::servlet::jsp::JspTagException::JspTagException;
use crate::admin::taghelpers::BasketHelperFactory::BasketHelperFactory;
use crate::admin::taghelpers::BasketRequestHelperFactory::BasketRequestHelperFactory;
use crate::org::allbinary::business::context::modules::storefront::StoreFrontData::StoreFrontData;
use crate::org::allbinary::business::user::commerce::inventory::basket::BasketData::BasketData;
use crate::org::allbinary::logic::StdUtil::StdUtil;
use crate::org::allbinary::logic::communication::http::request::AbResponseHandler::AbResponseHandler;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::system::security::licensing::LicensingException::LicensingException;
use crate::tags::CustomTagSupport::CustomTagSupport;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct BasketTag {
    pub base: CustomTagSupport,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub command: String,
    pub storeName: String,
    pub propertiesHashMap: Option<Rc<RefCell<HashMap>>>,
}


impl BasketTag {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: CustomTagSupport::new_struct(),
            logUtil: None,
            command: String::new(),
            storeName: String::new(),
            propertiesHashMap: None,
        };
        this.logUtil = LogUtil::getInstance();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn setCommand(&mut self, mut command: String) {
        self.command = command.clone();
    }
    

    pub fn setStoreName(&mut self, mut value: String) {
        self.storeName = value.clone();
    }
    

    // throws LicensingException
    pub fn isBasketEmpty(&mut self) -> bool {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = jcall!(BasketHelperFactory::new()).getInstance(self.propertiesHashMap.clone(), self.pageContext.clone());
            let mut method: Option<Rc<RefCell<Method>>> = jcall!(std::any::type_name_of_val(&anyType)).getMethod(String::from("isBasketEmpty"), None);
            let mut emptyBoolean: Option<bool> = (jcall!(method).invoke(anyType.clone(), None)).clone() /* as Boolean */;
            return emptyBoolean.unwrap();
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            panic!("{:?}", e);
            }
        }
    }
    

    // throws LicensingException
    pub fn addItemToBasket(&mut self) -> bool {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = jcall!(BasketRequestHelperFactory::new()).getInstance(self.propertiesHashMap.clone(), self.pageContext.clone());
            let mut method: Option<Rc<RefCell<Method>>> = jcall!(std::any::type_name_of_val(&anyType)).getMethod(String::from("addItemToBasket"), None);
            let mut emptyBoolean: Option<bool> = (jcall!(method).invoke(anyType.clone(), None)).clone() /* as Boolean */;
            return emptyBoolean.unwrap();
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            panic!("{:?}", e);
            }
        }
    }
    

    // throws LicensingException
    pub fn removeItemFromBasket(&mut self) -> bool {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = jcall!(BasketRequestHelperFactory::new()).getInstance(self.propertiesHashMap.clone(), self.pageContext.clone());
            let mut method: Option<Rc<RefCell<Method>>> = jcall!(std::any::type_name_of_val(&anyType)).getMethod(String::from("removeItemFromBasket"), None);
            let mut emptyBoolean: Option<bool> = (jcall!(method).invoke(anyType.clone(), None)).clone() /* as Boolean */;
            return emptyBoolean.unwrap();
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            panic!("{:?}", e);
            }
        }
    }
    

    // throws LicensingException
    pub fn adjustBasket(&mut self) -> bool {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = jcall!(BasketRequestHelperFactory::new()).getInstance(self.propertiesHashMap.clone(), self.pageContext.clone());
            let mut method: Option<Rc<RefCell<Method>>> = jcall!(std::any::type_name_of_val(&anyType)).getMethod(String::from("adjustBasket"), None);
            let mut emptyBoolean: Option<bool> = (jcall!(method).invoke(anyType.clone(), None)).clone() /* as Boolean */;
            return emptyBoolean.unwrap();
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
            if true {
                self.propertiesHashMap = jcall!(StdUtil::getInstance()).createHashMap();
                { let java_receiver = self.propertiesHashMap.clone(); let java_arguments = (jcall!(StoreFrontData::getInstance()).NAME.clone(), self.storeName.clone(),); jcall!(java_receiver).put(java_arguments.0, java_arguments.1); };
                if (self.command.cmp(&crate::org::allbinary::business::user::commerce::inventory::basket::BasketData::BasketData_INSERT.with(Clone::clone).clone()) as i32) == 0 {
                    if !self.addItemToBasket() {
                        jcall!(jcall!(self.pageContext).getOut()).println(String::from("Item is not currently being sold.<p/>"));
                    }
                } else if (self.command.cmp(&crate::org::allbinary::business::user::commerce::inventory::basket::BasketData::BasketData_DELETE.with(Clone::clone).clone()) as i32) == 0 {
                    self.removeItemFromBasket();
                } else if (self.command.cmp(&crate::org::allbinary::business::user::commerce::inventory::basket::BasketData::BasketData_ADJUST.with(Clone::clone).clone()) as i32) == 0 {
                    self.adjustBasket();
                } else if (self.command.cmp(&crate::org::allbinary::business::user::commerce::inventory::basket::BasketData::BasketData_ISEMPTY.with(Clone::clone).clone()) as i32) == 0 {
                    if self.isBasketEmpty() {
                        return self.EVAL_BODY_INCLUDE.clone();
                    } else {
                        return SKIP_BODY.clone();
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
impl std::ops::Deref for BasketTag {
    type Target = CustomTagSupport;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for BasketTag {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::tags::CustomTagSupport::CustomTagSupportDyn for BasketTag {
    fn as_CustomTagSupport(&self) -> &crate::tags::CustomTagSupport::CustomTagSupport { &self.base }
    fn as_CustomTagSupport_mut(&mut self) -> &mut crate::tags::CustomTagSupport::CustomTagSupport { &mut self.base }
}


