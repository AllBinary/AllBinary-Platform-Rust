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
// package org.allbinary.logic.visual.transform.template.customizer.hedges.heading
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;
use super::*;
// use crate::java::util::HashMap::HashMap;
use crate::org::allbinary::data::tree::dom::DomNodeInterface::DomNodeInterface;
use crate::org::allbinary::data::tree::dom::DomSearchHelper::DomSearchHelper;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::control::validate::Validation::Validation;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::visual::transform::template::customizer::widgets::logo::LogoData::LogoData;
use crate::org::allbinary::logic::visual::transform::template::customizer::widgets::logo::LogoValidation::LogoValidation;
use crate::org::allbinary::logic::visual::transform::template::customizer::widgets::title::TitleData::TitleData;
use crate::org::allbinary::logic::visual::transform::template::customizer::widgets::title::TitleValidation::TitleValidation;
use crate::org::w3c::dom::Document::Document;
use crate::org::w3c::dom::Node::Node;
use crate::org::w3c::dom::NodeList::NodeList;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct HeadingValidation {
    pub base: Validation,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub title: Option<Rc<RefCell<TitleValidation>>>,
    pub logo: Option<Rc<RefCell<LogoValidation>>>,
}

impl DomNodeInterface for HeadingValidation {
    fn toXmlNode(&mut self, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Node>>> { Self::toXmlNode(self, document) }
}
impl crate::org::allbinary::logic::control::validate::ValidationInterface::ValidationInterface for HeadingValidation {
    fn isValid(&mut self) -> Option<bool> { Self::isValid(self) }
    fn toValidationInfoNode(&mut self, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Node>>> { Self::toValidationInfoNode(self, document) }
    fn toValidationInfoDoc(&mut self) -> Option<Rc<RefCell<Document>>> { Self::toValidationInfoDoc(self) }
    fn validationInfo(&mut self) -> String { Self::validationInfo(self) }
}

impl HeadingValidation {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: Validation::new_struct(),
            logUtil: None,
            title: None,
            logo: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.title = TitleValidation::new();
        this.logo = LogoValidation::new();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn new_struct1(mut document: Option<Rc<RefCell<Document>>>) -> Self {
        let mut this = Self {
            base: Validation::new_struct(),
            logUtil: None,
            title: None,
            logo: None,
        };
        this.logUtil = LogUtil::getInstance();
        let mut headingNodeList: Option<Rc<RefCell<NodeList>>> = jcall!(document).getElementsByTagName(HeadingData_NAME.with(Clone::clone).clone());
        let mut titleNode: Option<Rc<RefCell<Node>>> = DomSearchHelper::getNode(jcall!(TitleData::getInstance()).NAME.clone(), jcall!(jcall!(headingNodeList).item(0)).getChildNodes());
        let mut logoNode: Option<Rc<RefCell<Node>>> = DomSearchHelper::getNode(jcall!(LogoData::getInstance()).NAME.clone(), jcall!(jcall!(headingNodeList).item(0)).getChildNodes());
        this.title = TitleValidation::new1(titleNode.clone());
        this.logo = LogoValidation::new1(logoNode.clone());
        this
    }

    pub fn new1(mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct1(document))))
    }
    

    pub fn new_struct1(mut hashMap: Option<Rc<RefCell<HashMap>>>) -> Self {
        let mut this = Self {
            base: Validation::new_struct(),
            logUtil: None,
            title: None,
            logo: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.getFormData(hashMap.clone());
        this
    }

    pub fn new1(mut hashMap: Option<Rc<RefCell<HashMap>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct1(hashMap))))
    }
    

    // throws Exception
    pub fn getFormData(&mut self, mut hashMap: Option<Rc<RefCell<HashMap>>>) {
        self.title = TitleValidation::new1(hashMap.clone());
        self.logo = LogoValidation::new1(hashMap.clone());
    }
    

    pub fn isValid(&mut self) -> Option<bool> {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut valid: Option<bool> = true;
            let mut titleValid: Option<bool> = true;
            let mut logoValid: Option<bool> = true;
            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).VIEW.clone()) {
                jcall!(self.logUtil).putF(jcall!(self.commonStrings).START.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).IS_VALID.clone());
            }
            if !jcall!(self.title).isValid().unwrap() {
                titleValid = false;
            }
            if !jcall!(self.logo).isValid().unwrap() {
                logoValid = false;
            } else {
                jcall!(self.logo).processLogoFile();
            }
            if !titleValid.unwrap() && !logoValid.unwrap() {
                valid = false;
            }
            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).VIEW.clone()) {
                jcall!(self.logUtil).putF(format!("{}{}", String::from("HeadingValidation: "), valid), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).IS_VALID.clone());
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
            let mut valid: Option<bool> = true;
            let mut titleValid: Option<bool> = true;
            let mut logoValid: Option<bool> = true;
            let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
            if !jcall!(self.title).isValid().unwrap() {
                titleValid = false;
            }
            if !jcall!(self.logo).isValid().unwrap() {
                logoValid = false;
            }
            if !titleValid.unwrap() && !logoValid.unwrap() {
                valid = false;
            }
            if !valid.unwrap() {
                jcall!(stringBuffer).append(String::from("Include a title and/or logo for the header."));
            }
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
    

    // throws Exception
    pub fn toXmlNode(&mut self, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Node>>> {
        let mut node: Option<Rc<RefCell<Node>>> = jcall!(document).createElement(HeadingData_NAME.with(Clone::clone).clone());
        jcall!(node).appendChild(jcall!(self.logo).toXmlNode(document.clone()));
        jcall!(node).appendChild(jcall!(self.title).toXmlNode(document.clone()));
        return node.clone();
    }
}
impl std::ops::Deref for HeadingValidation {
    type Target = Validation;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for HeadingValidation {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::logic::control::validate::Validation::ValidationDyn for HeadingValidation {
    fn as_Validation(&self) -> &crate::org::allbinary::logic::control::validate::Validation::Validation { &self.base }
    fn as_Validation_mut(&mut self) -> &mut crate::org::allbinary::logic::control::validate::Validation::Validation { &mut self.base }
    fn isValid(&mut self) -> Option<bool> { Self::isValid(self) }
    fn toValidationInfoNode(&mut self, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Node>>> { Self::toValidationInfoNode(self, document) }
    fn toValidationInfoDoc(&mut self) -> Option<Rc<RefCell<Document>>> { Self::toValidationInfoDoc(self) }
    fn validationInfo(&mut self) -> String { Self::validationInfo(self) }
}


