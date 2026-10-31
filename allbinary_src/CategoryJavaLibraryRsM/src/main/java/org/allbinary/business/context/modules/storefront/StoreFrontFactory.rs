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
// package org.allbinary.business.context.modules.storefront
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::data::tables::context::module::storefronts::StoreFrontsEntity::StoreFrontsEntity;
use crate::org::allbinary::data::tables::context::module::storefronts::StoreFrontsEntityFactory::StoreFrontsEntityFactory;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use super::StoreFront::StoreFront;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct StoreFrontFactory {
}


impl StoreFrontFactory {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance(mut storeName: String) -> Option<Rc<RefCell<StoreFront>>> {
        let mut logUtil: Option<Rc<RefCell<LogUtil>>> = LogUtil::getInstance();
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut storeFronts: Option<Rc<RefCell<StoreFrontsEntity>>> = jcall!(StoreFrontsEntityFactory::getInstance()).getStoreFrontsEntityInstance();
            return jcall!(storeFronts).getStoreFrontInterface(storeName.clone());
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            let mut commonStrings: Option<Rc<RefCell<CommonStrings>>> = CommonStrings::getInstance();
            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).FACTORYERROR.clone()) {
                jcall!(logUtil).put(jcall!(commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from("StoreFrontFactory")), jcall!(commonStrings).GET_INSTANCE.clone(), crate::java_into_object(e.clone()));
            }
            return None;
            }
        }
    }
}

