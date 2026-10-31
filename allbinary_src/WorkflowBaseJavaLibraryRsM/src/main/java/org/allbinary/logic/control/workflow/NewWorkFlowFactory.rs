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
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use crate::org::allbinary::logic::system::loader::AbeFactory::AbeFactory;
use crate::org::allbinary::logic::system::security::licensing::AbeClientInformationInterface::AbeClientInformationInterface;
use crate::org::allbinary::logic::system::security::licensing::LicensingException::LicensingException;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use super::WorkFlowWrapper::WorkFlowWrapper;
use super::WorkFlowInterface::WorkFlowInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct NewWorkFlowFactory {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
}

thread_local! { pub static NewWorkFlowFactory_instance: Option<Rc<RefCell<NewWorkFlowFactory>>> = NewWorkFlowFactory::new(); }

impl NewWorkFlowFactory {
    

    pub fn getInstance() -> Option<Rc<RefCell<NewWorkFlowFactory>>> {
        return NewWorkFlowFactory_instance.with(Clone::clone).clone();
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
    pub fn getInstance3(&mut self, mut abeClientInformation: Option<Rc<RefCell<dyn AbeClientInformationInterface>>>, mut hashMap: Option<Rc<RefCell<HashMap>>>, mut pageContext: Option<Rc<RefCell<PageContext>>>) -> Option<Rc<RefCell<dyn WorkFlowInterface>>> {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut className: String = (jcall!(hashMap).get(crate::org::allbinary::business::DynamicObjectData::DynamicObjectData_NAME.with(Clone::clone).clone())).clone() /* as String */;
            let mut params: Vec<Option<Rc<RefCell<Box<dyn std::any::Any>>>>> = vec![None; (2) as usize];
            let mut classes: Vec<&'static str> = vec![""; (2) as usize];
            classes[(0) as usize] = std::any::type_name_of_val(&hashMap);
            classes[(1) as usize] = jcall!(AbeFactory::getInstance()).getClass(abeClientInformation.clone(), String::from("javax.servlet.jsp.PageContext"));
            params[(0) as usize] = crate::java_into_object((hashMap).clone() /* as Any */);
            params[(1) as usize] = crate::java_into_object((pageContext).clone() /* as Any */);
            let mut anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = jcall!(AbeFactory::getInstance()).getInstance4(abeClientInformation.clone(), className.clone(), classes.clone(), params.clone());
            return (WorkFlowWrapper::new(anyType.clone()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::control::workflow::WorkFlowWrapper::WorkFlowWrapperDyn>>)).clone() /* as WorkFlowInterface */;
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).TAGHELPERFACTORYERROR.clone()) {
                let mut commonStrings: Option<Rc<RefCell<CommonStrings>>> = CommonStrings::getInstance();
                jcall!(self.logUtil).put(jcall!(commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("getInstance(HashMap)"), crate::java_into_object(e.clone()));
            }
            panic!("{:?}", e);
            }
        }
    }
    

    // throws Exception, LicensingException
    // Java overloaded method: Rust needs a different name
    pub fn getInstance2(&mut self, mut abeClientInformation: Option<Rc<RefCell<dyn AbeClientInformationInterface>>>, mut className: String) -> Option<Rc<RefCell<dyn WorkFlowInterface>>> {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut stringUtil: Option<Rc<RefCell<StringUtil>>> = StringUtil::getInstance();
            if true && (className.cmp(&jcall!(stringUtil).EMPTY_STRING.clone()) as i32) != 0 {
                let mut anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = jcall!(AbeFactory::getInstance()).getInstance4(abeClientInformation.clone(), className.clone(), None, None);
                return (WorkFlowWrapper::new(anyType.clone()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::control::workflow::WorkFlowWrapper::WorkFlowWrapperDyn>>)).clone() /* as WorkFlowInterface */;
            } else {
                panic!("{:?}", format!("{}: {:?}", "Exception", (String::from("No WorkFlow ClassName"))));
            }
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).TAGHELPERFACTORYERROR.clone()) {
                let mut commonStrings: Option<Rc<RefCell<CommonStrings>>> = CommonStrings::getInstance();
                jcall!(self.logUtil).put(jcall!(commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(commonStrings).GET_INSTANCE.clone(), crate::java_into_object(e.clone()));
            }
            panic!("{:?}", e);
            }
        }
    }
}

