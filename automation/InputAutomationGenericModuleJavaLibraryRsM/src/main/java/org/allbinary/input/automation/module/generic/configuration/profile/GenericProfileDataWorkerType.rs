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
// package org.allbinary.input.automation.module.generic.configuration.profile
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;
use super::*;
// use crate::java::util::HashMap::HashMap;
use crate::org::allbinary::data::tree::dom::DomNodeHelper::DomNodeHelper;
use crate::org::allbinary::data::tree::dom::ModDomHelper::ModDomHelper;
use crate::org::allbinary::input::media::image::InputImageType::InputImageType;
use crate::org::allbinary::logic::StdUtil::StdUtil;
use crate::org::w3c::dom::Document::Document;
use crate::org::w3c::dom::Node::Node;
use super::GenericProfileDataWorkerTypeInterface::GenericProfileDataWorkerTypeInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct GenericProfileDataWorkerType {
    pub name: String,
}

pub static mut GenericProfileDataWorkerType_hashMap: std::cell::LazyCell<Option<Rc<RefCell<HashMap>>>> = std::cell::LazyCell::new(|| jcall!(StdUtil::getInstance()).createHashMap());
pub static mut GenericProfileDataWorkerType_SAVED_CAPTURE: std::cell::LazyCell<Option<Rc<RefCell<dyn GenericProfileDataWorkerTypeDyn>>>> = std::cell::LazyCell::new(|| unsafe { SavedCaptureGenericProfileDataWorkerType_SAVED_CAPTURE.clone() }.clone());
pub static mut GenericProfileDataWorkerType_SCREEN_CAPTURE: std::cell::LazyCell<Option<Rc<RefCell<dyn GenericProfileDataWorkerTypeDyn>>>> = std::cell::LazyCell::new(|| GenericProfileDataWorkerType::new(format!("{}{}", String::from("Screen "), jcall!(unsafe { crate::org::allbinary::input::media::image::InputImageType::InputImageType_CAPTURE.clone() }).getName())).map(|reference| reference as Rc<RefCell<dyn GenericProfileDataWorkerTypeDyn>>));
pub static mut GenericProfileDataWorkerType_COMPARISON: std::cell::LazyCell<Option<Rc<RefCell<dyn GenericProfileDataWorkerTypeDyn>>>> = std::cell::LazyCell::new(|| GenericProfileDataWorkerType::new(jcall!(unsafe { crate::org::allbinary::input::media::image::InputImageType::InputImageType_COMPARISON.clone() }).getName()).map(|reference| reference as Rc<RefCell<dyn GenericProfileDataWorkerTypeDyn>>));
pub static mut GenericProfileDataWorkerType_MOTION: std::cell::LazyCell<Option<Rc<RefCell<dyn GenericProfileDataWorkerTypeDyn>>>> = std::cell::LazyCell::new(|| GenericProfileDataWorkerType::new(jcall!(unsafe { crate::org::allbinary::input::media::image::InputImageType::InputImageType_MOTION.clone() }).getName()).map(|reference| reference as Rc<RefCell<dyn GenericProfileDataWorkerTypeDyn>>));
impl GenericProfileDataWorkerTypeInterface for GenericProfileDataWorkerType {
    fn getName(&mut self) -> String { Self::getName(self) }
    fn setName(&mut self, mut name: String) { Self::setName(self, name) }
    fn toString(&mut self) -> String { Self::toString(self) }
    fn toXmlNode(&mut self, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Node>>> { Self::toXmlNode(self, document) }
}

impl GenericProfileDataWorkerType {
    

    pub fn new_struct(mut name: String) -> Self {
        let mut this = Self {
            name: String::new(),
        };
        this.setName(name.clone());
        this
    }

    pub fn new(mut name: String) -> Option<Rc<RefCell<Self>>> {
        let this = Some(Rc::new(RefCell::new(Self::new_struct(name))));
            jcall!(unsafe { GenericProfileDataWorkerType_hashMap.clone() }).put(this.getName(), (|java_value: Option<Rc<RefCell<_>>>| java_value.map(|reference| Rc::new(RefCell::new(Box::new(Some(reference)) as Box<dyn std::any::Any>))))(this.clone()));
        this
    }
    

    pub fn getName(&mut self) -> String {
        return self.name.clone();
    }
    

    pub fn setName(&mut self, mut name: String) {
        self.name = name.clone();
    }
    

    // throws Exception
    pub fn toXmlNode(&mut self, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Node>>> {
        let mut node: Option<Rc<RefCell<Node>>> = ModDomHelper::createTextNode(document.clone(), unsafe { GenericProfileDataWorkerData_NAME.clone() }.clone(), self.name.clone());
        return node.clone();
    }
    

    pub fn toString(&mut self) -> String {
        return format!("{}{}", String::from("GenericProfileDataWorkerType: "), self.getName());
    }
    

    pub fn getInstance(mut node: Option<Rc<RefCell<Node>>>) -> Option<Rc<RefCell<dyn GenericProfileDataWorkerTypeDyn>>> {
        let mut name: String = DomNodeHelper::getTextNodeValue(node.clone());
        return (jcall!(unsafe { GenericProfileDataWorkerType_hashMap.clone() }).get(name.clone())).clone() /* as GenericProfileDataWorkerType */;
    }
}
pub trait GenericProfileDataWorkerTypeDyn {
    fn as_GenericProfileDataWorkerType(&self) -> &GenericProfileDataWorkerType;
    fn as_GenericProfileDataWorkerType_mut(&mut self) -> &mut GenericProfileDataWorkerType;
    fn getName(&mut self) -> String { GenericProfileDataWorkerType::getName(self.as_GenericProfileDataWorkerType_mut()) }
    fn setName(&mut self, mut name: String) { GenericProfileDataWorkerType::setName(self.as_GenericProfileDataWorkerType_mut(), name) }
    fn toXmlNode(&mut self, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Node>>> { GenericProfileDataWorkerType::toXmlNode(self.as_GenericProfileDataWorkerType_mut(), document) }
    fn toString(&mut self) -> String { GenericProfileDataWorkerType::toString(self.as_GenericProfileDataWorkerType_mut()) }
}
impl GenericProfileDataWorkerTypeDyn for GenericProfileDataWorkerType {
    fn as_GenericProfileDataWorkerType(&self) -> &GenericProfileDataWorkerType { self }
    fn as_GenericProfileDataWorkerType_mut(&mut self) -> &mut GenericProfileDataWorkerType { self }
}


