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
use super::*;
use crate::java::util::HashMap::HashMap;
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
use crate::java::lang::Exception::*;
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
        this.logoAbPath = { let java_value = (AbPath::new(jcall!(DomSearchHelper::getNode({ let java_field = jcall!(LogoData::LogoData::getInstance()).IMAGEPATH.clone(); java_field }.clone(), jcall!(node).getChildNodes())).getNodeValue(), { let java_field = jcall!(StringUtil::getInstance()).EMPTY_STRING.clone(); java_field }.clone()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>)).clone(); java_value };
        this.logoFile = { let java_value = (jcall!(DomSearchHelper::getNode({ let java_field = jcall!(LogoData::LogoData::getInstance()).IMAGEFILENAME.clone(); java_field }.clone(), jcall!(node).getChildNodes())).getNodeValue()).clone(); java_value };
        this
    }

    pub fn new1(mut node: Option<Rc<RefCell<Node>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct1(node))))
    }
    

    pub fn new_struct1_HashMap(mut hashMap: Option<Rc<RefCell<HashMap<crate::JavaObject, crate::JavaObject>>>>) -> Self {
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

    pub fn new1_HashMap(mut hashMap: Option<Rc<RefCell<HashMap<crate::JavaObject, crate::JavaObject>>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct1_HashMap(hashMap))))
    }
    

    // throws Exception
    pub fn getFormData(&mut self, mut hashMap: Option<Rc<RefCell<HashMap<crate::JavaObject, crate::JavaObject>>>>) {
        self.logoFileItem = { let java_value = ((jcall!(hashMap).get(crate::java_into_object({ let java_field = jcall!(LogoData::LogoData::getInstance()).IMAGE.clone(); java_field }.clone()))).clone() /* as FileItem */).clone(); java_value };
        self.logoAbPath = { let java_value = (AbPath::new({ let java_object = (jcall!(hashMap).get(crate::java_into_object({ let java_field = jcall!(LogoData::LogoData::getInstance()).IMAGEPATH.clone(); java_field }.clone()))).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() }, { let java_field = jcall!(StringUtil::getInstance()).EMPTY_STRING.clone(); java_field }.clone()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>)).clone(); java_value };
        if self.logoFileItem.is_some() {
            self.logoFile = { let java_value = (jcall!(HttpRequestUtil::getInstance()).generateFileName(jcall!(self.logoFileItem).getName())).clone(); java_value };
        }
        if jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypes.clone(); java_field }).LOGGING.clone(); java_field }).contains({ let java_field = jcall!(jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypeFactory.clone(); java_field }).getInstance()).VIEW.clone(); java_field }.clone()) {
            let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
            jcall!(stringBuffer).append(String::from("HashMap: "));
            jcall!(stringBuffer).append(jcall!(hashMap).toString());
            jcall!(stringBuffer).append(String::from("\nLogoFileName: "));
            jcall!(stringBuffer).append(self.logoFile.clone());
            { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(stringBuffer).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("getFormData()"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        }
    }
    

    pub fn isValid(&mut self) -> Option<bool> {
        let java_try_result = match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut valid: Option<bool> = true;
            let mut fileData: Option<Rc<RefCell<FileData>>> = FileData::getInstance();
            if jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypes.clone(); java_field }).LOGGING.clone(); java_field }).contains({ let java_field = jcall!(jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypeFactory.clone(); java_field }).getInstance()).VIEW.clone(); java_field }.clone()) {
                jcall!(self.logUtil).putF(String::from("LogoValidation"), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(self.commonStrings).IS_VALID.clone(); java_field }.clone());
            }
            if !jcall!(StringValidationUtil::getInstance()).isValidRequired(jcall!(self.logoAbPath).toString(), 0, 512) {
                if jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypes.clone(); java_field }).LOGGING.clone(); java_field }).contains({ let java_field = jcall!(jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypeFactory.clone(); java_field }).getInstance()).VIEW.clone(); java_field }.clone()) {
                    { let java_receiver = self.logUtil.clone(); let java_arguments = (format!("{}{}", String::from("logoPath is invalid: "), jcall!(self.logoAbPath).toString()), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(self.commonStrings).IS_VALID.clone(); java_field }.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
                }
                return Some(false);
            }
            if !jcall!(StringValidationUtil::getInstance()).isValidRequired(self.logoFile.clone(), { let java_field = jcall!(fileData).MINLEN.clone(); java_field }.clone(), { let java_field = jcall!(fileData).MAXLEN.clone(); java_field }.clone()) {
                if jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypes.clone(); java_field }).LOGGING.clone(); java_field }).contains({ let java_field = jcall!(jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypeFactory.clone(); java_field }).getInstance()).VIEW.clone(); java_field }.clone()) {
                    jcall!(self.logUtil).putF(format!("{}{}", String::from("logoFile is invalid: "), self.logoFile), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(self.commonStrings).IS_VALID.clone(); java_field }.clone());
                }
                return Some(false);
            }
            if self.isFile {
                if self.logoFileItem.is_none() {
                    return Some(false);
                }
                if jcall!(self.logoFileItem).getSize() > { let java_field = jcall!(fileData).MAXIMAGEFILESIZE.clone(); java_field } || jcall!(self.logoFileItem).getSize() < { let java_field = jcall!(fileData).MINIMAGEFILESIZE.clone(); java_field } {
                    if jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypes.clone(); java_field }).LOGGING.clone(); java_field }).contains({ let java_field = jcall!(jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypeFactory.clone(); java_field }).getInstance()).VIEW.clone(); java_field }.clone()) {
                        jcall!(self.logUtil).putF(format!("{}{}", format!("{}{}", format!("{}{}", String::from("Image File Is Not The Right Size. "), { let java_field = jcall!(fileData).MINIMAGEFILESIZE.clone(); java_field }), String::from("< > ")), { let java_field = jcall!(fileData).MAXIMAGEFILESIZE.clone(); java_field }), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(self.commonStrings).IS_VALID.clone(); java_field }.clone());
                    }
                    return Some(false);
                }
            }
            if jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypes.clone(); java_field }).LOGGING.clone(); java_field }).contains({ let java_field = jcall!(jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypeFactory.clone(); java_field }).getInstance()).VIEW.clone(); java_field }.clone()) {
                jcall!(self.logUtil).putF(format!("{}{}", String::from("Logo Validation: "), valid), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(self.commonStrings).IS_VALID.clone(); java_field }.clone());
            }
            return Some(valid.clone());
        })) {
            Ok(java_return) => java_return,
            Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            if jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypes.clone(); java_field }).LOGGING.clone(); java_field }).contains({ let java_field = jcall!(jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypeFactory.clone(); java_field }).getInstance()).SQLTAGSERROR.clone(); java_field }.clone()) {
                jcall!(self.logUtil).put(String::from("Failed to validate form"), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(self.commonStrings).IS_VALID.clone(); java_field }.clone(), crate::java_into_object(e.clone()));
            }
            return false;
            None
            }
        };if let Some(java_return) = java_try_result { return java_return; }unreachable!("try statement")
    }
    

    pub fn validationInfo(&mut self) -> String {
        let java_try_result = match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
            jcall!(stringBuffer).append(String::from("Logo is not valid."));
            return Some(jcall!(stringBuffer).toString());
        })) {
            Ok(java_return) => java_return,
            Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            if jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypes.clone(); java_field }).LOGGING.clone(); java_field }).contains({ let java_field = jcall!(jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypeFactory.clone(); java_field }).getInstance()).SQLTAGSERROR.clone(); java_field }.clone()) {
                jcall!(self.logUtil).put(String::from("Failed to generate validation error info"), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("validationInfo()"), crate::java_into_object(e.clone()));
            }
            return String::from("Error Validating Form");
            None
            }
        };if let Some(java_return) = java_try_result { return java_return; }unreachable!("try statement")
    }
    

    pub fn toValidationInfoDoc(&mut self) -> Option<Rc<RefCell<Document>>> {
        return None;
    }
    

    pub fn toValidationInfoNode(&mut self, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Node>>> {
        return None;
    }
    

    pub fn toHashMap(&mut self) -> Option<Rc<RefCell<HashMap<crate::JavaObject, crate::JavaObject>>>> {
        let mut stringUtil: Option<Rc<RefCell<StringUtil>>> = StringUtil::getInstance();
        let mut hashMap: Option<Rc<RefCell<HashMap<crate::JavaObject, crate::JavaObject>>>> = jcall!(StdUtil::getInstance()).createHashMap();
        jcall!(hashMap).put(crate::java_into_object({ let java_field = jcall!(LogoData::LogoData::getInstance()).IMAGE.clone(); java_field }.clone()), crate::java_into_object({ let java_field = jcall!(stringUtil).EMPTY_STRING.clone(); java_field }.clone()));
        if true {
            jcall!(hashMap).put(crate::java_into_object({ let java_field = jcall!(LogoData::LogoData::getInstance()).IMAGEPATH.clone(); java_field }.clone()), crate::java_into_object(jcall!(self.logoAbPath).toString()));
            jcall!(hashMap).put(crate::java_into_object({ let java_field = jcall!(LogoData::LogoData::getInstance()).IMAGEFILENAME.clone(); java_field }.clone()), crate::java_into_object(self.logoFile.clone()));
        } else {
            jcall!(hashMap).put(crate::java_into_object({ let java_field = jcall!(LogoData::LogoData::getInstance()).IMAGEPATH.clone(); java_field }.clone()), None);
            jcall!(hashMap).put(crate::java_into_object({ let java_field = jcall!(LogoData::LogoData::getInstance()).IMAGEFILENAME.clone(); java_field }.clone()), crate::java_into_object(self.logoFile.clone()));
        }
        return hashMap.clone();
    }
    

    // throws Exception
    pub fn toXmlNode(&mut self, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Node>>> {
        let mut hashMap: Option<Rc<RefCell<HashMap<crate::JavaObject, crate::JavaObject>>>> = self.toHashMap();
        return ModDomHelper::createNameValueNodes_Document_String_HashMap(document.clone(), { let java_field = jcall!(LogoData::LogoData::getInstance()).NAME.clone(); java_field }.clone(), hashMap.clone());
    }
    

    // throws Exception
    pub fn processLogoFile(&mut self) {
        if jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypes.clone(); java_field }).LOGGING.clone(); java_field }).contains({ let java_field = jcall!(jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypeFactory.clone(); java_field }).getInstance()).VIEW.clone(); java_field }.clone()) {
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


