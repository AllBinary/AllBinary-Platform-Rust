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
// use crate::javax::servlet::http::HttpServletResponse::HttpServletResponse;
// use crate::javax::servlet::jsp::PageContext::PageContext;
use crate::admin::taghelpers::TagHelper::TagHelper;
use crate::org::allbinary::business::context::modules::storefront::StoreFrontData::StoreFrontData;
use crate::org::allbinary::business::context::modules::storefront::StoreFrontFactory::StoreFrontFactory;
use crate::org::allbinary::business::context::modules::storefront::StoreFrontInterface::StoreFrontInterface;
use crate::org::allbinary::business::user::address::StreetAddress::StreetAddress;
use crate::org::allbinary::business::user::commerce::inventory::item::BasicItemData::BasicItemData;
use crate::org::allbinary::logic::communication::http::AcceptableResponseGenerator::AcceptableResponseGenerator;
use crate::org::allbinary::logic::communication::http::request::session::WeblisketSession::WeblisketSession;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::control::search::SearchParams::SearchParams;
use crate::org::allbinary::logic::control::search::SearchRequest::SearchRequest;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use crate::org::allbinary::logic::visual::transform::info::TransformInfoData::TransformInfoData;
use crate::views::admin::inventory::listings::InventoryViewFactory::InventoryViewFactory;
use crate::views::admin::inventory::listings::InventoryViewSearchInterface::InventoryViewSearchInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct InventoryHelper {
    pub base: TagHelper,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub basicItemData: Option<Rc<RefCell<BasicItemData>>>,
    pub weblisketSession: Option<Rc<RefCell<WeblisketSession>>>,
    pub response: Option<Rc<RefCell<HttpServletResponse>>>,
    pub request: Option<Rc<RefCell<HttpServletRequest>>>,
    pub streetAddress: Option<Rc<RefCell<StreetAddress>>>,
    pub id: String,
    pub searchRequest: Option<Rc<RefCell<SearchRequest>>>,
}


impl InventoryHelper {
    

    pub fn new_struct(mut propertiesHashMap: Option<Rc<RefCell<HashMap>>>, mut pageContext: Option<Rc<RefCell<PageContext>>>) -> Self {
        let mut this = Self {
            base: TagHelper::new_struct(),
            logUtil: None,
            basicItemData: None,
            weblisketSession: None,
            response: None,
            request: None,
            streetAddress: None,
            id: String::new(),
            searchRequest: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.basicItemData = BasicItemData::getInstance();
        this.response = (jcall!(pageContext).getResponse()).clone() /* as HttpServletResponse */;
        this.request = (jcall!(pageContext).getRequest()).clone() /* as HttpServletRequest */;
        let mut storeName: String = (jcall!(propertiesHashMap).get(jcall!(StoreFrontData::getInstance()).NAME.clone())).clone() /* as String */;
        let mut xslFile: String = (jcall!(propertiesHashMap).get(jcall!(TransformInfoData::getInstance()).TEMPLATEFILENAME.clone())).clone() /* as String */;
        if true {
            let mut storeFrontInterface: Option<Rc<RefCell<dyn StoreFrontInterface>>> = StoreFrontFactory::getInstance(storeName.clone());
            let mut contentType: String = jcall!(AcceptableResponseGenerator::getInstance()).get(this.request.clone());
            let mut searchParams: Option<Rc<RefCell<SearchParams>>> = SearchParams::new(this.request.clone());
            this.searchRequest = SearchRequest::new(storeFrontInterface.clone(), searchParams.clone(), xslFile.clone(), contentType.clone(), propertiesHashMap.clone(), pageContext.clone());
        }
        this.getFormData();
        this
    }

    pub fn new(mut propertiesHashMap: Option<Rc<RefCell<HashMap>>>, mut pageContext: Option<Rc<RefCell<PageContext>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(propertiesHashMap, pageContext))))
    }
    

    pub fn getFormData(&mut self) {
        self.id = jcall!(self.request).getParameter(jcall!(self.basicItemData).ID.clone());
    }
    

    pub fn viewSummary(&mut self) -> String {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut inventoryViewSearchInterface: Option<Rc<RefCell<dyn InventoryViewSearchInterface>>> = InventoryViewFactory::getInstance(self.searchRequest.clone());
            let mut result: String = jcall!(inventoryViewSearchInterface).getProduct(self.id.clone());
            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).SQLTAGS.clone()) {
                jcall!(self.logUtil).putF(String::from("Successfull Inventory View Summary"), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("viewSummary()"));
            }
            return result.clone();
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            let mut error: String = String::from("Failed to View Summary");
            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).SQLTAGSERROR.clone()) {
                jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("viewSummary()"), crate::java_into_object(e.clone()));
            }
            return format!("{}{}", format!("{}{}", format!("{}{}", format!("{}{}", error, String::from("<br>")), String::from("Exception: ")), e), String::from("<br>"));
            }
        }
    }
    

    pub fn search(&mut self) -> String {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut stringUtil: Option<Rc<RefCell<StringUtil>>> = StringUtil::getInstance();
            let mut keywords: String = (jcall!(jcall!(jcall!(self.searchRequest).getParams()).get()).get(jcall!(self.basicItemData).KEYWORDS.clone())).clone() /* as String */;
            if false || (keywords.cmp(&jcall!(stringUtil).EMPTY_STRING.clone()) as i32) == 0 {
                return String::from("Please enter a keyword in the search field above the Search button.");
            }
            let mut inventoryViewSearchInterface: Option<Rc<RefCell<dyn InventoryViewSearchInterface>>> = InventoryViewFactory::getInstance(self.searchRequest.clone());
            let mut result: String = jcall!(inventoryViewSearchInterface).searchSingleDynamicPage();
            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).SQLTAGS.clone()) {
                jcall!(self.logUtil).putF(String::from("Successful Inventory Dynamic Search"), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("searchSingleDynamicPage()"));
            }
            return result.clone();
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            let mut error: String = String::from("Failed to search Inventory by keyword");
            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).SQLTAGSERROR.clone()) {
                jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("searchSingleDynamicPage()"), crate::java_into_object(e.clone()));
            }
            return format!("{}{}", format!("{}{}", format!("{}{}", format!("{}{}", error, String::from("<br>")), String::from("Exception: ")), e), String::from("<br>"));
            }
        }
    }
}
impl std::ops::Deref for InventoryHelper {
    type Target = TagHelper;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for InventoryHelper {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::admin::taghelpers::TagHelper::TagHelperDyn for InventoryHelper {
    fn as_TagHelper(&self) -> &crate::admin::taghelpers::TagHelper::TagHelper { &self.base }
    fn as_TagHelper_mut(&mut self) -> &mut crate::admin::taghelpers::TagHelper::TagHelper { &mut self.base }
}
impl crate::admin::taghelpers::TagHelperInterface::TagHelperInterface for InventoryHelper {
}


