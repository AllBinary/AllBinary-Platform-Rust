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
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
// use crate::javax::servlet::http::HttpServletRequest::HttpServletRequest;
// use crate::javax::servlet::jsp::PageContext::PageContext;
use crate::org::allbinary::business::context::modules::storefront::StoreFrontData::StoreFrontData;
use crate::org::allbinary::business::context::modules::storefront::StoreFrontFactory::StoreFrontFactory;
use crate::org::allbinary::business::context::modules::storefront::StoreFrontInterface::StoreFrontInterface;
use crate::org::allbinary::business::user::commerce::inventory::order::Order::Order;
use crate::org::allbinary::business::user::commerce::inventory::order::OrderInterface::OrderInterface;
use crate::org::allbinary::business::user::commerce::inventory::order::OrderProcessorUtil::OrderProcessorUtil;
use crate::org::allbinary::business::user::commerce::money::payment::gateway::PaymentGatewayData::PaymentGatewayData;
use crate::org::allbinary::business::user::commerce::money::payment::gateway::PaymentGatewayInterface::PaymentGatewayInterface;
use crate::org::allbinary::business::user::commerce::money::payment::types::BasicPaymentType::BasicPaymentType;
use crate::org::allbinary::data::tables::user::commerce::money::payment::gateway::PaymentGatewayEntity::PaymentGatewayEntity;
use crate::org::allbinary::data::tables::user::commerce::money::payment::gateway::PaymentGatewayEntityFactory::PaymentGatewayEntityFactory;
use crate::org::allbinary::logic::communication::http::request::session::WeblisketSession::WeblisketSession;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::string::StringValidationUtil::StringValidationUtil;
use super::TagHelper::TagHelper;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct OrderHelper {
    pub base: TagHelper,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub weblisketSession: Option<Rc<RefCell<WeblisketSession>>>,
    pub storeFrontInterface: Option<Rc<RefCell<dyn StoreFrontInterface>>>,
    pub propertiesHashMap: Option<Rc<RefCell<HashMap>>>,
    pub pageContext: Option<Rc<RefCell<PageContext>>>,
    pub request: Option<Rc<RefCell<HttpServletRequest>>>,
}


impl OrderHelper {
    

    pub fn new_struct(mut propertiesHashMap: Option<Rc<RefCell<HashMap>>>, mut pageContext: Option<Rc<RefCell<PageContext>>>) -> Self {
        let mut this = Self {
            base: TagHelper::new_struct(),
            logUtil: None,
            weblisketSession: None,
            storeFrontInterface: None,
            propertiesHashMap: None,
            pageContext: None,
            request: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.propertiesHashMap = propertiesHashMap.clone();
        this.pageContext = pageContext.clone();
        this.request = (jcall!(pageContext).getRequest()).clone() /* as HttpServletRequest */;
        let mut storeName: String = (jcall!(propertiesHashMap).get(jcall!(StoreFrontData::getInstance()).NAME.clone())).clone() /* as String */;
        if !jcall!(StringValidationUtil::getInstance()).isEmpty(storeName.clone()) {
            this.storeFrontInterface = StoreFrontFactory::getInstance(storeName.clone());
        }
        this.weblisketSession = WeblisketSession::new2(this.propertiesHashMap.clone(), this.pageContext.clone());
        this
    }

    pub fn new(mut propertiesHashMap: Option<Rc<RefCell<HashMap>>>, mut pageContext: Option<Rc<RefCell<PageContext>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(propertiesHashMap, pageContext))))
    }
    

    pub fn setPaymentGateway(&mut self) -> Option<bool> {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut paymentGatewayBoolean: Option<bool> = false;
            let mut orderInterface: Option<Rc<RefCell<dyn OrderInterface>>> = jcall!(self.weblisketSession).getOrder();
            let mut requestPaymentGateway: String = ({ let java_receiver = self.request.clone(); let java_arguments = (jcall!(crate::org::allbinary::business::user::commerce::money::payment::gateway::PaymentGatewayData::PaymentGatewayData_NAME.with(Clone::clone)).toString(),); let java_result = jcall!(java_receiver).getParameter(java_arguments.0); java_result }).clone() /* as String */;
            if !jcall!(StringValidationUtil::getInstance()).isEmpty(requestPaymentGateway.clone()) {
                jcall!(self.weblisketSession).setPaymentMethod(requestPaymentGateway.clone());
                jcall!(orderInterface).setPaymentMethod(requestPaymentGateway.clone());
                paymentGatewayBoolean = true;
            }
            let mut paymentGatewayEntityInterface: Option<Rc<RefCell<PaymentGatewayEntity>>> = (PaymentGatewayEntityFactory::getInstance()).clone() /* as PaymentGatewayEntity */;
            let mut paymentTypeVector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = jcall!(paymentGatewayEntityInterface).findPaymentTypeVectorByStore(jcall!(self.weblisketSession).getStoreName());
            if jcall!(paymentTypeVector).size() == 1 {
                let mut paymentType: Option<Rc<RefCell<BasicPaymentType>>> = { let java_object = (jcall!(paymentTypeVector).get(0)).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<BasicPaymentType>>>>().expect("ClassCastException").clone()) };
                let mut paymentGatewayInterface: Option<Rc<RefCell<dyn PaymentGatewayInterface>>> = (jcall!(paymentGatewayEntityInterface).getPaymentGatewayInterface(jcall!(self.weblisketSession).getStoreName(), paymentType.clone())).clone() /* as PaymentGatewayInterface */;
                let mut paymentGateway: String = jcall!(paymentGatewayInterface).getName();
                jcall!(self.weblisketSession).setPaymentMethod(paymentGateway.clone());
                jcall!(orderInterface).setPaymentMethod(paymentGateway.clone());
                paymentGatewayBoolean = true;
            }
            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).SQLTAGS.clone()) {
                let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
                jcall!(stringBuffer).append(String::from("Successfully set PaymentGateway Order: "));
                jcall!(stringBuffer).append(jcall!(orderInterface).getId());
                jcall!(stringBuffer).append(String::from(" to: "));
                jcall!(stringBuffer).append(jcall!(orderInterface).getPaymentMethod());
                { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(stringBuffer).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("setPaymentGateway()"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
            }
            return paymentGatewayBoolean.clone();
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).SQLTAGSERROR.clone()) {
                let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
                jcall!(stringBuffer).append(String::from("Failed to set PaymentGateway for Order: "));
                match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    let mut orderInterface: Option<Rc<RefCell<dyn OrderInterface>>> = jcall!(self.weblisketSession).getOrder();
                    if orderInterface.is_some() {
                        jcall!(stringBuffer).append(jcall!(orderInterface).getId());
                        jcall!(stringBuffer).append(String::from(" to: "));
                        jcall!(stringBuffer).append(jcall!(orderInterface).getPaymentMethod());
                    }
                })) {
                    Ok(_) => {}
                    Err(ex) => {
            let ex: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(ex as Box<dyn std::any::Any>)));

                    jcall!(stringBuffer).append(String::from(" Exception Getting"));
                    }
                }
                { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(stringBuffer).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("setPaymentGateway()"), crate::java_into_object(e.clone()),); jcall!(java_receiver).put(java_arguments.0, java_arguments.1, java_arguments.2, java_arguments.3); };
            }
            return false;
            }
        }
    }
    

    pub fn process(&mut self) -> String {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut order: Option<Rc<RefCell<dyn OrderInterface>>> = jcall!(self.weblisketSession).getOrder();
            jcall!(order).setStoreName(jcall!(self.storeFrontInterface).getName());
            let mut result: String = jcall!(OrderProcessorUtil::getInstance()).process(jcall!(self.weblisketSession).getUserName(), (order).clone() /* as Order */);
            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).SQLTAGS.clone()) {
                jcall!(self.logUtil).putF(format!("{}{}", String::from("Successfully Processed Order: "), result), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("processOrder()"));
            }
            return result.clone();
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
            jcall!(stringBuffer).append(String::from("Failed to Process Order: "));
            match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                jcall!(stringBuffer).append(jcall!(jcall!(self.weblisketSession).getOrder()).getId());
            })) {
                Ok(_) => {}
                Err(ex) => {
            let ex: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(ex as Box<dyn std::any::Any>)));

                jcall!(stringBuffer).append(String::from(" Exception Getting Id"));
                }
            }
            let mut error: String = jcall!(stringBuffer).toString();
            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).SQLTAGSERROR.clone()) {
                jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).PROCESS.clone(), crate::java_into_object(e.clone()));
            }
            return error.clone();
            }
        }
    }
}
impl std::ops::Deref for OrderHelper {
    type Target = TagHelper;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for OrderHelper {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::admin::taghelpers::TagHelper::TagHelperDyn for OrderHelper {
    fn as_TagHelper(&self) -> &crate::admin::taghelpers::TagHelper::TagHelper { &self.base }
    fn as_TagHelper_mut(&mut self) -> &mut crate::admin::taghelpers::TagHelper::TagHelper { &mut self.base }
}
impl crate::admin::taghelpers::TagHelperInterface::TagHelperInterface for OrderHelper {
}


