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
// package org.allbinary.logic.visual.transform.template
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;
use super::*;
// use crate::java::util::HashMap::HashMap;
// use crate::javax::servlet::jsp::PageContext::PageContext;
use crate::org::allbinary::data::tables::transform::info::TransformInfoEntity::TransformInfoEntity;
use crate::org::allbinary::data::tables::transform::info::TransformInfoEntityBuilder::TransformInfoEntityBuilder;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::system::security::licensing::AbeClientInformationInterface::AbeClientInformationInterface;
use crate::org::allbinary::logic::visual::transform::info::TransformInfoInterface::TransformInfoInterface;
use crate::org::allbinary::logic::visual::transform::info::TransformInfoObjectFactory::TransformInfoObjectFactory;
use super::TransformTemplateInterface::TransformTemplateInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct TransformTemplateFactory {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
}

thread_local! { pub static TransformTemplateFactory_instance: Option<Rc<RefCell<TransformTemplateFactory>>> = TransformTemplateFactory::new(); }

impl TransformTemplateFactory {
    

    pub fn getInstance() -> Option<Rc<RefCell<TransformTemplateFactory>>> {
        return TransformTemplateFactory_instance.with(Clone::clone).clone();
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            logUtil: None,
        };
        this.logUtil = LogUtil::getInstance();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    // throws Exception
    // Java overloaded method: Rust needs a different name
    pub fn getInstance4(&mut self, mut abeClientInformation: Option<Rc<RefCell<dyn AbeClientInformationInterface>>>, mut templateName: String, mut propertiesHashMap: Option<Rc<RefCell<HashMap>>>, mut pageContext: Option<Rc<RefCell<PageContext>>>) -> Option<Rc<RefCell<dyn TransformTemplateInterface>>> {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut transformInfoEntity: Option<Rc<RefCell<TransformInfoEntity>>> = TransformInfoEntityBuilder::getInstance();
            let mut transformInfoInterface: Option<Rc<RefCell<dyn TransformInfoInterface>>> = jcall!(transformInfoEntity).get(templateName.clone(), propertiesHashMap.clone(), pageContext.clone());
            return (jcall!(TransformInfoObjectFactory::getInstance()).getInstance2(abeClientInformation.clone(), transformInfoInterface.clone())).clone() /* as TransformTemplateInterface */;
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            let mut error: String = String::from("Failed To Get Instance");
            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).TAGHELPERFACTORYERROR.clone()) {
                jcall!(self.logUtil).put(format!("{}{}", format!("{}{}", error, templateName), String::from("->TemplateFactory")), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("getInstance(viewName, HashMap, PageContext)"), crate::java_into_object(e.clone()));
            }
            panic!("{:?}", e);
            }
        }
    }
}

