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
// package views.compound.objectConfig
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::java::io::ByteArrayInputStream::ByteArrayInputStream;
// use crate::java::io::InputStream::InputStream;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
use crate::org::allbinary::data::tree::dom::document::DocumentToNode::DocumentToNode;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use crate::org::allbinary::logic::string::regex::replace::Replace::Replace;
use crate::org::allbinary::logic::system::security::licensing::AbeClientInformationInterface::AbeClientInformationInterface;
use crate::org::allbinary::logic::visual::transform::TransformFactory::TransformFactory;
use crate::org::allbinary::logic::visual::transform::TransformInterface::TransformInterface;
use crate::org::allbinary::logic::visual::transform::info::RootTransformInfoData::RootTransformInfoData;
use crate::org::allbinary::logic::visual::transform::info::TransformInfoData::TransformInfoData;
use crate::org::allbinary::logic::visual::transform::info::TransformInfoDomNode::TransformInfoDomNode;
use crate::org::allbinary::logic::visual::transform::info::TransformInfoHttpInterface::TransformInfoHttpInterface;
use crate::org::allbinary::logic::visual::transform::info::TransformInfoInterface::TransformInfoInterface;
use crate::org::allbinary::logic::visual::transform::info::objectConfig::GenericStoreTransformInfoObjectConfig::GenericStoreTransformInfoObjectConfig;
use crate::org::allbinary::string::CommonSeps::CommonSeps;
use crate::org::w3c::dom::Document::Document;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct CompoundContextTransformInfoObjectConfig {
    pub base: GenericStoreTransformInfoObjectConfig,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
}


impl CompoundContextTransformInfoObjectConfig {
    

    pub fn new_struct2(mut abeClientInformation: Option<Rc<RefCell<dyn AbeClientInformationInterface>>>, mut transformInfoInterface: Option<Rc<RefCell<dyn TransformInfoInterface>>>) -> Self {
        let mut this = Self {
            base: GenericStoreTransformInfoObjectConfig::new_struct2(abeClientInformation.clone(), transformInfoInterface.clone()),
            logUtil: None,
        };
        this.logUtil = LogUtil::getInstance();
        // super(...) is in the struct literal above
        this
    }

    pub fn new2(mut abeClientInformation: Option<Rc<RefCell<dyn AbeClientInformationInterface>>>, mut transformInfoInterface: Option<Rc<RefCell<dyn TransformInfoInterface>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct2(abeClientInformation, transformInfoInterface))))
    }
    

    pub fn new_struct3(mut abeClientInformation: Option<Rc<RefCell<dyn AbeClientInformationInterface>>>, mut transformInfoInterface: Option<Rc<RefCell<dyn TransformInfoInterface>>>, mut document: Option<Rc<RefCell<Document>>>) -> Self {
        let mut this = Self {
            base: GenericStoreTransformInfoObjectConfig::new_struct3(abeClientInformation.clone(), transformInfoInterface.clone(), document.clone()),
            logUtil: None,
        };
        this.logUtil = LogUtil::getInstance();
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
            logUtil: None,
        };
        this.logUtil = LogUtil::getInstance();
        // super(...) is in the struct literal above
        { let java_arguments = ({ let java_arguments = (this.toXmlDoc(),); this.generate(java_arguments.0) },); this.setDocument(java_arguments.0) };
        this
    }

    pub fn new4(mut abeClientInformation: Option<Rc<RefCell<dyn AbeClientInformationInterface>>>, mut transformInfoInterface: Option<Rc<RefCell<dyn TransformInfoInterface>>>, mut name: String, mut r#type: String) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct4(abeClientInformation, transformInfoInterface, name, r#type))))
    }
    

    // throws Exception
    pub fn get(&mut self) -> String {
        let mut storeName: String = jcall!(self.getTransformInfoInterface()).getStoreName();
        let mut httpTransformInfoInterface: Option<Rc<RefCell<dyn TransformInfoHttpInterface>>> = (self.getTransformInfoInterface()).clone() /* as TransformInfoHttpInterface */;
        let mut templateNameOverride: String = jcall!(StringUtil::getInstance()).getNonNull((jcall!(jcall!(httpTransformInfoInterface).getPropertiesHashMap()).get(jcall!(TransformInfoData::getInstance()).PARTIAL.clone())).clone() /* as String */);
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        jcall!(stringBuffer).append(storeName.clone());
        jcall!(stringBuffer).append(templateNameOverride.clone());
        jcall!(stringBuffer).append(jcall!(CommonSeps::getInstance()).SPACE.clone());
        jcall!(stringBuffer).append(crate::org::allbinary::logic::visual::transform::info::RootTransformInfoData::RootTransformInfoData_NAME.with(Clone::clone).clone());
        let mut rootComponentInterface: Option<Rc<RefCell<dyn TransformInterface>>> = jcall!(TransformFactory::getInstance()).getInstance3(self.base.abeClientInformation.clone(), jcall!(stringBuffer).toString(), self.getTransformInfoInterface());
        let mut viewVector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = self.getTransformDomNodes();
        if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).VIEW.clone()) {
            { let java_receiver = self.logUtil.clone(); let java_arguments = (format!("{}{}", String::from("Root View Called with: "), jcall!(viewVector).size()), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("get(transformInfoInterface)"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        }
        let mut rootView: String = jcall!(rootComponentInterface).view();
        let mut startXMLHeader: String = String::from("<xsl:text disable-output-escaping=\"yes\" ><![CDATA[");
        let mut endXMLHeader: String = String::from("]]></xsl:text>");
        let mut size: i32 = jcall!(viewVector).size();
        for index in (0..size) {
            let mut objectConfigTransformInfoDomNode: Option<Rc<RefCell<dyn crate::org::allbinary::logic::visual::transform::info::TransformInfoDomNode::TransformInfoDomNodeDyn>>> = { let java_object = (jcall!(viewVector).get(index.clone())).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn crate::org::allbinary::logic::visual::transform::info::TransformInfoDomNode::TransformInfoDomNodeDyn>>>>().expect("ClassCastException").clone()) };
            let mut templateKey: String = jcall!(objectConfigTransformInfoDomNode).getReplaceKey();
            let mut componentInterface: Option<Rc<RefCell<dyn TransformInterface>>> = jcall!(TransformFactory::getInstance()).getInstance3(self.base.abeClientInformation.clone(), jcall!(jcall!(objectConfigTransformInfoDomNode).getTransformInfoInterface()).getName(), self.getTransformInfoInterface());
            let mut replacementViewString: String = jcall!(componentInterface).view();
            replacementViewString = DocumentToNode::convertDocumentToNodeString(replacementViewString.clone());
            jcall!(stringBuffer).delete(0, jcall!(stringBuffer).length());
            jcall!(stringBuffer).append(startXMLHeader.clone());
            jcall!(stringBuffer).append(replacementViewString.clone());
            jcall!(stringBuffer).append(endXMLHeader.clone());
            let mut templateValue: String = jcall!(stringBuffer).toString();
            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).VIEW.clone()) {
                jcall!(stringBuffer).delete(0, jcall!(stringBuffer).length());
                jcall!(stringBuffer).append(String::from("Replacing: "));
                jcall!(stringBuffer).append(templateKey.clone());
                jcall!(stringBuffer).append(String::from(" with "));
                jcall!(stringBuffer).append(templateValue.clone());
                { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(stringBuffer).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("get()"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
            }
            let mut replace: Option<Rc<RefCell<Replace>>> = Replace::new2(templateKey.clone(), templateValue.clone());
            rootView = jcall!(replace).all(rootView.clone());
        }
        if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).VIEW.clone()) {
            jcall!(self.logUtil).putF(format!("{}{}", String::from("End: Result of a compound view: "), rootView), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("get()"));
        }
        return rootView.clone();
    }
    

    // throws Exception
    pub fn createInputStream(&mut self) -> Option<Rc<RefCell<InputStream>>> {
        let mut completeTemplateViewBytes: Vec<i8> = jcall!(self.get()).getBytes();
        let mut bais: Option<Rc<RefCell<ByteArrayInputStream>>> = ByteArrayInputStream::new(completeTemplateViewBytes.clone());
        if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).VIEW.clone()) {
            { let java_receiver = self.logUtil.clone(); let java_arguments = (format!("{}{}", String::from("Template: "), jcall!(String::from_utf8(completeTemplateViewBytes.clone().iter().map(|value| *value as u8).collect()).unwrap()).toString()), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("createInputStream()"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        }
        return bais.clone();
    }
}
impl std::ops::Deref for CompoundContextTransformInfoObjectConfig {
    type Target = GenericStoreTransformInfoObjectConfig;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for CompoundContextTransformInfoObjectConfig {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::logic::visual::transform::info::objectConfig::GenericStoreTransformInfoObjectConfig::GenericStoreTransformInfoObjectConfigDyn for CompoundContextTransformInfoObjectConfig {
    fn as_GenericStoreTransformInfoObjectConfig(&self) -> &crate::org::allbinary::logic::visual::transform::info::objectConfig::GenericStoreTransformInfoObjectConfig::GenericStoreTransformInfoObjectConfig { &self.base }
    fn as_GenericStoreTransformInfoObjectConfig_mut(&mut self) -> &mut crate::org::allbinary::logic::visual::transform::info::objectConfig::GenericStoreTransformInfoObjectConfig::GenericStoreTransformInfoObjectConfig { &mut self.base }
}
impl crate::org::allbinary::logic::visual::transform::info::objectConfig::TransformInfoObjectConfig::TransformInfoObjectConfigDyn for CompoundContextTransformInfoObjectConfig {
    fn as_TransformInfoObjectConfig(&self) -> &crate::org::allbinary::logic::visual::transform::info::objectConfig::TransformInfoObjectConfig::TransformInfoObjectConfig { &self.base.base }
    fn as_TransformInfoObjectConfig_mut(&mut self) -> &mut crate::org::allbinary::logic::visual::transform::info::objectConfig::TransformInfoObjectConfig::TransformInfoObjectConfig { &mut self.base.base }
}


