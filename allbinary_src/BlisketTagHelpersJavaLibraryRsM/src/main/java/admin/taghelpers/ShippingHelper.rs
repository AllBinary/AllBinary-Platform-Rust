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
// package admin.taghelpers
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;
use super::*;
// use crate::java::util::HashMap::HashMap;
// use crate::javax::servlet::http::HttpServletRequest::HttpServletRequest;
// use crate::javax::servlet::jsp::PageContext::PageContext;
use crate::org::allbinary::business::context::modules::storefront::StoreFrontData::StoreFrontData;
use crate::org::allbinary::business::context::modules::storefront::StoreFrontFactory::StoreFrontFactory;
use crate::org::allbinary::business::context::modules::storefront::StoreFrontInterface::StoreFrontInterface;
use crate::org::allbinary::business::user::commerce::inventory::order::OrderInterface::OrderInterface;
use crate::org::allbinary::business::user::commerce::shipping::ShippingMethodData::ShippingMethodData;
use crate::org::allbinary::logic::communication::http::request::session::WeblisketSession::WeblisketSession;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use super::TagHelper::TagHelper;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct ShippingHelper {
    pub base: TagHelper,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub weblisketSession: Option<Rc<RefCell<WeblisketSession>>>,
    pub storeFrontInterface: Option<Rc<RefCell<dyn StoreFrontInterface>>>,
    pub propertiesHashMap: Option<Rc<RefCell<HashMap>>>,
    pub pageContext: Option<Rc<RefCell<PageContext>>>,
    pub request: Option<Rc<RefCell<HttpServletRequest>>>,
    pub shippingType: String,
}


impl ShippingHelper {
    

    pub fn new_struct(mut hashMap: Option<Rc<RefCell<HashMap>>>, mut pageContext: Option<Rc<RefCell<PageContext>>>) -> Self {
        let mut this = Self {
            base: TagHelper::new_struct(),
            logUtil: None,
            weblisketSession: None,
            storeFrontInterface: None,
            propertiesHashMap: None,
            pageContext: None,
            request: None,
            shippingType: String::new(),
        };
        this.logUtil = LogUtil::getInstance();
        this.pageContext = pageContext.clone();
        this.request = (jcall!(pageContext).getRequest()).clone() /* as HttpServletRequest */;
        let mut storeName: String = (jcall!(hashMap).get(jcall!(StoreFrontData::getInstance()).NAME.clone())).clone() /* as String */;
        if true {
            this.storeFrontInterface = StoreFrontFactory::getInstance(storeName.clone());
        }
        this.weblisketSession = WeblisketSession::new2(hashMap.clone(), pageContext.clone());
        this.getFormData();
        this
    }

    pub fn new(mut hashMap: Option<Rc<RefCell<HashMap>>>, mut pageContext: Option<Rc<RefCell<PageContext>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(hashMap, pageContext))))
    }
    

    pub fn getFormData(&mut self) {
        self.shippingType = jcall!(self.request).getParameter(crate::org::allbinary::business::user::commerce::shipping::ShippingMethodData::ShippingMethodData_NAME.with(Clone::clone).clone());
    }
    

    pub fn setShippingType(&mut self) -> String {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut success: String = String::from("Successfully Set Shipping Type");
            let mut order: Option<Rc<RefCell<dyn OrderInterface>>> = jcall!(self.weblisketSession).getOrder();
            jcall!(order).setShippingMethod(self.shippingType.clone());
            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).SQLTAGS.clone()) {
                jcall!(self.logUtil).putF(success.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("setShippingType()"));
            }
            return success.clone();
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            let mut error: String = String::from("Failed to view Shipping Type");
            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).SQLTAGSERROR.clone()) {
                jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("setShippingType()"), crate::java_into_object(e.clone()));
            }
            return error.clone();
            }
        }
    }
}
impl std::ops::Deref for ShippingHelper {
    type Target = TagHelper;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for ShippingHelper {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::admin::taghelpers::TagHelper::TagHelperDyn for ShippingHelper {
    fn as_TagHelper(&self) -> &crate::admin::taghelpers::TagHelper::TagHelper { &self.base }
    fn as_TagHelper_mut(&mut self) -> &mut crate::admin::taghelpers::TagHelper::TagHelper { &mut self.base }
}
impl crate::admin::taghelpers::TagHelperInterface::TagHelperInterface for ShippingHelper {
}


