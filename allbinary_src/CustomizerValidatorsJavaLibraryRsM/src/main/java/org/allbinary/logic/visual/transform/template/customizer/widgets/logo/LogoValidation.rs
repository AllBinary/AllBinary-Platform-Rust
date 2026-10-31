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
// package org.allbinary.logic.visual.transform.template.customizer.widgets.logo
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;
use super::*;
// use crate::java::util::HashMap::HashMap;
use crate::org::allbinary::data::tree::dom::DomNodeInterface::DomNodeInterface;
use crate::org::allbinary::data::tree::dom::DomSearchHelper::DomSearchHelper;
use crate::org::allbinary::data::tree::dom::ModDomHelper::ModDomHelper;
use crate::org::allbinary::logic::StdUtil::StdUtil;
use crate::org::allbinary::logic::communication::http::request::HttpRequestUtil::HttpRequestUtil;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::control::validate::Validation::Validation;
use crate::org::allbinary::logic::io::file::FileData::FileData;
use crate::org::allbinary::logic::io::path::AbPath::AbPath;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use crate::org::allbinary::logic::string::StringValidationUtil::StringValidationUtil;
use crate::org::apache::commons::fileupload::FileItem::FileItem;
use crate::org::w3c::dom::Document::Document;
use crate::org::w3c::dom::Node::Node;
use super::LogoImageFileUtil::LogoImageFileUtil;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct LogoValidation {
    pub base: Validation,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub logoFileItem: Option<Rc<RefCell<FileItem>>>,
    pub logoAbPath: Option<Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>>,
    pub logoFile: String,
    pub isFile: bool,
}

impl DomNodeInterface for LogoValidation {
    fn toXmlNode(&mut self, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Node>>> { Self::toXmlNode(self, document) }
}
impl crate::org::allbinary::logic::control::validate::ValidationInterface::ValidationInterface for LogoValidation {
    fn isValid(&mut self) -> Option<bool> { Self::isValid(self) }
    fn toValidationInfoNode(&mut self, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Node>>> { Self::toValidationInfoNode(self, document) }
    fn toValidationInfoDoc(&mut self) -> Option<Rc<RefCell<Document>>> { Self::toValidationInfoDoc(self) }
    fn validationInfo(&mut self) -> String { Self::validationInfo(self) }
}

impl LogoValidation {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: Validation::new_struct(),
            logUtil: None,
            logoFileItem: None,
            logoAbPath: None,
            logoFile: String::new(),
            isFile: false,
        };
        this.logUtil = LogUtil::getInstance();
        this.isFile = false;
        this.logoAbPath = None;
        this.logoFile = None;
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn new_struct1(mut node: Option<Rc<RefCell<Node>>>) -> Self {
        let mut this = Self {
            base: Validation::new_struct(),
            logUtil: None,
            logoFileItem: None,
            logoAbPath: None,
            logoFile: String::new(),
            isFile: false,
        };
        this.logUtil = LogUtil::getInstance();
        this.isFile = false;
        this.logoAbPath = AbPath::new(jcall!(DomSearchHelper::getNode(jcall!(LogoData::LogoData::getInstance()).IMAGEPATH.clone(), jcall!(node).getChildNodes())).getNodeValue(), jcall!(StringUtil::getInstance()).EMPTY_STRING.clone()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>);
        this.logoFile = jcall!(DomSearchHelper::getNode(jcall!(LogoData::LogoData::getInstance()).IMAGEFILENAME.clone(), jcall!(node).getChildNodes())).getNodeValue();
        this
    }

    pub fn new1(mut node: Option<Rc<RefCell<Node>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct1(node))))
    }
    

    pub fn new_struct1(mut hashMap: Option<Rc<RefCell<HashMap>>>) -> Self {
        let mut this = Self {
            base: Validation::new_struct(),
            logUtil: None,
            logoFileItem: None,
            logoAbPath: None,
            logoFile: String::new(),
            isFile: false,
        };
        this.logUtil = LogUtil::getInstance();
        this.isFile = true;
        this.getFormData(hashMap.clone());
        this
    }

    pub fn new1(mut hashMap: Option<Rc<RefCell<HashMap>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct1(hashMap))))
    }
    

    // throws Exception
    pub fn getFormData(&mut self, mut hashMap: Option<Rc<RefCell<HashMap>>>) {
        self.logoFileItem = (jcall!(hashMap).get(jcall!(LogoData::LogoData::getInstance()).IMAGE.clone())).clone() /* as FileItem */;
        self.logoAbPath = AbPath::new((jcall!(hashMap).get(jcall!(LogoData::LogoData::getInstance()).IMAGEPATH.clone())).clone() /* as String */, jcall!(StringUtil::getInstance()).EMPTY_STRING.clone()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>);
        if self.logoFileItem.is_some() {
            self.logoFile = jcall!(HttpRequestUtil::getInstance()).generateFileName(jcall!(self.logoFileItem).getName());
        }
        if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).VIEW.clone()) {
            let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
            jcall!(stringBuffer).append(String::from("HashMap: "));
            jcall!(stringBuffer).append(jcall!(hashMap).toString());
            jcall!(stringBuffer).append(String::from("\nLogoFileName: "));
            jcall!(stringBuffer).append(self.logoFile.clone());
            { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(stringBuffer).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("getFormData()"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        }
    }
    

    pub fn isValid(&mut self) -> Option<bool> {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut valid: Option<bool> = true;
            let mut fileData: Option<Rc<RefCell<FileData>>> = FileData::getInstance();
            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).VIEW.clone()) {
                jcall!(self.logUtil).putF(String::from("LogoValidation"), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).IS_VALID.clone());
            }
            if !jcall!(StringValidationUtil::getInstance()).isValidRequired(jcall!(self.logoAbPath).toString(), 0, 512) {
                if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).VIEW.clone()) {
                    { let java_receiver = self.logUtil.clone(); let java_arguments = (format!("{}{}", String::from("logoPath is invalid: "), jcall!(self.logoAbPath).toString()), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).IS_VALID.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
                }
                return false;
            }
            if !jcall!(StringValidationUtil::getInstance()).isValidRequired(self.logoFile.clone(), jcall!(fileData).MINLEN.clone(), jcall!(fileData).MAXLEN.clone()) {
                if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).VIEW.clone()) {
                    jcall!(self.logUtil).putF(format!("{}{}", String::from("logoFile is invalid: "), self.logoFile), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).IS_VALID.clone());
                }
                return false;
            }
            if self.isFile {
                if self.logoFileItem.is_none() {
                    return false;
                }
                if jcall!(self.logoFileItem).getSize() > jcall!(fileData).MAXIMAGEFILESIZE || jcall!(self.logoFileItem).getSize() < jcall!(fileData).MINIMAGEFILESIZE {
                    if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).VIEW.clone()) {
                        jcall!(self.logUtil).putF(format!("{}{}", format!("{}{}", format!("{}{}", String::from("Image File Is Not The Right Size. "), jcall!(fileData).MINIMAGEFILESIZE), String::from("< > ")), jcall!(fileData).MAXIMAGEFILESIZE), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).IS_VALID.clone());
                    }
                    return false;
                }
            }
            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).VIEW.clone()) {
                jcall!(self.logUtil).putF(format!("{}{}", String::from("Logo Validation: "), valid), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).IS_VALID.clone());
            }
            return valid.clone();
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).SQLTAGSERROR.clone()) {
                jcall!(self.logUtil).put(String::from("Failed to validate form"), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).IS_VALID.clone(), crate::java_into_object(e.clone()));
            }
            return false;
            }
        }
    }
    

    pub fn validationInfo(&mut self) -> String {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
            jcall!(stringBuffer).append(String::from("Logo is not valid."));
            return jcall!(stringBuffer).toString();
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).SQLTAGSERROR.clone()) {
                jcall!(self.logUtil).put(String::from("Failed to generate validation error info"), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("validationInfo()"), crate::java_into_object(e.clone()));
            }
            return String::from("Error Validating Form");
            }
        }
    }
    

    pub fn toValidationInfoDoc(&mut self) -> Option<Rc<RefCell<Document>>> {
        return None;
    }
    

    pub fn toValidationInfoNode(&mut self, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Node>>> {
        return None;
    }
    

    pub fn toHashMap(&mut self) -> Option<Rc<RefCell<HashMap>>> {
        let mut stringUtil: Option<Rc<RefCell<StringUtil>>> = StringUtil::getInstance();
        let mut hashMap: Option<Rc<RefCell<HashMap>>> = jcall!(StdUtil::getInstance()).createHashMap();
        jcall!(hashMap).put(jcall!(LogoData::LogoData::getInstance()).IMAGE.clone(), jcall!(stringUtil).EMPTY_STRING.clone());
        if true {
            jcall!(hashMap).put(jcall!(LogoData::LogoData::getInstance()).IMAGEPATH.clone(), jcall!(self.logoAbPath).toString());
            jcall!(hashMap).put(jcall!(LogoData::LogoData::getInstance()).IMAGEFILENAME.clone(), self.logoFile.clone());
        } else {
            jcall!(hashMap).put(jcall!(LogoData::LogoData::getInstance()).IMAGEPATH.clone(), None);
            jcall!(hashMap).put(jcall!(LogoData::LogoData::getInstance()).IMAGEFILENAME.clone(), self.logoFile.clone());
        }
        return hashMap.clone();
    }
    

    // throws Exception
    pub fn toXmlNode(&mut self, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Node>>> {
        let mut hashMap: Option<Rc<RefCell<HashMap>>> = self.toHashMap();
        return ModDomHelper::createNameValueNodes(document.clone(), jcall!(LogoData::LogoData::getInstance()).NAME.clone(), hashMap.clone());
    }
    

    // throws Exception
    pub fn processLogoFile(&mut self) {
        if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).VIEW.clone()) {
            let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
            jcall!(stringBuffer).append(String::from("Processing logoFile: "));
            jcall!(stringBuffer).append(jcall!(self.logoAbPath).toString());
            jcall!(stringBuffer).append(self.logoFile.clone());
            { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(stringBuffer).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("processLogoFile()"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        }
        jcall!(LogoImageFileUtil::new(self.logoAbPath.clone(), self.logoFile.clone())).saveFiles(self.logoFileItem.clone());
    }
}
impl std::ops::Deref for LogoValidation {
    type Target = Validation;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for LogoValidation {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::logic::control::validate::Validation::ValidationDyn for LogoValidation {
    fn as_Validation(&self) -> &crate::org::allbinary::logic::control::validate::Validation::Validation { &self.base }
    fn as_Validation_mut(&mut self) -> &mut crate::org::allbinary::logic::control::validate::Validation::Validation { &mut self.base }
    fn isValid(&mut self) -> Option<bool> { Self::isValid(self) }
    fn toValidationInfoNode(&mut self, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Node>>> { Self::toValidationInfoNode(self, document) }
    fn toValidationInfoDoc(&mut self) -> Option<Rc<RefCell<Document>>> { Self::toValidationInfoDoc(self) }
    fn validationInfo(&mut self) -> String { Self::validationInfo(self) }
}


