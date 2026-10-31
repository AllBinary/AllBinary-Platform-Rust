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
// use crate::javax::servlet::jsp::PageContext::PageContext;
use crate::org::allbinary::business::context::modules::storefront::StoreFrontData::StoreFrontData;
use crate::org::allbinary::business::context::modules::storefront::StoreFrontFactory::StoreFrontFactory;
use crate::org::allbinary::business::context::modules::storefront::StoreFrontInterface::StoreFrontInterface;
use crate::org::allbinary::business::user::commerce::inventory::basket::BasketInterface::BasketInterface;
use crate::org::allbinary::logic::communication::http::request::session::WeblisketSession::WeblisketSession;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use super::TagHelper::TagHelper;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct BasketHelper {
    pub base: TagHelper,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub weblisketSession: Option<Rc<RefCell<WeblisketSession>>>,
    pub storeFrontInterface: Option<Rc<RefCell<dyn StoreFrontInterface>>>,
    pub propertiesHashMap: Option<Rc<RefCell<HashMap>>>,
    pub pageContext: Option<Rc<RefCell<PageContext>>>,
    pub MAX: i32,
}


impl BasketHelper {
    

    pub fn new_struct(mut hashMap: Option<Rc<RefCell<HashMap>>>, mut pageContext: Option<Rc<RefCell<PageContext>>>) -> Self {
        let mut this = Self {
            base: TagHelper::new_struct(),
            logUtil: None,
            weblisketSession: None,
            storeFrontInterface: None,
            propertiesHashMap: None,
            pageContext: None,
            MAX: 0,
        };
        this.logUtil = LogUtil::getInstance();
        this.MAX = 200;
        this.propertiesHashMap = hashMap.clone();
        this.pageContext = pageContext.clone();
        let mut storeName: String = ({ let java_receiver = this.propertiesHashMap.clone(); let java_arguments = (jcall!(StoreFrontData::getInstance()).NAME.clone(),); let java_result = jcall!(java_receiver).get(java_arguments.0); java_result }).clone() /* as String */;
        if true {
            this.storeFrontInterface = StoreFrontFactory::getInstance(storeName.clone());
        }
        this.weblisketSession = WeblisketSession::new2(hashMap.clone(), pageContext.clone());
        this
    }

    pub fn new(mut hashMap: Option<Rc<RefCell<HashMap>>>, mut pageContext: Option<Rc<RefCell<PageContext>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(hashMap, pageContext))))
    }
    

    pub fn isBasketEmpty(&mut self) -> Option<bool> {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut basket: Option<Rc<RefCell<dyn BasketInterface>>> = jcall!(jcall!(self.weblisketSession).getOrder()).getBasket();
            if ((jcall!(basket).getNumberOfItems()).unwrap() as i32) <= 0 {
                return true;
            } else {
                return false;
            }
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).SQLTAGSERROR.clone()) {
                jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("isBasketEmpty()"), crate::java_into_object(e.clone()));
            }
            return true;
            }
        }
    }
}
impl std::ops::Deref for BasketHelper {
    type Target = TagHelper;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for BasketHelper {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::admin::taghelpers::TagHelper::TagHelperDyn for BasketHelper {
    fn as_TagHelper(&self) -> &crate::admin::taghelpers::TagHelper::TagHelper { &self.base }
    fn as_TagHelper_mut(&mut self) -> &mut crate::admin::taghelpers::TagHelper::TagHelper { &mut self.base }
}
impl crate::admin::taghelpers::TagHelperInterface::TagHelperInterface for BasketHelper {
}


