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
use crate::org::allbinary::business::context::modules::storefront::StoreFront::StoreFront;
use crate::org::allbinary::business::context::modules::storefront::StoreFrontData::StoreFrontData;
use crate::org::allbinary::business::context::modules::storefront::StoreFrontFactory::StoreFrontFactory;
use crate::org::allbinary::business::context::modules::storefront::StoreFrontInterface::StoreFrontInterface;
use crate::org::allbinary::business::user::commerce::inventory::basket::BasketData::BasketData;
use crate::org::allbinary::business::user::commerce::inventory::basket::BasketInterface::BasketInterface;
use crate::org::allbinary::business::user::commerce::inventory::item::BasicItemData::BasicItemData;
use crate::org::allbinary::data::tables::user::commerce::inventory::item::InventoryEntityFactory::InventoryEntityFactory;
use crate::org::allbinary::logic::communication::http::request::session::WeblisketSession::WeblisketSession;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use super::TagHelper::TagHelper;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct BasketRequestHelper {
    pub base: TagHelper,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub basicItemData: Option<Rc<RefCell<BasicItemData>>>,
    pub weblisketSession: Option<Rc<RefCell<WeblisketSession>>>,
    pub storeFrontInterface: Option<Rc<RefCell<dyn StoreFrontInterface>>>,
    pub propertiesHashMap: Option<Rc<RefCell<HashMap>>>,
    pub pageContext: Option<Rc<RefCell<PageContext>>>,
    pub request: Option<Rc<RefCell<HttpServletRequest>>>,
    pub id: String,
    pub num: String,
    pub MAX: i32,
}


impl BasketRequestHelper {
    

    pub fn new_struct(mut propertiesHashMap: Option<Rc<RefCell<HashMap>>>, mut pageContext: Option<Rc<RefCell<PageContext>>>) -> Self {
        let mut this = Self {
            base: TagHelper::new_struct(),
            logUtil: None,
            basicItemData: None,
            weblisketSession: None,
            storeFrontInterface: None,
            propertiesHashMap: None,
            pageContext: None,
            request: None,
            id: String::new(),
            num: String::new(),
            MAX: 0,
        };
        this.logUtil = LogUtil::getInstance();
        this.basicItemData = BasicItemData::getInstance();
        this.MAX = 200;
        this.propertiesHashMap = propertiesHashMap.clone();
        this.pageContext = pageContext.clone();
        this.request = (jcall!(pageContext).getRequest()).clone() /* as HttpServletRequest */;
        let mut storeName: String = (jcall!(propertiesHashMap).get(jcall!(StoreFrontData::getInstance()).NAME.clone())).clone() /* as String */;
        if true {
            this.storeFrontInterface = StoreFrontFactory::getInstance(storeName.clone());
        } else {
            this.storeFrontInterface = StoreFront::new().map(|reference| reference as Rc<RefCell<dyn StoreFrontInterface>>);
        }
        this.weblisketSession = WeblisketSession::new2(propertiesHashMap.clone(), pageContext.clone());
        this.getFormData();
        this
    }

    pub fn new(mut propertiesHashMap: Option<Rc<RefCell<HashMap>>>, mut pageContext: Option<Rc<RefCell<PageContext>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(propertiesHashMap, pageContext))))
    }
    

    pub fn getFormData(&mut self) {
        self.id = jcall!(self.request).getParameter(jcall!(self.basicItemData).ID.clone());
        self.num = jcall!(self.request).getParameter(crate::org::allbinary::business::user::commerce::inventory::basket::BasketData::BasketData_ITEMTOTALINBASKET.with(Clone::clone).clone());
    }
    

    pub fn addItemToBasket(&mut self) -> Option<bool> {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            if true && true {
                let mut basket: Option<Rc<RefCell<dyn BasketInterface>>> = jcall!(jcall!(self.weblisketSession).getOrder()).getBasket();
                if jcall!(jcall!(InventoryEntityFactory::getInstance()).getInventoryEntityInstance()).getItem(self.id.clone()).is_none() {
                    return false;
                }
                jcall!(basket).addItem(self.id.clone(), self.num.clone());
            }
            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).SQLTAGS.clone()) {
                jcall!(self.logUtil).putF(String::from("Successfully added item to Basket"), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("addItemToBasket()"));
            }
            return true;
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).SQLTAGSERROR.clone()) {
                jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("addItemToBasket()"), crate::java_into_object(e.clone()));
            }
            return false;
            }
        }
    }
    

    pub fn removeItemFromBasket(&mut self) -> Option<bool> {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            if true {
                let mut basket: Option<Rc<RefCell<dyn BasketInterface>>> = jcall!(jcall!(self.weblisketSession).getOrder()).getBasket();
                jcall!(basket).removeItem(self.id.clone());
            }
            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).SQLTAGS.clone()) {
                jcall!(self.logUtil).putF(String::from("Successfully removed item from Basket"), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("removeItemFromBasket()"));
            }
            return true;
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).SQLTAGSERROR.clone()) {
                jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("removeItemFromBasket()"), crate::java_into_object(e.clone()));
            }
            return false;
            }
        }
    }
    

    pub fn adjustBasket(&mut self) -> Option<bool> {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut basket: Option<Rc<RefCell<dyn BasketInterface>>> = jcall!(jcall!(self.weblisketSession).getOrder()).getBasket();
            if true && true {
                jcall!(basket).adjustItem(self.id.clone(), self.num.clone());
            }
            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).SQLTAGS.clone()) {
                jcall!(self.logUtil).putF(String::from("Successfull Adjusted Basket"), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("adjustBasket()"));
            }
            return true;
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).SQLTAGSERROR.clone()) {
                jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("adjustBasket()"), crate::java_into_object(e.clone()));
            }
            return false;
            }
        }
    }
}
impl std::ops::Deref for BasketRequestHelper {
    type Target = TagHelper;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for BasketRequestHelper {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::admin::taghelpers::TagHelper::TagHelperDyn for BasketRequestHelper {
    fn as_TagHelper(&self) -> &crate::admin::taghelpers::TagHelper::TagHelper { &self.base }
    fn as_TagHelper_mut(&mut self) -> &mut crate::admin::taghelpers::TagHelper::TagHelper { &mut self.base }
}
impl crate::admin::taghelpers::TagHelperInterface::TagHelperInterface for BasketRequestHelper {
}


