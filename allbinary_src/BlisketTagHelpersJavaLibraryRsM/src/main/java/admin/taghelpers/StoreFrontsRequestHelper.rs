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
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
// use crate::javax::servlet::http::HttpServletRequest::HttpServletRequest;
// use crate::javax::servlet::jsp::PageContext::PageContext;
use crate::org::allbinary::business::context::modules::storefront::StoreFront::StoreFront;
use crate::org::allbinary::business::context::modules::storefront::StoreFrontInterface::StoreFrontInterface;
use crate::org::allbinary::business::installer::Portion::Portion;
use crate::org::allbinary::data::tables::context::module::storefronts::StoreFrontsEntityFactory::StoreFrontsEntityFactory;
use crate::org::allbinary::logic::communication::http::request::session::WeblisketSession::WeblisketSession;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::communication::smtp::event::UserEmailEventNameData::UserEmailEventNameData;
use crate::org::allbinary::logic::communication::smtp::event::handler::UserEmailEventHandler::UserEmailEventHandler;
use crate::org::allbinary::logic::communication::smtp::event::handler::factory::AdminUserEmailEventHandlerSingletons::AdminUserEmailEventHandlerSingletons;
use crate::org::allbinary::logic::communication::smtp::event::handler::factory::StoreAdminUserEmailEventHandlerSingletons::StoreAdminUserEmailEventHandlerSingletons;
use crate::org::allbinary::logic::communication::smtp::info::AdminEmailInfo::AdminEmailInfo;
use crate::org::allbinary::logic::communication::smtp::info::BasicEmailInfo::BasicEmailInfo;
use crate::org::allbinary::logic::communication::smtp::info::EmailInfo::EmailInfo;
use crate::org::allbinary::logic::communication::smtp::info::StoreEmailInfo::StoreEmailInfo;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::system::security::licensing::AbeClientInformationInterface::AbeClientInformationInterface;
use crate::org::allbinary::logic::system::security::licensing::ServiceClientInformationInterfaceFactory::ServiceClientInformationInterfaceFactory;
use crate::java::lang::Exception::*;
use super::ModifyTable::ModifyTable;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct StoreFrontsRequestHelper {
    pub base: ModifyTable,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub abeClientInformation: Option<Rc<RefCell<dyn AbeClientInformationInterface>>>,
    pub pageContext: Option<Rc<RefCell<PageContext>>>,
    pub request: Option<Rc<RefCell<HttpServletRequest>>>,
    pub weblisketSession: Option<Rc<RefCell<WeblisketSession>>>,
    pub storeName: String,
    pub modifyingStoreFrontInterface: Option<Rc<RefCell<dyn StoreFrontInterface>>>,
    pub portion: Option<Rc<RefCell<Portion>>>,
}


impl StoreFrontsRequestHelper {
    

    pub fn new_struct(mut hashMap: Option<Rc<RefCell<HashMap<crate::JavaObject, crate::JavaObject>>>>, mut pageContext: Option<Rc<RefCell<PageContext>>>) -> Self {
        let mut this = Self {
            base: ModifyTable::new_struct(),
            logUtil: None,
            abeClientInformation: None,
            pageContext: None,
            request: None,
            weblisketSession: None,
            storeName: String::new(),
            modifyingStoreFrontInterface: None,
            portion: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.abeClientInformation = ServiceClientInformationInterfaceFactory::getInstance();
        this.pageContext = pageContext.clone();
        this.request = { let java_value = ((jcall!(pageContext).getRequest()).clone() /* as HttpServletRequest */).clone(); java_value };
        this.weblisketSession = WeblisketSession::new2(hashMap.clone(), pageContext.clone());
        this.portion = Portion::new(hashMap.clone());
        this.getFormData();
        this
    }

    pub fn new(mut hashMap: Option<Rc<RefCell<HashMap<crate::JavaObject, crate::JavaObject>>>>, mut pageContext: Option<Rc<RefCell<PageContext>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(hashMap, pageContext))))
    }
    

    // throws Exception
    pub fn getFormData(&mut self) {
        self.storeName = { let java_value = (jcall!(self.weblisketSession).getStoreName()).clone(); java_value };
        self.modifyingStoreFrontInterface = (StoreFront::new1(self.request.clone())).clone() /* as StoreFrontInterface */;
    }
    

    pub fn update(&mut self) -> String {
        let java_try_result = match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut success: String = String::from("Updated Successfully");
            let mut hashMapData: Option<Rc<RefCell<HashMap<crate::JavaObject, crate::JavaObject>>>> = jcall!(self.modifyingStoreFrontInterface).toHashMap();
            jcall!(jcall!(StoreFrontsEntityFactory::getInstance()).getStoreFrontsEntityInstance()).update(hashMapData.clone());
            if jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypes.clone(); java_field }).LOGGING.clone(); java_field }).contains({ let java_field = jcall!(jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypeFactory.clone(); java_field }).getInstance()).SQLTAGS.clone(); java_field }.clone()) {
                jcall!(self.logUtil).putF(success.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("update()"));
            }
            return Some(success.clone());
        })) {
            Ok(java_return) => java_return,
            Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            let mut error: String = String::from("Failed to update storefronts table");
            if jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypes.clone(); java_field }).LOGGING.clone(); java_field }).contains({ let java_field = jcall!(jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypeFactory.clone(); java_field }).getInstance()).SQLTAGSERROR.clone(); java_field }.clone()) {
                jcall!(self.logUtil).put({ let java_field = jcall!(self.commonStrings).EXCEPTION.clone(); java_field }.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("update()"), crate::java_into_object(e.clone()));
            }
            return error.clone();
            None
            }
        };if let Some(java_return) = java_try_result { return java_return; }unreachable!("try statement")
    }
    

    // throws Exception
    pub fn sendStoreCreatedEmails(&mut self) {
        let mut storeManagerEmailSubject: String = format!("{}{}", String::from("Store Manager Nofification For Store: "), jcall!(self.modifyingStoreFrontInterface).getName());
        let mut adminEmailSubject: String = format!("{}{}", String::from("Admin Notification For Store: "), jcall!(self.modifyingStoreFrontInterface).getName());
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        jcall!(stringBuffer).append(String::from("Store Created: "));
        jcall!(stringBuffer).append(jcall!(self.modifyingStoreFrontInterface).getName());
        jcall!(stringBuffer).append(String::from("\n\n"));
        jcall!(stringBuffer).append(String::from("Click here for the Admin Interface:\n"));
        jcall!(stringBuffer).append(jcall!(self.modifyingStoreFrontInterface).getCurrentHomeHostName());
        jcall!(stringBuffer).append(String::from("/admin/login.jsp\n\n"));
        jcall!(stringBuffer).append(String::from("Goto Your Store:\n"));
        jcall!(stringBuffer).append(jcall!(self.modifyingStoreFrontInterface).getCurrentHomeHostName());
        jcall!(stringBuffer).append(jcall!(self.modifyingStoreFrontInterface).getName());
        jcall!(stringBuffer).append(String::from("/index.jsp"));
        jcall!(stringBuffer).append(String::from("\n\n"));
        jcall!(stringBuffer).append(String::from("UserName: "));
        jcall!(stringBuffer).append(jcall!(self.weblisketSession).getUserName());
        let mut adminEmailTextBody: String = jcall!(stringBuffer).toString();
        let mut adminBasicEmailInfo: Option<Rc<RefCell<dyn crate::org::allbinary::logic::communication::smtp::info::BasicEmailInfo::BasicEmailInfoDyn>>> = (AdminEmailInfo::new(adminEmailSubject.clone(), adminEmailTextBody.clone())).clone() /* as BasicEmailInfo */;
        let mut storeAdminBasicEmailInfo: Option<Rc<RefCell<dyn crate::org::allbinary::logic::communication::smtp::info::BasicEmailInfo::BasicEmailInfoDyn>>> = (StoreEmailInfo::new(self.modifyingStoreFrontInterface.clone(), storeManagerEmailSubject.clone(), adminEmailTextBody.clone())).clone() /* as BasicEmailInfo */;
        let mut storeAdminEmailInfo: Option<Rc<RefCell<EmailInfo>>> = EmailInfo::new1(storeAdminBasicEmailInfo.clone());
        let mut adminEmailInfo: Option<Rc<RefCell<EmailInfo>>> = EmailInfo::new1(adminBasicEmailInfo.clone());
        let mut adminUserEmailEventHandler: Option<Rc<RefCell<UserEmailEventHandler>>> = jcall!(AdminUserEmailEventHandlerSingletons::getInstance()).getInstance2(self.abeClientInformation.clone(), crate::org::allbinary::logic::communication::smtp::event::UserEmailEventNameData::UserEmailEventNameData_STORECREATED.with(Clone::clone).clone());
        let mut storeAdminUserEmailEventHandler: Option<Rc<RefCell<UserEmailEventHandler>>> = jcall!(StoreAdminUserEmailEventHandlerSingletons::getInstance()).getInstance3(crate::org::allbinary::logic::communication::smtp::event::UserEmailEventNameData::UserEmailEventNameData_STORECREATED.with(Clone::clone).clone(), self.abeClientInformation.clone(), self.modifyingStoreFrontInterface.clone());
        jcall!(storeAdminUserEmailEventHandler).receiveEmailInfo(crate::org::allbinary::logic::communication::smtp::event::UserEmailEventNameData::UserEmailEventNameData_STORECREATED.with(Clone::clone).clone(), storeAdminEmailInfo.clone());
        jcall!(adminUserEmailEventHandler).receiveEmailInfo(crate::org::allbinary::logic::communication::smtp::event::UserEmailEventNameData::UserEmailEventNameData_STORECREATED.with(Clone::clone).clone(), adminEmailInfo.clone());
    }
    

    // synchronized
    pub fn install(&mut self) -> String {
        let java_try_result = match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
            jcall!(stringBuffer).append(String::from("Intall StoreFront Successfully: "));
            jcall!(stringBuffer).appendint(((jcall!(self.portion).getCurrent()).unwrap() as i32));
            jcall!(stringBuffer).append(String::from(" of "));
            jcall!(stringBuffer).appendint(((jcall!(self.portion).getTotal()).unwrap() as i32));
            let mut success: String = jcall!(stringBuffer).toString();
            { let java_receiver = self.modifyingStoreFrontInterface.clone(); let java_arguments = (((jcall!(self.portion).getCurrent()).unwrap() as i32), ((jcall!(self.portion).getTotal()).unwrap() as i32),); jcall!(java_receiver).install(java_arguments.0, java_arguments.1); };
            if jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypes.clone(); java_field }).LOGGING.clone(); java_field }).contains({ let java_field = jcall!(jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypeFactory.clone(); java_field }).getInstance()).SQLTAGS.clone(); java_field }.clone()) {
                jcall!(self.logUtil).putF(success.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("install()"));
            }
            return Some(success.clone());
        })) {
            Ok(java_return) => java_return,
            Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            let mut error: String = String::from("Failed to Install storefront");
            if jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypes.clone(); java_field }).LOGGING.clone(); java_field }).contains({ let java_field = jcall!(jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypeFactory.clone(); java_field }).getInstance()).SQLTAGSERROR.clone(); java_field }.clone()) {
                jcall!(self.logUtil).put({ let java_field = jcall!(self.commonStrings).EXCEPTION.clone(); java_field }.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("install()"), crate::java_into_object(e.clone()));
            }
            return error.clone();
            None
            }
        };if let Some(java_return) = java_try_result { return java_return; }unreachable!("try statement")
    }
    

    // synchronized
    pub fn insert(&mut self) -> String {
        let java_try_result = match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut success: String = String::from("Added Successfully");
            let mut values: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = jcall!(self.modifyingStoreFrontInterface).toVector();
            jcall!(jcall!(StoreFrontsEntityFactory::getInstance()).getStoreFrontsEntityInstance()).insert(values.clone());
            if jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypes.clone(); java_field }).LOGGING.clone(); java_field }).contains({ let java_field = jcall!(jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypeFactory.clone(); java_field }).getInstance()).SQLTAGS.clone(); java_field }.clone()) {
                jcall!(self.logUtil).putF(success.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("insert()"));
            }
            return Some(success.clone());
        })) {
            Ok(java_return) => java_return,
            Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            let mut error: String = String::from("Failed to add storefront");
            if jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypes.clone(); java_field }).LOGGING.clone(); java_field }).contains({ let java_field = jcall!(jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypeFactory.clone(); java_field }).getInstance()).SQLTAGSERROR.clone(); java_field }.clone()) {
                jcall!(self.logUtil).put({ let java_field = jcall!(self.commonStrings).EXCEPTION.clone(); java_field }.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("insert()"), crate::java_into_object(e.clone()));
            }
            return error.clone();
            None
            }
        };if let Some(java_return) = java_try_result { return java_return; }unreachable!("try statement")
    }
    

    pub fn delete(&mut self) -> String {
        let java_try_result = match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut success: String = String::from("Delete Successfully");
            jcall!(jcall!(StoreFrontsEntityFactory::getInstance()).getStoreFrontsEntityInstance()).delete(self.storeName.clone());
            if jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypes.clone(); java_field }).LOGGING.clone(); java_field }).contains({ let java_field = jcall!(jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypeFactory.clone(); java_field }).getInstance()).SQLTAGS.clone(); java_field }.clone()) {
                jcall!(self.logUtil).putF(success.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("delete()"));
            }
            return Some(success.clone());
        })) {
            Ok(java_return) => java_return,
            Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            let mut error: String = String::from("Failed to delete storefronts table");
            if jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypes.clone(); java_field }).LOGGING.clone(); java_field }).contains({ let java_field = jcall!(jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypeFactory.clone(); java_field }).getInstance()).SQLTAGSERROR.clone(); java_field }.clone()) {
                jcall!(self.logUtil).put({ let java_field = jcall!(self.commonStrings).EXCEPTION.clone(); java_field }.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("delete()"), crate::java_into_object(e.clone()));
            }
            return error.clone();
            None
            }
        };if let Some(java_return) = java_try_result { return java_return; }unreachable!("try statement")
    }
}
impl std::ops::Deref for StoreFrontsRequestHelper {
    type Target = ModifyTable;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for StoreFrontsRequestHelper {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::admin::taghelpers::ModifyTable::ModifyTableDyn for StoreFrontsRequestHelper {
    fn as_ModifyTable(&self) -> &crate::admin::taghelpers::ModifyTable::ModifyTable { &self.base }
    fn as_ModifyTable_mut(&mut self) -> &mut crate::admin::taghelpers::ModifyTable::ModifyTable { &mut self.base }
}
impl crate::admin::taghelpers::ModifyTableInterface::ModifyTableInterface for StoreFrontsRequestHelper {
    fn insert(&mut self) -> String { Self::insert(self) }
    fn delete(&mut self) -> String { Self::delete(self) }
    fn update(&mut self) -> String { Self::update(self) }
}


