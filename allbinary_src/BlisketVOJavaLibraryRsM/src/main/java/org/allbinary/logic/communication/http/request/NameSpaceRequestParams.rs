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
// package org.allbinary.logic.communication.http.request
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;
use super::*;
// use crate::java::util::HashMap::HashMap;
// use crate::java::util::Set::Set;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
// use crate::javax::servlet::http::HttpServletRequest::HttpServletRequest;
// use crate::javax::servlet::jsp::PageContext::PageContext;
use crate::org::allbinary::data::tree::dom::DomData::DomData;
use crate::org::allbinary::data::tree::dom::DomNodeHelper::DomNodeHelper;
use crate::org::allbinary::data::tree::dom::DomSearchHelper::DomSearchHelper;
use crate::org::allbinary::data::tree::dom::ModDomHelper::ModDomHelper;
use crate::org::allbinary::data::tree::dom::document::DomDocumentHelper::DomDocumentHelper;
use crate::org::allbinary::data::tree::dom::document::XmlDocumentHelper::XmlDocumentHelper;
use crate::org::allbinary::logic::StdUtil::StdUtil;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use crate::org::allbinary::string::CommonSeps::CommonSeps;
use crate::org::w3c::dom::Document::Document;
use crate::org::w3c::dom::Node::Node;
use super::RequestParams::RequestParams;
use super::NameSpaceRequestParam::NameSpaceRequestParam;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

//containing string values and other hashmaps of the like
pub struct NameSpaceRequestParams {
    pub base: RequestParams,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
}


impl NameSpaceRequestParams {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: RequestParams::new_struct(),
            logUtil: None,
        };
        this.logUtil = LogUtil::getInstance();
        // super(...) is in the struct literal above
        if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).HTTPREQUEST.clone()) {
            jcall!(this.logUtil).putF(String::from("Constructing empty"), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(this.commonStrings).CONSTRUCTOR.clone());
        }
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn new_struct1(mut httpServletRequest: Option<Rc<RefCell<HttpServletRequest>>>) -> Self {
        let mut this = Self {
            base: RequestParams::new_struct1(httpServletRequest.clone()),
            logUtil: None,
        };
        this.logUtil = LogUtil::getInstance();
        // super(...) is in the struct literal above
        if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).HTTPREQUEST.clone()) {
            jcall!(this.logUtil).putF(String::from("Constructing from HttpServletRequest"), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(this.commonStrings).CONSTRUCTOR.clone());
        }
        this
    }

    pub fn new1(mut httpServletRequest: Option<Rc<RefCell<HttpServletRequest>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct1(httpServletRequest))))
    }
    

    pub fn new_struct1(mut pageContext: Option<Rc<RefCell<PageContext>>>) -> Self {
        let mut this = Self {
            base: RequestParams::new_struct1(pageContext.clone()),
            logUtil: None,
        };
        this.logUtil = LogUtil::getInstance();
        // super(...) is in the struct literal above
        if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).HTTPREQUEST.clone()) {
            jcall!(this.logUtil).putF(String::from("Constructing from PageContext"), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(this.commonStrings).CONSTRUCTOR.clone());
        }
        this
    }

    pub fn new1(mut pageContext: Option<Rc<RefCell<PageContext>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct1(pageContext))))
    }
    

    // throws Exception
    pub fn createPackageNode(&mut self, mut document: Option<Rc<RefCell<Document>>>, mut packageName: String, mut packagePropertiesHashMap: Option<Rc<RefCell<HashMap>>>) -> Option<Rc<RefCell<Node>>> {
        let mut node: Option<Rc<RefCell<Node>>> = jcall!(document).createElement(packageName.clone());
        let mut propertyKeySet: Option<Rc<RefCell<dyn Set>>> = jcall!(packagePropertiesHashMap).keySet();
        let mut propertyKeySetArray: Vec<Option<Rc<RefCell<Box<dyn std::any::Any>>>>> = jcall!(propertyKeySet).toArray();
        let mut size: i32 = (propertyKeySetArray.len() as i32).clone();
        for index in (0..size) {
            let mut propertyName: String = (propertyKeySetArray[(index) as usize]).clone() /* as String */;
            let mut propertyValue: String = (jcall!(packagePropertiesHashMap).get(propertyName.clone())).clone() /* as String */;
            let mut propertyNode: Option<Rc<RefCell<Node>>> = ModDomHelper::createTextNode(document.clone(), propertyName.clone(), propertyValue.clone());
            jcall!(node).appendChild(propertyNode.clone());
        }
        return node.clone();
    }
    

    // throws Exception
    pub fn createPackageMultiNode(&mut self, mut document: Option<Rc<RefCell<Document>>>, mut nameSpaceRequestParam: Option<Rc<RefCell<NameSpaceRequestParam>>>, mut packageName: String, mut packagePropertiesHashMap: Option<Rc<RefCell<HashMap>>>) -> Option<Rc<RefCell<Node>>> {
        let mut beginIndex: i32 = packageName.find('[').map(|index| index as i32).unwrap_or(-1);
        let mut indexValue: String = packageName[(beginIndex + 1) as usize..((packageName.chars().count() as i32) - 1) as usize].to_string();
        let mut multiElementPackage: String = packageName[(0) as usize..(beginIndex) as usize].to_string();
        let mut selectedValue: String = jcall!(nameSpaceRequestParam).getValue();
        let mut propertyValue: String = (jcall!(packagePropertiesHashMap).get(crate::org::allbinary::data::tree::dom::DomData::DomData_VALUE.with(Clone::clone).clone())).clone() /* as String */;
        if (propertyValue.cmp(&NameSpaceRequestParamData_VALUE.with(Clone::clone).clone()) as i32) != 0 {
            selectedValue = propertyValue.clone();
        }
        if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).HTTPREQUEST.clone()) {
            jcall!(self.logUtil).putF(format!("{}{}", format!("{}{}", format!("{}{}", format!("{}{}", format!("{}{}", String::from("\nAppending Multinode Element: "), multiElementPackage), String::from("[")), indexValue), String::from("] ")), selectedValue), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("createPackageMultiNode"));
        }
        let mut nextNode: Option<Rc<RefCell<Node>>> = ModDomHelper::createNameValueNodes(document.clone(), multiElementPackage.clone(), selectedValue.clone());
        let mut indexNode: Option<Rc<RefCell<Node>>> = ModDomHelper::createTextNode(document.clone(), crate::org::allbinary::data::tree::dom::DomData::DomData_INDEX.with(Clone::clone).clone(), indexValue.clone());
        jcall!(nextNode).appendChild(indexNode.clone());
        return nextNode.clone();
    }
    

    // throws Exception
    pub fn getRootNode(&mut self, mut rootElementPackage: String, mut packagePropertiesHashMap: Option<Rc<RefCell<HashMap>>>, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Node>>> {
        let mut rootNode: Option<Rc<RefCell<Node>>> = DomSearchHelper::getNodeNoThrow(rootElementPackage.clone(), jcall!(document).getChildNodes());
        if rootNode.is_none() {
            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).HTTPREQUEST.clone()) {
                { let java_receiver = self.logUtil.clone(); let java_arguments = (format!("{}{}", format!("{}{}", format!("{}{}", format!("{}{}", String::from("\nAppending Document Root: "), String::from("\nElementName: ")), rootElementPackage), String::from("\nProperties: ")), jcall!(packagePropertiesHashMap).toString()), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("getRootNode"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
            }
            let mut node: Option<Rc<RefCell<Node>>> = self.createPackageNode(document.clone(), rootElementPackage.clone(), packagePropertiesHashMap.clone());
            jcall!(document).appendChild(node.clone());
            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).HTTPREQUEST.clone()) {
                { let java_receiver = self.logUtil.clone(); let java_arguments = (format!("{}{}", String::from("\nAppended Document Created: "), XmlDocumentHelper::toString(document.clone())), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("getRootNode"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
            }
            return node.clone();
        } else {
            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).HTTPREQUEST.clone()) {
                jcall!(self.logUtil).putF(String::from("\nUsing Existing Root Node"), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("getRootNode"));
            }
            return rootNode.clone();
        }
    }
    

    // throws Exception
    pub fn addNewProperties(&mut self, mut document: Option<Rc<RefCell<Document>>>, mut node: Option<Rc<RefCell<Node>>>, mut nextPackagePropertiesHashMap: Option<Rc<RefCell<HashMap>>>) -> Option<Rc<RefCell<Node>>> {
        if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).HTTPREQUEST.clone()) {
            { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(StringMaker::new()).append(String::from("\nAdding Any New Properties: ")).append(jcall!(nextPackagePropertiesHashMap).toString()).append(String::from("\nto node: ")).append(jcall!(node).getNodeName()).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("addNewProperties"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        }
        let mut propertyNameSet: Option<Rc<RefCell<dyn Set>>> = jcall!(nextPackagePropertiesHashMap).keySet();
        let mut propertyNameSetArray: Vec<Option<Rc<RefCell<Box<dyn std::any::Any>>>>> = jcall!(propertyNameSet).toArray();
        let mut size: i32 = (propertyNameSetArray.len() as i32).clone();
        for index in (0..size) {
            let mut nextPropertyName: String = (propertyNameSetArray[(index) as usize]).clone() /* as String */;
            let mut propertyValue: String = jcall!(StringUtil::getInstance()).getNonNull((jcall!(nextPackagePropertiesHashMap).get(nextPropertyName.clone())).clone() /* as String */);
            let mut valueNode: Option<Rc<RefCell<Node>>> = DomSearchHelper::getNodeNoThrow(nextPropertyName.clone(), jcall!(node).getChildNodes());
            if valueNode.is_some() {
                let mut existingElementNodeTextNodeValue: String = DomNodeHelper::getTextNodeValue(valueNode.clone());
                if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).HTTPREQUEST.clone()) {
                    { let java_receiver = self.logUtil.clone(); let java_arguments = (format!("{}{}", format!("{}{}", format!("{}{}", format!("{}{}", format!("{}{}", format!("{}{}", format!("{}{}", format!("{}{}", String::from("\nIf property: \""), nextPropertyName), String::from(" is the same as:\n")), String::from("Node: ")), jcall!(node).getNodeName()), String::from(" then: ")), propertyValue), String::from("==")), existingElementNodeTextNodeValue), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("addNewProperties"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
                }
                if (propertyValue.cmp(&existingElementNodeTextNodeValue.clone()) as i32) != 0 {
                    if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).HTTPREQUEST.clone()) {
                        jcall!(self.logUtil).putF(String::from("At least one Property from NameSpaceRequest Package was different from Node"), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("addNewProperties"));
                    }
                    panic!("{:?}", format!("{}: {:?}", "Exception", (String::from("Property Modifications Should Not Occur"))));
                }
            } else {
                if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).HTTPREQUEST.clone()) {
                    jcall!(self.logUtil).putF(format!("{}{}", format!("{}{}", format!("{}{}", String::from("Adding a new Property: "), nextPropertyName), String::from("=")), propertyValue), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("addNewProperties"));
                }
                let mut newPropertyNode: Option<Rc<RefCell<Node>>> = ModDomHelper::createTextNode(document.clone(), nextPropertyName.clone(), propertyValue.clone());
                jcall!(node).appendChild(newPropertyNode.clone());
            }
        }
        return node.clone();
    }
    

    // throws Exception
    pub fn isElementValueTextNodeEqual(&mut self, mut nextPackagePropertiesHashMap: Option<Rc<RefCell<HashMap>>>, mut node: Option<Rc<RefCell<Node>>>) -> bool {
        if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).HTTPREQUEST.clone()) {
            { let java_receiver = self.logUtil.clone(); let java_arguments = (format!("{}{}", format!("{}{}", format!("{}{}", String::from("\nComparing the value property in: "), jcall!(nextPackagePropertiesHashMap).toString()), String::from("\nto node child leaves of node: ")), jcall!(node).getNodeName()), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("isElementValueTextNodeEqual"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        }
        let mut propertyValue: String = (jcall!(nextPackagePropertiesHashMap).get(crate::org::allbinary::data::tree::dom::DomData::DomData_VALUE.with(Clone::clone).clone())).clone() /* as String */;
        let mut valueNode: Option<Rc<RefCell<Node>>> = DomSearchHelper::getNodeNoThrow(crate::org::allbinary::data::tree::dom::DomData::DomData_VALUE.with(Clone::clone).clone(), jcall!(node).getChildNodes());
        if valueNode.is_some() {
            let mut existingElementNodeTextNodeValue: String = DomNodeHelper::getTextNodeValue(valueNode.clone());
            if (propertyValue.cmp(&existingElementNodeTextNodeValue.clone()) as i32) == 0 {
                if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).HTTPREQUEST.clone()) {
                    jcall!(self.logUtil).putF(String::from("Value Node is the same"), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("isElementValueTextNodeEqual"));
                }
                return true;
            }
        } else {
            return true;
        }
        return false;
    }
    

    // throws Exception
    pub fn isElementValueTextNodeUnique(&mut self, mut nextPackagePropertiesHashMap: Option<Rc<RefCell<HashMap>>>, mut elementNodeVector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) -> i32 {
        if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).HTTPREQUEST.clone()) {
            { let java_receiver = self.logUtil.clone(); let java_arguments = (format!("{}{}", format!("{}{}", String::from("\nComparing Properties of: "), jcall!(elementNodeVector).size()), String::from(" Nodes")), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("isElementValueTextNodeUnique"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        }
        let mut size: i32 = jcall!(elementNodeVector).size();
        for index in (0..size) {
            let mut existingElementNode: Option<Rc<RefCell<Node>>> = (jcall!(elementNodeVector).get(index.clone())).clone() /* as Node */;
            if self.isElementValueTextNodeEqual(nextPackagePropertiesHashMap.clone(), existingElementNode.clone()) {
                return index.clone();
            }
        }
        return -1;
    }
    

    // throws Exception
    pub fn addChildren(&mut self, mut document: Option<Rc<RefCell<Document>>>, mut rootNode: Option<Rc<RefCell<Node>>>, mut nameSpaceRequestParam: Option<Rc<RefCell<NameSpaceRequestParam>>>) -> Option<Rc<RefCell<Document>>> {
        let mut node: Option<Rc<RefCell<Node>>> = rootNode.clone();
        let mut packageVector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = jcall!(nameSpaceRequestParam).getPackages();
        let mut size: i32 = jcall!(packageVector).size();
        for index in (1..size) {
            let mut nextPackageName: String = (jcall!(packageVector).get(index.clone())).clone() /* as String */;
            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).HTTPREQUEST.clone()) {
                { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(StringMaker::new()).append(String::from("\nTrying to Add Child to Node: ")).append(jcall!(node).getNodeName()).append(String::from("\nwith new PackageName: ")).append(nextPackageName.clone()).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("addChildren"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
            }
            let mut nextPackagePropertiesHashMap: Option<Rc<RefCell<HashMap>>> = jcall!(nameSpaceRequestParam).getPackageProperties(index.clone());
            let mut elementNodeVector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = DomSearchHelper::getAllNodesNoThrow(nextPackageName.clone(), jcall!(node).getChildNodes());
            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).HTTPREQUEST.clone()) {
                { let java_receiver = self.logUtil.clone(); let java_arguments = (format!("{}{}", format!("{}{}", String::from("\nDocument Contains "), jcall!(elementNodeVector).size()), String::from(" Node(s) With Same Name")), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("addChildren"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
            }
            let mut isElementValueTextNodeUniqueIndex: i32 = self.isElementValueTextNodeUnique(nextPackagePropertiesHashMap.clone(), elementNodeVector.clone());
            if (jcall!(elementNodeVector).size() == 0 || isElementValueTextNodeUniqueIndex == -1) && !nextPackageName.ends_with(&jcall!(CommonSeps::getInstance()).BRACKET_CLOSE.clone()) {
                let mut nextNode: Option<Rc<RefCell<Node>>> = self.createPackageNode(document.clone(), nextPackageName.clone(), nextPackagePropertiesHashMap.clone());
                jcall!(node).appendChild(nextNode.clone());
                node = nextNode.clone();
                if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).HTTPREQUEST.clone()) {
                    { let java_receiver = self.logUtil.clone(); let java_arguments = (format!("{}{}", String::from("\nAppended Package: "), jcall!(node).getNodeName()), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("addChildren"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
                    { let java_receiver = self.logUtil.clone(); let java_arguments = (format!("{}{}", String::from("\nAppended Document Created: "), XmlDocumentHelper::toString(document.clone())), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("addChildren"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
                }
            } else if nextPackageName.ends_with(&jcall!(CommonSeps::getInstance()).BRACKET_CLOSE.clone()) {
                if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).HTTPREQUEST.clone()) {
                    jcall!(self.logUtil).putF(String::from("Package is MultiNode: just append"), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("addChildren"));
                }
                let mut nextNode: Option<Rc<RefCell<Node>>> = self.createPackageMultiNode(document.clone(), nameSpaceRequestParam.clone(), nextPackageName.clone(), nextPackagePropertiesHashMap.clone());
                jcall!(node).appendChild(nextNode.clone());
                node = nextNode.clone();
                if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).HTTPREQUEST.clone()) {
                    { let java_receiver = self.logUtil.clone(); let java_arguments = (format!("{}{}", String::from("\nAppended Multinode Element: "), XmlDocumentHelper::toString(document.clone())), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("addChildren"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
                }
            } else {
                if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).HTTPREQUEST.clone()) {
                    jcall!(self.logUtil).putF(String::from("Value Node already exists: Now compare (properties to leaf nodes)"), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("addChildren"));
                }
                let mut nodeNameDuplicateNode: Option<Rc<RefCell<Node>>> = (jcall!(elementNodeVector).get(isElementValueTextNodeUniqueIndex.clone())).clone() /* as Node */;
                node = self.addNewProperties(document.clone(), nodeNameDuplicateNode.clone(), nextPackagePropertiesHashMap.clone());
                if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).HTTPREQUEST.clone()) {
                    jcall!(self.logUtil).putF(String::from("\nDuplicate Node Found: Moving to next child ^"), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("addChildre"));
                }
            }
        }
        return document.clone();
    }
    

    // throws Exception
    pub fn addNameSpace(&mut self, mut key: String, mut value: String, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Document>>> {
        if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).HTTPREQUEST.clone()) {
            { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(StringMaker::new()).append(String::from("NameSpace key: ")).append(key.clone()).append(String::from(" Value: ")).append(value.clone()).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("addNameSpace"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        }
        let mut nameSpaceRequestParam: Option<Rc<RefCell<NameSpaceRequestParam>>> = NameSpaceRequestParam::new(key.clone(), value.clone());
        let mut packageVector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = jcall!(nameSpaceRequestParam).getPackages();
        let mut size: i32 = jcall!(packageVector).size();
        for index in (0..size) {
            let mut packageName: String = (jcall!(packageVector).get(index.clone())).clone() /* as String */;
            let mut packagePropertiesHashMap: Option<Rc<RefCell<HashMap>>> = jcall!(nameSpaceRequestParam).getPackageProperties(jcall!(packageVector).indexOf(crate::java_into_object(packageName.clone())));
            let mut rootNode: Option<Rc<RefCell<Node>>> = self.getRootNode(packageName.clone(), packagePropertiesHashMap.clone(), document.clone());
            if index < size - 1 {
                return self.addChildren(document.clone(), (rootNode).clone() /* as Node */, nameSpaceRequestParam.clone());
            }
        }
        return document.clone();
    }
    

    // throws Exception
    pub fn toHashMap(&mut self) -> Option<Rc<RefCell<HashMap>>> {
        let mut document: Option<Rc<RefCell<Document>>> = DomDocumentHelper::create();
        let mut hashMap: Option<Rc<RefCell<HashMap>>> = jcall!(StdUtil::getInstance()).createHashMap();
        let mut keys: Option<Rc<RefCell<dyn Set>>> = jcall!(self.getMap()).keySet();
        let mut keyArray: Vec<Option<Rc<RefCell<Box<dyn std::any::Any>>>>> = jcall!(keys).toArray();
        let mut keySize: i32 = (keyArray.len() as i32).clone();
        if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).HTTPREQUEST.clone()) {
            { let java_receiver = self.logUtil.clone(); let java_arguments = (format!("{}{}", String::from("NameSpace Request Size: "), jcall!(keys).size()), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("toHashMap"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        }
        for i in (0..keySize) {
            let mut key: String = (keyArray[(i) as usize]).clone() /* as String */;
            let mut anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = jcall!(self.getMap()).get(key.clone());
            let mut className: String = (std::any::type_name_of_val(&anyType).to_string()).clone() /* as String */;
            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).HTTPREQUEST.clone()) {
                jcall!(self.logUtil).putF(format!("{}{}", String::from("Request Param Class: "), className), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("toHashMap"));
            }
            if (jcall!(self.getMap()).get(key.clone())).is_some() /* instanceof String */ {
                let mut value: String = (jcall!(self.getMap()).get(key.clone())).clone() /* as String */;
                if key.find(&NameSpaceRequestParamData_NAME.with(Clone::clone).clone()).map(|index| index as i32).unwrap_or(-1) < 0 {
                    if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).HTTPREQUEST.clone()) {
                        jcall!(self.logUtil).putF(format!("{}{}", format!("{}{}", format!("{}{}", String::from("Not NameSpace key: "), key), String::from(" Value: ")), value), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("toHashMap()"));
                    }
                    jcall!(hashMap).put(key.clone().clone(), value.clone().clone());
                } else {
                    document = self.addNameSpace(key.clone().clone(), value.clone().clone(), document.clone());
                }
            } else if (jcall!(self.getMap()).get(key.clone())).is_some() /* instanceof  */ {
                let mut values: Vec<String> = (jcall!(self.getMap()).get(key.clone())).clone() /* as  */;
                if key.find(&NameSpaceRequestParamData_NAME.with(Clone::clone).clone()).map(|index| index as i32).unwrap_or(-1) < 0 {
                    if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).HTTPREQUEST.clone()) {
                        jcall!(self.logUtil).putF(format!("{}{}", format!("{}{}", format!("{}{}", String::from("Not NameSpace key: "), key), String::from(" Value: ")), values[(0) as usize]), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("toHashMap()"));
                    }
                    jcall!(hashMap).put(key.clone().clone(), values[(0) as usize].clone().clone());
                } else {
                    document = self.addNameSpace(key.clone().clone(), values[(0) as usize].clone().clone(), document.clone());
                }
            }
        }
        if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).HTTPREQUEST.clone()) {
            { let java_receiver = self.logUtil.clone(); let java_arguments = (format!("{}{}", format!("{}{}", format!("{}{}", format!("{}{}", format!("{}{}", String::from("NameSpaceRequestParams: "), jcall!(self.getMap()).toString()), String::from("\ntoHashMap(): ")), jcall!(hashMap).toString()), String::from("\nDocument Created: ")), XmlDocumentHelper::toString(document.clone())), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("toHashMap()"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        }
        jcall!(hashMap).put(NameSpaceRequestParamData_DOCUMENT.with(Clone::clone).clone(), document.clone());
        return hashMap.clone();
    }
}
impl std::ops::Deref for NameSpaceRequestParams {
    type Target = RequestParams;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for NameSpaceRequestParams {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::logic::communication::http::request::RequestParams::RequestParamsDyn for NameSpaceRequestParams {
    fn as_RequestParams(&self) -> &crate::org::allbinary::logic::communication::http::request::RequestParams::RequestParams { &self.base }
    fn as_RequestParams_mut(&mut self) -> &mut crate::org::allbinary::logic::communication::http::request::RequestParams::RequestParams { &mut self.base }
    fn toHashMap(&mut self) -> Option<Rc<RefCell<HashMap>>> { Self::toHashMap(self) }
}


