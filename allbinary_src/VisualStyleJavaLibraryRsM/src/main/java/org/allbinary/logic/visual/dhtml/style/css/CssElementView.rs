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
// package org.allbinary.logic.visual.dhtml.style.css
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
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use crate::org::allbinary::logic::visual::dhtml::style::css::property::CssPropertiesValidationFactory::CssPropertiesValidationFactory;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::org::w3c::dom::Document::Document;
use crate::org::w3c::dom::Node::Node;
use crate::org::w3c::dom::NodeList::NodeList;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct CssElementView {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub commonStrings: Option<Rc<RefCell<CommonStrings>>>,
    pub TITLE: String,
    pub DESCRIPTION: String,
    pub value: String,
    pub title: String,
    pub description: String,
    pub cssElementVector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>,
    pub cssPropertyVector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>,
}

impl DomNodeInterface for CssElementView {
    fn toXmlNode(&mut self, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Node>>> { Self::toXmlNode(self, document) }
}

impl CssElementView {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            logUtil: None,
            commonStrings: None,
            TITLE: String::new(),
            DESCRIPTION: String::new(),
            value: String::new(),
            title: String::new(),
            description: String::new(),
            cssElementVector: None,
            cssPropertyVector: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.commonStrings = CommonStrings::getInstance();
        this.TITLE = String::from("title");
        this.DESCRIPTION = String::from("description");
        this.cssPropertyVector = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        let mut stringUtil: Option<Rc<RefCell<StringUtil>>> = StringUtil::getInstance();
        this.title = jcall!(stringUtil).EMPTY_STRING.clone();
        this.description = jcall!(stringUtil).EMPTY_STRING.clone();
        this.value = jcall!(stringUtil).EMPTY_STRING.clone();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn new_struct1(mut node: Option<Rc<RefCell<Node>>>) -> Self {
        let mut this = Self {
            logUtil: None,
            commonStrings: None,
            TITLE: String::new(),
            DESCRIPTION: String::new(),
            value: String::new(),
            title: String::new(),
            description: String::new(),
            cssElementVector: None,
            cssPropertyVector: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.commonStrings = CommonStrings::getInstance();
        this.TITLE = String::from("title");
        this.DESCRIPTION = String::from("description");
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut valueNode: Option<Rc<RefCell<Node>>> = DomSearchHelper::getNode(crate::org::allbinary::data::tree::dom::DomData::DomData_VALUE.with(Clone::clone).clone(), jcall!(node).getChildNodes());
            this.value = DomNodeHelper::getTextNodeValue(valueNode.clone());
            let mut titleValueNode: Option<Rc<RefCell<Node>>> = DomSearchHelper::getNodeNoThrow(this.TITLE.clone(), jcall!(node).getChildNodes());
            if titleValueNode.is_some() {
                this.title = DomNodeHelper::getTextNodeValue(titleValueNode.clone());
            } else {
                let mut stringUtil: Option<Rc<RefCell<StringUtil>>> = StringUtil::getInstance();
                this.title = jcall!(stringUtil).EMPTY_STRING.clone();
            }
            let mut descriptionValueNode: Option<Rc<RefCell<Node>>> = DomSearchHelper::getNodeNoThrow(this.DESCRIPTION.clone(), jcall!(node).getChildNodes());
            if descriptionValueNode.is_some() {
                this.description = DomNodeHelper::getTextNodeValue(descriptionValueNode.clone());
            } else {
                this.description = jcall!(StringUtil::getInstance()).EMPTY_STRING.clone();
            }
            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).VIEW.clone()) {
                { let java_receiver = this.logUtil.clone(); let java_arguments = (jcall!(StringMaker::new()).append(String::from("Value: ")).append(this.value.clone()).append(String::from("\nTitle: ")).append(this.title.clone()).append(String::from("\nDescription: ")).append(this.description.clone()).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(this.commonStrings).CONSTRUCTOR.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
            }
            let mut cssElementStyleNodeVector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = DomSearchHelper::getAllNodes(jcall!(CssElementData::CssElementData::getInstance()).NAME.clone(), jcall!(node).getChildNodes());
            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).VIEW.clone()) {
                { let java_receiver = this.logUtil.clone(); let java_arguments = (format!("{}{}", String::from("Child Css Elements: "), jcall!(cssElementStyleNodeVector).size()), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("CssElementView()"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
            }
            this.cssElementVector = CssElementsValidationFactory::CssElementsValidationFactory::getInstance(cssElementStyleNodeVector.clone());
            let mut nodeList: Option<Rc<RefCell<NodeList>>> = jcall!(node).getChildNodes();
            this.cssPropertyVector = CssPropertiesValidationFactory::getInstance(nodeList.clone());
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).VIEWERROR.clone()) {
                { let java_receiver = this.logUtil.clone(); let java_arguments = (jcall!(StringMaker::new()).append(String::from("Value: ")).append(this.value.clone()).append(String::from("\nTitle: ")).append(this.title.clone()).append(String::from("\nDescription: ")).append(this.description.clone()).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(this.commonStrings).CONSTRUCTOR.clone(), crate::java_into_object(e.clone()),); jcall!(java_receiver).put(java_arguments.0, java_arguments.1, java_arguments.2, java_arguments.3); };
            }
            panic!("{:?}", format!("{}: {:?}", "Exception", (String::from("CssElementView"))));
            }
        }
        this
    }

    pub fn new1(mut node: Option<Rc<RefCell<Node>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct1(node))))
    }
    

    pub fn getValue(&mut self) -> String {
        return self.value.clone();
    }
    

    pub fn toHashMap(&mut self) -> Option<Rc<RefCell<HashMap>>> {
        let mut hashMap: Option<Rc<RefCell<HashMap>>> = jcall!(StdUtil::getInstance()).createHashMap();
        jcall!(hashMap).put(crate::org::allbinary::data::tree::dom::DomData::DomData_VALUE.with(Clone::clone).clone(), self.value.clone());
        jcall!(hashMap).put(self.TITLE.clone(), self.title.clone());
        jcall!(hashMap).put(self.DESCRIPTION.clone(), self.description.clone());
        return hashMap.clone();
    }
    

    // throws Exception
    pub fn toXmlNode(&mut self, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Node>>> {
        let mut hashMap: Option<Rc<RefCell<HashMap>>> = self.toHashMap();
        if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).VIEW.clone()) {
            { let java_receiver = self.logUtil.clone(); let java_arguments = (format!("{}{}", String::from("CssView HashMap: "), jcall!(hashMap).toString()), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("toXmlNode"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        }
        let mut node: Option<Rc<RefCell<Node>>> = ModDomHelper::createNodeWithValueNodes(document.clone(), jcall!(CssElementData::CssElementData::getInstance()).NAME.clone(), hashMap.clone());
        let mut size: i32 = jcall!(self.cssPropertyVector).size();
        for index in (0..size) {
            let mut cssPropertyDomNodeInterface: Option<Rc<RefCell<dyn DomNodeInterface>>> = { let java_object = jcall!(self.cssPropertyVector).get(index.clone()); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn DomNodeInterface>>>>().expect("ClassCastException").clone()) };
            jcall!(node).appendChild(jcall!(cssPropertyDomNodeInterface).toXmlNode(document.clone()));
        }
        let mut size2: i32 = jcall!(self.cssElementVector).size();
        for index in (0..size2) {
            let mut cssElementDomNodeInterface: Option<Rc<RefCell<dyn DomNodeInterface>>> = { let java_object = jcall!(self.cssElementVector).get(index.clone()); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn DomNodeInterface>>>>().expect("ClassCastException").clone()) };
            jcall!(node).appendChild(jcall!(cssElementDomNodeInterface).toXmlNode(document.clone()));
        }
        return node.clone();
    }
}
pub trait CssElementViewDyn {
    fn as_CssElementView(&self) -> &CssElementView;
    fn as_CssElementView_mut(&mut self) -> &mut CssElementView;
    fn getValue(&mut self) -> String { CssElementView::getValue(self.as_CssElementView_mut()) }
    fn toHashMap(&mut self) -> Option<Rc<RefCell<HashMap>>> { CssElementView::toHashMap(self.as_CssElementView_mut()) }
    fn toXmlNode(&mut self, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Node>>> { CssElementView::toXmlNode(self.as_CssElementView_mut(), document) }
}
impl CssElementViewDyn for CssElementView {
    fn as_CssElementView(&self) -> &CssElementView { self }
    fn as_CssElementView_mut(&mut self) -> &mut CssElementView { self }
}


