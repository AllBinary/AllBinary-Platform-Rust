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
// package taghelpers
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;
use super::*;
// use crate::java::util::HashMap::HashMap;
// use crate::javax::servlet::http::HttpServletRequest::HttpServletRequest;
// use crate::javax::servlet::jsp::PageContext::PageContext;
use crate::admin::taghelpers::TagHelper::TagHelper;
use crate::org::allbinary::business::context::modules::storefront::StoreFrontData::StoreFrontData;
use crate::org::allbinary::business::context::modules::storefront::StoreFrontFactory::StoreFrontFactory;
use crate::org::allbinary::business::context::modules::storefront::StoreFrontInterface::StoreFrontInterface;
use crate::org::allbinary::business::user::address::StreetAddress::StreetAddress;
use crate::org::allbinary::business::user::commerce::inventory::order::OrderData::OrderData;
use crate::org::allbinary::business::user::commerce::inventory::order::OrderHistoryData::OrderHistoryData;
use crate::org::allbinary::business::user::commerce::inventory::order::OrderHistoryFactory::OrderHistoryFactory;
use crate::org::allbinary::business::user::commerce::inventory::order::OrderHistoryInterface::OrderHistoryInterface;
use crate::org::allbinary::business::user::commerce::shipping::ShippingMethodData::ShippingMethodData;
use crate::org::allbinary::data::tables::user::commerce::inventory::order::OrderHistoryEntityFactory::OrderHistoryEntityFactory;
use crate::org::allbinary::data::tables::user::commerce::inventory::order::OrderHistoryEntityInterface::OrderHistoryEntityInterface;
use crate::org::allbinary::logic::communication::http::request::session::WeblisketSession::WeblisketSession;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct OrderHistoryHelper {
    pub base: TagHelper,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub weblisketSession: Option<Rc<RefCell<WeblisketSession>>>,
    pub storeFrontInterface: Option<Rc<RefCell<dyn StoreFrontInterface>>>,
    pub propertiesHashMap: Option<Rc<RefCell<HashMap>>>,
    pub pageContext: Option<Rc<RefCell<PageContext>>>,
    pub request: Option<Rc<RefCell<HttpServletRequest>>>,
    pub streetAddress: Option<Rc<RefCell<StreetAddress>>>,
    pub id: String,
    pub groupId: String,
    pub status: String,
    pub newStatus: String,
}


impl OrderHistoryHelper {
    

    pub fn new_struct(mut propertiesHashMap: Option<Rc<RefCell<HashMap>>>, mut pageContext: Option<Rc<RefCell<PageContext>>>) -> Self {
        let mut this = Self {
            base: TagHelper::new_struct(),
            logUtil: None,
            weblisketSession: None,
            storeFrontInterface: None,
            propertiesHashMap: None,
            pageContext: None,
            request: None,
            streetAddress: None,
            id: String::new(),
            groupId: String::new(),
            status: String::new(),
            newStatus: String::new(),
        };
        this.logUtil = LogUtil::getInstance();
        this.propertiesHashMap = propertiesHashMap.clone();
        this.pageContext = pageContext.clone();
        this.request = (jcall!(pageContext).getRequest()).clone() /* as HttpServletRequest */;
        let mut storeName: String = (jcall!(propertiesHashMap).get(jcall!(StoreFrontData::getInstance()).NAME.clone())).clone() /* as String */;
        this.newStatus = (jcall!(propertiesHashMap).get(crate::org::allbinary::business::user::commerce::inventory::order::OrderHistoryData::OrderHistoryData_STATUS.with(Clone::clone).clone())).clone() /* as String */;
        if true {
            this.storeFrontInterface = StoreFrontFactory::getInstance(storeName.clone());
        }
        this.getFormData();
        this
    }

    pub fn new(mut propertiesHashMap: Option<Rc<RefCell<HashMap>>>, mut pageContext: Option<Rc<RefCell<PageContext>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(propertiesHashMap, pageContext))))
    }
    

    pub fn getFormData(&mut self) {
        self.id = jcall!(self.request).getParameter(crate::org::allbinary::business::user::commerce::inventory::order::OrderData::OrderData_ID.with(Clone::clone).clone());
        self.groupId = jcall!(self.request).getParameter(crate::org::allbinary::business::user::commerce::shipping::ShippingMethodData::ShippingMethodData_GROUP.with(Clone::clone).clone());
        if false {
            self.newStatus = jcall!(self.request).getParameter(crate::org::allbinary::business::user::commerce::inventory::order::OrderHistoryData::OrderHistoryData_STATUS.with(Clone::clone).clone());
        }
    }
    

    pub fn setOrderStatus(&mut self) -> String {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut success: String = format!("{}{}", String::from("Status successfully set to: "), self.status);
            let mut orderReviewInterface: Option<Rc<RefCell<dyn OrderHistoryInterface>>> = OrderHistoryFactory::getInstance(self.id.clone());
            let mut status: String = jcall!(orderReviewInterface).getStatus();
            if (status.cmp(&crate::org::allbinary::business::user::commerce::inventory::order::OrderHistoryData::OrderHistoryData_CANCELLED.with(Clone::clone).clone()) as i32) == 0 {
                return format!("{}{}", format!("{}{}", String::from("Order "), self.id), String::from(" Already Cancelled<br/>"));
            } else if (status.cmp(&crate::org::allbinary::business::user::commerce::inventory::order::OrderHistoryData::OrderHistoryData_SHIPPED.with(Clone::clone).clone()) as i32) == 0 {
                return format!("{}{}", format!("{}{}", String::from("Order "), self.id), String::from(" Already Shipped. You can't cancel a shipped order.<br/>"));
            } else if (status.cmp(&crate::org::allbinary::business::user::commerce::inventory::order::OrderHistoryData::OrderHistoryData_PREPROCESSING.with(Clone::clone).clone()) as i32) == 0 {
                return format!("{}{}", format!("{}{}", String::from("Order "), self.id), String::from(" has been cancelled.<br/>"));
            } else if (status.cmp(&crate::org::allbinary::business::user::commerce::inventory::order::OrderHistoryData::OrderHistoryData_PARTIALLYSHIPPED.with(Clone::clone).clone()) as i32) == 0 {
                return format!("{}{}", format!("{}{}", String::from("Order "), self.id), String::from(" Partially Shipped. You can cancel unshipped items, but not the whole order.<br/>"));
            } else if (status.cmp(&crate::org::allbinary::business::user::commerce::inventory::order::OrderHistoryData::OrderHistoryData_PROCESSING.with(Clone::clone).clone()) as i32) == 0 {
                let mut orderHistoryEntityInterface: Option<Rc<RefCell<dyn OrderHistoryEntityInterface>>> = OrderHistoryEntityFactory::getInstance().map(|reference| reference as Rc<RefCell<dyn OrderHistoryEntityInterface>>);
                jcall!(orderHistoryEntityInterface).setStatus(self.id.clone(), self.newStatus.clone());
                return format!("{}{}", format!("{}{}", String::from("Order "), self.id), String::from(" has been cancelled.<br/>"));
            }
            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).SQLTAGS.clone()) {
                jcall!(self.logUtil).putF(success.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("setOrderStatus()"));
            }
            return String::from("Error Setting Order Status");
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            let mut error: String = String::from("Failed to set order status");
            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).SQLTAGSERROR.clone()) {
                jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("setOrderStatus()"), crate::java_into_object(e.clone()));
            }
            return format!("{}{}", format!("{}{}", format!("{}{}", format!("{}{}", error, String::from("<br/>")), String::from("Exception: ")), e), String::from("<br/>"));
            }
        }
    }
}
impl std::ops::Deref for OrderHistoryHelper {
    type Target = TagHelper;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for OrderHistoryHelper {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::admin::taghelpers::TagHelper::TagHelperDyn for OrderHistoryHelper {
    fn as_TagHelper(&self) -> &crate::admin::taghelpers::TagHelper::TagHelper { &self.base }
    fn as_TagHelper_mut(&mut self) -> &mut crate::admin::taghelpers::TagHelper::TagHelper { &mut self.base }
}
impl crate::admin::taghelpers::TagHelperInterface::TagHelperInterface for OrderHistoryHelper {
}


