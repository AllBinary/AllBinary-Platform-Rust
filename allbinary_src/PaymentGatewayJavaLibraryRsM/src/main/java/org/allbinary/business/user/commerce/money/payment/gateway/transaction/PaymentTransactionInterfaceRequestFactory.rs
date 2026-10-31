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
// package org.allbinary.business.user.commerce.money.payment.gateway.transaction
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;
use super::*;
// use crate::java::util::HashMap::HashMap;
// use crate::javax::servlet::http::HttpServletRequest::HttpServletRequest;
// use crate::javax::servlet::jsp::PageContext::PageContext;
use crate::org::allbinary::business::context::modules::storefront::StoreFrontFactory::StoreFrontFactory;
use crate::org::allbinary::business::context::modules::storefront::StoreFrontInterface::StoreFrontInterface;
use crate::org::allbinary::business::user::address::StreetAddress::StreetAddress;
use crate::org::allbinary::business::user::commerce::inventory::basket::Basket::Basket;
use crate::org::allbinary::business::user::commerce::inventory::order::OrderHistory::OrderHistory;
use crate::org::allbinary::business::user::commerce::inventory::order::OrderInterface::OrderInterface;
use crate::org::allbinary::business::user::commerce::money::Money::Money;
use crate::org::allbinary::business::user::commerce::money::payment::Payment::Payment;
use crate::org::allbinary::business::user::commerce::money::payment::PaymentData::PaymentData;
use crate::org::allbinary::business::user::commerce::money::payment::gateway::PaymentGatewayData::PaymentGatewayData;
use crate::org::allbinary::business::user::commerce::money::payment::types::TenderTypeFactory::TenderTypeFactory;
use crate::org::allbinary::business::user::commerce::money::payment::types::TransactionTypeFactory::TransactionTypeFactory;
use crate::org::allbinary::data::generator::OrderIdGenerator::OrderIdGenerator;
use crate::org::allbinary::data::tables::user::commerce::inventory::order::OrderHistoryEntityFactory::OrderHistoryEntityFactory;
use crate::org::allbinary::data::tables::user::commerce::inventory::order::OrderHistoryEntityInterface::OrderHistoryEntityInterface;
use crate::org::allbinary::logic::communication::http::request::session::WeblisketSession::WeblisketSession;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use crate::org::allbinary::logic::string::StringValidationUtil::StringValidationUtil;
use crate::org::allbinary::logic::visual::transform::info::TransformInfoHttpInterface::TransformInfoHttpInterface;
use crate::org::allbinary::logic::visual::transform::info::TransformInfoInterface::TransformInfoInterface;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use super::PaymentTransactionInterface::PaymentTransactionInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct PaymentTransactionInterfaceRequestFactory {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub commonStrings: Option<Rc<RefCell<CommonStrings>>>,
}

thread_local! { pub static PaymentTransactionInterfaceRequestFactory_instance: Option<Rc<RefCell<PaymentTransactionInterfaceRequestFactory>>> = PaymentTransactionInterfaceRequestFactory::new(); }

impl PaymentTransactionInterfaceRequestFactory {
    

    pub fn getInstance() -> Option<Rc<RefCell<PaymentTransactionInterfaceRequestFactory>>> {
        return PaymentTransactionInterfaceRequestFactory_instance.with(Clone::clone).clone();
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            logUtil: None,
            commonStrings: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.commonStrings = CommonStrings::getInstance();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    // throws Exception
    // Java overloaded method: Rust needs a different name
    pub fn getInstance1(&mut self, mut transformInfoInterface: Option<Rc<RefCell<dyn TransformInfoInterface>>>) -> Option<Rc<RefCell<dyn PaymentTransactionInterface>>> {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut httpTransformInfoInterface: Option<Rc<RefCell<dyn TransformInfoHttpInterface>>> = panic!("ClassCastException: a Rust dyn TransformInfoInterface cannot be cast to dyn TransformInfoHttpInterface");
            let mut pageContext: Option<Rc<RefCell<PageContext>>> = jcall!(httpTransformInfoInterface).getPageContext();
            let mut httpServletRequest: Option<Rc<RefCell<HttpServletRequest>>> = (jcall!(pageContext).getRequest()).clone() /* as HttpServletRequest */;
            let mut command: String = jcall!(httpServletRequest).getParameter(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).globals).GLOBALS2).ADMINCOMMAND.clone());
            if true && (command.cmp(&jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).globals).GLOBALS2).AUTHORIZEFORMEVALBODYONERROR.clone()) as i32) == 0 {
                return self.generateFromTestData(transformInfoInterface.clone());
            } else {
                let mut propertiesHashMap: Option<Rc<RefCell<HashMap>>> = jcall!(httpTransformInfoInterface).getPropertiesHashMap();
                let mut weblisketSession: Option<Rc<RefCell<WeblisketSession>>> = WeblisketSession::new2(propertiesHashMap.clone(), pageContext.clone());
                let mut storeFrontInterface: Option<Rc<RefCell<dyn StoreFrontInterface>>> = StoreFrontFactory::getInstance(jcall!(weblisketSession).getStoreName());
                let mut order: Option<Rc<RefCell<dyn OrderInterface>>> = jcall!(weblisketSession).getOrder();
                let mut orderId: String = jcall!(order).getId();
                let mut orderHistoryEntityInterface: Option<Rc<RefCell<dyn OrderHistoryEntityInterface>>> = OrderHistoryEntityFactory::getInstance().map(|reference| reference as Rc<RefCell<dyn OrderHistoryEntityInterface>>);
                let mut orderReview: Option<Rc<RefCell<OrderHistory>>> = jcall!(orderHistoryEntityInterface).getOrder(orderId.clone());
                let mut orderPaymentInfo: Option<Rc<RefCell<Payment>>> = jcall!(orderReview).getPaymentInfo();
                jcall!(orderPaymentInfo).setTransactionType(jcall!(jcall!(TransactionTypeFactory::getInstance()).SALE).toString());
                jcall!(orderPaymentInfo).setTenderType(jcall!(jcall!(TenderTypeFactory::getInstance()).CREDITCARD).toString());
                jcall!(orderReview).setPaymentInfo(orderPaymentInfo.clone());
                if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).PAYMENT.clone()) {
                    { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(orderReview).getId(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("getPaymentTransactionInterface()"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
                }
                let mut paymentTransactionInterface: Option<Rc<RefCell<dyn PaymentTransactionInterface>>> = jcall!(PaymentTransactionInterfaceFactory::PaymentTransactionInterfaceFactory::getInstance()).getInstance1(orderReview.clone());
                return paymentTransactionInterface.clone();
            }
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).PAYMENTERROR.clone()) {
                jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("getPaymentTransactionInterface()"), crate::java_into_object(e.clone()));
            }
            panic!("{:?}", e);
            }
        }
    }
    

    // throws Exception
    pub fn generateFromTestData(&mut self, mut transformInfoInterface: Option<Rc<RefCell<dyn TransformInfoInterface>>>) -> Option<Rc<RefCell<dyn PaymentTransactionInterface>>> {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut httpTransformInfoInterface: Option<Rc<RefCell<dyn TransformInfoHttpInterface>>> = panic!("ClassCastException: a Rust dyn TransformInfoInterface cannot be cast to dyn TransformInfoHttpInterface");
            let mut propertiesHashMap: Option<Rc<RefCell<HashMap>>> = jcall!(httpTransformInfoInterface).getPropertiesHashMap();
            let mut pageContext: Option<Rc<RefCell<PageContext>>> = jcall!(httpTransformInfoInterface).getPageContext();
            let mut weblisketSession: Option<Rc<RefCell<WeblisketSession>>> = WeblisketSession::new2(propertiesHashMap.clone(), pageContext.clone());
            let mut storeFrontInterface: Option<Rc<RefCell<dyn StoreFrontInterface>>> = StoreFrontFactory::getInstance(jcall!(weblisketSession).getStoreName());
            let mut httpServletRequest: Option<Rc<RefCell<HttpServletRequest>>> = (jcall!(pageContext).getRequest()).clone() /* as HttpServletRequest */;
            let mut gatewayName: String = (jcall!(weblisketSession).getPaymentMethod()).clone() /* as String */;
            if jcall!(StringValidationUtil::getInstance()).isEmpty(gatewayName.clone()) {
                gatewayName = jcall!(httpServletRequest).getParameter(jcall!(crate::org::allbinary::business::user::commerce::money::payment::gateway::PaymentGatewayData::PaymentGatewayData_NAME.with(Clone::clone)).toString());
            }
            let mut transactionType: String = jcall!(httpServletRequest).getParameter(crate::org::allbinary::business::user::commerce::money::payment::PaymentData::PaymentData_TRANSACTIONTYPE.with(Clone::clone).clone());
            let mut tenderType: String = jcall!(httpServletRequest).getParameter(crate::org::allbinary::business::user::commerce::money::payment::PaymentData::PaymentData_TENDERTYPE.with(Clone::clone).clone());
            let mut payment: Option<Rc<RefCell<Payment>>> = Payment::new();
            jcall!(payment).setTransactionType(transactionType.clone());
            jcall!(payment).setTenderType(tenderType.clone());
            let mut orderReview: Option<Rc<RefCell<OrderHistory>>> = OrderHistory::new1(Basket::new());
            jcall!(orderReview).setStoreName(jcall!(storeFrontInterface).getName());
            jcall!(orderReview).setPaymentMethod(gatewayName.clone());
            jcall!(orderReview).setUserName(String::from("testing"));
            let mut orderId: String = jcall!(httpServletRequest).getParameter(String::from("orderNumber"));
            if false || (orderId.cmp(&jcall!(StringUtil::getInstance()).EMPTY_STRING.clone()) as i32) == 0 {
                orderId = jcall!(OrderIdGenerator::new()).getNext();
                jcall!(orderReview).setId(orderId.clone());
                let mut account: String = jcall!(httpServletRequest).getParameter(String::from("account"));
                let mut expirationDate: String = jcall!(httpServletRequest).getParameter(String::from("expirationDate"));
                let mut checkNumber: String = jcall!(httpServletRequest).getParameter(String::from("checkNumber"));
                let mut driversLicense: String = jcall!(httpServletRequest).getParameter(String::from("driversLicense"));
                let mut magneticInkCheckReader: String = jcall!(httpServletRequest).getParameter(String::from("magneticInkCheckReader"));
                let mut aba: String = jcall!(httpServletRequest).getParameter(String::from("aba"));
                let mut accountType: String = jcall!(httpServletRequest).getParameter(String::from("accountType"));
                let mut name: String = jcall!(httpServletRequest).getParameter(String::from("name"));
                jcall!(payment).setName(name.clone());
                jcall!(payment).setNumber(account.clone());
                jcall!(payment).setExpiration(expirationDate.clone());
                jcall!(payment).setCheckNumber(checkNumber.clone());
                jcall!(payment).setDriversLicense(driversLicense.clone());
                jcall!(payment).setMagneticInkCheckReader(magneticInkCheckReader.clone());
                jcall!(payment).setAba(aba.clone());
                jcall!(payment).setAccountType(accountType.clone());
                let mut street: String = jcall!(httpServletRequest).getParameter(String::from("street"));
                let mut city: String = jcall!(httpServletRequest).getParameter(String::from("city"));
                let mut state: String = jcall!(httpServletRequest).getParameter(String::from("state"));
                let mut zip: String = jcall!(httpServletRequest).getParameter(String::from("zip"));
                let mut streetAddress: Option<Rc<RefCell<StreetAddress>>> = StreetAddress::new();
                jcall!(streetAddress).setName(name.clone());
                jcall!(streetAddress).setStreet(street.clone());
                jcall!(streetAddress).setCity(city.clone());
                jcall!(streetAddress).setState(state.clone());
                jcall!(streetAddress).setCode(zip.clone());
                let mut email: String = jcall!(httpServletRequest).getParameter(String::from("email"));
                let mut amount: String = jcall!(httpServletRequest).getParameter(String::from("amount"));
                jcall!(orderReview).setTotal(Money::new1(amount.clone()));
                jcall!(orderReview).setBillingAddress(streetAddress.clone());
            } else {
                jcall!(orderReview).setId(orderId.clone());
            }
            jcall!(orderReview).setPaymentInfo(payment.clone());
            let mut paymentTransactionInterface: Option<Rc<RefCell<dyn PaymentTransactionInterface>>> = jcall!(PaymentTransactionInterfaceFactory::PaymentTransactionInterfaceFactory::getInstance()).getInstance1(orderReview.clone());
            return paymentTransactionInterface.clone();
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).PAYMENTERROR.clone()) {
                jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("generatePaymentTransactionInterfaceFromTestData()"), crate::java_into_object(e.clone()));
            }
            panic!("{:?}", e);
            }
        }
    }
}

