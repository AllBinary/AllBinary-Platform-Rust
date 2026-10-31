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
// package org.allbinary.logic.visual.dhtml.style.css.property
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;
use super::*;
// use crate::java::util::HashMap::HashMap;
use crate::org::allbinary::data::tree::dom::DomData::DomData;
use crate::org::allbinary::data::tree::dom::DomNodeHelper::DomNodeHelper;
use crate::org::allbinary::data::tree::dom::DomNodeInterface::DomNodeInterface;
use crate::org::allbinary::data::tree::dom::DomSearchHelper::DomSearchHelper;
use crate::org::allbinary::data::tree::dom::ModDomHelper::ModDomHelper;
use crate::org::allbinary::logic::StdUtil::StdUtil;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::w3c::dom::Document::Document;
use crate::org::w3c::dom::Node::Node;
use super::CssPropertyValues::CssPropertyValues;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct CssProperty {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub propertyName: String,
    pub cssPropertyValues: Option<Rc<RefCell<CssPropertyValues>>>,
}

impl DomNodeInterface for CssProperty {
    fn toXmlNode(&mut self, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Node>>> { Self::toXmlNode(self, document) }
}

impl CssProperty {
    

    pub fn new_struct1(mut name: String) -> Self {
        let mut this = Self {
            logUtil: None,
            propertyName: String::new(),
            cssPropertyValues: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.propertyName = name.clone();
        this.cssPropertyValues = CssPropertyValues::new();
        this
    }

    pub fn new1(mut name: String) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct1(name))))
    }
    

    pub fn new_struct1(mut node: Option<Rc<RefCell<Node>>>) -> Self {
        let mut this = Self {
            logUtil: None,
            propertyName: String::new(),
            cssPropertyValues: None,
        };
        this.logUtil = LogUtil::getInstance();
        let mut propertyNode: Option<Rc<RefCell<Node>>> = DomSearchHelper::getNode(crate::org::allbinary::data::tree::dom::DomData::DomData_VALUE.with(Clone::clone).clone(), jcall!(node).getChildNodes());
        this.propertyName = DomNodeHelper::getTextNodeValue(propertyNode.clone());
        if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).STYLE.clone()) {
            { let java_receiver = this.logUtil.clone(); let java_arguments = (format!("{}{}", String::from("Name: "), this.getName()), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("CssProperty()"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        }
        let mut propertyValuesNode: Option<Rc<RefCell<Node>>> = DomSearchHelper::getNode(jcall!(CssPropertyData::CssPropertyData::getInstance()).VALUES.clone(), jcall!(node).getChildNodes());
        this.cssPropertyValues = CssPropertyValues::new1(propertyValuesNode.clone());
        this
    }

    pub fn new1(mut node: Option<Rc<RefCell<Node>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct1(node))))
    }
    

    pub fn getName(&mut self) -> String {
        return self.propertyName.clone();
    }
    

    pub fn toHashMap(&mut self) -> Option<Rc<RefCell<HashMap>>> {
        let mut hashMap: Option<Rc<RefCell<HashMap>>> = jcall!(StdUtil::getInstance()).createHashMap();
        jcall!(hashMap).put(jcall!(CssPropertyData::CssPropertyData::getInstance()).NAME.clone(), self.propertyName.clone());
        return hashMap.clone();
    }
    

    // throws Exception
    pub fn toXmlNode(&mut self, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Node>>> {
        let mut hashMap: Option<Rc<RefCell<HashMap>>> = self.toHashMap();
        if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).VIEW.clone()) {
            { let java_receiver = self.logUtil.clone(); let java_arguments = (format!("{}{}", String::from("CssView HashMap: "), jcall!(hashMap).toString()), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("toXmlNode()"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        }
        let mut node: Option<Rc<RefCell<Node>>> = ModDomHelper::createNameValueNodes(document.clone(), jcall!(CssPropertyData::CssPropertyData::getInstance()).NAME.clone(), self.propertyName.clone());
        jcall!(node).appendChild(jcall!(self.cssPropertyValues).toXmlNode(document.clone()));
        return node.clone();
    }
}

