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
// package org.allbinary.logic.visual.transform.info.objectConfig
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::data::tree::dom::document::DomDocumentHelper::DomDocumentHelper;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::control::crypt::file::CryptFileReader::CryptFileReader;
use crate::org::allbinary::logic::io::path::AbPath::AbPath;
use crate::org::allbinary::logic::visual::transform::info::TransformInfoInterface::TransformInfoInterface;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use super::TransformInfoObjectConfigData::TransformInfoObjectConfigData;
use super::TransformInfoObjectConfig::TransformInfoObjectConfig;
use super::TransformInfoObjectConfigInterface::TransformInfoObjectConfigInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct TransformInfoObjectConfigNoManipulationFactory {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
}

thread_local! { pub static TransformInfoObjectConfigNoManipulationFactory_instance: Option<Rc<RefCell<TransformInfoObjectConfigNoManipulationFactory>>> = TransformInfoObjectConfigNoManipulationFactory::new(); }

impl TransformInfoObjectConfigNoManipulationFactory {
    

    pub fn getInstance() -> Option<Rc<RefCell<TransformInfoObjectConfigNoManipulationFactory>>> {
        return TransformInfoObjectConfigNoManipulationFactory_instance.with(Clone::clone).clone();
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
    pub fn getInstance2(&mut self, mut transformInfoInterface: Option<Rc<RefCell<dyn TransformInfoInterface>>>, mut objectConfigFileAbPath: Option<Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>>) -> Option<Rc<RefCell<dyn TransformInfoObjectConfigInterface>>> {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut transformInfoObjectConfigData: Option<Rc<RefCell<TransformInfoObjectConfigData>>> = TransformInfoObjectConfigData::getInstance();
            let mut data: String = jcall!(CryptFileReader::new(jcall!(transformInfoObjectConfigData).UNCRYPTED_EXTENSION.clone(), jcall!(jcall!(transformInfoObjectConfigData).getInstance()).ENCRYPTED_EXTENSION.clone())).get(objectConfigFileAbPath.clone());
            return TransformInfoObjectConfig::new2(transformInfoInterface.clone(), DomDocumentHelper::create1(data.clone())).map(|reference| reference as Rc<RefCell<dyn TransformInfoObjectConfigInterface>>);
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            let mut commonStrings: Option<Rc<RefCell<CommonStrings>>> = CommonStrings::getInstance();
            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).VIEWERROR.clone()) {
                jcall!(self.logUtil).put(String::from("Could Not Load Object Config"), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(commonStrings).GET_INSTANCE.clone(), crate::java_into_object(e.clone()));
            }
            panic!("{:?}", e);
            }
        }
    }
}

