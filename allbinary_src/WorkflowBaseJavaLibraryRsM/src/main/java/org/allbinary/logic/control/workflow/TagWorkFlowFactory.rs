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
// package org.allbinary.logic.control.workflow
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;
use super::*;
// use crate::java::util::HashMap::HashMap;
// use crate::javax::servlet::jsp::PageContext::PageContext;
use crate::org::allbinary::business::DynamicObjectData::DynamicObjectData;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::system::loader::AbeFactory::AbeFactory;
use crate::org::allbinary::logic::system::security::licensing::AbeClientInformationInterface::AbeClientInformationInterface;
use crate::org::allbinary::logic::system::security::licensing::LicensingException::LicensingException;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use super::WorkFlowInterface::WorkFlowInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct TagWorkFlowFactory {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
}

thread_local! { pub static TagWorkFlowFactory_instance: Option<Rc<RefCell<TagWorkFlowFactory>>> = TagWorkFlowFactory::new(); }

impl TagWorkFlowFactory {
    

    pub fn getInstance() -> Option<Rc<RefCell<TagWorkFlowFactory>>> {
        return TagWorkFlowFactory_instance.with(Clone::clone).clone();
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
    

    // throws Exception, LicensingException
    // Java overloaded method: Rust needs a different name
    pub fn getInstance3(&mut self, mut abeClientInformation: Option<Rc<RefCell<dyn AbeClientInformationInterface>>>, mut propertiesHashMap: Option<Rc<RefCell<HashMap>>>, mut pageContext: Option<Rc<RefCell<PageContext>>>) -> Option<Rc<RefCell<dyn WorkFlowInterface>>> {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut workFlowClassName: String = (jcall!(propertiesHashMap).get(crate::org::allbinary::business::DynamicObjectData::DynamicObjectData_NAME.with(Clone::clone).clone())).clone() /* as String */;
            let mut params: Vec<Option<Rc<RefCell<Box<dyn std::any::Any>>>>> = vec![None; (2) as usize];
            let mut classes: Vec<&'static str> = vec![""; (2) as usize];
            classes[(0) as usize] = std::any::type_name_of_val(&propertiesHashMap);
            classes[(1) as usize] = jcall!(AbeFactory::getInstance()).getClass(abeClientInformation.clone(), String::from("javax.servlet.jsp.PageContext"));
            params[(0) as usize] = crate::java_into_object((propertiesHashMap).clone() /* as Any */);
            params[(1) as usize] = crate::java_into_object((pageContext).clone() /* as Any */);
            return (jcall!(AbeFactory::getInstance()).getInstance4(abeClientInformation.clone(), workFlowClassName.clone(), classes.clone(), params.clone())).clone() /* as WorkFlowInterface */;
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).TAGHELPERFACTORYERROR.clone()) {
                let mut workFlowClassName: String = (jcall!(propertiesHashMap).get(crate::org::allbinary::business::DynamicObjectData::DynamicObjectData_NAME.with(Clone::clone).clone())).clone() /* as String */;
                let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
                jcall!(stringBuffer).append(String::from("Failed To Get Instance: "));
                jcall!(stringBuffer).append(workFlowClassName.clone());
                jcall!(stringBuffer).append(String::from("->"));
                jcall!(stringBuffer).append(std::any::type_name_of_val(&TagWorkFlowFactory_instance.with(Clone::clone)).to_string());
                let mut commonStrings: Option<Rc<RefCell<CommonStrings>>> = CommonStrings::getInstance();
                { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(stringBuffer).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(commonStrings).GET_INSTANCE.clone(), crate::java_into_object(e.clone()),); jcall!(java_receiver).put(java_arguments.0, java_arguments.1, java_arguments.2, java_arguments.3); };
            }
            panic!("{:?}", e);
            }
        }
    }
}

