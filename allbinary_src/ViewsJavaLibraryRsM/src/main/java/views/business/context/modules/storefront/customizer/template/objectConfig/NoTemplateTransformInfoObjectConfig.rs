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
// package views.business.context.modules.storefront.customizer.template.objectConfig
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
use crate::org::allbinary::logic::string::regex::replace::Replace::Replace;
use crate::org::allbinary::logic::visual::transform::info::TransformInfoData::TransformInfoData;
use crate::org::allbinary::logic::visual::transform::info::TransformInfoInterface::TransformInfoInterface;
use crate::org::allbinary::logic::visual::transform::info::objectConfig::TransformInfoObjectConfig::TransformInfoObjectConfig;
use crate::org::allbinary::logic::visual::transform::info::objectConfig::TransformInfoObjectConfigData::TransformInfoObjectConfigData;
use crate::org::allbinary::logic::visual::transform::template::util::TransformTemplateCustomizerUtil::TransformTemplateCustomizerUtil;
use crate::org::w3c::dom::Document::Document;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct NoTemplateTransformInfoObjectConfig {
    pub base: TransformInfoObjectConfig,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
}


impl NoTemplateTransformInfoObjectConfig {
    

    pub fn new_struct1(mut transformInfoInterface: Option<Rc<RefCell<dyn TransformInfoInterface>>>) -> Self {
        let mut this = Self {
            base: TransformInfoObjectConfig::new_struct1(transformInfoInterface.clone()),
            logUtil: None,
        };
        this.logUtil = LogUtil::getInstance();
        // super(...) is in the struct literal above
        this
    }

    pub fn new1(mut transformInfoInterface: Option<Rc<RefCell<dyn TransformInfoInterface>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct1(transformInfoInterface))))
    }
    

    pub fn new_struct2(mut transformInfoInterface: Option<Rc<RefCell<dyn TransformInfoInterface>>>, mut document: Option<Rc<RefCell<Document>>>) -> Self {
        let mut this = Self {
            base: TransformInfoObjectConfig::new_struct2(transformInfoInterface.clone(), document.clone()),
            logUtil: None,
        };
        this.logUtil = LogUtil::getInstance();
        // super(...) is in the struct literal above
        { let java_arguments = ({ let java_arguments = (this.toXmlDoc(),); this.generate(java_arguments.0) },); this.setDocument(java_arguments.0) };
        this
    }

    pub fn new2(mut transformInfoInterface: Option<Rc<RefCell<dyn TransformInfoInterface>>>, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct2(transformInfoInterface, document))))
    }
    

    pub fn new_struct3(mut transformInfoInterface: Option<Rc<RefCell<dyn TransformInfoInterface>>>, mut name: String, mut r#type: String) -> Self {
        let mut this = Self {
            base: TransformInfoObjectConfig::new_struct3(transformInfoInterface.clone(), name.clone(), r#type.clone()),
            logUtil: None,
        };
        this.logUtil = LogUtil::getInstance();
        // super(...) is in the struct literal above
        { let java_arguments = ({ let java_arguments = (this.toXmlDoc(),); this.generate(java_arguments.0) },); this.setDocument(java_arguments.0) };
        this
    }

    pub fn new3(mut transformInfoInterface: Option<Rc<RefCell<dyn TransformInfoInterface>>>, mut name: String, mut r#type: String) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct3(transformInfoInterface, name, r#type))))
    }
    

    // throws Exception
    pub fn generate(&mut self, mut objectConfigDocument: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Document>>> {
        let mut docString: String = XmlDocumentHelper::toString(objectConfigDocument.clone());
        if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).VIEW.clone()) {
            jcall!(self.logUtil).putF(format!("{}{}", String::from("Initial ObjectConfig: "), docString), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("generate()"));
        }
        let mut storeName: String = jcall!(self.getTransformInfoInterface()).getStoreName();
        let mut viewName: String = jcall!(self.getTransformInfoInterface()).getName();
        let mut hashMap: Option<Rc<RefCell<HashMap>>> = jcall!(StdUtil::getInstance()).createHashMap();
        let mut VARKEY: String = jcall!(TransformInfoObjectConfigData::getInstance()).VARKEY.clone();
        jcall!(hashMap).put(format!("{}{}", VARKEY, jcall!(StoreFrontData::getInstance()).NAME), storeName.clone());
        jcall!(hashMap).put(format!("{}{}", VARKEY, jcall!(TransformInfoData::getInstance()).OWNER), viewName.clone());
        let mut pageName: String = jcall!(TransformTemplateCustomizerUtil::getInstance()).getPageNameHack(jcall!(self.getTransformInfoInterface()).getName(), jcall!(self.getTransformInfoInterface()).getStoreName());
        jcall!(hashMap).put(format!("{}{}", VARKEY, jcall!(TransformInfoData::getInstance()).PARTIAL), pageName.clone());
        let mut replace: Option<Rc<RefCell<Replace>>> = Replace::new1(hashMap.clone());
        let mut newObjectConfigDocument: Option<Rc<RefCell<Document>>> = DomDocumentHelper::create1(jcall!(replace).all(docString.clone()));
        if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).VIEW.clone()) {
            { let java_receiver = self.logUtil.clone(); let java_arguments = (format!("{}{}", String::from("Final ObjectConfig: "), XmlDocumentHelper::toString(newObjectConfigDocument.clone())), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("generate()"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        }
        return newObjectConfigDocument.clone();
    }
}
impl std::ops::Deref for NoTemplateTransformInfoObjectConfig {
    type Target = TransformInfoObjectConfig;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for NoTemplateTransformInfoObjectConfig {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

pub trait NoTemplateTransformInfoObjectConfigDyn: crate::org::allbinary::logic::visual::transform::info::objectConfig::TransformInfoObjectConfig::TransformInfoObjectConfigDyn {
    fn as_NoTemplateTransformInfoObjectConfig(&self) -> &NoTemplateTransformInfoObjectConfig;
    fn as_NoTemplateTransformInfoObjectConfig_mut(&mut self) -> &mut NoTemplateTransformInfoObjectConfig;
    fn generate(&mut self, mut objectConfigDocument: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Document>>> { NoTemplateTransformInfoObjectConfig::generate(self.as_NoTemplateTransformInfoObjectConfig_mut(), objectConfigDocument) }
}
impl NoTemplateTransformInfoObjectConfigDyn for NoTemplateTransformInfoObjectConfig {
    fn as_NoTemplateTransformInfoObjectConfig(&self) -> &NoTemplateTransformInfoObjectConfig { self }
    fn as_NoTemplateTransformInfoObjectConfig_mut(&mut self) -> &mut NoTemplateTransformInfoObjectConfig { self }
}

impl crate::org::allbinary::logic::visual::transform::info::objectConfig::TransformInfoObjectConfig::TransformInfoObjectConfigDyn for NoTemplateTransformInfoObjectConfig {
    fn as_TransformInfoObjectConfig(&self) -> &crate::org::allbinary::logic::visual::transform::info::objectConfig::TransformInfoObjectConfig::TransformInfoObjectConfig { &self.base }
    fn as_TransformInfoObjectConfig_mut(&mut self) -> &mut crate::org::allbinary::logic::visual::transform::info::objectConfig::TransformInfoObjectConfig::TransformInfoObjectConfig { &mut self.base }
}


