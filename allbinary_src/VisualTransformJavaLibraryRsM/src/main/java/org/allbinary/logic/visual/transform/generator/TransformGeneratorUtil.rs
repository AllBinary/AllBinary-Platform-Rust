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
// package org.allbinary.logic.visual.transform.generator
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::io::InputOutputTypeData::InputOutputTypeData;
use crate::org::allbinary::logic::system::security::licensing::AbeClientInformationInterface::AbeClientInformationInterface;
use crate::org::allbinary::logic::visual::transform::TransformFactory::TransformFactory;
use crate::org::allbinary::logic::visual::transform::TransformInterface::TransformInterface;
use crate::org::allbinary::logic::visual::transform::info::TransformInfoDomNode::TransformInfoDomNode;
use crate::org::allbinary::logic::visual::transform::info::TransformInfoHttpInterface::TransformInfoHttpInterface;
use crate::org::allbinary::logic::visual::transform::info::TransformInfoInterface::TransformInfoInterface;
use crate::org::allbinary::logic::visual::transform::info::objectConfig::generator::StoreFileGenerator::StoreFileGenerator;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct TransformGeneratorUtil {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
}

thread_local! { pub static TransformGeneratorUtil_instance: Option<Rc<RefCell<TransformGeneratorUtil>>> = TransformGeneratorUtil::new(); }

impl TransformGeneratorUtil {
    

    pub fn getInstance() -> Option<Rc<RefCell<TransformGeneratorUtil>>> {
        return TransformGeneratorUtil_instance.with(Clone::clone).clone();
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
    pub fn generate(&mut self, mut abeClientInformation: Option<Rc<RefCell<dyn AbeClientInformationInterface>>>, mut transformInfoInterface: Option<Rc<RefCell<dyn TransformInfoInterface>>>, mut ownerTransformInfoInterface: Option<Rc<RefCell<dyn TransformInfoInterface>>>) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).VIEW.clone()) {
                { let java_receiver = self.logUtil.clone(); let java_arguments = (format!("{}{}", String::from("Generating View: "), jcall!(transformInfoInterface).getName()), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("generate()"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
            }
            let mut componentInterface: Option<Rc<RefCell<dyn TransformInterface>>> = jcall!(TransformFactory::getInstance()).getInstance3(abeClientInformation.clone(), jcall!(transformInfoInterface).getName(), ownerTransformInfoInterface.clone());
            let mut result: String = jcall!(componentInterface).view();
            let mut httpTransformInfoInterface: Option<Rc<RefCell<dyn TransformInfoHttpInterface>>> = (jcall!(componentInterface).getTransformInfoInterface()).clone() /* as TransformInfoHttpInterface */;
            let mut inputOutputTypeData: Option<Rc<RefCell<InputOutputTypeData>>> = InputOutputTypeData::getInstance();
            if result.find(&String::from("<HTML>")).map(|index| index as i32).unwrap_or(-1) >= 0 {
                jcall!(jcall!(httpTransformInfoInterface).getPropertiesHashMap()).put(jcall!(inputOutputTypeData).NAME.clone(), jcall!(inputOutputTypeData).DEFAULT.clone());
            } else {
                jcall!(jcall!(httpTransformInfoInterface).getPropertiesHashMap()).put(jcall!(inputOutputTypeData).NAME.clone(), jcall!(inputOutputTypeData).DEFAULT_FRAGMENT.clone());
            }
            jcall!(StoreFileGenerator::new(jcall!(componentInterface).getTransformInfoInterface())).process(result.clone());
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).SQLTAGSERROR.clone()) {
                jcall!(self.logUtil).put(String::from("Failed to generate a view"), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("generate()"), crate::java_into_object(e.clone()));
            }
            panic!("{:?}", e);
            }
        }
    }
    

    // throws Exception
    // Java overloaded method: Rust needs a different name
    pub fn generate(&mut self, mut abeClientInformation: Option<Rc<RefCell<dyn AbeClientInformationInterface>>>, mut transformInfoDomNode: Option<Rc<RefCell<dyn crate::org::allbinary::logic::visual::transform::info::TransformInfoDomNode::TransformInfoDomNodeDyn>>>, mut ownerTransformInfoInterface: Option<Rc<RefCell<dyn TransformInfoInterface>>>) {
        { let java_arguments = (abeClientInformation.clone(), jcall!(transformInfoDomNode).getTransformInfoInterface(), ownerTransformInfoInterface.clone(),); self.generate(java_arguments.0, java_arguments.1, java_arguments.2) };
    }
}

