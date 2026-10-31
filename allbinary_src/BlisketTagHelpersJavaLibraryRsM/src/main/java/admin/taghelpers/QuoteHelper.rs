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
// use crate::javax::servlet::jsp::PageContext::PageContext;
use crate::org::allbinary::business::context::modules::storefront::StoreFront::StoreFront;
use crate::org::allbinary::business::context::modules::storefront::StoreFrontData::StoreFrontData;
use crate::org::allbinary::business::context::modules::storefront::StoreFrontFactory::StoreFrontFactory;
use crate::org::allbinary::business::context::modules::storefront::StoreFrontInterface::StoreFrontInterface;
use crate::org::allbinary::business::installer::Portion::Portion;
use crate::org::allbinary::business::user::UserInterface::UserInterface;
use crate::org::allbinary::business::user::quoterequest::QuoteRequest::QuoteRequest;
use crate::org::allbinary::data::tables::user::UserEntityFactory::UserEntityFactory;
use crate::org::allbinary::data::tables::user::quoterequest::QuoteRequestEntity::QuoteRequestEntity;
use crate::org::allbinary::data::tables::user::quoterequest::QuoteRequestEntityFactory::QuoteRequestEntityFactory;
use crate::org::allbinary::logic::communication::http::request::session::WeblisketSession::WeblisketSession;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::communication::smtp::event::UserEmailEventNameData::UserEmailEventNameData;
use crate::org::allbinary::logic::communication::smtp::event::handler::UserEmailEventHandler::UserEmailEventHandler;
use crate::org::allbinary::logic::communication::smtp::event::handler::factory::AdminUserEmailEventHandlerSingletons::AdminUserEmailEventHandlerSingletons;
use crate::org::allbinary::logic::communication::smtp::event::handler::factory::StoreAdminUserEmailEventHandlerSingletons::StoreAdminUserEmailEventHandlerSingletons;
use crate::org::allbinary::logic::communication::smtp::event::handler::factory::UserEmailEventHandlerSingletons::UserEmailEventHandlerSingletons;
use crate::org::allbinary::logic::communication::smtp::info::BasicEmailInfo::BasicEmailInfo;
use crate::org::allbinary::logic::communication::smtp::info::EmailInfo::EmailInfo;
use crate::org::allbinary::logic::communication::smtp::info::StoreEmailInfo::StoreEmailInfo;
use crate::org::allbinary::logic::communication::sql::AbSqlTableUtil::AbSqlTableUtil;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::system::security::licensing::AbeClientInformationInterface::AbeClientInformationInterface;
use crate::org::allbinary::logic::system::security::licensing::ServiceClientInformationInterfaceFactory::ServiceClientInformationInterfaceFactory;
use super::BasicTable::BasicTable;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct QuoteHelper {
    pub base: BasicTable,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub abeClientInformation: Option<Rc<RefCell<dyn AbeClientInformationInterface>>>,
    pub weblisketSession: Option<Rc<RefCell<WeblisketSession>>>,
    pub storeFrontInterface: Option<Rc<RefCell<dyn StoreFrontInterface>>>,
    pub portion: Option<Rc<RefCell<Portion>>>,
}


impl QuoteHelper {
    

    pub fn new_struct(mut hashMap: Option<Rc<RefCell<HashMap>>>, mut pageContext: Option<Rc<RefCell<PageContext>>>) -> Self {
        let mut this = Self {
            base: BasicTable::new_struct(),
            logUtil: None,
            abeClientInformation: None,
            weblisketSession: None,
            storeFrontInterface: None,
            portion: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.abeClientInformation = ServiceClientInformationInterfaceFactory::getInstance();
        let mut storeName: String = (jcall!(hashMap).get(jcall!(StoreFrontData::getInstance()).NAME.clone())).clone() /* as String */;
        if true {
            this.storeFrontInterface = StoreFrontFactory::getInstance(storeName.clone());
        } else {
            this.storeFrontInterface = StoreFront::new().map(|reference| reference as Rc<RefCell<dyn StoreFrontInterface>>);
        }
        this.weblisketSession = WeblisketSession::new2(hashMap.clone(), pageContext.clone());
        this.portion = Portion::new(hashMap.clone());
        this
    }

    pub fn new(mut hashMap: Option<Rc<RefCell<HashMap>>>, mut pageContext: Option<Rc<RefCell<PageContext>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(hashMap, pageContext))))
    }
    

    // throws Exception
    pub fn emailUser(&mut self, mut quoteRequest: Option<Rc<RefCell<QuoteRequest>>>) {
        let mut user: Option<Rc<RefCell<dyn UserInterface>>> = jcall!(UserEntityFactory::getInstance()).getUser(jcall!(quoteRequest).getUserName());
        let mut userEmailSubject: String = String::from("Quote Request Receipt");
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        jcall!(stringBuffer).append(String::from("This is just a verification email. "));
        jcall!(stringBuffer).append(String::from("We usually respond to quote request within 24 hours."));
        jcall!(stringBuffer).append(String::from("\n\nThank You For Your Business."));
        let mut userEmailTextBody: String = jcall!(stringBuffer).toString();
        let mut basicEmailInfo: Option<Rc<RefCell<dyn crate::org::allbinary::logic::communication::smtp::info::BasicEmailInfo::BasicEmailInfoDyn>>> = (StoreEmailInfo::new(self.storeFrontInterface.clone(), userEmailSubject.clone(), userEmailTextBody.clone())).clone() /* as BasicEmailInfo */;
        let mut emailInfo: Option<Rc<RefCell<EmailInfo>>> = EmailInfo::new1(basicEmailInfo.clone());
        let mut userEmailEventHandler: Option<Rc<RefCell<UserEmailEventHandler>>> = jcall!(UserEmailEventHandlerSingletons::getInstance()).getInstance3(self.abeClientInformation.clone(), crate::org::allbinary::logic::communication::smtp::event::UserEmailEventNameData::UserEmailEventNameData_QUOTEREQUEST.with(Clone::clone).clone(), user.clone());
        jcall!(userEmailEventHandler).receiveEmailInfo(crate::org::allbinary::logic::communication::smtp::event::UserEmailEventNameData::UserEmailEventNameData_QUOTEREQUEST.with(Clone::clone).clone(), emailInfo.clone());
    }
    

    // throws Exception
    pub fn emailAdmins(&mut self, mut quoteRequest: Option<Rc<RefCell<QuoteRequest>>>) {
        let mut adminEmailSubject: String = String::from("Quote Request");
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        jcall!(stringBuffer).append(String::from("\nUserName: "));
        jcall!(stringBuffer).append(jcall!(quoteRequest).getUserName());
        jcall!(stringBuffer).append(String::from("\nProject Info: \n"));
        jcall!(stringBuffer).append(jcall!(quoteRequest).getProjectInfo());
        jcall!(stringBuffer).append(String::from("\nUser Comments: \n"));
        jcall!(stringBuffer).append(jcall!(quoteRequest).getUserComments());
        jcall!(stringBuffer).append(String::from("\nBudget: \n"));
        jcall!(stringBuffer).append(jcall!(quoteRequest).getBudget());
        jcall!(stringBuffer).append(String::from("\nTime Frame: \n"));
        jcall!(stringBuffer).append(jcall!(quoteRequest).getTimeFrame());
        jcall!(stringBuffer).append(String::from("\nComments: \n"));
        jcall!(stringBuffer).append(jcall!(quoteRequest).getComments());
        let mut adminEmailTextBody: String = jcall!(stringBuffer).toString();
        let mut basicEmailInfo: Option<Rc<RefCell<dyn crate::org::allbinary::logic::communication::smtp::info::BasicEmailInfo::BasicEmailInfoDyn>>> = (StoreEmailInfo::new(self.storeFrontInterface.clone(), adminEmailSubject.clone(), adminEmailTextBody.clone())).clone() /* as BasicEmailInfo */;
        let mut emailInfo: Option<Rc<RefCell<EmailInfo>>> = EmailInfo::new1(basicEmailInfo.clone());
        let mut storeAdminUserEmailEventHandler: Option<Rc<RefCell<UserEmailEventHandler>>> = jcall!(AdminUserEmailEventHandlerSingletons::getInstance()).getInstance2(self.abeClientInformation.clone(), crate::org::allbinary::logic::communication::smtp::event::UserEmailEventNameData::UserEmailEventNameData_QUOTEREQUEST.with(Clone::clone).clone());
        let mut adminUserEmailEventHandler: Option<Rc<RefCell<UserEmailEventHandler>>> = jcall!(StoreAdminUserEmailEventHandlerSingletons::getInstance()).getInstance3(crate::org::allbinary::logic::communication::smtp::event::UserEmailEventNameData::UserEmailEventNameData_QUOTEREQUEST.with(Clone::clone).clone(), self.abeClientInformation.clone(), self.storeFrontInterface.clone());
        jcall!(storeAdminUserEmailEventHandler).receiveEmailInfo(crate::org::allbinary::logic::communication::smtp::event::UserEmailEventNameData::UserEmailEventNameData_QUOTEREQUEST.with(Clone::clone).clone(), emailInfo.clone());
        jcall!(adminUserEmailEventHandler).receiveEmailInfo(crate::org::allbinary::logic::communication::smtp::event::UserEmailEventNameData::UserEmailEventNameData_QUOTEREQUEST.with(Clone::clone).clone(), emailInfo.clone());
    }
    

    // throws Exception
    pub fn email(&mut self) -> String {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut quoteRequestEntity: Option<Rc<RefCell<QuoteRequestEntity>>> = jcall!(QuoteRequestEntityFactory::getInstance()).getQuoteRequestEntityInstance();
            let mut userName: String = jcall!(self.weblisketSession).getUserName();
            let mut vector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = jcall!(quoteRequestEntity).getIds(userName.clone());
            let mut id: i32 = 0;
            let mut size: i32 = jcall!(vector).size();
            for index in (0..size) {
                let mut nextId: i32 = ((({ let java_object = jcall!(vector).get(index.clone()); java_object.as_ref().map(|object| { let object = object.borrow(); object.downcast_ref::<i32>().copied().or_else(|| object.downcast_ref::<Option<i32>>().copied().flatten()).expect("ClassCastException") }) })).unwrap() as i32);
                if id < nextId {
                    id = nextId.clone();
                }
            }
            let mut quoteRequest: Option<Rc<RefCell<QuoteRequest>>> = jcall!(quoteRequestEntity).get(userName.clone(), id.clone());
            if quoteRequest.is_some() {
                self.emailUser(quoteRequest.clone());
                self.emailAdmins(quoteRequest.clone());
                return String::from("Thank You For Your Business.<p/>");
            } else {
                panic!("{:?}", format!("{}: {:?}", "Exception", (String::from("No Quote Request"))));
            }
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).EMAILLOGGINGERROR.clone()) {
                jcall!(self.logUtil).put(jcall!(self.commonStrings).FAILURE.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("email"), crate::java_into_object(e.clone()));
            }
            return String::from("Thank You For Your Business.<p>");
            }
        }
    }
    

    pub fn drop(&mut self) -> String {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut success: String = jcall!(jcall!(QuoteRequestEntityFactory::getInstance()).getQuoteRequestEntityInstance()).dropTable();
            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).SQLTAGS.clone()) {
                jcall!(self.logUtil).putF(success.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).DROP.clone());
            }
            return success.clone();
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            let mut error: String = String::from("Failed to drop QuoteRequest table");
            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).SQLTAGSERROR.clone()) {
                jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).DROP.clone(), crate::java_into_object(e.clone()));
            }
            return error.clone();
            }
        }
    }
    

    pub fn create(&mut self) -> String {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut success: String = jcall!(jcall!(QuoteRequestEntityFactory::getInstance()).getQuoteRequestEntityInstance()).createTable();
            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).SQLTAGS.clone()) {
                jcall!(self.logUtil).putF(success.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("create()"));
            }
            return success.clone();
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            let mut error: String = String::from("Failed to create new QuoteRequest table");
            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).SQLTAGSERROR.clone()) {
                jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("create()"), crate::java_into_object(e.clone()));
            }
            return error.clone();
            }
        }
    }
    

    pub fn restore(&mut self) -> String {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut success: String = String::from("Restore Successful");
            let mut result: String = jcall!(AbSqlTableUtil::getInstance()).restoreTable(jcall!(QuoteRequestEntityFactory::getInstance()).getQuoteRequestEntityInstance(), self.portion.clone());
            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).SQLTAGS.clone()) {
                jcall!(self.logUtil).putF(success.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("restore()"));
            }
            return result.clone();
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            let mut error: String = String::from("Failed to restore backup");
            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).SQLTAGSERROR.clone()) {
                jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("restore()"), crate::java_into_object(e.clone()));
            }
            return error.clone();
            }
        }
    }
    

    pub fn backup(&mut self) -> String {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut success: String = String::from("Restore Successful");
            let mut result: String = jcall!(AbSqlTableUtil::getInstance()).backupTable(jcall!(QuoteRequestEntityFactory::getInstance()).getQuoteRequestEntityInstance());
            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).SQLTAGS.clone()) {
                jcall!(self.logUtil).putF(success.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("backup()"));
            }
            return result.clone();
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            let mut error: String = String::from("Failed to make backup");
            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).SQLTAGSERROR.clone()) {
                jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("backup()"), crate::java_into_object(e.clone()));
            }
            return error.clone();
            }
        }
    }
}
impl std::ops::Deref for QuoteHelper {
    type Target = BasicTable;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for QuoteHelper {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::admin::taghelpers::BasicTable::BasicTableDyn for QuoteHelper {
    fn as_BasicTable(&self) -> &crate::admin::taghelpers::BasicTable::BasicTable { &self.base }
    fn as_BasicTable_mut(&mut self) -> &mut crate::admin::taghelpers::BasicTable::BasicTable { &mut self.base }
}
impl crate::admin::taghelpers::BasicTableInterface::BasicTableInterface for QuoteHelper {
    fn drop(&mut self) -> String { Self::drop(self) }
    fn create(&mut self) -> String { Self::create(self) }
    fn restore(&mut self) -> String { Self::restore(self) }
    fn backup(&mut self) -> String { Self::backup(self) }
}


