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
use crate::org::allbinary::business::context::AbContext::AbContext;
use crate::org::allbinary::business::context::modules::storefront::StoreFrontData::StoreFrontData;
use crate::org::allbinary::business::context::modules::storefront::StoreFrontFactory::StoreFrontFactory;
use crate::org::allbinary::business::context::modules::storefront::StoreFrontInterface::StoreFrontInterface;
use crate::org::allbinary::globals::URLGLOBALS::URLGLOBALS;
use crate::org::allbinary::logic::communication::http::AcceptableResponseGenerator::AcceptableResponseGenerator;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::communication::smtp::event::UserEmailEventNameData::UserEmailEventNameData;
use crate::org::allbinary::logic::communication::smtp::event::handler::UserEmailEventHandler::UserEmailEventHandler;
use crate::org::allbinary::logic::communication::smtp::event::handler::factory::AdminUserEmailEventHandlerSingletons::AdminUserEmailEventHandlerSingletons;
use crate::org::allbinary::logic::communication::smtp::event::handler::factory::StoreAdminUserEmailEventHandlerSingletons::StoreAdminUserEmailEventHandlerSingletons;
use crate::org::allbinary::logic::communication::smtp::info::AdminEmailInfo::AdminEmailInfo;
use crate::org::allbinary::logic::communication::smtp::info::BasicEmailInfo::BasicEmailInfo;
use crate::org::allbinary::logic::communication::smtp::info::EmailInfo::EmailInfo;
use crate::org::allbinary::logic::communication::smtp::info::StoreEmailInfo::StoreEmailInfo;
use crate::org::allbinary::logic::control::search::SearchParams::SearchParams;
use crate::org::allbinary::logic::control::search::SearchRequest::SearchRequest;
use crate::org::allbinary::logic::io::file::FileUtil::FileUtil;
use crate::org::allbinary::logic::io::path::AbPath::AbPath;
use crate::org::allbinary::logic::io::path::AbPathData::AbPathData;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use crate::org::allbinary::logic::system::security::licensing::AbeClientInformationInterface::AbeClientInformationInterface;
use crate::org::allbinary::logic::system::security::licensing::ServiceClientInformationInterfaceFactory::ServiceClientInformationInterfaceFactory;
use crate::org::allbinary::logic::visual::transform::info::TransformInfoData::TransformInfoData;
use crate::views::admin::inventory::listings::ProductListingFactory::ProductListingFactory;
use super::TagHelperInterface::TagHelperInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct StaticPagesRequestHelper {
    pub base: AbContext,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub abeClientInformation: Option<Rc<RefCell<dyn AbeClientInformationInterface>>>,
    pub request: Option<Rc<RefCell<HttpServletRequest>>>,
    pub storeName: String,
    pub searchParams: Option<Rc<RefCell<SearchParams>>>,
    pub xslFile: String,
}

impl TagHelperInterface for StaticPagesRequestHelper {
}

impl StaticPagesRequestHelper {
    

    pub fn new_struct(mut propertiesHashMap: Option<Rc<RefCell<HashMap>>>, mut pageContext: Option<Rc<RefCell<PageContext>>>) -> Self {
        let mut this = Self {
            base: AbContext::new_struct(propertiesHashMap.clone(), pageContext.clone()),
            logUtil: None,
            abeClientInformation: None,
            request: None,
            storeName: String::new(),
            searchParams: None,
            xslFile: String::new(),
        };
        this.logUtil = LogUtil::getInstance();
        this.abeClientInformation = ServiceClientInformationInterfaceFactory::getInstance();
        // super(...) is in the struct literal above
        this.request = (jcall!(pageContext).getRequest()).clone() /* as HttpServletRequest */;
        this.xslFile = (jcall!(propertiesHashMap).get(jcall!(TransformInfoData::getInstance()).TEMPLATEFILENAME.clone())).clone() /* as String */;
        this.getFormData();
        this
    }

    pub fn new(mut propertiesHashMap: Option<Rc<RefCell<HashMap>>>, mut pageContext: Option<Rc<RefCell<PageContext>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(propertiesHashMap, pageContext))))
    }
    

    pub fn getFormData(&mut self) {
        self.storeName = { let java_receiver = self.request.clone(); let java_arguments = (jcall!(StoreFrontData::getInstance()).NAME.clone(),); let java_result = jcall!(java_receiver).getParameter(java_arguments.0); java_result };
        self.searchParams = SearchParams::new(self.request.clone());
    }
    

    // throws Exception
    pub fn email(&mut self) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).EMAILLOGGING.clone()) {
                jcall!(self.logUtil).putF(String::from("Generated Static Pages Notification Email"), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("email"));
            }
            let mut storeFrontInterface: Option<Rc<RefCell<dyn StoreFrontInterface>>> = StoreFrontFactory::getInstance(self.storeName.clone());
            let mut adminEmailSubject: String = String::from("Generated Static Pages Email Notification");
            let mut adminEmailTextBody: String = format!("{}{}", String::from("Generated static pages for store: "), self.storeName);
            let mut adminBasicEmailInfo: Option<Rc<RefCell<dyn crate::org::allbinary::logic::communication::smtp::info::BasicEmailInfo::BasicEmailInfoDyn>>> = (AdminEmailInfo::new(adminEmailSubject.clone(), adminEmailTextBody.clone())).clone() /* as BasicEmailInfo */;
            let mut storeAdminBasicEmailInfo: Option<Rc<RefCell<dyn crate::org::allbinary::logic::communication::smtp::info::BasicEmailInfo::BasicEmailInfoDyn>>> = (StoreEmailInfo::new(storeFrontInterface.clone(), adminEmailSubject.clone(), adminEmailTextBody.clone())).clone() /* as BasicEmailInfo */;
            let mut storeAdminEmailInfo: Option<Rc<RefCell<EmailInfo>>> = EmailInfo::new1(storeAdminBasicEmailInfo.clone());
            let mut adminEmailInfo: Option<Rc<RefCell<EmailInfo>>> = EmailInfo::new1(adminBasicEmailInfo.clone());
            let mut adminUserEmailEventHandler: Option<Rc<RefCell<UserEmailEventHandler>>> = jcall!(AdminUserEmailEventHandlerSingletons::getInstance()).getInstance2(self.abeClientInformation.clone(), crate::org::allbinary::logic::communication::smtp::event::UserEmailEventNameData::UserEmailEventNameData_STOREGENERATINGSTATICPAGES.with(Clone::clone).clone());
            let mut storeAdminUserEmailEventHandler: Option<Rc<RefCell<UserEmailEventHandler>>> = jcall!(StoreAdminUserEmailEventHandlerSingletons::getInstance()).getInstance3(crate::org::allbinary::logic::communication::smtp::event::UserEmailEventNameData::UserEmailEventNameData_STOREGENERATINGSTATICPAGES.with(Clone::clone).clone(), self.abeClientInformation.clone(), storeFrontInterface.clone());
            jcall!(storeAdminUserEmailEventHandler).receiveEmailInfo(crate::org::allbinary::logic::communication::smtp::event::UserEmailEventNameData::UserEmailEventNameData_STOREGENERATINGSTATICPAGES.with(Clone::clone).clone(), storeAdminEmailInfo.clone());
            jcall!(adminUserEmailEventHandler).receiveEmailInfo(crate::org::allbinary::logic::communication::smtp::event::UserEmailEventNameData::UserEmailEventNameData_STOREGENERATINGSTATICPAGES.with(Clone::clone).clone(), adminEmailInfo.clone());
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).EMAILLOGGINGERROR.clone()) {
                jcall!(self.logUtil).put(jcall!(self.commonStrings).FAILURE.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("email"), crate::java_into_object(e.clone()));
            }
            }
        }
    }
    

    pub fn generateStaticPages(&mut self) -> String {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut contentType: String = jcall!(AcceptableResponseGenerator::getInstance()).get(self.request.clone());
            let mut searchRequest: Option<Rc<RefCell<SearchRequest>>> = SearchRequest::new(None, self.searchParams.clone(), self.xslFile.clone(), contentType.clone(), self.getPropertiesHashMap(), self.getPageContext());
            let mut success: String = jcall!(ProductListingFactory::getInstance(searchRequest.clone())).generateAll(self.storeName.clone());
            self.email();
            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).SQLTAGS.clone()) {
                jcall!(self.logUtil).putF(success.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("generateStaticPages()"));
            }
            return success.clone();
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            let mut error: String = String::from("Failed to generate staticpages table");
            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).SQLTAGSERROR.clone()) {
                jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("generateStaticPages()"), crate::java_into_object(e.clone()));
            }
            return error.clone();
            }
        }
    }
    

    pub fn makePublic(&mut self) -> String {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut storeFrontInterface: Option<Rc<RefCell<dyn StoreFrontInterface>>> = StoreFrontFactory::getInstance(self.storeName.clone());
            let mut fromAbPath: Option<Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>> = AbPath::new(jcall!(storeFrontInterface).getTestHtmlPath() + jcall!(storeFrontInterface).getStaticPath(), jcall!(StringUtil::getInstance()).EMPTY_STRING.clone()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>);
            let mut toAbPath: Option<Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>> = AbPath::new(format!("{}{}", format!("{}{}", format!("{}{}", URLGLOBALS::getWebappPath(), jcall!(storeFrontInterface).getName()), jcall!(AbPathData::getInstance()).SEPARATOR), jcall!(storeFrontInterface).getStaticPath()), jcall!(StringUtil::getInstance()).EMPTY_STRING.clone()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>);
            jcall!(FileUtil::getInstance()).copy(fromAbPath.clone(), toAbPath.clone());
            let mut success: String = String::from("Made Public");
            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).SQLTAGS.clone()) {
                jcall!(self.logUtil).putF(success.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("makePublic()"));
            }
            return success.clone();
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            let mut error: String = String::from("Failed to makePublic");
            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).SQLTAGSERROR.clone()) {
                jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("makePublic()"), crate::java_into_object(e.clone()));
            }
            return error.clone();
            }
        }
    }
}
impl std::ops::Deref for StaticPagesRequestHelper {
    type Target = AbContext;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for StaticPagesRequestHelper {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::business::context::AbContext::AbContextDyn for StaticPagesRequestHelper {
    fn as_AbContext(&self) -> &crate::org::allbinary::business::context::AbContext::AbContext { &self.base }
    fn as_AbContext_mut(&mut self) -> &mut crate::org::allbinary::business::context::AbContext::AbContext { &mut self.base }
}


