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
use crate::org::allbinary::logic::string::StringValidationUtil::StringValidationUtil;
use crate::org::allbinary::logic::system::security::licensing::AbeClientInformationInterface::AbeClientInformationInterface;
use crate::org::allbinary::logic::visual::transform::info::TransformInfoInterface::TransformInfoInterface;
use crate::org::w3c::dom::Document::Document;
use super::TransformInfoObjectConfigAndManipulatorFactoryBase::TransformInfoObjectConfigAndManipulatorFactoryBase;
use super::TransformInfoObjectConfigInterface::TransformInfoObjectConfigInterface;
use super::TransformInfoObjectConfig::TransformInfoObjectConfig;
use super::GenericStoreTransformInfoObjectConfig::GenericStoreTransformInfoObjectConfig;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct TransformInfoObjectConfigAndManipulatorFactory {
    pub base: TransformInfoObjectConfigAndManipulatorFactoryBase,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
}

thread_local! { pub static TransformInfoObjectConfigAndManipulatorFactory_instance: Option<Rc<RefCell<TransformInfoObjectConfigAndManipulatorFactory>>> = TransformInfoObjectConfigAndManipulatorFactory::new(); }

impl TransformInfoObjectConfigAndManipulatorFactory {
    

    pub fn getInstance() -> Option<Rc<RefCell<TransformInfoObjectConfigAndManipulatorFactory>>> {
        return TransformInfoObjectConfigAndManipulatorFactory_instance.with(Clone::clone).clone();
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: TransformInfoObjectConfigAndManipulatorFactoryBase::new_struct(),
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
    pub fn getInstance3(&mut self, mut abeClientInformation: Option<Rc<RefCell<dyn AbeClientInformationInterface>>>, mut transformInfoInterface: Option<Rc<RefCell<dyn TransformInfoInterface>>>, mut objectConfigFileAbPath: Option<Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>>) -> Option<Rc<RefCell<dyn TransformInfoObjectConfigInterface>>> {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut data: String = jcall!(CryptFileReader::new(jcall!(TransformInfoObjectConfigData::TransformInfoObjectConfigData::getInstance()).UNCRYPTED_EXTENSION.clone(), jcall!(TransformInfoObjectConfigData::TransformInfoObjectConfigData::getInstance()).ENCRYPTED_EXTENSION.clone())).get(objectConfigFileAbPath.clone());
            return { let java_arguments = (abeClientInformation.clone(), transformInfoInterface.clone(), DomDocumentHelper::create1(data.clone()),); self.getInstance3(java_arguments.0, java_arguments.1, java_arguments.2) };
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).VIEWERROR.clone()) {
                { let java_receiver = self.logUtil.clone(); let java_arguments = (String::from("Could Not Load Object Config"), crate::java_into_object(Self::getInstance()), jcall!(self.commonStrings).GET_INSTANCE.clone(), crate::java_into_object(e.clone()),); jcall!(java_receiver).put(java_arguments.0, java_arguments.1, java_arguments.2, java_arguments.3); };
            }
            panic!("{:?}", e);
            }
        }
    }
    

    // throws Exception
    // Java overloaded method: Rust needs a different name
    pub fn getInstance2(&mut self, mut abeClientInformation: Option<Rc<RefCell<dyn AbeClientInformationInterface>>>, mut transformInfoInterface: Option<Rc<RefCell<dyn TransformInfoInterface>>>) -> Option<Rc<RefCell<dyn TransformInfoObjectConfigInterface>>> {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut stringValidationUtil: Option<Rc<RefCell<StringValidationUtil>>> = StringValidationUtil::getInstance();
            if transformInfoInterface.is_some() && !jcall!(stringValidationUtil).isEmpty(jcall!(transformInfoInterface).getStoreName()) {
                return (GenericStoreTransformInfoObjectConfig::new2(abeClientInformation.clone(), transformInfoInterface.clone()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::visual::transform::info::objectConfig::GenericStoreTransformInfoObjectConfig::GenericStoreTransformInfoObjectConfigDyn>>)).clone() /* as TransformInfoObjectConfigInterface */;
            } else {
                return (TransformInfoObjectConfig::new1(transformInfoInterface.clone()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::visual::transform::info::objectConfig::TransformInfoObjectConfig::TransformInfoObjectConfigDyn>>)).clone() /* as TransformInfoObjectConfigInterface */;
            }
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            let mut error: String = String::from("Failed To Get Instance: ");
            if transformInfoInterface.is_some() {
                error = format!("{}{}", error, jcall!(transformInfoInterface).getName());
            }
            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).TAGHELPERFACTORYERROR.clone()) {
                { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(self.base.commonStrings).EXCEPTION.clone(), crate::java_into_object(Self::getInstance()), jcall!(self.base.commonStrings).GET_INSTANCE.clone(), crate::java_into_object(e.clone()),); jcall!(java_receiver).put(java_arguments.0, java_arguments.1, java_arguments.2, java_arguments.3); };
            }
            panic!("{:?}", e);
            }
        }
    }
    

    // throws Exception
    // Java overloaded method: Rust needs a different name
    pub fn getInstance3(&mut self, mut abeClientInformation: Option<Rc<RefCell<dyn AbeClientInformationInterface>>>, mut transformInfoInterface: Option<Rc<RefCell<dyn TransformInfoInterface>>>, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<dyn TransformInfoObjectConfigInterface>>> {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut stringValidationUtil: Option<Rc<RefCell<StringValidationUtil>>> = StringValidationUtil::getInstance();
            if transformInfoInterface.is_some() && !jcall!(stringValidationUtil).isEmpty(jcall!(transformInfoInterface).getStoreName()) {
                return (GenericStoreTransformInfoObjectConfig::new3(abeClientInformation.clone(), transformInfoInterface.clone(), document.clone()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::visual::transform::info::objectConfig::GenericStoreTransformInfoObjectConfig::GenericStoreTransformInfoObjectConfigDyn>>)).clone() /* as TransformInfoObjectConfigInterface */;
            } else {
                return (TransformInfoObjectConfig::new2(transformInfoInterface.clone(), document.clone()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::visual::transform::info::objectConfig::TransformInfoObjectConfig::TransformInfoObjectConfigDyn>>)).clone() /* as TransformInfoObjectConfigInterface */;
            }
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            let mut error: String = String::from("Failed To Get Instance: ");
            if transformInfoInterface.is_some() {
                error = format!("{}{}", error, jcall!(transformInfoInterface).getName());
            }
            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).TAGHELPERFACTORYERROR.clone()) {
                { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(self.base.commonStrings).EXCEPTION.clone(), crate::java_into_object(Self::getInstance()), String::from("getInstance(document)"), crate::java_into_object(e.clone()),); jcall!(java_receiver).put(java_arguments.0, java_arguments.1, java_arguments.2, java_arguments.3); };
            }
            panic!("{:?}", e);
            }
        }
    }
}
impl std::ops::Deref for TransformInfoObjectConfigAndManipulatorFactory {
    type Target = TransformInfoObjectConfigAndManipulatorFactoryBase;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for TransformInfoObjectConfigAndManipulatorFactory {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::logic::visual::transform::info::objectConfig::TransformInfoObjectConfigAndManipulatorFactoryBase::TransformInfoObjectConfigAndManipulatorFactoryBaseDyn for TransformInfoObjectConfigAndManipulatorFactory {
    fn as_TransformInfoObjectConfigAndManipulatorFactoryBase(&self) -> &crate::org::allbinary::logic::visual::transform::info::objectConfig::TransformInfoObjectConfigAndManipulatorFactoryBase::TransformInfoObjectConfigAndManipulatorFactoryBase { &self.base }
    fn as_TransformInfoObjectConfigAndManipulatorFactoryBase_mut(&mut self) -> &mut crate::org::allbinary::logic::visual::transform::info::objectConfig::TransformInfoObjectConfigAndManipulatorFactoryBase::TransformInfoObjectConfigAndManipulatorFactoryBase { &mut self.base }
    fn getInstance(&mut self, mut abeClientInformation: Option<Rc<RefCell<dyn AbeClientInformationInterface>>>, mut transformInfoInterface: Option<Rc<RefCell<dyn TransformInfoInterface>>>) -> Option<Rc<RefCell<dyn TransformInfoObjectConfigInterface>>> { Self::getInstance2(self, abeClientInformation, transformInfoInterface) }
}


