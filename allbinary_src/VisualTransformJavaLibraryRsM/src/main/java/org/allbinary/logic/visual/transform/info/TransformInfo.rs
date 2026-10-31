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
use crate::Calendar;
use std::collections::HashMap;
use super::*;
// use crate::java::util::Calendar::Calendar;
// use crate::java::util::HashMap::HashMap;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
use crate::org::allbinary::business::context::modules::storefront::StoreFrontData::StoreFrontData;
use crate::org::allbinary::business::entry::EntryData::EntryData;
use crate::org::allbinary::data::tree::dom::document::DomDocumentHelper::DomDocumentHelper;
use crate::org::allbinary::data::tree::dom::document::XmlDocumentHelper::XmlDocumentHelper;
use crate::org::allbinary::globals::FREEBLISKET_PATH_GLOBALS::FREEBLISKET_PATH_GLOBALS;
use crate::org::allbinary::globals::URLGLOBALS::URLGLOBALS;
use crate::org::allbinary::logic::StdUtil::StdUtil;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::control::crypt::Encoder::Encoder;
use crate::org::allbinary::logic::control::crypt::file::CryptFileReader::CryptFileReader;
use crate::org::allbinary::logic::io::OutputTypeData::OutputTypeData;
use crate::org::allbinary::logic::io::path::AbFilePath::AbFilePath;
use crate::org::allbinary::logic::io::path::AbPath::AbPath;
use crate::org::allbinary::logic::io::path::AbPathData::AbPathData;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use crate::org::allbinary::logic::string::StringValidationUtil::StringValidationUtil;
use crate::org::allbinary::logic::system::security::licensing::AbeClientInformationInterface::AbeClientInformationInterface;
use crate::org::allbinary::logic::system::security::licensing::ServiceClientInformationInterfaceFactory::ServiceClientInformationInterfaceFactory;
use crate::org::allbinary::logic::visual::transform::info::data::TransformInfoDataData::TransformInfoDataData;
use crate::org::allbinary::logic::visual::transform::info::objectConfig::TransformInfoObjectConfig::TransformInfoObjectConfig;
use crate::org::allbinary::logic::visual::transform::info::objectConfig::TransformInfoObjectConfigAndManipulatorFactory::TransformInfoObjectConfigAndManipulatorFactory;
use crate::org::allbinary::logic::visual::transform::info::objectConfig::TransformInfoObjectConfigInterface::TransformInfoObjectConfigInterface;
use crate::org::allbinary::logic::visual::transform::info::template::TransformInfoTemplateData::TransformInfoTemplateData;
use crate::org::allbinary::logic::visual::transform::info::viewObject::TransformInfoObjectData::TransformInfoObjectData;
use crate::org::allbinary::string::CommonSeps::CommonSeps;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::org::w3c::dom::Document::Document;
use super::TransformInfoInterface::TransformInfoInterface;
use super::TransformInfoData::TransformInfoData;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct TransformInfo {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub commonStrings: Option<Rc<RefCell<CommonStrings>>>,
    pub abeClientInformation: Option<Rc<RefCell<dyn AbeClientInformationInterface>>>,
    pub isChild: bool,
    pub name: String,
    pub storeName: String,
    pub objectFileName: String,
    pub anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>>,
    pub objectConfigFileName: String,
    pub objectConfigInterface: Option<Rc<RefCell<dyn TransformInfoObjectConfigInterface>>>,
    pub template: String,
    pub templateFileName: String,
    pub dataFileName: String,
    pub data: String,
}

impl TransformInfoInterface for TransformInfo {
    fn log(&mut self) -> String { Self::log(self) }
    fn r#override(&mut self, mut hashMap: Option<Rc<RefCell<HashMap>>>) { Self::r#override(self, hashMap) }
    fn getName(&mut self) -> String { Self::getName(self) }
    fn setStoreName(&mut self, mut value: String) { Self::setStoreName(self, value) }
    fn getStoreName(&mut self) -> String { Self::getStoreName(self) }
    fn getObjectFile(&mut self) -> String { Self::getObjectFile(self) }
    fn getObject(&mut self) -> Option<Rc<RefCell<Box<dyn std::any::Any>>>> { Self::getObject(self) }
    fn getTemplateFilePath(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>> { Self::getTemplateFilePath(self) }
    fn getTemplateFile(&mut self) -> String { Self::getTemplateFile(self) }
    fn getTemplate(&mut self) -> String { Self::getTemplate(self) }
    fn getDataFilePath(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>> { Self::getDataFilePath(self) }
    fn getDataFile(&mut self) -> String { Self::getDataFile(self) }
    fn getDataDocument(&mut self) -> Option<Rc<RefCell<Document>>> { Self::getDataDocument(self) }
    fn setName(&mut self, mut value: String) { Self::setName(self, value) }
    fn setObjectFile(&mut self, mut value: String) { Self::setObjectFile(self, value) }
    fn setObject(&mut self, mut anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) { Self::setObject(self, anyType) }
    fn setObjectConfigInterface(&mut self, mut objectConfigInterface: Option<Rc<RefCell<dyn TransformInfoObjectConfigInterface>>>) { Self::setObjectConfigInterface(self, objectConfigInterface) }
    fn getObjectConfigInterface(&mut self) -> Option<Rc<RefCell<dyn TransformInfoObjectConfigInterface>>> { Self::getObjectConfigInterface(self) }
    fn setTemplateFile(&mut self, mut value: String) { Self::setTemplateFile(self, value) }
    fn setTemplate(&mut self, mut value: String) { Self::setTemplate(self, value) }
    fn setDataFile(&mut self, mut value: String) { Self::setDataFile(self, value) }
    fn setData(&mut self, mut value: String) { Self::setData(self, value) }
    fn setChild(&mut self) { Self::setChild(self) }
    fn isChild(&mut self) -> bool { Self::isChild(self) }
}
impl crate::org::allbinary::data::tables::TableMappingInterface::TableMappingInterface for TransformInfo {
    fn getKey(&mut self) -> Option<Rc<RefCell<Box<dyn std::any::Any>>>> { Self::getKey(self) }
    fn toVector(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> { Self::toVector(self) }
    fn toHashMap(&mut self) -> Option<Rc<RefCell<HashMap>>> { Self::toHashMap(self) }
}

impl TransformInfo {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            logUtil: None,
            commonStrings: None,
            abeClientInformation: None,
            isChild: false,
            name: String::new(),
            storeName: String::new(),
            objectFileName: String::new(),
            anyType: None,
            objectConfigFileName: String::new(),
            objectConfigInterface: None,
            template: String::new(),
            templateFileName: String::new(),
            dataFileName: String::new(),
            data: String::new(),
        };
        this.logUtil = LogUtil::getInstance();
        this.commonStrings = CommonStrings::getInstance();
        this.abeClientInformation = ServiceClientInformationInterfaceFactory::getInstance();
        this.isChild = false;
        this.data = jcall!(StringUtil::getInstance()).EMPTY_STRING.clone();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn new_struct1(mut name: String) -> Self {
        let mut this = Self {
            logUtil: None,
            commonStrings: None,
            abeClientInformation: None,
            isChild: false,
            name: String::new(),
            storeName: String::new(),
            objectFileName: String::new(),
            anyType: None,
            objectConfigFileName: String::new(),
            objectConfigInterface: None,
            template: String::new(),
            templateFileName: String::new(),
            dataFileName: String::new(),
            data: String::new(),
        };
        this.logUtil = LogUtil::getInstance();
        this.commonStrings = CommonStrings::getInstance();
        this.abeClientInformation = ServiceClientInformationInterfaceFactory::getInstance();
        this.isChild = false;
        this.data = jcall!(StringUtil::getInstance()).EMPTY_STRING.clone();
        this.setName(name.clone());
        this
    }

    pub fn new1(mut name: String) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct1(name))))
    }
    

    pub fn new_struct5(mut name: String, mut objectFileName: String, mut objectConfigFileName: String, mut templateFileName: String, mut dataFileName: String) -> Self {
        let mut this = Self {
            logUtil: None,
            commonStrings: None,
            abeClientInformation: None,
            isChild: false,
            name: String::new(),
            storeName: String::new(),
            objectFileName: String::new(),
            anyType: None,
            objectConfigFileName: String::new(),
            objectConfigInterface: None,
            template: String::new(),
            templateFileName: String::new(),
            dataFileName: String::new(),
            data: String::new(),
        };
        this.logUtil = LogUtil::getInstance();
        this.commonStrings = CommonStrings::getInstance();
        this.abeClientInformation = ServiceClientInformationInterfaceFactory::getInstance();
        this.isChild = false;
        this.data = jcall!(StringUtil::getInstance()).EMPTY_STRING.clone();
        this.setName(name.clone());
        this.setObjectFile(objectFileName.clone());
        this.setObjectConfigFile(objectConfigFileName.clone());
        this.setTemplateFile(templateFileName.clone());
        this.setDataFile(dataFileName.clone());
        this
    }

    pub fn new5(mut name: String, mut objectFileName: String, mut objectConfigFileName: String, mut templateFileName: String, mut dataFileName: String) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct5(name, objectFileName, objectConfigFileName, templateFileName, dataFileName))))
    }
    

    // throws Exception
    pub fn r#override(&mut self, mut hashMap: Option<Rc<RefCell<HashMap>>>) {
        let mut transformInfoData: Option<Rc<RefCell<TransformInfoData>>> = TransformInfoData::getInstance();
        let mut stringValidationUtil: Option<Rc<RefCell<StringValidationUtil>>> = StringValidationUtil::getInstance();
        let mut aName: String = (jcall!(hashMap).get(jcall!(transformInfoData).NAME.clone())).clone() /* as String */;
        if !jcall!(stringValidationUtil).isEmpty(aName.clone()) {
            self.name = aName.clone();
        }
        if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).VIEW.clone()) {
            let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
            jcall!(stringBuffer).append(String::from("TransformInfo properties overridden for: "));
            jcall!(stringBuffer).append(self.getName());
            jcall!(stringBuffer).append(String::from("\n properties: "));
            jcall!(stringBuffer).append(jcall!(hashMap).toString());
            { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(stringBuffer).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("override(HashMap hashMap)"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        }
        let mut aStoreName: String = (jcall!(hashMap).get(jcall!(StoreFrontData::getInstance()).NAME.clone())).clone() /* as String */;
        if !jcall!(stringValidationUtil).isEmpty(aStoreName.clone()) {
            self.storeName = aStoreName.clone();
        }
        let mut aObjectFileName: String = (jcall!(hashMap).get(jcall!(transformInfoData).OBJECTFILENAME.clone())).clone() /* as String */;
        if !jcall!(stringValidationUtil).isEmpty(aObjectFileName.clone()) {
            self.objectFileName = aObjectFileName.clone();
        }
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        jcall!(stringBuffer).append(URLGLOBALS::getMainPath());
        jcall!(stringBuffer).append(jcall!(FREEBLISKET_PATH_GLOBALS::getInstance()).XSLPATH.clone());
        jcall!(stringBuffer).append(self.storeName.clone());
        jcall!(stringBuffer).append(jcall!(AbPathData::getInstance()).SEPARATOR.clone());
        let mut fileAbPath: Option<Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>> = AbPath::new(jcall!(stringBuffer).toString(), jcall!(StringUtil::getInstance()).EMPTY_STRING.clone()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>);
        let mut aObject: String = (jcall!(hashMap).get(jcall!(transformInfoData).OBJECT.clone())).clone() /* as String */;
        if !jcall!(stringValidationUtil).isEmpty(aObject.clone()) {
            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).VIEW.clone()) {
                { let java_receiver = self.logUtil.clone(); let java_arguments = (format!("{}{}", String::from("TransformInfo override view Object for: "), self.getName()), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("override(HashMap hashMap)"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
            }
            let mut fileDataString: String = jcall!(CryptFileReader::new(jcall!(TransformInfoObjectData::getInstance()).UNCRYPTED_EXTENSION.clone(), jcall!(TransformInfoObjectData::getInstance()).ENCRYPTED_EXTENSION.clone())).get(AbPath::new(jcall!(fileAbPath).toString(), aObject.clone()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>));
            if true {
                self.anyType = fileDataString.clone();
            }
        }
        if !jcall!(stringValidationUtil).isEmpty((jcall!(hashMap).get(jcall!(transformInfoData).OBJECTCONFIG.clone())).clone() /* as String */) {
            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).VIEW.clone()) {
                { let java_receiver = self.logUtil.clone(); let java_arguments = (format!("{}{}", String::from("TransformInfo override ObjectConfig for: "), self.getName()), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("override(HashMap hashMap)"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
            }
            { let java_arguments = ((jcall!(hashMap).get(jcall!(transformInfoData).OBJECTCONFIG.clone())).clone() /* as String */,); self.setObjectConfig(java_arguments.0) };
        }
        let mut objectConfigFileName: String = (jcall!(hashMap).get(jcall!(transformInfoData).OBJECTCONFIGFILENAME.clone())).clone() /* as String */;
        if !jcall!(stringValidationUtil).isEmpty(objectConfigFileName.clone()) {
            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).VIEW.clone()) {
                { let java_receiver = self.logUtil.clone(); let java_arguments = (format!("{}{}", String::from("TransformInfo override ObjectConfig File for: "), self.getName()), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("override(HashMap hashMap)"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
            }
            self.setObjectConfigFile(objectConfigFileName.clone());
        }
        let mut r#type: String = (jcall!(hashMap).get(jcall!(OutputTypeData::getInstance()).NAME.clone())).clone() /* as String */;
        if !jcall!(stringValidationUtil).isEmpty(r#type.clone()) {
            if self.getObjectConfigInterface().is_none() {
                self.setObjectConfigInterface(TransformInfoObjectConfig::new1(None /* Java this: a &mut self method has no Rc of itself */).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::visual::transform::info::objectConfig::TransformInfoObjectConfig::TransformInfoObjectConfigDyn>>));
            }
            jcall!(self.getObjectConfigInterface()).setOutputTypeName(r#type.clone());
        }
        let mut aTemplateFileName: String = (jcall!(hashMap).get(jcall!(transformInfoData).TEMPLATEFILENAME.clone())).clone() /* as String */;
        if !jcall!(stringValidationUtil).isEmpty(aTemplateFileName.clone()) {
            self.templateFileName = aTemplateFileName.clone();
        }
        let mut aTemplate: String = (jcall!(hashMap).get(jcall!(transformInfoData).TEMPLATE.clone())).clone() /* as String */;
        if !jcall!(stringValidationUtil).isEmpty(aTemplate.clone()) {
            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).VIEW.clone()) {
                { let java_receiver = self.logUtil.clone(); let java_arguments = (format!("{}{}", String::from("TransformInfo override Template for: "), self.getName()), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("override(HashMap hashMap)"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
            }
            let mut fileDataString: String = jcall!(CryptFileReader::new(jcall!(TransformInfoTemplateData::getInstance()).UNCRYPTED_EXTENSION.clone(), jcall!(TransformInfoTemplateData::getInstance()).ENCRYPTED_EXTENSION.clone())).get(AbPath::new(jcall!(fileAbPath).toString(), aTemplate.clone()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>));
            if !jcall!(stringValidationUtil).isEmpty(fileDataString.clone()) {
                self.template = fileDataString.clone();
            }
        }
        let mut aDataFileName: String = (jcall!(hashMap).get(jcall!(transformInfoData).DATAFILENAME.clone())).clone() /* as String */;
        if !jcall!(stringValidationUtil).isEmpty(aDataFileName.clone()) {
            self.dataFileName = aDataFileName.clone();
        }
        let mut aData: String = (jcall!(hashMap).get(jcall!(transformInfoData).DATA.clone())).clone() /* as String */;
        if !jcall!(stringValidationUtil).isEmpty(aData.clone()) {
            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).VIEW.clone()) {
                { let java_receiver = self.logUtil.clone(); let java_arguments = (format!("{}{}", String::from("TransformInfo override Data for: "), self.getName()), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("override(HashMap hashMap)"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
            }
            let mut fileDataString: String = jcall!(CryptFileReader::new(jcall!(TransformInfoDataData::getInstance()).UNCRYPTED_EXTENSION.clone(), jcall!(TransformInfoDataData::getInstance()).ENCRYPTED_EXTENSION.clone())).get(AbPath::new(jcall!(fileAbPath).toString(), aData.clone()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>));
            if !jcall!(stringValidationUtil).isEmpty(fileDataString.clone()) {
                self.data = fileDataString.clone();
            }
        }
        if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).VIEW.clone()) {
            stringBuffer = StringMaker::new();
            jcall!(stringBuffer).append(String::from("Name: "));
            jcall!(stringBuffer).append(aName.clone());
            jcall!(stringBuffer).append(String::from("\nTemp Object File: "));
            jcall!(stringBuffer).append(aObject.clone());
            jcall!(stringBuffer).append(String::from("\nTemp Object Config File: "));
            jcall!(stringBuffer).append(jcall!(self.getObjectConfigFilePath()).toString());
            jcall!(stringBuffer).append(String::from("\nTemp Template File: "));
            jcall!(stringBuffer).append(aTemplate.clone());
            jcall!(stringBuffer).append(String::from("\nTemp Data File: "));
            jcall!(stringBuffer).append(aData.clone());
            jcall!(stringBuffer).append(String::from("\nStore Name: "));
            jcall!(stringBuffer).append(aStoreName.clone());
            jcall!(stringBuffer).append(String::from("\nObjectFile: "));
            jcall!(stringBuffer).append(self.getObjectFile());
            jcall!(stringBuffer).append(String::from("->TransformInfo"));
            { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(stringBuffer).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("TransformInfoInterface(HashMap)"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        }
        if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).TAGHELPERFACTORY.clone()) {
            { let java_receiver = self.logUtil.clone(); let java_arguments = (self.log(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("override"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        }
    }
    

    pub fn log(&mut self) -> String {
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        jcall!(stringBuffer).append(String::from("Name: "));
        jcall!(stringBuffer).append(self.name.clone());
        jcall!(stringBuffer).append(String::from("\nObject File: "));
        jcall!(stringBuffer).append(self.objectFileName.clone());
        jcall!(stringBuffer).append(String::from("\nObject Config File Path: "));
        jcall!(stringBuffer).append(self.getObjectConfigFile());
        jcall!(stringBuffer).append(String::from("\nTemplate File: "));
        jcall!(stringBuffer).append(self.templateFileName.clone());
        jcall!(stringBuffer).append(String::from("\nData File: "));
        jcall!(stringBuffer).append(self.dataFileName.clone());
        jcall!(stringBuffer).append(String::from("\nStore Name: "));
        jcall!(stringBuffer).append(self.storeName.clone());
        jcall!(stringBuffer).append(jcall!(CommonSeps::getInstance()).NEW_LINE.clone());
        jcall!(stringBuffer).append(self.getObjectFile());
        jcall!(stringBuffer).append(String::from("->TransformInfo"));
        jcall!(stringBuffer).append(String::from("TransformInfoInterface(HashMap)"));
        return jcall!(stringBuffer).toString();
    }
    

    pub fn setName(&mut self, mut value: String) {
        self.name = value.clone();
    }
    

    pub fn setStoreName(&mut self, mut value: String) {
        self.storeName = value.clone();
    }
    

    pub fn getStoreName(&mut self) -> String {
        return self.storeName.clone();
    }
    

    pub fn setObject(&mut self, mut anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) {
        self.anyType = anyType.clone();
    }
    

    pub fn setObjectFile(&mut self, mut value: String) {
        self.objectFileName = value.clone();
    }
    

    pub fn setTemplate(&mut self, mut value: String) {
        self.template = value.clone();
    }
    

    pub fn setTemplateFile(&mut self, mut value: String) {
        self.templateFileName = value.clone();
    }
    

    // throws Exception
    pub fn setObjectConfigFileName(&mut self, mut value: String) {
        self.objectConfigFileName = value.clone();
    }
    

    // throws Exception
    pub fn setObjectConfigFile(&mut self, mut value: String) {
        self.objectConfigFileName = value.clone();
        { let java_arguments = (jcall!(TransformInfoObjectConfigAndManipulatorFactory::getInstance()).getInstance3(self.abeClientInformation.clone(), None /* Java this: a &mut self method has no Rc of itself */, self.getObjectConfigFilePath()),); self.setObjectConfigInterface(java_arguments.0) };
    }
    

    // throws Exception
    pub fn setObjectConfig(&mut self, mut value: String) {
        let mut document: Option<Rc<RefCell<Document>>> = DomDocumentHelper::create1(value.clone());
        { let java_arguments = (jcall!(TransformInfoObjectConfigAndManipulatorFactory::getInstance()).getInstance3(self.abeClientInformation.clone(), None /* Java this: a &mut self method has no Rc of itself */, document.clone()),); self.setObjectConfigInterface(java_arguments.0) };
    }
    

    pub fn setDataFile(&mut self, mut value: String) {
        self.dataFileName = value.clone();
    }
    

    pub fn setData(&mut self, mut value: String) {
        self.data = value.clone();
    }
    

    pub fn getName(&mut self) -> String {
        return self.name.clone();
    }
    

    pub fn getObject(&mut self) -> Option<Rc<RefCell<Box<dyn std::any::Any>>>> {
        return self.anyType.clone();
    }
    

    pub fn getObjectFile(&mut self) -> String {
        return self.objectFileName.clone();
    }
    

    pub fn getTemplate(&mut self) -> String {
        return self.template.clone();
    }
    

    pub fn getTemplateFile(&mut self) -> String {
        return self.templateFileName.clone();
    }
    

    // throws Exception
    pub fn getTemplateFilePath(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>> {
        return AbFilePath::new(self.getTemplateFile()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>);
    }
    

    // throws Exception
    pub fn getObjectConfigFilePath(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>> {
        return AbFilePath::new(self.getObjectConfigFile()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>);
    }
    

    // throws Exception
    pub fn getDataFilePath(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>> {
        return AbFilePath::new(self.getDataFile()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>);
    }
    

    pub fn getObjectConfigFile(&mut self) -> String {
        return self.objectConfigFileName.clone();
    }
    

    pub fn getData(&mut self) -> String {
        let mut stringValidationUtil: Option<Rc<RefCell<StringValidationUtil>>> = StringValidationUtil::getInstance();
        let mut dataFileString: String = self.getDataFile();
        if !jcall!(stringValidationUtil).isEmpty(dataFileString.clone()) {
            match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                let mut fileData: String = jcall!(CryptFileReader::new(jcall!(TransformInfoDataData::getInstance()).UNCRYPTED_EXTENSION.clone(), jcall!(TransformInfoDataData::getInstance()).ENCRYPTED_EXTENSION.clone())).get(self.getDataFilePath());
                if !jcall!(stringValidationUtil).isEmpty(self.data.clone()) {
                    if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).VIEW.clone()) {
                        { let java_receiver = self.logUtil.clone(); let java_arguments = (format!("{}{}", String::from("Data File overriding existing data: "), self.getDataFilePath()), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("getData()"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
                    }
                }
                self.data = fileData.clone();
            })) {
                Ok(_) => {}
                Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

                if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).VIEWERROR.clone()) {
                    { let java_receiver = self.logUtil.clone(); let java_arguments = (format!("{}{}", String::from("Could Not Load Data from: "), self.getDataFile()), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("getData()"), crate::java_into_object(e.clone()),); jcall!(java_receiver).put(java_arguments.0, java_arguments.1, java_arguments.2, java_arguments.3); };
                }
                }
            }
        }
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            if !jcall!(stringValidationUtil).isEmpty(self.data.clone()) {
                if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).VIEWERROR.clone()) {
                    { let java_receiver = self.logUtil.clone(); let java_arguments = (format!("{}{}", String::from("Data: "), XmlDocumentHelper::toString(DomDocumentHelper::create1(self.data.clone()))), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("getData()"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
                }
            } else {
                if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).VIEWERROR.clone()) {
                    let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
                    jcall!(stringBuffer).append(String::from("Name: "));
                    jcall!(stringBuffer).append(self.name.clone());
                    jcall!(stringBuffer).append(String::from("\nEmpty Data For: "));
                    jcall!(stringBuffer).append(dataFileString.clone());
                    jcall!(stringBuffer).append(String::from("\nPath: "));
                    jcall!(stringBuffer).append(jcall!(self.getDataFilePath()).toFileSystemString());
                    if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).VIEWERROR.clone()) {
                        { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(stringBuffer).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("getData()"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
                    }
                }
            }
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).VIEWERROR.clone()) {
                jcall!(self.logUtil).put(format!("{}{}", String::from("Could Not Preview Data: "), self.data), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("getData()"), crate::java_into_object(e.clone()));
            }
            }
        }
        return self.data.clone();
    }
    

    // throws Exception
    pub fn getDataDocument(&mut self) -> Option<Rc<RefCell<Document>>> {
        let mut localData: String = self.getData();
        let mut stringValidationUtil: Option<Rc<RefCell<StringValidationUtil>>> = StringValidationUtil::getInstance();
        if jcall!(stringValidationUtil).isEmpty(localData.clone()) {
            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).VIEWERROR.clone()) {
                jcall!(self.logUtil).putF(String::from("No Data So Creating New Document"), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("getDataDocument()"));
            }
            return DomDocumentHelper::create();
        } else {
            let mut document: Option<Rc<RefCell<Document>>> = DomDocumentHelper::create1(localData.clone());
            return document.clone();
        }
    }
    

    pub fn getDataFile(&mut self) -> String {
        return self.dataFileName.clone();
    }
    

    pub fn toHashMap(&mut self) -> Option<Rc<RefCell<HashMap>>> {
        let mut transformInfoData: Option<Rc<RefCell<TransformInfoData>>> = TransformInfoData::getInstance();
        let mut hashMap: Option<Rc<RefCell<HashMap>>> = jcall!(StdUtil::getInstance()).createHashMap();
        jcall!(hashMap).put(jcall!(transformInfoData).NAME.clone(), self.name.clone());
        jcall!(hashMap).put(jcall!(StoreFrontData::getInstance()).NAME.clone(), self.storeName.clone());
        jcall!(hashMap).put(jcall!(transformInfoData).OBJECTFILENAME.clone(), self.objectFileName.clone());
        jcall!(hashMap).put(jcall!(transformInfoData).OBJECT.clone(), self.anyType.clone());
        jcall!(hashMap).put(jcall!(transformInfoData).OBJECTCONFIGFILENAME.clone(), self.getObjectConfigFile());
        jcall!(hashMap).put(jcall!(transformInfoData).OBJECTCONFIG.clone(), Encoder::encode(jcall!(self.getObjectConfigInterface()).toString().bytes().map(|value| value as i8).collect::<Vec<i8>>()));
        jcall!(hashMap).put(jcall!(transformInfoData).TEMPLATEFILENAME.clone(), self.templateFileName.clone());
        jcall!(hashMap).put(jcall!(transformInfoData).TEMPLATE.clone(), self.template.clone());
        jcall!(hashMap).put(jcall!(transformInfoData).DATAFILENAME.clone(), self.dataFileName.clone());
        jcall!(hashMap).put(jcall!(transformInfoData).DATA.clone(), Encoder::encode(self.data.bytes().map(|value| value as i8).collect::<Vec<i8>>()));
        let mut calendar: Option<Rc<RefCell<Calendar>>> = Calendar::getInstance();
        let mut time: String = jcall!(Some((jcall!(calendar).getTimeInMillis()) as i64)).toString().clone();
        jcall!(hashMap).put(jcall!(EntryData::getInstance()).LASTMODIFIED.clone(), time.clone());
        return hashMap.clone();
    }
    

    pub fn toVector(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        let mut vector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        jcall!(vector).add(crate::java_into_object(self.name.clone()));
        jcall!(vector).add(crate::java_into_object(self.storeName.clone()));
        jcall!(vector).add(crate::java_into_object(self.objectFileName.clone()));
        jcall!(vector).add(crate::java_into_object(self.anyType.clone()));
        jcall!(vector).add(crate::java_into_object(self.getObjectConfigFile()));
        jcall!(vector).add(crate::java_into_object(Encoder::encode(jcall!(self.getObjectConfigInterface()).toString().bytes().map(|value| value as i8).collect::<Vec<i8>>())));
        jcall!(vector).add(crate::java_into_object(self.templateFileName.clone()));
        jcall!(vector).add(crate::java_into_object(self.template.clone()));
        jcall!(vector).add(crate::java_into_object(self.dataFileName.clone()));
        jcall!(vector).add(crate::java_into_object(Encoder::encode(self.data.bytes().map(|value| value as i8).collect::<Vec<i8>>())));
        let mut calendar: Option<Rc<RefCell<Calendar>>> = Calendar::getInstance();
        let mut time: String = jcall!(Some((jcall!(calendar).getTimeInMillis()) as i64)).toString().clone();
        jcall!(vector).add(crate::java_into_object(time.clone()));
        jcall!(vector).add(crate::java_into_object(time.clone()));
        return vector.clone();
    }
    

    pub fn getKey(&mut self) -> Option<Rc<RefCell<Box<dyn std::any::Any>>>> {
        return (self.getName()).clone() /* as Any */;
    }
    

    pub fn setChild(&mut self) {
        self.isChild = true;
    }
    

    pub fn isChild(&mut self) -> bool {
        return self.isChild.clone();
    }
    

    pub fn getObjectConfigInterface(&mut self) -> Option<Rc<RefCell<dyn TransformInfoObjectConfigInterface>>> {
        return self.objectConfigInterface.clone();
    }
    

    pub fn setObjectConfigInterface(&mut self, mut objectConfigInterface: Option<Rc<RefCell<dyn TransformInfoObjectConfigInterface>>>) {
        self.objectConfigInterface = objectConfigInterface.clone();
    }
}
pub trait TransformInfoDyn {
    fn as_TransformInfo(&self) -> &TransformInfo;
    fn as_TransformInfo_mut(&mut self) -> &mut TransformInfo;
    fn r#override(&mut self, mut hashMap: Option<Rc<RefCell<HashMap>>>) { TransformInfo::r#override(self.as_TransformInfo_mut(), hashMap) }
    fn log(&mut self) -> String { TransformInfo::log(self.as_TransformInfo_mut()) }
    fn setName(&mut self, mut value: String) { TransformInfo::setName(self.as_TransformInfo_mut(), value) }
    fn setStoreName(&mut self, mut value: String) { TransformInfo::setStoreName(self.as_TransformInfo_mut(), value) }
    fn getStoreName(&mut self) -> String { TransformInfo::getStoreName(self.as_TransformInfo_mut()) }
    fn setObject(&mut self, mut anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) { TransformInfo::setObject(self.as_TransformInfo_mut(), anyType) }
    fn setObjectFile(&mut self, mut value: String) { TransformInfo::setObjectFile(self.as_TransformInfo_mut(), value) }
    fn setTemplate(&mut self, mut value: String) { TransformInfo::setTemplate(self.as_TransformInfo_mut(), value) }
    fn setTemplateFile(&mut self, mut value: String) { TransformInfo::setTemplateFile(self.as_TransformInfo_mut(), value) }
    fn setObjectConfigFileName(&mut self, mut value: String) { TransformInfo::setObjectConfigFileName(self.as_TransformInfo_mut(), value) }
    fn setObjectConfigFile(&mut self, mut value: String) { TransformInfo::setObjectConfigFile(self.as_TransformInfo_mut(), value) }
    fn setObjectConfig(&mut self, mut value: String) { TransformInfo::setObjectConfig(self.as_TransformInfo_mut(), value) }
    fn setDataFile(&mut self, mut value: String) { TransformInfo::setDataFile(self.as_TransformInfo_mut(), value) }
    fn setData(&mut self, mut value: String) { TransformInfo::setData(self.as_TransformInfo_mut(), value) }
    fn getName(&mut self) -> String { TransformInfo::getName(self.as_TransformInfo_mut()) }
    fn getObject(&mut self) -> Option<Rc<RefCell<Box<dyn std::any::Any>>>> { TransformInfo::getObject(self.as_TransformInfo_mut()) }
    fn getObjectFile(&mut self) -> String { TransformInfo::getObjectFile(self.as_TransformInfo_mut()) }
    fn getTemplate(&mut self) -> String { TransformInfo::getTemplate(self.as_TransformInfo_mut()) }
    fn getTemplateFile(&mut self) -> String { TransformInfo::getTemplateFile(self.as_TransformInfo_mut()) }
    fn getTemplateFilePath(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>> { TransformInfo::getTemplateFilePath(self.as_TransformInfo_mut()) }
    fn getObjectConfigFilePath(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>> { TransformInfo::getObjectConfigFilePath(self.as_TransformInfo_mut()) }
    fn getDataFilePath(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>> { TransformInfo::getDataFilePath(self.as_TransformInfo_mut()) }
    fn getObjectConfigFile(&mut self) -> String { TransformInfo::getObjectConfigFile(self.as_TransformInfo_mut()) }
    fn getData(&mut self) -> String { TransformInfo::getData(self.as_TransformInfo_mut()) }
    fn getDataDocument(&mut self) -> Option<Rc<RefCell<Document>>> { TransformInfo::getDataDocument(self.as_TransformInfo_mut()) }
    fn getDataFile(&mut self) -> String { TransformInfo::getDataFile(self.as_TransformInfo_mut()) }
    fn toHashMap(&mut self) -> Option<Rc<RefCell<HashMap>>> { TransformInfo::toHashMap(self.as_TransformInfo_mut()) }
    fn toVector(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> { TransformInfo::toVector(self.as_TransformInfo_mut()) }
    fn getKey(&mut self) -> Option<Rc<RefCell<Box<dyn std::any::Any>>>> { TransformInfo::getKey(self.as_TransformInfo_mut()) }
    fn setChild(&mut self) { TransformInfo::setChild(self.as_TransformInfo_mut()) }
    fn isChild(&mut self) -> bool { TransformInfo::isChild(self.as_TransformInfo_mut()) }
    fn getObjectConfigInterface(&mut self) -> Option<Rc<RefCell<dyn TransformInfoObjectConfigInterface>>> { TransformInfo::getObjectConfigInterface(self.as_TransformInfo_mut()) }
    fn setObjectConfigInterface(&mut self, mut objectConfigInterface: Option<Rc<RefCell<dyn TransformInfoObjectConfigInterface>>>) { TransformInfo::setObjectConfigInterface(self.as_TransformInfo_mut(), objectConfigInterface) }
}
impl TransformInfoDyn for TransformInfo {
    fn as_TransformInfo(&self) -> &TransformInfo { self }
    fn as_TransformInfo_mut(&mut self) -> &mut TransformInfo { self }
}


