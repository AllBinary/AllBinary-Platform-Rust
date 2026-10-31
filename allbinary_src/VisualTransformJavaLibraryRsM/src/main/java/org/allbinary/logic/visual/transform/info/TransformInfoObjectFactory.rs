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
use super::*;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::system::loader::AbeFactory::AbeFactory;
use crate::org::allbinary::logic::system::security::licensing::AbeClientInformationInterface::AbeClientInformationInterface;
use crate::org::allbinary::logic::system::security::licensing::LicensingException::LicensingException;
use crate::org::allbinary::logic::visual::transform::TransformInterface::TransformInterface;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use super::TransformInfoInterface::TransformInfoInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct TransformInfoObjectFactory {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
}

thread_local! { pub static TransformInfoObjectFactory_instance: Option<Rc<RefCell<TransformInfoObjectFactory>>> = TransformInfoObjectFactory::new(); }

impl TransformInfoObjectFactory {
    

    pub fn getInstance() -> Option<Rc<RefCell<TransformInfoObjectFactory>>> {
        return TransformInfoObjectFactory_instance.with(Clone::clone).clone();
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
    pub fn getInstance2(&mut self, mut abeClientInformation: Option<Rc<RefCell<dyn AbeClientInformationInterface>>>, mut transformInfoInterface: Option<Rc<RefCell<dyn TransformInfoInterface>>>) -> Option<Rc<RefCell<dyn TransformInterface>>> {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).TAGHELPERFACTORY.clone()) {
                let mut commonStrings: Option<Rc<RefCell<CommonStrings>>> = CommonStrings::getInstance();
                { let java_receiver = self.logUtil.clone(); let java_arguments = (format!("{}{}", String::from("Creating View: "), jcall!(transformInfoInterface).getName()), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(commonStrings).GET_INSTANCE.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
            }
            let mut params: Vec<Option<Rc<RefCell<Box<dyn std::any::Any>>>>> = vec![None; (1) as usize];
            let mut classes: Vec<&'static str> = vec![""; (1) as usize];
            classes[(0) as usize] = std::any::type_name::<TransformInfoInterface>();
            params[(0) as usize] = crate::java_into_object((transformInfoInterface).clone() /* as Any */);
            let mut anyType: Option<Rc<RefCell<dyn TransformInterface>>> = (jcall!(AbeFactory::getInstance()).getInstance4(abeClientInformation.clone(), jcall!(transformInfoInterface).getObjectFile(), classes.clone(), params.clone())).clone() /* as TransformInterface */;
            if anyType.is_none() {
                if transformInfoInterface.is_some() {
                    let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
                    jcall!(stringBuffer).append(String::from("\nNo Such View Name: "));
                    jcall!(stringBuffer).append(jcall!(transformInfoInterface).getName());
                    jcall!(stringBuffer).append(String::from("\nNo Such View Object: "));
                    jcall!(stringBuffer).append(jcall!(transformInfoInterface).getObjectFile());
                    panic!("{:?}", format!("{}: {:?}", "Exception", (jcall!(stringBuffer).toString())));
                } else {
                    panic!("{:?}", format!("{}: {:?}", "Exception", (String::from("No Such View Object since transformInfo is null"))));
                }
            }
            return anyType.clone();
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            let mut error: String = String::from("Failed To Get Instance: ");
            if transformInfoInterface.is_some() {
                error = format!("{}{}", error, jcall!(transformInfoInterface).getName());
            }
            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).TAGHELPERFACTORYERROR.clone()) {
                let mut commonStrings: Option<Rc<RefCell<CommonStrings>>> = CommonStrings::getInstance();
                jcall!(self.logUtil).put(error.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(commonStrings).GET_INSTANCE.clone(), crate::java_into_object(e.clone()));
            }
            panic!("{:?}", e);
            }
        }
    }
}

