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
use std::collections::HashMap;
use super::*;
// use crate::java::util::HashMap::HashMap;
use crate::org::allbinary::business::context::modules::storefront::StoreFrontData::StoreFrontData;
use crate::org::allbinary::data::tree::dom::document::DomDocumentHelper::DomDocumentHelper;
use crate::org::allbinary::data::tree::dom::document::XmlDocumentHelper::XmlDocumentHelper;
use crate::org::allbinary::logic::StdUtil::StdUtil;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use crate::org::allbinary::logic::string::regex::replace::Replace::Replace;
use crate::org::allbinary::logic::system::security::licensing::AbeClientInformationInterface::AbeClientInformationInterface;
use crate::org::allbinary::logic::visual::transform::info::RootTransformInfoData::RootTransformInfoData;
use crate::org::allbinary::logic::visual::transform::info::TransformInfoData::TransformInfoData;
use crate::org::allbinary::logic::visual::transform::info::TransformInfoHttp::TransformInfoHttp;
use crate::org::allbinary::logic::visual::transform::info::TransformInfoInterface::TransformInfoInterface;
use crate::org::allbinary::logic::visual::transform::template::TransformTemplateFactory::TransformTemplateFactory;
use crate::org::allbinary::logic::visual::transform::template::TransformTemplateInterface::TransformTemplateInterface;
use crate::org::allbinary::logic::visual::transform::template::util::TransformTemplateCustomizerUtil::TransformTemplateCustomizerUtil;
use crate::org::allbinary::string::CommonSeps::CommonSeps;
use crate::org::w3c::dom::Document::Document;
use super::TransformInfoObjectConfig::TransformInfoObjectConfig;
use super::TransformInfoObjectConfigData::TransformInfoObjectConfigData;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct GenericStoreTransformInfoObjectConfig {
    pub base: TransformInfoObjectConfig,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub abeClientInformation: Option<Rc<RefCell<dyn AbeClientInformationInterface>>>,
}


impl GenericStoreTransformInfoObjectConfig {
    

    pub fn new_struct2(mut abeClientInformation: Option<Rc<RefCell<dyn AbeClientInformationInterface>>>, mut transformInfoInterface: Option<Rc<RefCell<dyn TransformInfoInterface>>>) -> Self {
        let mut this = Self {
            base: TransformInfoObjectConfig::new_struct1(transformInfoInterface.clone()),
            logUtil: None,
            abeClientInformation: None,
        };
        this.logUtil = LogUtil::getInstance();
        // super(...) is in the struct literal above
        this.abeClientInformation = abeClientInformation.clone();
        this
    }

    pub fn new2(mut abeClientInformation: Option<Rc<RefCell<dyn AbeClientInformationInterface>>>, mut transformInfoInterface: Option<Rc<RefCell<dyn TransformInfoInterface>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct2(abeClientInformation, transformInfoInterface))))
    }
    

    pub fn new_struct3(mut abeClientInformation: Option<Rc<RefCell<dyn AbeClientInformationInterface>>>, mut transformInfoInterface: Option<Rc<RefCell<dyn TransformInfoInterface>>>, mut document: Option<Rc<RefCell<Document>>>) -> Self {
        let mut this = Self {
            base: TransformInfoObjectConfig::new_struct2(transformInfoInterface.clone(), document.clone()),
            logUtil: None,
            abeClientInformation: None,
        };
        this.logUtil = LogUtil::getInstance();
        // super(...) is in the struct literal above
        this.abeClientInformation = abeClientInformation.clone();
        { let java_arguments = ({ let java_arguments = (this.toXmlDoc(),); this.generate(java_arguments.0) },); this.setDocument(java_arguments.0) };
        this
    }

    pub fn new3(mut abeClientInformation: Option<Rc<RefCell<dyn AbeClientInformationInterface>>>, mut transformInfoInterface: Option<Rc<RefCell<dyn TransformInfoInterface>>>, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct3(abeClientInformation, transformInfoInterface, document))))
    }
    

    pub fn new_struct4(mut abeClientInformation: Option<Rc<RefCell<dyn AbeClientInformationInterface>>>, mut transformInfoInterface: Option<Rc<RefCell<dyn TransformInfoInterface>>>, mut name: String, mut r#type: String) -> Self {
        let mut this = Self {
            base: TransformInfoObjectConfig::new_struct3(transformInfoInterface.clone(), name.clone(), r#type.clone()),
            logUtil: None,
            abeClientInformation: None,
        };
        this.logUtil = LogUtil::getInstance();
        // super(...) is in the struct literal above
        this.abeClientInformation = abeClientInformation.clone();
        { let java_arguments = ({ let java_arguments = (this.toXmlDoc(),); this.generate(java_arguments.0) },); this.setDocument(java_arguments.0) };
        this
    }

    pub fn new4(mut abeClientInformation: Option<Rc<RefCell<dyn AbeClientInformationInterface>>>, mut transformInfoInterface: Option<Rc<RefCell<dyn TransformInfoInterface>>>, mut name: String, mut r#type: String) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct4(abeClientInformation, transformInfoInterface, name, r#type))))
    }
    

    // throws Exception
    pub fn generate(&mut self, mut objectConfigDocument: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Document>>> {
        if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).VIEW.clone()) {
            let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
            jcall!(stringBuffer).append(String::from("TransformInfo: "));
            if self.getTransformInfoInterface().is_some() {
                jcall!(stringBuffer).append(jcall!(self.getTransformInfoInterface()).getName());
            } else {
                jcall!(stringBuffer).append(String::from("No Owner!?#@"));
            }
            { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(stringBuffer).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("generate()"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        }
        let mut transformInfoHttpStoreInterface: Option<Rc<RefCell<dyn crate::org::allbinary::logic::visual::transform::info::TransformInfoHttp::TransformInfoHttpDyn>>> = (self.getTransformInfoInterface()).clone() /* as TransformInfoHttp */;
        let mut objectConfigDocumentString: String = XmlDocumentHelper::toString(objectConfigDocument.clone());
        let mut replaceHashMap: Option<Rc<RefCell<HashMap>>> = self.createReplaceHashMap(transformInfoHttpStoreInterface.clone(), objectConfigDocumentString.clone());
        return self.generate2(objectConfigDocumentString.clone(), replaceHashMap.clone());
    }
    

    // throws Exception
    pub fn createReplaceHashMap(&mut self, mut transformInfoHttpStoreInterface: Option<Rc<RefCell<dyn crate::org::allbinary::logic::visual::transform::info::TransformInfoHttp::TransformInfoHttpDyn>>>, mut objectConfigDocumentString: String) -> Option<Rc<RefCell<HashMap>>> {
        let mut storeName: String = jcall!(transformInfoHttpStoreInterface).getStoreName();
        let mut hashMap: Option<Rc<RefCell<HashMap>>> = self.createHashMap(transformInfoHttpStoreInterface.clone(), objectConfigDocumentString.clone());
        let mut transformInfoObjectConfigData: Option<Rc<RefCell<TransformInfoObjectConfigData>>> = TransformInfoObjectConfigData::getInstance();
        jcall!(hashMap).put(format!("{}{}", jcall!(transformInfoObjectConfigData).VARKEY, jcall!(StoreFrontData::getInstance()).NAME), storeName.clone());
        let mut pageName: String = jcall!(TransformTemplateCustomizerUtil::getInstance()).getPageNameHack(jcall!(self.getTransformInfoInterface()).getName(), storeName.clone());
        jcall!(hashMap).put(format!("{}{}", jcall!(transformInfoObjectConfigData).VARKEY, jcall!(TransformInfoData::getInstance()).PARTIAL), pageName.clone());
        return hashMap.clone();
    }
    

    // throws Exception
    pub fn createHashMap(&mut self, mut transformInfoHttpStoreInterface: Option<Rc<RefCell<dyn crate::org::allbinary::logic::visual::transform::info::TransformInfoHttp::TransformInfoHttpDyn>>>, mut objectConfigDocumentString: String) -> Option<Rc<RefCell<HashMap>>> {
        let mut hashMap: Option<Rc<RefCell<HashMap>>> = jcall!(StdUtil::getInstance()).createHashMap();
        let mut storeName: String = jcall!(transformInfoHttpStoreInterface).getStoreName();
        let mut propertiesHashMap: Option<Rc<RefCell<HashMap>>> = jcall!(transformInfoHttpStoreInterface).getPropertiesHashMap();
        let mut templateNameOverride: String = jcall!(StringUtil::getInstance()).getNonNull((jcall!(propertiesHashMap).get(jcall!(TransformInfoData::getInstance()).PARTIAL.clone())).clone() /* as String */);
        if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).VIEW.clone()) {
            jcall!(self.logUtil).putF(format!("{}{}", String::from("TemplateNameOverride: "), templateNameOverride), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("generate()"));
        }
        let mut templateNameStringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        jcall!(templateNameStringBuffer).append(storeName.clone());
        jcall!(templateNameStringBuffer).append(templateNameOverride.clone());
        jcall!(templateNameStringBuffer).append(jcall!(CommonSeps::getInstance()).SPACE.clone());
        jcall!(templateNameStringBuffer).append(crate::org::allbinary::logic::visual::transform::info::RootTransformInfoData::RootTransformInfoData_NAME.with(Clone::clone).clone());
        let mut templateNameKey: String = format!("{}{}", jcall!(TransformInfoObjectConfigData::getInstance()).VARKEY, jcall!(TransformInfoData::getInstance()).OWNER);
        if objectConfigDocumentString.find(&templateNameKey.clone()).map(|index| index as i32).unwrap_or(-1) != -1 {
            let mut templateInterface: Option<Rc<RefCell<dyn TransformTemplateInterface>>> = jcall!(TransformTemplateFactory::getInstance()).getInstance4(self.abeClientInformation.clone(), jcall!(templateNameStringBuffer).toString(), jcall!(transformInfoHttpStoreInterface).getPropertiesHashMap(), jcall!(transformInfoHttpStoreInterface).getPageContext());
            let mut selectedTemplate: String = jcall!(templateInterface).getName();
            jcall!(hashMap).put(templateNameKey.clone(), selectedTemplate.clone());
        }
        return hashMap.clone();
    }
    

    // throws Exception
    // Java overloaded method: Rust needs a different name
    pub fn generate2(&mut self, mut objectConfigDocumentString: String, mut hashMap: Option<Rc<RefCell<HashMap>>>) -> Option<Rc<RefCell<Document>>> {
        let mut replace: Option<Rc<RefCell<Replace>>> = Replace::new1(hashMap.clone());
        let mut newObjectConfigDocument: Option<Rc<RefCell<Document>>> = DomDocumentHelper::create1(jcall!(replace).all(objectConfigDocumentString.clone()));
        if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).VIEW.clone()) {
            { let java_receiver = self.logUtil.clone(); let java_arguments = (format!("{}{}", String::from("Final ObjectConfig: "), XmlDocumentHelper::toString(newObjectConfigDocument.clone())), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("generate()"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        }
        return newObjectConfigDocument.clone();
    }
}
impl std::ops::Deref for GenericStoreTransformInfoObjectConfig {
    type Target = TransformInfoObjectConfig;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for GenericStoreTransformInfoObjectConfig {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

pub trait GenericStoreTransformInfoObjectConfigDyn: crate::org::allbinary::logic::visual::transform::info::objectConfig::TransformInfoObjectConfig::TransformInfoObjectConfigDyn {
    fn as_GenericStoreTransformInfoObjectConfig(&self) -> &GenericStoreTransformInfoObjectConfig;
    fn as_GenericStoreTransformInfoObjectConfig_mut(&mut self) -> &mut GenericStoreTransformInfoObjectConfig;
    fn generate(&mut self, mut objectConfigDocument: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Document>>> { GenericStoreTransformInfoObjectConfig::generate(self.as_GenericStoreTransformInfoObjectConfig_mut(), objectConfigDocument) }
    fn createReplaceHashMap(&mut self, mut transformInfoHttpStoreInterface: Option<Rc<RefCell<dyn crate::org::allbinary::logic::visual::transform::info::TransformInfoHttp::TransformInfoHttpDyn>>>, mut objectConfigDocumentString: String) -> Option<Rc<RefCell<HashMap>>> { GenericStoreTransformInfoObjectConfig::createReplaceHashMap(self.as_GenericStoreTransformInfoObjectConfig_mut(), transformInfoHttpStoreInterface, objectConfigDocumentString) }
    fn createHashMap(&mut self, mut transformInfoHttpStoreInterface: Option<Rc<RefCell<dyn crate::org::allbinary::logic::visual::transform::info::TransformInfoHttp::TransformInfoHttpDyn>>>, mut objectConfigDocumentString: String) -> Option<Rc<RefCell<HashMap>>> { GenericStoreTransformInfoObjectConfig::createHashMap(self.as_GenericStoreTransformInfoObjectConfig_mut(), transformInfoHttpStoreInterface, objectConfigDocumentString) }
    fn generate2(&mut self, mut objectConfigDocumentString: String, mut hashMap: Option<Rc<RefCell<HashMap>>>) -> Option<Rc<RefCell<Document>>> { GenericStoreTransformInfoObjectConfig::generate2(self.as_GenericStoreTransformInfoObjectConfig_mut(), objectConfigDocumentString, hashMap) }
}
impl GenericStoreTransformInfoObjectConfigDyn for GenericStoreTransformInfoObjectConfig {
    fn as_GenericStoreTransformInfoObjectConfig(&self) -> &GenericStoreTransformInfoObjectConfig { self }
    fn as_GenericStoreTransformInfoObjectConfig_mut(&mut self) -> &mut GenericStoreTransformInfoObjectConfig { self }
}

impl crate::org::allbinary::logic::visual::transform::info::objectConfig::TransformInfoObjectConfig::TransformInfoObjectConfigDyn for GenericStoreTransformInfoObjectConfig {
    fn as_TransformInfoObjectConfig(&self) -> &crate::org::allbinary::logic::visual::transform::info::objectConfig::TransformInfoObjectConfig::TransformInfoObjectConfig { &self.base }
    fn as_TransformInfoObjectConfig_mut(&mut self) -> &mut crate::org::allbinary::logic::visual::transform::info::objectConfig::TransformInfoObjectConfig::TransformInfoObjectConfig { &mut self.base }
}


