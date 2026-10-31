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
// package org.allbinary.business.user.commerce.shipping.modules
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::business::user::commerce::shipping::ShippingMethodData::ShippingMethodData;
use crate::org::allbinary::data::tree::dom::DomNodeInterface::DomNodeInterface;
use crate::org::allbinary::data::tree::dom::ModDomHelper::ModDomHelper;
use crate::org::allbinary::data::tree::dom::document::DomDocumentHelper::DomDocumentHelper;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::w3c::dom::Document::Document;
use crate::org::w3c::dom::Node::Node;
use super::BasicWeightShippingModule::BasicWeightShippingModule;
use super::dom::dom;
use super::w3c::w3c;
use super::org::org;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct BasicWeightShippingModuleView {
    pub base: BasicWeightShippingModule,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
}

impl DomNodeInterface for BasicWeightShippingModuleView {
    fn toXmlNode(&mut self, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Node>>> { Self::toXmlNode(self, document) }
}

impl BasicWeightShippingModuleView {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: BasicWeightShippingModule::new_struct(),
            logUtil: None,
        };
        this.logUtil = LogUtil::getInstance();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn isValid(&mut self) -> Option<bool> {
        return None;
    }
    

    pub fn toValidationInfoDoc(&mut self) -> Option<Rc<RefCell<Document>>> {
        return None;
    }
    

    pub fn toValidationInfoNode(&mut self, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Node>>> {
        return None;
    }
    

    pub fn validationInfo(&mut self) -> String {
        return None;
    }
    

    pub fn toXmlNode(&mut self, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Node>>> {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut shippingMethodNode: Option<Rc<RefCell<Node>>> = jcall!(document).createElement(crate::org::allbinary::business::user::commerce::shipping::ShippingMethodData::ShippingMethodData_NAME.with(Clone::clone).clone());
            jcall!(shippingMethodNode).appendChild(ModDomHelper::createNameValueNodes(document.clone(), crate::org::allbinary::business::user::commerce::shipping::ShippingMethodData::ShippingMethodData_NAME.with(Clone::clone).clone(), self.getName()));
            jcall!(shippingMethodNode).appendChild(ModDomHelper::createNameValueNodes(document.clone(), crate::org::allbinary::business::user::commerce::shipping::ShippingMethodData::ShippingMethodData_DESCRIPTION.with(Clone::clone).clone(), self.getDescription()));
            return shippingMethodNode.clone();
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).SQLLOGGING.clone()) {
                jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("toXmlNode"), crate::java_into_object(e.clone()));
            }
            return None;
            }
        }
    }
    

    pub fn toXmlDoc(&mut self) -> Option<Rc<RefCell<Document>>> {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut document: Option<Rc<RefCell<Document>>> = DomDocumentHelper::create();
            return document.clone();
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).SQLLOGGING.clone()) {
                jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("toXmlDoc"), crate::java_into_object(e.clone()));
            }
            return None;
            }
        }
    }
    

    pub fn view(&mut self) -> String {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            return jcall!(self.base.commonStrings).NOT_IMPLEMENTED.clone();
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).SQLLOGGING.clone()) {
                jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("view"), crate::java_into_object(e.clone()));
            }
            return None;
            }
        }
    }
}
impl std::ops::Deref for BasicWeightShippingModuleView {
    type Target = BasicWeightShippingModule;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for BasicWeightShippingModuleView {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::business::user::commerce::shipping::modules::BasicWeightShippingModule::BasicWeightShippingModuleDyn for BasicWeightShippingModuleView {
    fn as_BasicWeightShippingModule(&self) -> &crate::org::allbinary::business::user::commerce::shipping::modules::BasicWeightShippingModule::BasicWeightShippingModule { &self.base }
    fn as_BasicWeightShippingModule_mut(&mut self) -> &mut crate::org::allbinary::business::user::commerce::shipping::modules::BasicWeightShippingModule::BasicWeightShippingModule { &mut self.base }
}
impl crate::org::allbinary::business::user::commerce::shipping::modules::ShippingInterface::ShippingInterface for BasicWeightShippingModuleView {
    fn getCost(&mut self, p1: Option<Rc<RefCell<dyn crate::org::allbinary::business::user::commerce::inventory::order::OrderInterface::OrderInterface>>>) -> Option<Rc<RefCell<crate::org::allbinary::business::user::commerce::money::Money::Money>>> { crate::org::allbinary::business::user::commerce::shipping::modules::BasicWeightShippingModule::BasicWeightShippingModule::getCost(crate::org::allbinary::business::user::commerce::shipping::modules::BasicWeightShippingModule::BasicWeightShippingModuleDyn::as_BasicWeightShippingModule_mut(self), p1) }
    fn getName(&mut self) -> String { crate::org::allbinary::business::user::commerce::shipping::modules::BasicWeightShippingModule::BasicWeightShippingModule::getName(crate::org::allbinary::business::user::commerce::shipping::modules::BasicWeightShippingModule::BasicWeightShippingModuleDyn::as_BasicWeightShippingModule_mut(self)) }
    fn getDescription(&mut self) -> String { crate::org::allbinary::business::user::commerce::shipping::modules::BasicWeightShippingModule::BasicWeightShippingModule::getDescription(crate::org::allbinary::business::user::commerce::shipping::modules::BasicWeightShippingModule::BasicWeightShippingModuleDyn::as_BasicWeightShippingModule_mut(self)) }
}


