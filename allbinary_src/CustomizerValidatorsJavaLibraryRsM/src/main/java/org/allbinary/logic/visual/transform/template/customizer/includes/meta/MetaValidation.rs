//
// AllBinary Open License Version 1
// Copyright (c) 2011 AllBinary
// 
// By agreeing to this license you and any business entity you represent are
// legally bound to the AllBinary Open License Version 1 legal agreement.
// 
// You may obtain the AllBinary Open License Version 1 legal agreement from
// AllBinary or the root directory of AllBinary's AllBinary Platform repository.
// III
// Created By: Travis Berthelot
//  
// Generated Code Do Not Modify
// package org.allbinary.logic.visual.transform.template.customizer.includes.meta
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::java::util::HashMap::HashMap;
use crate::org::allbinary::data::tree::dom::DomNodeInterface::DomNodeInterface;
use crate::org::allbinary::data::tree::dom::ModDomHelper::ModDomHelper;
use crate::org::allbinary::logic::StdUtil::StdUtil;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::control::validate::Validation::Validation;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::visual::dhtml::html::head::meta::HtmlMetaAttributeData::HtmlMetaAttributeData;
use crate::org::allbinary::logic::visual::dhtml::html::head::meta::HtmlMetaAttributeDataFactory::HtmlMetaAttributeDataFactory;
use crate::org::allbinary::logic::visual::dhtml::html::head::meta::HtmlMetaData::HtmlMetaData;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::org::w3c::dom::Document::Document;
use crate::org::w3c::dom::Node::Node;
use crate::java::lang::Exception::*;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct MetaValidation {
    pub base: Validation,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub label: String,
    pub name: String,
    pub httpEquiv: String,
    pub content: String,
}

impl DomNodeInterface for MetaValidation {
    fn toXmlNode(&mut self, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Node>>> { Self::toXmlNode(self, document) }
}
impl crate::org::allbinary::logic::control::validate::ValidationInterface::ValidationInterface for MetaValidation {
    fn isValid(&mut self) -> Option<bool> { Self::isValid(self) }
    fn toValidationInfoNode(&mut self, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Node>>> { Self::toValidationInfoNode(self, document) }
    fn toValidationInfoDoc(&mut self) -> Option<Rc<RefCell<Document>>> { Self::toValidationInfoDoc(self) }
    fn validationInfo(&mut self) -> String { Self::validationInfo(self) }
}

impl MetaValidation {
    

    pub fn new_struct4(mut htmlMetaAttributeData: Option<Rc<RefCell<HtmlMetaAttributeData>>>, mut label: String, mut attributeValue: String, mut contentValue: String) -> Self {
        let mut this = Self {
            base: Validation::new_struct(),
            logUtil: None,
            label: String::new(),
            name: String::new(),
            httpEquiv: String::new(),
            content: String::new(),
        };
        this.logUtil = LogUtil::getInstance();
        this.label = label.clone();
        if jcall!(htmlMetaAttributeData).equals({ let java_field = jcall!(HtmlMetaAttributeDataFactory::getInstance()).HTTP_EQUIV.clone(); java_field }.clone()) {
            this.httpEquiv = attributeValue.clone();
        } else if jcall!(htmlMetaAttributeData).equals({ let java_field = jcall!(HtmlMetaAttributeDataFactory::getInstance()).NAME.clone(); java_field }.clone()) {
            this.name = attributeValue.clone();
        }
        this.content = contentValue.clone();
        this
    }

    pub fn new4(mut htmlMetaAttributeData: Option<Rc<RefCell<HtmlMetaAttributeData>>>, mut label: String, mut attributeValue: String, mut contentValue: String) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct4(htmlMetaAttributeData, label, attributeValue, contentValue))))
    }
    

    pub fn new_struct1(mut document: Option<Rc<RefCell<Document>>>) -> Self {
        let mut this = Self {
            base: Validation::new_struct(),
            logUtil: None,
            label: String::new(),
            name: String::new(),
            httpEquiv: String::new(),
            content: String::new(),
        };
        this.logUtil = LogUtil::getInstance();
        panic!("{:?}", format!("{}: {:?}", "Exception", ({ let java_field = jcall!(CommonStrings::getInstance()).NOT_IMPLEMENTED.clone(); java_field }.clone())));
        this
    }

    pub fn new1(mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct1(document))))
    }
    

    pub fn isValid(&mut self) -> Option<bool> {
        let java_try_result = match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut valid: Option<bool> = true;
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
        let mut hashMap: Option<Rc<RefCell<HashMap<crate::JavaObject, crate::JavaObject>>>> = jcall!(StdUtil::getInstance()).createHashMap();
        let mut htmlMetaAttributeDataFactory: Option<Rc<RefCell<HtmlMetaAttributeDataFactory>>> = HtmlMetaAttributeDataFactory::getInstance();
        jcall!(hashMap).put(crate::java_into_object({ let java_field = jcall!(HtmlMetaData::getInstance()).LABEL.clone(); java_field }.clone()), crate::java_into_object(self.label.clone()));
        jcall!(hashMap).put(crate::java_into_object(jcall!({ let java_field = jcall!(htmlMetaAttributeDataFactory).NAME.clone(); java_field }).toString()), crate::java_into_object(self.name.clone()));
        jcall!(hashMap).put(crate::java_into_object(jcall!({ let java_field = jcall!(htmlMetaAttributeDataFactory).HTTP_EQUIV.clone(); java_field }).toString()), crate::java_into_object(self.httpEquiv.clone()));
        jcall!(hashMap).put(crate::java_into_object(jcall!({ let java_field = jcall!(htmlMetaAttributeDataFactory).CONTENT.clone(); java_field }).toString()), crate::java_into_object(self.content.clone()));
        return hashMap.clone();
    }
    

    // throws Exception
    pub fn toXmlNode(&mut self, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Node>>> {
        let mut node: Option<Rc<RefCell<Node>>> = ModDomHelper::createNameValueNodes_Document_String_HashMap(document.clone(), { let java_argument = { let java_field = jcall!(HtmlMetaData::getInstance()).NAME.clone(); java_field }.clone(); java_argument }, self.toHashMap());
        return node.clone();
    }
}
impl std::ops::Deref for MetaValidation {
    type Target = Validation;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for MetaValidation {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::logic::control::validate::Validation::ValidationDyn for MetaValidation {
    fn as_Validation(&self) -> &crate::org::allbinary::logic::control::validate::Validation::Validation { &self.base }
    fn as_Validation_mut(&mut self) -> &mut crate::org::allbinary::logic::control::validate::Validation::Validation { &mut self.base }
    fn isValid(&mut self) -> Option<bool> { Self::isValid(self) }
    fn toValidationInfoNode(&mut self, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Node>>> { Self::toValidationInfoNode(self, document) }
    fn toValidationInfoDoc(&mut self) -> Option<Rc<RefCell<Document>>> { Self::toValidationInfoDoc(self) }
    fn validationInfo(&mut self) -> String { Self::validationInfo(self) }
}


