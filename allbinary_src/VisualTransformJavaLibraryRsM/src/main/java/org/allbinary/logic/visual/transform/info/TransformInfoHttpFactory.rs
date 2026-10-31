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
// package org.allbinary.logic.visual.transform.info
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;
use super::*;
// use crate::java::util::HashMap::HashMap;
// use crate::javax::servlet::jsp::PageContext::PageContext;
use crate::org::allbinary::business::context::modules::storefront::StoreFrontFactory::StoreFrontFactory;
use crate::org::allbinary::business::context::modules::storefront::StoreFrontInterface::StoreFrontInterface;
use crate::org::allbinary::logic::communication::http::request::session::WeblisketSession::WeblisketSession;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::string::StringValidationUtil::StringValidationUtil;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use super::TransformInfoHttpContext::TransformInfoHttpContext;
use super::TransformInfoInterface::TransformInfoInterface;
use super::TransformInfoHttpStore::TransformInfoHttpStore;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct TransformInfoHttpFactory {
}


impl TransformInfoHttpFactory {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    // throws Exception
    pub fn getInstance(mut propertiesHashMap: Option<Rc<RefCell<HashMap>>>, mut pageContext: Option<Rc<RefCell<PageContext>>>) -> Option<Rc<RefCell<dyn TransformInfoInterface>>> {
        let mut logUtil: Option<Rc<RefCell<LogUtil>>> = LogUtil::getInstance();
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut weblisketSession: Option<Rc<RefCell<WeblisketSession>>> = WeblisketSession::new2(propertiesHashMap.clone(), pageContext.clone());
            if !jcall!(StringValidationUtil::getInstance()).isEmpty(jcall!(weblisketSession).getStoreName()) {
                let mut storeFrontInterface: Option<Rc<RefCell<dyn StoreFrontInterface>>> = StoreFrontFactory::getInstance(jcall!(weblisketSession).getStoreName());
                if storeFrontInterface.is_some() {
                    return (TransformInfoHttpStore::new3(storeFrontInterface.clone(), propertiesHashMap.clone(), pageContext.clone()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::visual::transform::info::TransformInfoHttpStore::TransformInfoHttpStoreDyn>>)).clone() /* as TransformInfoInterface */;
                } else {
                    panic!("{:?}", format!("{}: {:?}", "Exception", (format!("{}{}", String::from("No StoreFront with: "), jcall!(weblisketSession).getStoreName()))));
                }
            } else {
                return (TransformInfoHttpContext::new2(propertiesHashMap.clone(), pageContext.clone())).clone() /* as TransformInfoInterface */;
            }
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).TAGHELPERFACTORYERROR.clone()) {
                let mut commonStrings: Option<Rc<RefCell<CommonStrings>>> = CommonStrings::getInstance();
                jcall!(logUtil).put(jcall!(commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from("TransformInfoFactory")), jcall!(commonStrings).GET_INSTANCE.clone(), crate::java_into_object(e.clone()));
            }
            panic!("{:?}", e);
            }
        }
    }
}

