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
// package org.allbinary.logic.visual.transform.template.customizer.widgets.title
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
use crate::org::allbinary::logic::string::StringValidationUtil::StringValidationUtil;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::org::w3c::dom::Document::Document;
use crate::org::w3c::dom::Node::Node;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct TitleView {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub commonStrings: Option<Rc<RefCell<CommonStrings>>>,
    pub title: String,
}

thread_local! { pub static TitleView_NAME: String = String::from("None"); }
impl DomNodeInterface for TitleView {
    fn toXmlNode(&mut self, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Node>>> { Self::toXmlNode(self, document) }
}

impl TitleView {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            logUtil: None,
            commonStrings: None,
            title: String::new(),
        };
        this.logUtil = LogUtil::getInstance();
        this.commonStrings = CommonStrings::getInstance();
        this.title = TitleView_NAME.with(Clone::clone).clone();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn new_struct1(mut node: Option<Rc<RefCell<Node>>>) -> Self {
        let mut this = Self {
            logUtil: None,
            commonStrings: None,
            title: String::new(),
        };
        this.logUtil = LogUtil::getInstance();
        this.commonStrings = CommonStrings::getInstance();
        let mut titleTextNode: Option<Rc<RefCell<Node>>> = DomSearchHelper::getNode(jcall!(TitleData::TitleData::getInstance()).TEXT.clone(), jcall!(node).getChildNodes());
        let mut valueNode: Option<Rc<RefCell<Node>>> = DomSearchHelper::getNode(crate::org::allbinary::data::tree::dom::DomData::DomData_VALUE.with(Clone::clone).clone(), jcall!(titleTextNode).getChildNodes());
        this.title = DomNodeHelper::getTextNodeValue(valueNode.clone());
        if jcall!(StringValidationUtil::getInstance()).isEmpty(this.title.clone()) {
            this.title = TitleView_NAME.with(Clone::clone).clone();
        }
        if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).VIEW.clone()) {
            { let java_receiver = this.logUtil.clone(); let java_arguments = (format!("{}{}", String::from("Title: "), this.getTitle()), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("Consructor()"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        }
        this
    }

    pub fn new1(mut node: Option<Rc<RefCell<Node>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct1(node))))
    }
    

    pub fn new_struct1(mut hashMap: Option<Rc<RefCell<HashMap>>>) -> Self {
        let mut this = Self {
            logUtil: None,
            commonStrings: None,
            title: String::new(),
        };
        this.logUtil = LogUtil::getInstance();
        this.commonStrings = CommonStrings::getInstance();
        if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).VIEW.clone()) {
            { let java_receiver = this.logUtil.clone(); let java_arguments = (format!("{}{}", String::from("HashMap: "), jcall!(hashMap).toString()), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(this.commonStrings).CONSTRUCTOR.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        }
        this.getFormData(hashMap.clone());
        this
    }

    pub fn new1(mut hashMap: Option<Rc<RefCell<HashMap>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct1(hashMap))))
    }
    

    pub fn getTitle(&mut self) -> String {
        return self.title.clone();
    }
    

    pub fn getFormData(&mut self, mut hashMap: Option<Rc<RefCell<HashMap>>>) {
        self.title = (jcall!(hashMap).get(jcall!(TitleData::TitleData::getInstance()).TEXT.clone())).clone() /* as String */;
        if jcall!(StringValidationUtil::getInstance()).isEmpty(self.title.clone()) {
            self.title = TitleView_NAME.with(Clone::clone).clone();
        }
    }
    

    pub fn toHashMap(&mut self) -> Option<Rc<RefCell<HashMap>>> {
        let mut hashMap: Option<Rc<RefCell<HashMap>>> = jcall!(StdUtil::getInstance()).createHashMap();
        jcall!(hashMap).put(jcall!(TitleData::TitleData::getInstance()).TEXT.clone(), self.getTitle());
        return hashMap.clone();
    }
    

    // throws Exception
    pub fn toXmlNode(&mut self, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Node>>> {
        let mut hashMap: Option<Rc<RefCell<HashMap>>> = self.toHashMap();
        if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).VIEW.clone()) {
            { let java_receiver = self.logUtil.clone(); let java_arguments = (format!("{}{}", String::from("TitleView HashMap: "), jcall!(hashMap).toString()), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("toXmlNode()"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        }
        return ModDomHelper::createNameValueNodes(document.clone(), jcall!(TitleData::TitleData::getInstance()).NAME.clone(), hashMap.clone());
    }
}
pub trait TitleViewDyn {
    fn as_TitleView(&self) -> &TitleView;
    fn as_TitleView_mut(&mut self) -> &mut TitleView;
    fn getTitle(&mut self) -> String { TitleView::getTitle(self.as_TitleView_mut()) }
    fn getFormData(&mut self, mut hashMap: Option<Rc<RefCell<HashMap>>>) { TitleView::getFormData(self.as_TitleView_mut(), hashMap) }
    fn toHashMap(&mut self) -> Option<Rc<RefCell<HashMap>>> { TitleView::toHashMap(self.as_TitleView_mut()) }
    fn toXmlNode(&mut self, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Node>>> { TitleView::toXmlNode(self.as_TitleView_mut(), document) }
}
impl TitleViewDyn for TitleView {
    fn as_TitleView(&self) -> &TitleView { self }
    fn as_TitleView_mut(&mut self) -> &mut TitleView { self }
}


