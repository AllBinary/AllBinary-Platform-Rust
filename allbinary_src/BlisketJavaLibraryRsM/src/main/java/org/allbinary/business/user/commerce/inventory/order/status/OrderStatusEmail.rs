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
// package org.allbinary.business.user.commerce.inventory.order.status
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::business::context::modules::storefront::StoreFrontFactory::StoreFrontFactory;
use crate::org::allbinary::business::context::modules::storefront::StoreFrontInterface::StoreFrontInterface;
use crate::org::allbinary::business::user::UserInterface::UserInterface;
use crate::org::allbinary::business::user::commerce::inventory::order::OrderHistory::OrderHistory;
use crate::org::allbinary::data::tables::user::UserEntityFactory::UserEntityFactory;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::communication::smtp::event::UserEmailEventNameData::UserEmailEventNameData;
use crate::org::allbinary::logic::communication::smtp::event::handler::UserEmailEventHandler::UserEmailEventHandler;
use crate::org::allbinary::logic::communication::smtp::event::handler::factory::AdminUserEmailEventHandlerSingletons::AdminUserEmailEventHandlerSingletons;
use crate::org::allbinary::logic::communication::smtp::event::handler::factory::StoreAdminUserEmailEventHandlerSingletons::StoreAdminUserEmailEventHandlerSingletons;
use crate::org::allbinary::logic::communication::smtp::event::handler::factory::UserEmailEventHandlerSingletons::UserEmailEventHandlerSingletons;
use crate::org::allbinary::logic::communication::smtp::info::AdminEmailInfo::AdminEmailInfo;
use crate::org::allbinary::logic::communication::smtp::info::BasicEmailInfo::BasicEmailInfo;
use crate::org::allbinary::logic::communication::smtp::info::EmailInfo::EmailInfo;
use crate::org::allbinary::logic::communication::smtp::info::StoreEmailInfo::StoreEmailInfo;
use crate::org::allbinary::logic::system::security::licensing::AbeClientInformationInterface::AbeClientInformationInterface;
use crate::org::allbinary::string::CommonStrings::CommonStrings;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct OrderStatusEmail {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub commonStrings: Option<Rc<RefCell<CommonStrings>>>,
    pub abeClientInformation: Option<Rc<RefCell<dyn AbeClientInformationInterface>>>,
    pub orderHistory: Option<Rc<RefCell<OrderHistory>>>,
    pub storeFrontInterface: Option<Rc<RefCell<dyn StoreFrontInterface>>>,
    pub user: Option<Rc<RefCell<dyn UserInterface>>>,
}


impl OrderStatusEmail {
    

    pub fn new_struct(mut abeClientInformation: Option<Rc<RefCell<dyn AbeClientInformationInterface>>>, mut orderHistory: Option<Rc<RefCell<OrderHistory>>>) -> Self {
        let mut this = Self {
            logUtil: None,
            commonStrings: None,
            abeClientInformation: None,
            orderHistory: None,
            storeFrontInterface: None,
            user: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.commonStrings = CommonStrings::getInstance();
        this.abeClientInformation = abeClientInformation.clone();
        this.orderHistory = orderHistory.clone();
        let mut storeName: String = jcall!(orderHistory).getStoreName();
        this.storeFrontInterface = StoreFrontFactory::getInstance(storeName.clone());
        this.user = jcall!(UserEntityFactory::getInstance()).getUser(jcall!(orderHistory).getUserName());
        this
    }

    pub fn new(mut abeClientInformation: Option<Rc<RefCell<dyn AbeClientInformationInterface>>>, mut orderHistory: Option<Rc<RefCell<OrderHistory>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(abeClientInformation, orderHistory))))
    }
    

    // throws Exception
    pub fn process(&mut self) {
        self.notifyStoreAdmin();
        self.notifyUser();
    }
    

    // throws Exception
    pub fn notifyStoreAdmin(&mut self) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).EMAILLOGGING.clone()) {
                jcall!(self.logUtil).putF(String::from("Order Email"), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("emailStoreAdmin"));
            }
            let mut adminEmailSubject: String = format!("{}{}", format!("{}{}", String::from("Order "), jcall!(self.orderHistory).getStatus()), String::from(" Attempt"));
            let mut adminEmailTextBody: String = format!("{}{}", format!("{}{}", format!("{}{}", String::from("Order attempt: "), jcall!(self.orderHistory).getId()), String::from(" Preprocessing by: ")), jcall!(self.user).getUserName());
            let mut adminBasicEmailInfo: Option<Rc<RefCell<dyn crate::org::allbinary::logic::communication::smtp::info::BasicEmailInfo::BasicEmailInfoDyn>>> = (AdminEmailInfo::new(adminEmailSubject.clone(), adminEmailTextBody.clone())).clone() /* as BasicEmailInfo */;
            let mut storeAdminBasicEmailInfo: Option<Rc<RefCell<dyn crate::org::allbinary::logic::communication::smtp::info::BasicEmailInfo::BasicEmailInfoDyn>>> = (StoreEmailInfo::new(self.storeFrontInterface.clone(), adminEmailSubject.clone(), adminEmailTextBody.clone())).clone() /* as BasicEmailInfo */;
            let mut storeAdminEmailInfo: Option<Rc<RefCell<EmailInfo>>> = EmailInfo::new1(storeAdminBasicEmailInfo.clone());
            let mut adminEmailInfo: Option<Rc<RefCell<EmailInfo>>> = EmailInfo::new1(adminBasicEmailInfo.clone());
            let mut userEmailEventNameData: Option<Rc<RefCell<UserEmailEventNameData>>> = UserEmailEventNameData::getInstance(jcall!(self.orderHistory).getStatus());
            let mut adminUserEmailEventHandler: Option<Rc<RefCell<UserEmailEventHandler>>> = jcall!(AdminUserEmailEventHandlerSingletons::getInstance()).getInstance2(self.abeClientInformation.clone(), userEmailEventNameData.clone());
            let mut storeAdminUserEmailEventHandler: Option<Rc<RefCell<UserEmailEventHandler>>> = jcall!(StoreAdminUserEmailEventHandlerSingletons::getInstance()).getInstance3(userEmailEventNameData.clone(), self.abeClientInformation.clone(), self.storeFrontInterface.clone());
            jcall!(storeAdminUserEmailEventHandler).receiveEmailInfo(userEmailEventNameData.clone(), storeAdminEmailInfo.clone());
            jcall!(adminUserEmailEventHandler).receiveEmailInfo(userEmailEventNameData.clone(), adminEmailInfo.clone());
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).EMAILLOGGINGERROR.clone()) {
                jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("emailAdmin"), crate::java_into_object(e.clone()));
            }
            }
        }
    }
    

    // throws Exception
    pub fn notifyUser(&mut self) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).EMAILLOGGING.clone()) {
                jcall!(self.logUtil).putF(String::from("Email User"), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("notifyUser()"));
            }
            let mut subject: String = format!("{}{}", format!("{}{}", String::from("Order "), jcall!(self.orderHistory).getStatus()), String::from(" Attempt"));
            let mut textBody: String = format!("{}{}", format!("{}{}", format!("{}{}", String::from("Order attempt: "), jcall!(self.orderHistory).getId()), String::from(" Preprocessing by: ")), jcall!(self.user).getUserName());
            let mut basicEmailInfo: Option<Rc<RefCell<dyn crate::org::allbinary::logic::communication::smtp::info::BasicEmailInfo::BasicEmailInfoDyn>>> = (StoreEmailInfo::new(self.storeFrontInterface.clone(), subject.clone(), textBody.clone())).clone() /* as BasicEmailInfo */;
            let mut emailInfo: Option<Rc<RefCell<EmailInfo>>> = EmailInfo::new1(basicEmailInfo.clone());
            let mut userEmailEventNameData: Option<Rc<RefCell<UserEmailEventNameData>>> = UserEmailEventNameData::getInstance(jcall!(self.orderHistory).getStatus());
            let mut userEmailEventHandler: Option<Rc<RefCell<UserEmailEventHandler>>> = jcall!(UserEmailEventHandlerSingletons::getInstance()).getInstance3(self.abeClientInformation.clone(), userEmailEventNameData.clone(), self.user.clone());
            jcall!(userEmailEventHandler).receiveEmailInfo(userEmailEventNameData.clone(), emailInfo.clone());
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).EMAILLOGGINGERROR.clone()) {
                jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("notifyUser"), crate::java_into_object(e.clone()));
            }
            panic!("{:?}", e);
            }
        }
    }
}

