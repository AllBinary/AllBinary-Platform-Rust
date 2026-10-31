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
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
use crate::org::allbinary::data::tree::dom::DomData::DomData;
use crate::org::allbinary::data::tree::dom::DomNodeHelper::DomNodeHelper;
use crate::org::allbinary::data::tree::dom::DomNodeInterface::DomNodeInterface;
use crate::org::allbinary::data::tree::dom::DomSearchHelper::DomSearchHelper;
use crate::org::allbinary::data::tree::dom::ModDomHelper::ModDomHelper;
use crate::org::allbinary::logic::StdUtil::StdUtil;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::string::CommonSeps::CommonSeps;
use crate::org::w3c::dom::Document::Document;
use crate::org::w3c::dom::Node::Node;
use super::CssPropertyValueData::CssPropertyValueData;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct CssPropertyValues {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub propertyValueVector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>,
}

impl DomNodeInterface for CssPropertyValues {
    fn toXmlNode(&mut self, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Node>>> { Self::toXmlNode(self, document) }
}

impl CssPropertyValues {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            logUtil: None,
            propertyValueVector: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.propertyValueVector = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn new_struct1(mut node: Option<Rc<RefCell<Node>>>) -> Self {
        let mut this = Self {
            logUtil: None,
            propertyValueVector: None,
        };
        this.logUtil = LogUtil::getInstance();
        let mut indexPropertyValueHashMap: Option<Rc<RefCell<HashMap>>> = jcall!(StdUtil::getInstance()).createHashMap();
        this.propertyValueVector = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        let mut cssPropertyNodeVector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = DomSearchHelper::getAllNodes(jcall!(CssPropertyValueData::getInstance()).NAME.clone(), jcall!(node).getChildNodes());
        if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).STYLE.clone()) {
            { let java_receiver = this.logUtil.clone(); let java_arguments = (format!("{}{}", String::from("Number Of Properties: "), jcall!(cssPropertyNodeVector).size()), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("CssProperties()"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        }
        let mut size: i32 = jcall!(cssPropertyNodeVector).size();
        for index in (0..size) {
            let mut cssPropertyValueNode: Option<Rc<RefCell<Node>>> = (jcall!(cssPropertyNodeVector).get(index.clone())).clone() /* as Node */;
            let mut valueNode: Option<Rc<RefCell<Node>>> = DomSearchHelper::getNode(crate::org::allbinary::data::tree::dom::DomData::DomData_VALUE.with(Clone::clone).clone(), jcall!(cssPropertyValueNode).getChildNodes());
            let mut propertyValue: String = DomNodeHelper::getTextNodeValue(valueNode.clone());
            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).STYLE.clone()) {
                jcall!(this.logUtil).putF(format!("{}{}", String::from("Property Value: "), propertyValue), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("CssProperties()"));
            }
            let mut indexNode: Option<Rc<RefCell<Node>>> = DomSearchHelper::getNode(crate::org::allbinary::data::tree::dom::DomData::DomData_INDEX.with(Clone::clone).clone(), jcall!(cssPropertyValueNode).getChildNodes());
            let mut indexValue: String = DomNodeHelper::getTextNodeValue(indexNode.clone());
            jcall!(indexPropertyValueHashMap).put(indexValue.clone(), propertyValue.clone());
        }
        let mut size2: i32 = jcall!(jcall!(indexPropertyValueHashMap).keySet()).size();
        for index in (0..size2) {
            let mut indexInt: Option<i32> = Some((index.clone()) as i32);
            let mut propertyValue: String = (jcall!(indexPropertyValueHashMap).get((indexInt).expect("NullPointerException").to_string())).clone() /* as String */;
            jcall!(this.propertyValueVector).add(crate::java_into_object(propertyValue.clone()));
        }
        this
    }

    pub fn new1(mut node: Option<Rc<RefCell<Node>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct1(node))))
    }
    

    pub fn toHashMap(&mut self) -> Option<Rc<RefCell<HashMap>>> {
        let mut commonSeps: Option<Rc<RefCell<CommonSeps>>> = CommonSeps::getInstance();
        let mut cssPropertyValueData: Option<Rc<RefCell<CssPropertyValueData>>> = CssPropertyValueData::getInstance();
        let mut hashMap: Option<Rc<RefCell<HashMap>>> = jcall!(StdUtil::getInstance()).createHashMap();
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        let mut size: i32 = jcall!(self.propertyValueVector).size();
        {
            let mut index: i32 = 0;
            while index < size {
            let mut value: String = (jcall!(self.propertyValueVector).get(index.clone())).clone() /* as String */;
            jcall!(stringBuffer).delete(0, jcall!(stringBuffer).length());
            jcall!(stringBuffer).append(jcall!(cssPropertyValueData).NAME.clone());
            jcall!(stringBuffer).append(jcall!(commonSeps).BRACKET_OPEN.clone());
            jcall!(stringBuffer).appendint(index.clone());
            jcall!(stringBuffer).append(jcall!(commonSeps).BRACKET_CLOSE.clone());
            jcall!(hashMap).put(jcall!(stringBuffer).toString(), value.clone());
            index += 1;
            index += 1;
            }
        }
        return hashMap.clone();
    }
    

    // throws Exception
    pub fn toXmlNode(&mut self, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Node>>> {
        if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).VIEW.clone()) {
            let mut hashMap: Option<Rc<RefCell<HashMap>>> = self.toHashMap();
            { let java_receiver = self.logUtil.clone(); let java_arguments = (format!("{}{}", String::from("CssPropertyValues HashMap: "), jcall!(hashMap).toString()), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("toXmlNode()"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        }
        return ModDomHelper::createNameValueIndexNodes(document.clone(), jcall!(CssPropertyData::CssPropertyData::getInstance()).VALUES.clone(), jcall!(CssPropertyValueData::getInstance()).NAME.clone(), self.propertyValueVector.clone());
    }
}

