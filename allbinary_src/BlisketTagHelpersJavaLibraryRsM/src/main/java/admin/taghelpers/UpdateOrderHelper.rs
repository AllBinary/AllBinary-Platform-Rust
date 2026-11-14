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
use super::*;
use crate::java::util::HashMap::HashMap;
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
use crate::java::lang::Exception::*;
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
    

    pub fn new_struct(mut hashMap: Option<Rc<RefCell<HashMap<crate::JavaObject, crate::JavaObject>>>>, mut pageContext: Option<Rc<RefCell<PageContext>>>) -> Self {
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
        this.request = { let java_value = ((jcall!(pageContext).getRequest()).clone() /* as HttpServletRequest */).clone(); java_value };
        this.getFormData();
        this
    }

    pub fn new(mut hashMap: Option<Rc<RefCell<HashMap<crate::JavaObject, crate::JavaObject>>>>, mut pageContext: Option<Rc<RefCell<PageContext>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(hashMap, pageContext))))
    }
    

    pub fn getFormData(&mut self) {
        self.entryId = { let java_value = ({ let java_receiver = self.request.clone(); let java_arguments = ({ let java_field = jcall!(EntryData::getInstance()).ID.clone(); java_field }.clone(),); let java_result = jcall!(java_receiver).getParameter(java_arguments.0); java_result }).clone(); java_value };
        self.orderId = { let java_value = (jcall!(self.request).getParameter(crate::org::allbinary::business::user::commerce::inventory::order::OrderData::OrderData_ID.with(Clone::clone).clone())).clone(); java_value };
        self.userName = { let java_value = (jcall!(self.request).getParameter(crate::org::allbinary::business::user::UserData::UserData_USERNAME.with(Clone::clone).clone())).clone(); java_value };
        self.storeName = { let java_value = ({ let java_receiver = self.request.clone(); let java_arguments = ({ let java_field = jcall!(StoreFrontData::getInstance()).NAME.clone(); java_field }.clone(),); let java_result = jcall!(java_receiver).getParameter(java_arguments.0); java_result }).clone(); java_value };
        self.shipDate = { let java_value = (jcall!(self.request).getParameter(crate::org::allbinary::business::user::commerce::inventory::order::OrderHistoryData::OrderHistoryData_SHIPPEDDATE.with(Clone::clone).clone())).clone(); java_value };
        self.orderDate = { let java_value = (jcall!(self.request).getParameter(crate::org::allbinary::business::user::commerce::inventory::order::OrderHistoryData::OrderHistoryData_ORDERDATE.with(Clone::clone).clone())).clone(); java_value };
        self.transDate = { let java_value = (jcall!(self.request).getParameter(crate::org::allbinary::business::user::commerce::inventory::order::OrderHistoryData::OrderHistoryData_TRANSDATE.with(Clone::clone).clone())).clone(); java_value };
        self.cancelDate = { let java_value = (jcall!(self.request).getParameter(crate::org::allbinary::business::user::commerce::inventory::order::OrderHistoryData::OrderHistoryData_CANCELDATE.with(Clone::clone).clone())).clone(); java_value };
        self.status = { let java_value = (jcall!(self.request).getParameter(crate::org::allbinary::business::user::commerce::inventory::order::OrderHistoryData::OrderHistoryData_STATUS.with(Clone::clone).clone())).clone(); java_value };
        self.paymentMethod = { let java_value = (jcall!(self.request).getParameter(crate::org::allbinary::business::user::commerce::money::payment::PaymentData::PaymentData_METHOD.with(Clone::clone).clone())).clone(); java_value };
        self.name = { let java_value = (jcall!(self.request).getParameter(crate::org::allbinary::business::user::commerce::money::payment::PaymentData::PaymentData_NAME.with(Clone::clone).clone())).clone(); java_value };
        self.r#type = { let java_value = (jcall!(self.request).getParameter(crate::org::allbinary::business::user::commerce::money::payment::PaymentData::PaymentData_TYPE.with(Clone::clone).clone())).clone(); java_value };
        self.expiration = { let java_value = (jcall!(self.request).getParameter(crate::org::allbinary::business::user::commerce::money::payment::PaymentData::PaymentData_EXPIRATION.with(Clone::clone).clone())).clone(); java_value };
        self.number = { let java_value = (jcall!(self.request).getParameter(crate::org::allbinary::business::user::commerce::money::payment::PaymentData::PaymentData_NUMBER.with(Clone::clone).clone())).clone(); java_value };
        self.encryption = { let java_value = ({ let java_receiver = self.request.clone(); let java_arguments = ({ let java_field = jcall!(EntryData::getInstance()).ENCRYPTION.clone(); java_field }.clone(),); let java_result = jcall!(java_receiver).getParameter(java_arguments.0); java_result }).clone(); java_value };
        self.billingName = { let java_value = (jcall!(self.request).getParameter(crate::org::allbinary::business::user::address::BillingAddressData::BillingAddressData_NAME.with(Clone::clone).clone())).clone(); java_value };
        self.billingStreet = { let java_value = (jcall!(self.request).getParameter(crate::org::allbinary::business::user::address::BillingAddressData::BillingAddressData_STREET.with(Clone::clone).clone())).clone(); java_value };
        self.billingCity = { let java_value = (jcall!(self.request).getParameter(crate::org::allbinary::business::user::address::BillingAddressData::BillingAddressData_CITY.with(Clone::clone).clone())).clone(); java_value };
        self.billingState = { let java_value = (jcall!(self.request).getParameter(crate::org::allbinary::business::user::address::BillingAddressData::BillingAddressData_STATE.with(Clone::clone).clone())).clone(); java_value };
        self.billingCode = { let java_value = (jcall!(self.request).getParameter(crate::org::allbinary::business::user::address::BillingAddressData::BillingAddressData_CODE.with(Clone::clone).clone())).clone(); java_value };
        self.billingCountry = { let java_value = (jcall!(self.request).getParameter(crate::org::allbinary::business::user::address::BillingAddressData::BillingAddressData_COUNTRY.with(Clone::clone).clone())).clone(); java_value };
        self.shippingName = { let java_value = (jcall!(self.request).getParameter(crate::org::allbinary::business::user::address::ShippingAddressData::ShippingAddressData_NAME.with(Clone::clone).clone())).clone(); java_value };
        self.shippingStreet = { let java_value = (jcall!(self.request).getParameter(crate::org::allbinary::business::user::address::ShippingAddressData::ShippingAddressData_STREET.with(Clone::clone).clone())).clone(); java_value };
        self.shippingCity = { let java_value = (jcall!(self.request).getParameter(crate::org::allbinary::business::user::address::ShippingAddressData::ShippingAddressData_CITY.with(Clone::clone).clone())).clone(); java_value };
        self.shippingState = { let java_value = (jcall!(self.request).getParameter(crate::org::allbinary::business::user::address::ShippingAddressData::ShippingAddressData_STATE.with(Clone::clone).clone())).clone(); java_value };
        self.shippingCode = { let java_value = (jcall!(self.request).getParameter(crate::org::allbinary::business::user::address::ShippingAddressData::ShippingAddressData_CODE.with(Clone::clone).clone())).clone(); java_value };
        self.shippingCountry = { let java_value = (jcall!(self.request).getParameter(crate::org::allbinary::business::user::address::ShippingAddressData::ShippingAddressData_COUNTRY.with(Clone::clone).clone())).clone(); java_value };
        self.shippingMethodName = { let java_value = (jcall!(self.request).getParameter(crate::org::allbinary::business::user::commerce::shipping::ShippingMethodData::ShippingMethodData_NAME.with(Clone::clone).clone())).clone(); java_value };
        self.subTotal = { let java_value = (jcall!(self.request).getParameter(crate::org::allbinary::business::user::commerce::inventory::order::OrderHistoryData::OrderHistoryData_SUBTOTAL.with(Clone::clone).clone())).clone(); java_value };
        self.shippingCost = { let java_value = (jcall!(self.request).getParameter(crate::org::allbinary::business::user::commerce::inventory::order::OrderHistoryData::OrderHistoryData_SHIPPINGCOST.with(Clone::clone).clone())).clone(); java_value };
        self.tax = { let java_value = (jcall!(self.request).getParameter(crate::org::allbinary::business::user::commerce::inventory::order::OrderHistoryData::OrderHistoryData_TAX.with(Clone::clone).clone())).clone(); java_value };
        self.total = { let java_value = (jcall!(self.request).getParameter(crate::org::allbinary::business::user::commerce::inventory::order::OrderHistoryData::OrderHistoryData_TOTAL.with(Clone::clone).clone())).clone(); java_value };
        self.special = { let java_value = ({ let java_receiver = self.request.clone(); let java_arguments = ({ let java_field = jcall!(EntryData::getInstance()).SPECIAL.clone(); java_field }.clone(),); let java_result = jcall!(java_receiver).getParameter(java_arguments.0); java_result }).clone(); java_value };
        self.userComment = { let java_value = (jcall!(self.request).getParameter(crate::org::allbinary::business::user::commerce::inventory::order::OrderData::OrderData_CUSTOMERCOMMENT.with(Clone::clone).clone())).clone(); java_value };
        self.userCancelComment = { let java_value = (jcall!(self.request).getParameter(crate::org::allbinary::business::user::commerce::inventory::order::OrderData::OrderData_CUSTOMERCANCELCOMMENT.with(Clone::clone).clone())).clone(); java_value };
        self.storeComment = { let java_value = (jcall!(self.request).getParameter(crate::org::allbinary::business::user::commerce::inventory::order::OrderData::OrderData_STORECOMMENT.with(Clone::clone).clone())).clone(); java_value };
        self.storeCancelComment = { let java_value = (jcall!(self.request).getParameter(crate::org::allbinary::business::user::commerce::inventory::order::OrderData::OrderData_STORECANCELCOMMENT.with(Clone::clone).clone())).clone(); java_value };
    }
    

    pub fn update(&mut self) -> String {
        let java_try_result = match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut success: String = String::from("Successful update");
            let mut orderHashMap: Option<Rc<RefCell<HashMap<crate::JavaObject, crate::JavaObject>>>> = jcall!(StdUtil::getInstance()).createHashMap();
            let mut whereHashMap: Option<Rc<RefCell<HashMap<crate::JavaObject, crate::JavaObject>>>> = jcall!(StdUtil::getInstance()).createHashMap();
            jcall!(orderHashMap).put(crate::java_into_object({ let java_field = jcall!(EntryData::getInstance()).ID.clone(); java_field }.clone()), crate::java_into_object(self.entryId.clone()));
            jcall!(orderHashMap).put(crate::java_into_object(crate::org::allbinary::business::user::commerce::inventory::order::OrderData::OrderData_ID.with(Clone::clone).clone()), crate::java_into_object(self.orderId.clone()));
            jcall!(orderHashMap).put(crate::java_into_object(crate::org::allbinary::business::user::UserData::UserData_USERNAME.with(Clone::clone).clone()), crate::java_into_object(self.userName.clone()));
            jcall!(orderHashMap).put(crate::java_into_object({ let java_field = jcall!(StoreFrontData::getInstance()).NAME.clone(); java_field }.clone()), crate::java_into_object(self.storeName.clone()));
            jcall!(orderHashMap).put(crate::java_into_object(crate::org::allbinary::business::user::commerce::inventory::order::OrderHistoryData::OrderHistoryData_SHIPPEDDATE.with(Clone::clone).clone()), crate::java_into_object(self.shipDate.clone()));
            jcall!(orderHashMap).put(crate::java_into_object(crate::org::allbinary::business::user::commerce::inventory::order::OrderHistoryData::OrderHistoryData_ORDERDATE.with(Clone::clone).clone()), crate::java_into_object(self.orderDate.clone()));
            jcall!(orderHashMap).put(crate::java_into_object(crate::org::allbinary::business::user::commerce::inventory::order::OrderHistoryData::OrderHistoryData_TRANSDATE.with(Clone::clone).clone()), crate::java_into_object(self.transDate.clone()));
            jcall!(orderHashMap).put(crate::java_into_object(crate::org::allbinary::business::user::commerce::inventory::order::OrderHistoryData::OrderHistoryData_CANCELDATE.with(Clone::clone).clone()), crate::java_into_object(self.cancelDate.clone()));
            jcall!(orderHashMap).put(crate::java_into_object(crate::org::allbinary::business::user::commerce::inventory::order::OrderHistoryData::OrderHistoryData_STATUS.with(Clone::clone).clone()), crate::java_into_object(self.status.clone()));
            jcall!(orderHashMap).put(crate::java_into_object(crate::org::allbinary::business::user::commerce::money::payment::PaymentData::PaymentData_METHOD.with(Clone::clone).clone()), crate::java_into_object(self.paymentMethod.clone()));
            jcall!(orderHashMap).put(crate::java_into_object(crate::org::allbinary::business::user::commerce::money::payment::PaymentData::PaymentData_NAME.with(Clone::clone).clone()), crate::java_into_object(self.name.clone()));
            jcall!(orderHashMap).put(crate::java_into_object(crate::org::allbinary::business::user::commerce::money::payment::PaymentData::PaymentData_TYPE.with(Clone::clone).clone()), crate::java_into_object(self.r#type.clone()));
            jcall!(orderHashMap).put(crate::java_into_object(crate::org::allbinary::business::user::commerce::money::payment::PaymentData::PaymentData_EXPIRATION.with(Clone::clone).clone()), crate::java_into_object(self.expiration.clone()));
            jcall!(orderHashMap).put(crate::java_into_object(crate::org::allbinary::business::user::address::BillingAddressData::BillingAddressData_NAME.with(Clone::clone).clone()), crate::java_into_object(self.billingName.clone()));
            jcall!(orderHashMap).put(crate::java_into_object(crate::org::allbinary::business::user::address::BillingAddressData::BillingAddressData_STREET.with(Clone::clone).clone()), crate::java_into_object(self.billingStreet.clone()));
            jcall!(orderHashMap).put(crate::java_into_object(crate::org::allbinary::business::user::address::BillingAddressData::BillingAddressData_CITY.with(Clone::clone).clone()), crate::java_into_object(self.billingCity.clone()));
            jcall!(orderHashMap).put(crate::java_into_object(crate::org::allbinary::business::user::address::BillingAddressData::BillingAddressData_STATE.with(Clone::clone).clone()), crate::java_into_object(self.billingState.clone()));
            jcall!(orderHashMap).put(crate::java_into_object(crate::org::allbinary::business::user::address::BillingAddressData::BillingAddressData_CODE.with(Clone::clone).clone()), crate::java_into_object(self.billingCode.clone()));
            jcall!(orderHashMap).put(crate::java_into_object(crate::org::allbinary::business::user::address::BillingAddressData::BillingAddressData_COUNTRY.with(Clone::clone).clone()), crate::java_into_object(self.billingCountry.clone()));
            jcall!(orderHashMap).put(crate::java_into_object(crate::org::allbinary::business::user::address::ShippingAddressData::ShippingAddressData_NAME.with(Clone::clone).clone()), crate::java_into_object(self.shippingName.clone()));
            jcall!(orderHashMap).put(crate::java_into_object(crate::org::allbinary::business::user::address::ShippingAddressData::ShippingAddressData_STREET.with(Clone::clone).clone()), crate::java_into_object(self.shippingStreet.clone()));
            jcall!(orderHashMap).put(crate::java_into_object(crate::org::allbinary::business::user::address::ShippingAddressData::ShippingAddressData_CITY.with(Clone::clone).clone()), crate::java_into_object(self.shippingCity.clone()));
            jcall!(orderHashMap).put(crate::java_into_object(crate::org::allbinary::business::user::address::ShippingAddressData::ShippingAddressData_STATE.with(Clone::clone).clone()), crate::java_into_object(self.shippingState.clone()));
            jcall!(orderHashMap).put(crate::java_into_object(crate::org::allbinary::business::user::address::ShippingAddressData::ShippingAddressData_CODE.with(Clone::clone).clone()), crate::java_into_object(self.shippingCode.clone()));
            jcall!(orderHashMap).put(crate::java_into_object(crate::org::allbinary::business::user::address::ShippingAddressData::ShippingAddressData_COUNTRY.with(Clone::clone).clone()), crate::java_into_object(self.shippingCountry.clone()));
            jcall!(orderHashMap).put(crate::java_into_object(crate::org::allbinary::business::user::commerce::shipping::ShippingMethodData::ShippingMethodData_NAME.with(Clone::clone).clone()), crate::java_into_object(self.shippingMethodName.clone()));
            jcall!(orderHashMap).put(crate::java_into_object(crate::org::allbinary::business::user::commerce::inventory::order::OrderHistoryData::OrderHistoryData_SUBTOTAL.with(Clone::clone).clone()), crate::java_into_object(self.subTotal.clone()));
            jcall!(orderHashMap).put(crate::java_into_object(crate::org::allbinary::business::user::commerce::inventory::order::OrderHistoryData::OrderHistoryData_SHIPPINGCOST.with(Clone::clone).clone()), crate::java_into_object(self.shippingCost.clone()));
            jcall!(orderHashMap).put(crate::java_into_object(crate::org::allbinary::business::user::commerce::inventory::order::OrderHistoryData::OrderHistoryData_TAX.with(Clone::clone).clone()), crate::java_into_object(self.tax.clone()));
            jcall!(orderHashMap).put(crate::java_into_object(crate::org::allbinary::business::user::commerce::inventory::order::OrderHistoryData::OrderHistoryData_TOTAL.with(Clone::clone).clone()), crate::java_into_object(self.total.clone()));
            jcall!(orderHashMap).put(crate::java_into_object({ let java_field = jcall!(EntryData::getInstance()).SPECIAL.clone(); java_field }.clone()), crate::java_into_object(self.special.clone()));
            jcall!(orderHashMap).put(crate::java_into_object(crate::org::allbinary::business::user::commerce::inventory::order::OrderData::OrderData_CUSTOMERCOMMENT.with(Clone::clone).clone()), crate::java_into_object(self.userComment.clone()));
            jcall!(orderHashMap).put(crate::java_into_object(crate::org::allbinary::business::user::commerce::inventory::order::OrderData::OrderData_CUSTOMERCANCELCOMMENT.with(Clone::clone).clone()), crate::java_into_object(self.userCancelComment.clone()));
            jcall!(orderHashMap).put(crate::java_into_object(crate::org::allbinary::business::user::commerce::inventory::order::OrderData::OrderData_STORECOMMENT.with(Clone::clone).clone()), crate::java_into_object(self.storeComment.clone()));
            jcall!(orderHashMap).put(crate::java_into_object(crate::org::allbinary::business::user::commerce::inventory::order::OrderData::OrderData_STORECANCELCOMMENT.with(Clone::clone).clone()), crate::java_into_object(self.storeCancelComment.clone()));
            jcall!(whereHashMap).put(crate::java_into_object(crate::org::allbinary::business::user::commerce::inventory::order::OrderData::OrderData_ID.with(Clone::clone).clone()), crate::java_into_object(self.orderId.clone()));
            jcall!(OrderHistoryEntityFactory::getInstance()).update(whereHashMap.clone(), orderHashMap.clone());
            if jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypes.clone(); java_field }).LOGGING.clone(); java_field }).contains({ let java_field = jcall!(jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypeFactory.clone(); java_field }).getInstance()).SQLTAGS.clone(); java_field }.clone()) {
                jcall!(self.logUtil).putF(success.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("update()"));
            }
            return Some(success.clone());
        })) {
            Ok(java_return) => java_return,
            Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            let mut error: String = String::from("Failed to update order table");
            if jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypes.clone(); java_field }).LOGGING.clone(); java_field }).contains({ let java_field = jcall!(jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypeFactory.clone(); java_field }).getInstance()).SQLTAGSERROR.clone(); java_field }.clone()) {
                jcall!(self.logUtil).put({ let java_field = jcall!(self.commonStrings).EXCEPTION.clone(); java_field }.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("update()"), crate::java_into_object(e.clone()));
            }
            return error.clone();
            None
            }
        };if let Some(java_return) = java_try_result { return java_return; }unreachable!("try statement")
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


