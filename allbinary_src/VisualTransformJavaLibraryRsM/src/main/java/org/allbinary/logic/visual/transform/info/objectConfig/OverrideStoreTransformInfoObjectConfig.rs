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
use crate::org::allbinary::logic::system::security::licensing::AbeClientInformationInterface::AbeClientInformationInterface;
use crate::org::allbinary::logic::visual::transform::info::TransformInfoInterface::TransformInfoInterface;
use crate::org::w3c::dom::Document::Document;
use super::GenericStoreTransformInfoObjectConfig::GenericStoreTransformInfoObjectConfig;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct OverrideStoreTransformInfoObjectConfig {
    pub base: GenericStoreTransformInfoObjectConfig,
}


impl OverrideStoreTransformInfoObjectConfig {
    

    pub fn new_struct2(mut abeClientInformation: Option<Rc<RefCell<dyn AbeClientInformationInterface>>>, mut transformInfoInterface: Option<Rc<RefCell<dyn TransformInfoInterface>>>) -> Self {
        let mut this = Self {
            base: GenericStoreTransformInfoObjectConfig::new_struct2(abeClientInformation.clone(), transformInfoInterface.clone()),
        };
        // super(...) is in the struct literal above
        this
    }

    pub fn new2(mut abeClientInformation: Option<Rc<RefCell<dyn AbeClientInformationInterface>>>, mut transformInfoInterface: Option<Rc<RefCell<dyn TransformInfoInterface>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct2(abeClientInformation, transformInfoInterface))))
    }
    

    pub fn new_struct3(mut abeClientInformation: Option<Rc<RefCell<dyn AbeClientInformationInterface>>>, mut transformInfoInterface: Option<Rc<RefCell<dyn TransformInfoInterface>>>, mut document: Option<Rc<RefCell<Document>>>) -> Self {
        let mut this = Self {
            base: GenericStoreTransformInfoObjectConfig::new_struct3(abeClientInformation.clone(), transformInfoInterface.clone(), document.clone()),
        };
        // super(...) is in the struct literal above
        { let java_arguments = ({ let java_arguments = (this.toXmlDoc(),); this.generate(java_arguments.0) },); this.setDocument(java_arguments.0) };
        this
    }

    pub fn new3(mut abeClientInformation: Option<Rc<RefCell<dyn AbeClientInformationInterface>>>, mut transformInfoInterface: Option<Rc<RefCell<dyn TransformInfoInterface>>>, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct3(abeClientInformation, transformInfoInterface, document))))
    }
    

    pub fn new_struct4(mut abeClientInformation: Option<Rc<RefCell<dyn AbeClientInformationInterface>>>, mut transformInfoInterface: Option<Rc<RefCell<dyn TransformInfoInterface>>>, mut name: String, mut r#type: String) -> Self {
        let mut this = Self {
            base: GenericStoreTransformInfoObjectConfig::new_struct4(abeClientInformation.clone(), transformInfoInterface.clone(), name.clone(), r#type.clone()),
        };
        // super(...) is in the struct literal above
        { let java_arguments = ({ let java_arguments = (this.toXmlDoc(),); this.generate(java_arguments.0) },); this.setDocument(java_arguments.0) };
        this
    }

    pub fn new4(mut abeClientInformation: Option<Rc<RefCell<dyn AbeClientInformationInterface>>>, mut transformInfoInterface: Option<Rc<RefCell<dyn TransformInfoInterface>>>, mut name: String, mut r#type: String) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct4(abeClientInformation, transformInfoInterface, name, r#type))))
    }
    

    // throws Exception
    pub fn generate(&mut self, mut objectConfigDocument: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Document>>> {
        let mut newObjectConfigDocument: Option<Rc<RefCell<Document>>> = self.base.generate(objectConfigDocument.clone());
        return newObjectConfigDocument.clone();
    }
}
impl std::ops::Deref for OverrideStoreTransformInfoObjectConfig {
    type Target = GenericStoreTransformInfoObjectConfig;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for OverrideStoreTransformInfoObjectConfig {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::logic::visual::transform::info::objectConfig::GenericStoreTransformInfoObjectConfig::GenericStoreTransformInfoObjectConfigDyn for OverrideStoreTransformInfoObjectConfig {
    fn as_GenericStoreTransformInfoObjectConfig(&self) -> &crate::org::allbinary::logic::visual::transform::info::objectConfig::GenericStoreTransformInfoObjectConfig::GenericStoreTransformInfoObjectConfig { &self.base }
    fn as_GenericStoreTransformInfoObjectConfig_mut(&mut self) -> &mut crate::org::allbinary::logic::visual::transform::info::objectConfig::GenericStoreTransformInfoObjectConfig::GenericStoreTransformInfoObjectConfig { &mut self.base }
    fn generate(&mut self, mut objectConfigDocument: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Document>>> { Self::generate(self, objectConfigDocument) }
}
impl crate::org::allbinary::logic::visual::transform::info::objectConfig::TransformInfoObjectConfig::TransformInfoObjectConfigDyn for OverrideStoreTransformInfoObjectConfig {
    fn as_TransformInfoObjectConfig(&self) -> &crate::org::allbinary::logic::visual::transform::info::objectConfig::TransformInfoObjectConfig::TransformInfoObjectConfig { &self.base.base }
    fn as_TransformInfoObjectConfig_mut(&mut self) -> &mut crate::org::allbinary::logic::visual::transform::info::objectConfig::TransformInfoObjectConfig::TransformInfoObjectConfig { &mut self.base.base }
}


