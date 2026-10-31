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
// package org.allbinary.logic.visual.dhtml.style
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;
use super::*;
// use crate::java::util::HashMap::HashMap;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
use crate::org::allbinary::data::tree::dom::DomSearchHelper::DomSearchHelper;
use crate::org::allbinary::data::tree::dom::document::XmlDocumentHelper::XmlDocumentHelper;
use crate::org::allbinary::logic::communication::http::request::NameSpaceRequestParamData::NameSpaceRequestParamData;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::visual::dhtml::style::css::CssElementData::CssElementData;
use crate::org::allbinary::logic::visual::dhtml::style::css::CssElementsValidationFactory::CssElementsValidationFactory;
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

pub struct StylesValidationFactory {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
}

thread_local! { pub static StylesValidationFactory_instance: Option<Rc<RefCell<StylesValidationFactory>>> = StylesValidationFactory::new(); }

impl StylesValidationFactory {
    

    pub fn getInstance() -> Option<Rc<RefCell<StylesValidationFactory>>> {
        return StylesValidationFactory_instance.with(Clone::clone).clone();
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            logUtil: None,
        };
        this.logUtil = LogUtil::getInstance();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    // throws Exception
    // Java overloaded method: Rust needs a different name
    pub fn getInstance1(&mut self, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        let mut commonStrings: Option<Rc<RefCell<CommonStrings>>> = CommonStrings::getInstance();
        let mut nodeList: Option<Rc<RefCell<NodeList>>> = jcall!(document).getElementsByTagName(jcall!(StylesData::StylesData::getInstance()).NAME.clone());
        if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).VIEW.clone()) {
            { let java_receiver = self.logUtil.clone(); let java_arguments = (format!("{}{}", String::from("Style Present: "), XmlDocumentHelper::toString(document.clone())), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(commonStrings).GET_INSTANCE.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        }
        if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).VIEW.clone()) {
            { let java_receiver = self.logUtil.clone(); let java_arguments = (format!("{}{}", String::from("Styles Present: "), jcall!(nodeList).getLength()), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(commonStrings).GET_INSTANCE.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        }
        for index in (0..jcall!(nodeList).getLength()) {
            let mut stylesNode: Option<Rc<RefCell<Node>>> = jcall!(nodeList).item(index.clone());
            let mut styleNodeList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = DomSearchHelper::getAllNodes(jcall!(StyleData::StyleData::getInstance()).NAME.clone(), jcall!(stylesNode).getChildNodes());
            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).VIEW.clone()) {
                { let java_receiver = self.logUtil.clone(); let java_arguments = (format!("{}{}", String::from("Number Of Style Nodes Present: "), jcall!(styleNodeList).size()), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(commonStrings).GET_INSTANCE.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
            }
            for styleNodesIndex in (0..jcall!(styleNodeList).size()) {
                let mut styleNode: Option<Rc<RefCell<Node>>> = (jcall!(styleNodeList).get(styleNodesIndex.clone())).clone() /* as Node */;
                let mut cssElementStyleNodeList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = DomSearchHelper::getAllNodes(jcall!(CssElementData::getInstance()).NAME.clone(), jcall!(styleNode).getChildNodes());
                if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).VIEW.clone()) {
                    { let java_receiver = self.logUtil.clone(); let java_arguments = (format!("{}{}", String::from("Number Of Element Nodes Present: "), jcall!(cssElementStyleNodeList).size()), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(commonStrings).GET_INSTANCE.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
                }
                return CssElementsValidationFactory::getInstance(cssElementStyleNodeList.clone());
            }
        }
        if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).VIEW.clone()) {
            jcall!(self.logUtil).putF(String::from("No Style Present"), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(commonStrings).GET_INSTANCE.clone());
        }
        return BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
    }
    

    // throws Exception
    // Java overloaded method: Rust needs a different name
    pub fn getInstance1(mut hashMap: Option<Rc<RefCell<HashMap>>>) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        let mut stylesDocument: Option<Rc<RefCell<Document>>> = (jcall!(hashMap).get(crate::org::allbinary::logic::communication::http::request::NameSpaceRequestParamData::NameSpaceRequestParamData_DOCUMENT.with(Clone::clone).clone())).clone() /* as Document */;
        return jcall!(StylesValidationFactory::getInstance()).getInstance1(stylesDocument.clone());
    }
}

