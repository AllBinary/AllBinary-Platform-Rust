//
// AllBinary Open License Version 1
// Copyright (c) 2011 AllBinary
// 
// By agreeing to this license you and any business entity you represent are
// legally bound to the AllBinary Open License Version 1 legal agreement.
// 
// You may obtain the AllBinary Open License Version 1 legal agreement from
// AllBinary or the root directory of AllBinary's AllBinary Platform repository.
// II
// Created By: Travis Berthelot
//  
// Generated Code Do Not Modify
// package org.allbinary.logic.visual.transform.template.customizer.includes.meta
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
use crate::org::allbinary::business::context::modules::storefront::StoreFrontFactory::StoreFrontFactory;
use crate::org::allbinary::business::context::modules::storefront::StoreFrontInterface::StoreFrontInterface;
use crate::org::allbinary::data::tree::dom::DomNodeInterface::DomNodeInterface;
use crate::org::allbinary::logic::StdUtil::StdUtil;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::control::validate::Validation::Validation;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::visual::dhtml::html::head::meta::HtmlMetaAttributeDataFactory::HtmlMetaAttributeDataFactory;
use crate::org::allbinary::logic::visual::dhtml::html::head::meta::HtmlMetaAttributeValuesData::HtmlMetaAttributeValuesData;
use crate::org::allbinary::logic::visual::dhtml::html::head::meta::HtmlMetasData::HtmlMetasData;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::org::w3c::dom::Document::Document;
use crate::org::w3c::dom::Node::Node;
use super::MetaValidation::MetaValidation;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct MetasValidation {
    pub base: Validation,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub storeFrontInterface: Option<Rc<RefCell<dyn StoreFrontInterface>>>,
    pub metaValidationVector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>,
}

impl DomNodeInterface for MetasValidation {
    fn toXmlNode(&mut self, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Node>>> { Self::toXmlNode(self, document) }
}
impl crate::org::allbinary::logic::control::validate::ValidationInterface::ValidationInterface for MetasValidation {
    fn isValid(&mut self) -> Option<bool> { Self::isValid(self) }
    fn toValidationInfoNode(&mut self, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Node>>> { Self::toValidationInfoNode(self, document) }
    fn toValidationInfoDoc(&mut self) -> Option<Rc<RefCell<Document>>> { Self::toValidationInfoDoc(self) }
    fn validationInfo(&mut self) -> String { Self::validationInfo(self) }
}

impl MetasValidation {
    

    pub fn new_struct1(mut storeName: String) -> Self {
        let mut this = Self {
            base: Validation::new_struct(),
            logUtil: None,
            storeFrontInterface: None,
            metaValidationVector: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.storeFrontInterface = StoreFrontFactory::getInstance(storeName.clone());
        this.metaValidationVector = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        let mut contentValue: String = format!("{}{}", jcall!(this.storeFrontInterface).getName(), String::from(" E-Commerce Site"));
        let mut keywordsMetaValidation: Option<Rc<RefCell<MetaValidation>>> = MetaValidation::new4(jcall!(HtmlMetaAttributeDataFactory::getInstance()).NAME.clone(), String::from("Keywords"), jcall!(HtmlMetaAttributeValuesData::getInstance()).KEYWORDS.clone(), contentValue.clone());
        let mut abstractMetaValidation: Option<Rc<RefCell<MetaValidation>>> = MetaValidation::new4(jcall!(HtmlMetaAttributeDataFactory::getInstance()).NAME.clone(), String::from("Abstract"), jcall!(HtmlMetaAttributeValuesData::getInstance()).ABSTRACT.clone(), contentValue.clone());
        let mut descriptionMetaValidation: Option<Rc<RefCell<MetaValidation>>> = MetaValidation::new4(jcall!(HtmlMetaAttributeDataFactory::getInstance()).NAME.clone(), String::from("Description"), jcall!(HtmlMetaAttributeValuesData::getInstance()).DESCRIPTION.clone(), contentValue.clone());
        jcall!(this.metaValidationVector).add(crate::java_into_object(keywordsMetaValidation.clone()));
        jcall!(this.metaValidationVector).add(crate::java_into_object(abstractMetaValidation.clone()));
        jcall!(this.metaValidationVector).add(crate::java_into_object(descriptionMetaValidation.clone()));
        this
    }

    pub fn new1(mut storeName: String) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct1(storeName))))
    }
    

    pub fn new_struct1(mut document: Option<Rc<RefCell<Document>>>) -> Self {
        let mut this = Self {
            base: Validation::new_struct(),
            logUtil: None,
            storeFrontInterface: None,
            metaValidationVector: None,
        };
        this.logUtil = LogUtil::getInstance();
        panic!("{:?}", format!("{}: {:?}", "Exception", (jcall!(CommonStrings::getInstance()).NOT_IMPLEMENTED.clone())));
        this
    }

    pub fn new1(mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct1(document))))
    }
    

    pub fn isValid(&mut self) -> Option<bool> {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut valid: Option<bool> = true;
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
        let mut node: Option<Rc<RefCell<Node>>> = jcall!(document).createElement(jcall!(HtmlMetasData::getInstance()).NAME.clone());
        let mut metaArray: Vec<Option<Rc<RefCell<MetaValidation>>>> = ({ let java_receiver = self.metaValidationVector.clone(); let java_arguments = (vec![None; (jcall!(self.metaValidationVector).size()) as usize],); let java_result = jcall!(java_receiver).toArrayType(java_arguments.0); java_result }).clone() /* as  */;
        let mut size: i32 = (metaArray.len() as i32).clone();
        for i in (0..size) {
            let mut metaValidation: Option<Rc<RefCell<MetaValidation>>> = metaArray[(i) as usize].clone();
            jcall!(node).appendChild(jcall!(metaValidation).toXmlNode(document.clone()));
        }
        return node.clone();
    }
}
impl std::ops::Deref for MetasValidation {
    type Target = Validation;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for MetasValidation {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::logic::control::validate::Validation::ValidationDyn for MetasValidation {
    fn as_Validation(&self) -> &crate::org::allbinary::logic::control::validate::Validation::Validation { &self.base }
    fn as_Validation_mut(&mut self) -> &mut crate::org::allbinary::logic::control::validate::Validation::Validation { &mut self.base }
    fn isValid(&mut self) -> Option<bool> { Self::isValid(self) }
    fn toValidationInfoNode(&mut self, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Node>>> { Self::toValidationInfoNode(self, document) }
    fn toValidationInfoDoc(&mut self) -> Option<Rc<RefCell<Document>>> { Self::toValidationInfoDoc(self) }
    fn validationInfo(&mut self) -> String { Self::validationInfo(self) }
}


