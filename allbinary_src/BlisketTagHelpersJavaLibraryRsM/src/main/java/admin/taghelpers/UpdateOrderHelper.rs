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
use crate::org::allbinary::business::entry::EntryData::EntryData;
use crate::org::allbinary::business::user::UserData::UserData;
use crate::org::allbinary::business::user::address::BillingAddressData::BillingAddressData;
use crate::org::allbinary::business::user::address::ShippingAddressData::ShippingAddressData;
use crate::org::allbinary::business::user::commerce::inventory::order::OrderData::OrderData;
use crate::org::allbinary::business::user::commerce::inventory::order::OrderHistoryData::OrderHistoryData;
use crate::org::allbinary::business::user::commerce::money::payment::PaymentData::PaymentData;
use crate::org::allbinary::business::user::commerce::shipping::ShippingMethodData::ShippingMethodData;
use crate::org::allbinary::data::tables::user::commerce::inventory::order::OrderHistoryEntityFactory::OrderHistoryEntityFactory;
use crate::org::allbinary::logic::StdUtil::StdUtil;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use super::TagHelper::TagHelper;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct UpdateOrderHelper {
    pub base: TagHelper,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub request: Option<Rc<RefCell<HttpServletRequest>>>,
    pub entryId: String,
    pub orderId: String,
    pub userName: String,
    pub storeName: String,
    pub shipDate: String,
    pub orderDate: String,
    pub transDate: String,
    pub cancelDate: String,
    pub status: String,
    pub paymentMethod: String,
    pub name: String,
    pub r#type: String,
    pub expiration: String,
    pub number: String,
    pub encryption: String,
    pub billingName: String,
    pub billingStreet: String,
    pub billingCity: String,
    pub billingState: String,
    pub billingCode: String,
    pub billingCountry: String,
    pub shippingName: String,
    pub shippingStreet: String,
    pub shippingCity: String,
    pub shippingState: String,
    pub shippingCode: String,
    pub shippingCountry: String,
    pub shippingMethodName: String,
    pub subTotal: String,
    pub shippingCost: String,
    pub tax: String,
    pub total: String,
    pub special: String,
    pub userComment: String,
    pub userCancelComment: String,
    pub storeComment: String,
    pub storeCancelComment: String,
}


impl UpdateOrderHelper {
    

    pub fn new_struct(mut hashMap: Option<Rc<RefCell<HashMap>>>, mut pageContext: Option<Rc<RefCell<PageContext>>>) -> Self {
        let mut this = Self {
            base: TagHelper::new_struct(),
            logUtil: None,
            request: None,
            entryId: String::new(),
            orderId: String::new(),
            userName: String::new(),
            storeName: String::new(),
            shipDate: String::new(),
            orderDate: String::new(),
            transDate: String::new(),
            cancelDate: String::new(),
            status: String::new(),
            paymentMethod: String::new(),
            name: String::new(),
            r#type: String::new(),
            expiration: String::new(),
            number: String::new(),
            encryption: String::new(),
            billingName: String::new(),
            billingStreet: String::new(),
            billingCity: String::new(),
            billingState: String::new(),
            billingCode: String::new(),
            billingCountry: String::new(),
            shippingName: String::new(),
            shippingStreet: String::new(),
            shippingCity: String::new(),
            shippingState: String::new(),
            shippingCode: String::new(),
            shippingCountry: String::new(),
            shippingMethodName: String::new(),
            subTotal: String::new(),
            shippingCost: String::new(),
            tax: String::new(),
            total: String::new(),
            special: String::new(),
            userComment: String::new(),
            userCancelComment: String::new(),
            storeComment: String::new(),
            storeCancelComment: String::new(),
        };
        this.logUtil = LogUtil::getInstance();
        this.request = (jcall!(pageContext).getRequest()).clone() /* as HttpServletRequest */;
        this.getFormData();
        this
    }

    pub fn new(mut hashMap: Option<Rc<RefCell<HashMap>>>, mut pageContext: Option<Rc<RefCell<PageContext>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(hashMap, pageContext))))
    }
    

    pub fn getFormData(&mut self) {
        self.entryId = { let java_receiver = self.request.clone(); let java_arguments = (jcall!(EntryData::getInstance()).ID.clone(),); let java_result = jcall!(java_receiver).getParameter(java_arguments.0); java_result };
        self.orderId = jcall!(self.request).getParameter(crate::org::allbinary::business::user::commerce::inventory::order::OrderData::OrderData_ID.with(Clone::clone).clone());
        self.userName = jcall!(self.request).getParameter(crate::org::allbinary::business::user::UserData::UserData_USERNAME.with(Clone::clone).clone());
        self.storeName = { let java_receiver = self.request.clone(); let java_arguments = (jcall!(StoreFrontData::getInstance()).NAME.clone(),); let java_result = jcall!(java_receiver).getParameter(java_arguments.0); java_result };
        self.shipDate = jcall!(self.request).getParameter(crate::org::allbinary::business::user::commerce::inventory::order::OrderHistoryData::OrderHistoryData_SHIPPEDDATE.with(Clone::clone).clone());
        self.orderDate = jcall!(self.request).getParameter(crate::org::allbinary::business::user::commerce::inventory::order::OrderHistoryData::OrderHistoryData_ORDERDATE.with(Clone::clone).clone());
        self.transDate = jcall!(self.request).getParameter(crate::org::allbinary::business::user::commerce::inventory::order::OrderHistoryData::OrderHistoryData_TRANSDATE.with(Clone::clone).clone());
        self.cancelDate = jcall!(self.request).getParameter(crate::org::allbinary::business::user::commerce::inventory::order::OrderHistoryData::OrderHistoryData_CANCELDATE.with(Clone::clone).clone());
        self.status = jcall!(self.request).getParameter(crate::org::allbinary::business::user::commerce::inventory::order::OrderHistoryData::OrderHistoryData_STATUS.with(Clone::clone).clone());
        self.paymentMethod = jcall!(self.request).getParameter(crate::org::allbinary::business::user::commerce::money::payment::PaymentData::PaymentData_METHOD.with(Clone::clone).clone());
        self.name = jcall!(self.request).getParameter(crate::org::allbinary::business::user::commerce::money::payment::PaymentData::PaymentData_NAME.with(Clone::clone).clone());
        self.r#type = jcall!(self.request).getParameter(crate::org::allbinary::business::user::commerce::money::payment::PaymentData::PaymentData_TYPE.with(Clone::clone).clone());
        self.expiration = jcall!(self.request).getParameter(crate::org::allbinary::business::user::commerce::money::payment::PaymentData::PaymentData_EXPIRATION.with(Clone::clone).clone());
        self.number = jcall!(self.request).getParameter(crate::org::allbinary::business::user::commerce::money::payment::PaymentData::PaymentData_NUMBER.with(Clone::clone).clone());
        self.encryption = { let java_receiver = self.request.clone(); let java_arguments = (jcall!(EntryData::getInstance()).ENCRYPTION.clone(),); let java_result = jcall!(java_receiver).getParameter(java_arguments.0); java_result };
        self.billingName = jcall!(self.request).getParameter(crate::org::allbinary::business::user::address::BillingAddressData::BillingAddressData_NAME.with(Clone::clone).clone());
        self.billingStreet = jcall!(self.request).getParameter(crate::org::allbinary::business::user::address::BillingAddressData::BillingAddressData_STREET.with(Clone::clone).clone());
        self.billingCity = jcall!(self.request).getParameter(crate::org::allbinary::business::user::address::BillingAddressData::BillingAddressData_CITY.with(Clone::clone).clone());
        self.billingState = jcall!(self.request).getParameter(crate::org::allbinary::business::user::address::BillingAddressData::BillingAddressData_STATE.with(Clone::clone).clone());
        self.billingCode = jcall!(self.request).getParameter(crate::org::allbinary::business::user::address::BillingAddressData::BillingAddressData_CODE.with(Clone::clone).clone());
        self.billingCountry = jcall!(self.request).getParameter(crate::org::allbinary::business::user::address::BillingAddressData::BillingAddressData_COUNTRY.with(Clone::clone).clone());
        self.shippingName = jcall!(self.request).getParameter(crate::org::allbinary::business::user::address::ShippingAddressData::ShippingAddressData_NAME.with(Clone::clone).clone());
        self.shippingStreet = jcall!(self.request).getParameter(crate::org::allbinary::business::user::address::ShippingAddressData::ShippingAddressData_STREET.with(Clone::clone).clone());
        self.shippingCity = jcall!(self.request).getParameter(crate::org::allbinary::business::user::address::ShippingAddressData::ShippingAddressData_CITY.with(Clone::clone).clone());
        self.shippingState = jcall!(self.request).getParameter(crate::org::allbinary::business::user::address::ShippingAddressData::ShippingAddressData_STATE.with(Clone::clone).clone());
        self.shippingCode = jcall!(self.request).getParameter(crate::org::allbinary::business::user::address::ShippingAddressData::ShippingAddressData_CODE.with(Clone::clone).clone());
        self.shippingCountry = jcall!(self.request).getParameter(crate::org::allbinary::business::user::address::ShippingAddressData::ShippingAddressData_COUNTRY.with(Clone::clone).clone());
        self.shippingMethodName = jcall!(self.request).getParameter(crate::org::allbinary::business::user::commerce::shipping::ShippingMethodData::ShippingMethodData_NAME.with(Clone::clone).clone());
        self.subTotal = jcall!(self.request).getParameter(crate::org::allbinary::business::user::commerce::inventory::order::OrderHistoryData::OrderHistoryData_SUBTOTAL.with(Clone::clone).clone());
        self.shippingCost = jcall!(self.request).getParameter(crate::org::allbinary::business::user::commerce::inventory::order::OrderHistoryData::OrderHistoryData_SHIPPINGCOST.with(Clone::clone).clone());
        self.tax = jcall!(self.request).getParameter(crate::org::allbinary::business::user::commerce::inventory::order::OrderHistoryData::OrderHistoryData_TAX.with(Clone::clone).clone());
        self.total = jcall!(self.request).getParameter(crate::org::allbinary::business::user::commerce::inventory::order::OrderHistoryData::OrderHistoryData_TOTAL.with(Clone::clone).clone());
        self.special = { let java_receiver = self.request.clone(); let java_arguments = (jcall!(EntryData::getInstance()).SPECIAL.clone(),); let java_result = jcall!(java_receiver).getParameter(java_arguments.0); java_result };
        self.userComment = jcall!(self.request).getParameter(crate::org::allbinary::business::user::commerce::inventory::order::OrderData::OrderData_CUSTOMERCOMMENT.with(Clone::clone).clone());
        self.userCancelComment = jcall!(self.request).getParameter(crate::org::allbinary::business::user::commerce::inventory::order::OrderData::OrderData_CUSTOMERCANCELCOMMENT.with(Clone::clone).clone());
        self.storeComment = jcall!(self.request).getParameter(crate::org::allbinary::business::user::commerce::inventory::order::OrderData::OrderData_STORECOMMENT.with(Clone::clone).clone());
        self.storeCancelComment = jcall!(self.request).getParameter(crate::org::allbinary::business::user::commerce::inventory::order::OrderData::OrderData_STORECANCELCOMMENT.with(Clone::clone).clone());
    }
    

    pub fn update(&mut self) -> String {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut success: String = String::from("Successful update");
            let mut orderHashMap: Option<Rc<RefCell<HashMap>>> = jcall!(StdUtil::getInstance()).createHashMap();
            let mut whereHashMap: Option<Rc<RefCell<HashMap>>> = jcall!(StdUtil::getInstance()).createHashMap();
            jcall!(orderHashMap).put(jcall!(EntryData::getInstance()).ID.clone(), self.entryId.clone());
            jcall!(orderHashMap).put(crate::org::allbinary::business::user::commerce::inventory::order::OrderData::OrderData_ID.with(Clone::clone).clone(), self.orderId.clone());
            jcall!(orderHashMap).put(crate::org::allbinary::business::user::UserData::UserData_USERNAME.with(Clone::clone).clone(), self.userName.clone());
            jcall!(orderHashMap).put(jcall!(StoreFrontData::getInstance()).NAME.clone(), self.storeName.clone());
            jcall!(orderHashMap).put(crate::org::allbinary::business::user::commerce::inventory::order::OrderHistoryData::OrderHistoryData_SHIPPEDDATE.with(Clone::clone).clone(), self.shipDate.clone());
            jcall!(orderHashMap).put(crate::org::allbinary::business::user::commerce::inventory::order::OrderHistoryData::OrderHistoryData_ORDERDATE.with(Clone::clone).clone(), self.orderDate.clone());
            jcall!(orderHashMap).put(crate::org::allbinary::business::user::commerce::inventory::order::OrderHistoryData::OrderHistoryData_TRANSDATE.with(Clone::clone).clone(), self.transDate.clone());
            jcall!(orderHashMap).put(crate::org::allbinary::business::user::commerce::inventory::order::OrderHistoryData::OrderHistoryData_CANCELDATE.with(Clone::clone).clone(), self.cancelDate.clone());
            jcall!(orderHashMap).put(crate::org::allbinary::business::user::commerce::inventory::order::OrderHistoryData::OrderHistoryData_STATUS.with(Clone::clone).clone(), self.status.clone());
            jcall!(orderHashMap).put(crate::org::allbinary::business::user::commerce::money::payment::PaymentData::PaymentData_METHOD.with(Clone::clone).clone(), self.paymentMethod.clone());
            jcall!(orderHashMap).put(crate::org::allbinary::business::user::commerce::money::payment::PaymentData::PaymentData_NAME.with(Clone::clone).clone(), self.name.clone());
            jcall!(orderHashMap).put(crate::org::allbinary::business::user::commerce::money::payment::PaymentData::PaymentData_TYPE.with(Clone::clone).clone(), self.r#type.clone());
            jcall!(orderHashMap).put(crate::org::allbinary::business::user::commerce::money::payment::PaymentData::PaymentData_EXPIRATION.with(Clone::clone).clone(), self.expiration.clone());
            jcall!(orderHashMap).put(crate::org::allbinary::business::user::address::BillingAddressData::BillingAddressData_NAME.with(Clone::clone).clone(), self.billingName.clone());
            jcall!(orderHashMap).put(crate::org::allbinary::business::user::address::BillingAddressData::BillingAddressData_STREET.with(Clone::clone).clone(), self.billingStreet.clone());
            jcall!(orderHashMap).put(crate::org::allbinary::business::user::address::BillingAddressData::BillingAddressData_CITY.with(Clone::clone).clone(), self.billingCity.clone());
            jcall!(orderHashMap).put(crate::org::allbinary::business::user::address::BillingAddressData::BillingAddressData_STATE.with(Clone::clone).clone(), self.billingState.clone());
            jcall!(orderHashMap).put(crate::org::allbinary::business::user::address::BillingAddressData::BillingAddressData_CODE.with(Clone::clone).clone(), self.billingCode.clone());
            jcall!(orderHashMap).put(crate::org::allbinary::business::user::address::BillingAddressData::BillingAddressData_COUNTRY.with(Clone::clone).clone(), self.billingCountry.clone());
            jcall!(orderHashMap).put(crate::org::allbinary::business::user::address::ShippingAddressData::ShippingAddressData_NAME.with(Clone::clone).clone(), self.shippingName.clone());
            jcall!(orderHashMap).put(crate::org::allbinary::business::user::address::ShippingAddressData::ShippingAddressData_STREET.with(Clone::clone).clone(), self.shippingStreet.clone());
            jcall!(orderHashMap).put(crate::org::allbinary::business::user::address::ShippingAddressData::ShippingAddressData_CITY.with(Clone::clone).clone(), self.shippingCity.clone());
            jcall!(orderHashMap).put(crate::org::allbinary::business::user::address::ShippingAddressData::ShippingAddressData_STATE.with(Clone::clone).clone(), self.shippingState.clone());
            jcall!(orderHashMap).put(crate::org::allbinary::business::user::address::ShippingAddressData::ShippingAddressData_CODE.with(Clone::clone).clone(), self.shippingCode.clone());
            jcall!(orderHashMap).put(crate::org::allbinary::business::user::address::ShippingAddressData::ShippingAddressData_COUNTRY.with(Clone::clone).clone(), self.shippingCountry.clone());
            jcall!(orderHashMap).put(crate::org::allbinary::business::user::commerce::shipping::ShippingMethodData::ShippingMethodData_NAME.with(Clone::clone).clone(), self.shippingMethodName.clone());
            jcall!(orderHashMap).put(crate::org::allbinary::business::user::commerce::inventory::order::OrderHistoryData::OrderHistoryData_SUBTOTAL.with(Clone::clone).clone(), self.subTotal.clone());
            jcall!(orderHashMap).put(crate::org::allbinary::business::user::commerce::inventory::order::OrderHistoryData::OrderHistoryData_SHIPPINGCOST.with(Clone::clone).clone(), self.shippingCost.clone());
            jcall!(orderHashMap).put(crate::org::allbinary::business::user::commerce::inventory::order::OrderHistoryData::OrderHistoryData_TAX.with(Clone::clone).clone(), self.tax.clone());
            jcall!(orderHashMap).put(crate::org::allbinary::business::user::commerce::inventory::order::OrderHistoryData::OrderHistoryData_TOTAL.with(Clone::clone).clone(), self.total.clone());
            jcall!(orderHashMap).put(jcall!(EntryData::getInstance()).SPECIAL.clone(), self.special.clone());
            jcall!(orderHashMap).put(crate::org::allbinary::business::user::commerce::inventory::order::OrderData::OrderData_CUSTOMERCOMMENT.with(Clone::clone).clone(), self.userComment.clone());
            jcall!(orderHashMap).put(crate::org::allbinary::business::user::commerce::inventory::order::OrderData::OrderData_CUSTOMERCANCELCOMMENT.with(Clone::clone).clone(), self.userCancelComment.clone());
            jcall!(orderHashMap).put(crate::org::allbinary::business::user::commerce::inventory::order::OrderData::OrderData_STORECOMMENT.with(Clone::clone).clone(), self.storeComment.clone());
            jcall!(orderHashMap).put(crate::org::allbinary::business::user::commerce::inventory::order::OrderData::OrderData_STORECANCELCOMMENT.with(Clone::clone).clone(), self.storeCancelComment.clone());
            jcall!(whereHashMap).put(crate::org::allbinary::business::user::commerce::inventory::order::OrderData::OrderData_ID.with(Clone::clone).clone(), self.orderId.clone());
            jcall!(OrderHistoryEntityFactory::getInstance()).update(whereHashMap.clone(), orderHashMap.clone());
            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).SQLTAGS.clone()) {
                jcall!(self.logUtil).putF(success.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("update()"));
            }
            return success.clone();
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            let mut error: String = String::from("Failed to update order table");
            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).SQLTAGSERROR.clone()) {
                jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("update()"), crate::java_into_object(e.clone()));
            }
            return error.clone();
            }
        }
    }
}
impl std::ops::Deref for UpdateOrderHelper {
    type Target = TagHelper;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for UpdateOrderHelper {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::admin::taghelpers::TagHelper::TagHelperDyn for UpdateOrderHelper {
    fn as_TagHelper(&self) -> &crate::admin::taghelpers::TagHelper::TagHelper { &self.base }
    fn as_TagHelper_mut(&mut self) -> &mut crate::admin::taghelpers::TagHelper::TagHelper { &mut self.base }
}
impl crate::admin::taghelpers::TagHelperInterface::TagHelperInterface for UpdateOrderHelper {
}


