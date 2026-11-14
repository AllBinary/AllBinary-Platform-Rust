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
use crate::java::lang::Exception::*;
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
        let java_try_result = match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut data: String = jcall!(CryptFileReader::new({ let java_field = jcall!(TransformInfoObjectConfigData::TransformInfoObjectConfigData::getInstance()).UNCRYPTED_EXTENSION.clone(); java_field }.clone(), { let java_field = jcall!(TransformInfoObjectConfigData::TransformInfoObjectConfigData::getInstance()).ENCRYPTED_EXTENSION.clone(); java_field }.clone())).get(objectConfigFileAbPath.clone());
            return Some({ let java_arguments = (abeClientInformation.clone(), transformInfoInterface.clone(), DomDocumentHelper::create1_String(data.clone()),); self.getInstance3_AbeClientInformationInterface_TransformInfoInterface_Document(java_arguments.0, java_arguments.1, java_arguments.2) });
        })) {
            Ok(java_return) => java_return,
            Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            if jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypes.clone(); java_field }).LOGGING.clone(); java_field }).contains({ let java_field = jcall!(jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypeFactory.clone(); java_field }).getInstance()).VIEWERROR.clone(); java_field }.clone()) {
                { let java_receiver = self.logUtil.clone(); let java_arguments = (String::from("Could Not Load Object Config"), crate::java_into_object(Self::getInstance()), { let java_field = jcall!(self.commonStrings).GET_INSTANCE.clone(); java_field }.clone(), crate::java_into_object(e.clone()),); jcall!(java_receiver).put(java_arguments.0, java_arguments.1, java_arguments.2, java_arguments.3); };
            }
            panic!("{:?}", e);
            None
            }
        };if let Some(java_return) = java_try_result { return java_return; }unreachable!("try statement")
    }
    

    // throws Exception
    // Java overloaded method: Rust needs a different name
    pub fn getInstance2(&mut self, mut abeClientInformation: Option<Rc<RefCell<dyn AbeClientInformationInterface>>>, mut transformInfoInterface: Option<Rc<RefCell<dyn TransformInfoInterface>>>) -> Option<Rc<RefCell<dyn TransformInfoObjectConfigInterface>>> {
        let java_try_result = match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut stringValidationUtil: Option<Rc<RefCell<StringValidationUtil>>> = StringValidationUtil::getInstance();
            if transformInfoInterface.is_some() && !jcall!(stringValidationUtil).isEmpty(jcall!(transformInfoInterface).getStoreName()) {
                return Some((GenericStoreTransformInfoObjectConfig::new2(abeClientInformation.clone(), transformInfoInterface.clone()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::visual::transform::info::objectConfig::GenericStoreTransformInfoObjectConfig::GenericStoreTransformInfoObjectConfigDyn>>)).clone() /* as TransformInfoObjectConfigInterface */);
            } else {
                return Some((TransformInfoObjectConfig::new1(transformInfoInterface.clone()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::visual::transform::info::objectConfig::TransformInfoObjectConfig::TransformInfoObjectConfigDyn>>)).clone() /* as TransformInfoObjectConfigInterface */);
            }None
        })) {
            Ok(java_return) => java_return,
            Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            let mut error: String = String::from("Failed To Get Instance: ");
            if transformInfoInterface.is_some() {
                error = { let java_value = (format!("{}{}", error, jcall!(transformInfoInterface).getName())).clone(); java_value };
            }
            if jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypes.clone(); java_field }).LOGGING.clone(); java_field }).contains({ let java_field = jcall!(jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypeFactory.clone(); java_field }).getInstance()).TAGHELPERFACTORYERROR.clone(); java_field }.clone()) {
                { let java_receiver = self.logUtil.clone(); let java_arguments = ({ let java_field = jcall!(self.base.commonStrings).EXCEPTION.clone(); java_field }.clone(), crate::java_into_object(Self::getInstance()), { let java_field = jcall!(self.base.commonStrings).GET_INSTANCE.clone(); java_field }.clone(), crate::java_into_object(e.clone()),); jcall!(java_receiver).put(java_arguments.0, java_arguments.1, java_arguments.2, java_arguments.3); };
            }
            panic!("{:?}", e);
            None
            }
        };if let Some(java_return) = java_try_result { return java_return; }unreachable!("try statement")
    }
    

    // throws Exception
    // Java overloaded method: Rust needs a different name
    pub fn getInstance3_AbeClientInformationInterface_TransformInfoInterface_Document(&mut self, mut abeClientInformation: Option<Rc<RefCell<dyn AbeClientInformationInterface>>>, mut transformInfoInterface: Option<Rc<RefCell<dyn TransformInfoInterface>>>, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<dyn TransformInfoObjectConfigInterface>>> {
        let java_try_result = match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut stringValidationUtil: Option<Rc<RefCell<StringValidationUtil>>> = StringValidationUtil::getInstance();
            if transformInfoInterface.is_some() && !jcall!(stringValidationUtil).isEmpty(jcall!(transformInfoInterface).getStoreName()) {
                return Some((GenericStoreTransformInfoObjectConfig::new3(abeClientInformation.clone(), transformInfoInterface.clone(), document.clone()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::visual::transform::info::objectConfig::GenericStoreTransformInfoObjectConfig::GenericStoreTransformInfoObjectConfigDyn>>)).clone() /* as TransformInfoObjectConfigInterface */);
            } else {
                return Some((TransformInfoObjectConfig::new2(transformInfoInterface.clone(), document.clone()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::visual::transform::info::objectConfig::TransformInfoObjectConfig::TransformInfoObjectConfigDyn>>)).clone() /* as TransformInfoObjectConfigInterface */);
            }None
        })) {
            Ok(java_return) => java_return,
            Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            let mut error: String = String::from("Failed To Get Instance: ");
            if transformInfoInterface.is_some() {
                error = { let java_value = (format!("{}{}", error, jcall!(transformInfoInterface).getName())).clone(); java_value };
            }
            if jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypes.clone(); java_field }).LOGGING.clone(); java_field }).contains({ let java_field = jcall!(jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypeFactory.clone(); java_field }).getInstance()).TAGHELPERFACTORYERROR.clone(); java_field }.clone()) {
                { let java_receiver = self.logUtil.clone(); let java_arguments = ({ let java_field = jcall!(self.base.commonStrings).EXCEPTION.clone(); java_field }.clone(), crate::java_into_object(Self::getInstance()), String::from("getInstance(document)"), crate::java_into_object(e.clone()),); jcall!(java_receiver).put(java_arguments.0, java_arguments.1, java_arguments.2, java_arguments.3); };
            }
            panic!("{:?}", e);
            None
            }
        };if let Some(java_return) = java_try_result { return java_return; }unreachable!("try statement")
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


