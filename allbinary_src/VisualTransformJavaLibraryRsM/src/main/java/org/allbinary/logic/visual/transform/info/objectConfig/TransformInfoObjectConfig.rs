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
// package org.allbinary.logic.visual.transform.info.objectConfig
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
use crate::org::allbinary::data::tree::dom::DomNodeHelper::DomNodeHelper;
use crate::org::allbinary::data::tree::dom::DomSearchHelper::DomSearchHelper;
use crate::org::allbinary::data::tree::dom::document::DomDocumentHelper::DomDocumentHelper;
use crate::org::allbinary::data::tree::dom::document::XmlDocumentHelper::XmlDocumentHelper;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::io::InputOutputTypeData::InputOutputTypeData;
use crate::org::allbinary::logic::io::OutputTypeData::OutputTypeData;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::visual::transform::info::TransformInfoData::TransformInfoData;
use crate::org::allbinary::logic::visual::transform::info::TransformInfoDomNode::TransformInfoDomNode;
use crate::org::allbinary::logic::visual::transform::info::TransformInfoInterface::TransformInfoInterface;
use crate::org::allbinary::logic::visual::transform::info::TransformInfosData::TransformInfosData;
use crate::org::allbinary::logic::visual::transform::template::XslData::XslData;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::org::w3c::dom::Attr::Attr;
use crate::org::w3c::dom::Document::Document;
use crate::org::w3c::dom::NamedNodeMap::NamedNodeMap;
use crate::org::w3c::dom::Node::Node;
use crate::org::w3c::dom::NodeList::NodeList;
use super::TransformInfoObjectConfigInterface::TransformInfoObjectConfigInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct TransformInfoObjectConfig {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub commonStrings: Option<Rc<RefCell<CommonStrings>>>,
    pub ownerTransformInfoInterface: Option<Rc<RefCell<dyn TransformInfoInterface>>>,
    pub document: Option<Rc<RefCell<Document>>>,
    pub outputTypeName: String,
}

impl TransformInfoObjectConfigInterface for TransformInfoObjectConfig {
    fn containsView(&mut self, mut transformInfoInterface: Option<Rc<RefCell<dyn TransformInfoInterface>>>) -> bool { Self::containsView(self, transformInfoInterface) }
    fn getName(&mut self) -> String { Self::getName(self) }
    fn getTransformsGroup(&mut self, mut group: String) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> { Self::getTransformsGroup(self, group) }
    fn getTransforms(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> { Self::getTransforms(self) }
    fn getParentTransforms(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> { Self::getParentTransforms(self) }
    fn getGroupTransforms(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> { Self::getGroupTransforms(self) }
    fn getOutputTypeName(&mut self) -> String { Self::getOutputTypeName(self) }
    fn setOutputTypeName(&mut self, mut outputTypeName: String) { Self::setOutputTypeName(self, outputTypeName) }
    fn getInputOutputTypeName(&mut self) -> String { Self::getInputOutputTypeName(self) }
    fn getInputOutputTypeFile(&mut self) -> String { Self::getInputOutputTypeFile(self) }
    fn getImportUriPath(&mut self) -> String { Self::getImportUriPath(self) }
    fn toString(&mut self) -> String { Self::toString(self) }
}
impl crate::org::allbinary::data::tree::dom::document::mapping::DomDocumentMappingInterface::DomDocumentMappingInterface for TransformInfoObjectConfig {
    fn toXmlDoc(&mut self) -> Option<Rc<RefCell<Document>>> { Self::toXmlDoc(self) }
}

impl TransformInfoObjectConfig {
    

    pub fn new_struct1(mut transformInfoInterface: Option<Rc<RefCell<dyn TransformInfoInterface>>>) -> Self {
        let mut this = Self {
            logUtil: None,
            commonStrings: None,
            ownerTransformInfoInterface: None,
            document: None,
            outputTypeName: String::new(),
        };
        this.logUtil = LogUtil::getInstance();
        this.commonStrings = CommonStrings::getInstance();
        this.ownerTransformInfoInterface = transformInfoInterface.clone();
        this.createDocument();
        if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).VIEW.clone()) {
            let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
            jcall!(stringBuffer).append(String::from("TransformInfo: "));
            if this.ownerTransformInfoInterface.is_some() {
                jcall!(stringBuffer).append(jcall!(this.ownerTransformInfoInterface).getName());
            } else {
                jcall!(stringBuffer).append(String::from("No Owner!?#@"));
            }
            jcall!(stringBuffer).append(String::from("\nConstructed with document: "));
            jcall!(stringBuffer).append(this.toString());
            { let java_receiver = this.logUtil.clone(); let java_arguments = (jcall!(stringBuffer).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("Constructor(TransformInfoInterface)"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        }
        this
    }

    pub fn new1(mut transformInfoInterface: Option<Rc<RefCell<dyn TransformInfoInterface>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct1(transformInfoInterface))))
    }
    

    pub fn new_struct2(mut transformInfoInterface: Option<Rc<RefCell<dyn TransformInfoInterface>>>, mut document: Option<Rc<RefCell<Document>>>) -> Self {
        let mut this = Self {
            logUtil: None,
            commonStrings: None,
            ownerTransformInfoInterface: None,
            document: None,
            outputTypeName: String::new(),
        };
        this.logUtil = LogUtil::getInstance();
        this.commonStrings = CommonStrings::getInstance();
        this.ownerTransformInfoInterface = transformInfoInterface.clone();
        this.document = document.clone();
        let mut node: Option<Rc<RefCell<Node>>> = DomSearchHelper::getNodeNoThrow(jcall!(OutputTypeData::getInstance()).NAME.clone(), jcall!(this.getRootNode()).getChildNodes());
        if node.is_some() {
            { let java_arguments = (DomNodeHelper::getTextNodeValue(node.clone()),); this.setOutputTypeName(java_arguments.0) };
        }
        if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).VIEW.clone()) {
            let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
            jcall!(stringBuffer).append(String::from("TransformInfo: "));
            if this.ownerTransformInfoInterface.is_some() {
                jcall!(stringBuffer).append(jcall!(this.ownerTransformInfoInterface).getName());
            } else {
                jcall!(stringBuffer).append(String::from("No Owner!?#@"));
            }
            jcall!(stringBuffer).append(String::from("\nConstructed with document: "));
            jcall!(stringBuffer).append(this.toString());
            { let java_receiver = this.logUtil.clone(); let java_arguments = (jcall!(stringBuffer).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("Constructor(TransformInfoInterface, Document)"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        }
        this
    }

    pub fn new2(mut transformInfoInterface: Option<Rc<RefCell<dyn TransformInfoInterface>>>, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct2(transformInfoInterface, document))))
    }
    

    pub fn new_struct3(mut transformInfoInterface: Option<Rc<RefCell<dyn TransformInfoInterface>>>, mut name: String, mut r#type: String) -> Self {
        let mut this = Self {
            logUtil: None,
            commonStrings: None,
            ownerTransformInfoInterface: None,
            document: None,
            outputTypeName: String::new(),
        };
        this.logUtil = LogUtil::getInstance();
        this.commonStrings = CommonStrings::getInstance();
        this.ownerTransformInfoInterface = transformInfoInterface.clone();
        this.createDocument();
        let mut configNameAttr: Option<Rc<RefCell<Attr>>> = { let java_receiver = this.document.clone(); let java_arguments = (jcall!(TransformInfoObjectConfigData::TransformInfoObjectConfigData::getInstance()).NAME.clone(),); let java_result = jcall!(java_receiver).createAttribute(java_arguments.0); java_result };
        jcall!(configNameAttr).setValue(name.clone());
        let mut objectConfigNode: Option<Rc<RefCell<Node>>> = jcall!({ let java_receiver = this.document.clone(); let java_arguments = (jcall!(TransformInfoObjectConfigData::TransformInfoObjectConfigData::getInstance()).NAME.clone(),); let java_result = jcall!(java_receiver).getElementsByTagName(java_arguments.0); java_result }).item(0);
        jcall!(objectConfigNode).appendChild(configNameAttr.clone());
        if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).VIEW.clone()) {
            let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
            jcall!(stringBuffer).append(String::from("TransformInfo: "));
            if this.ownerTransformInfoInterface.is_some() {
                jcall!(stringBuffer).append(jcall!(this.ownerTransformInfoInterface).getName());
            } else {
                jcall!(stringBuffer).append(String::from("No Owner!?#@"));
            }
            jcall!(stringBuffer).append(String::from("\nConstructed with document: "));
            jcall!(stringBuffer).append(this.toString());
            { let java_receiver = this.logUtil.clone(); let java_arguments = (jcall!(stringBuffer).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("Constructor(TransformInfoInterface, name, type)"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        }
        this
    }

    pub fn new3(mut transformInfoInterface: Option<Rc<RefCell<dyn TransformInfoInterface>>>, mut name: String, mut r#type: String) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct3(transformInfoInterface, name, r#type))))
    }
    

    pub fn createDocument(&mut self) {
        self.document = DomDocumentHelper::create();
        let mut objectConfigNode: Option<Rc<RefCell<Node>>> = { let java_receiver = self.document.clone(); let java_arguments = (jcall!(TransformInfoObjectConfigData::TransformInfoObjectConfigData::getInstance()).NAME.clone(),); let java_result = jcall!(java_receiver).createElement(java_arguments.0); java_result };
        jcall!(self.document).appendChild(objectConfigNode.clone());
    }
    

    pub fn getTransformInfoInterface(&mut self) -> Option<Rc<RefCell<dyn TransformInfoInterface>>> {
        return self.ownerTransformInfoInterface.clone();
    }
    

    // throws Exception
    pub fn toXmlDoc(&mut self) -> Option<Rc<RefCell<Document>>> {
        return self.document.clone();
    }
    

    pub fn setDocument(&mut self, mut document: Option<Rc<RefCell<Document>>>) {
        self.document = document.clone();
    }
    

    pub fn containsView(&mut self, mut transformInfoInterface: Option<Rc<RefCell<dyn TransformInfoInterface>>>) -> bool {
        let mut objectConfigNode: Option<Rc<RefCell<Node>>> = jcall!({ let java_receiver = self.document.clone(); let java_arguments = (jcall!(TransformInfoObjectConfigData::TransformInfoObjectConfigData::getInstance()).NAME.clone(),); let java_result = jcall!(java_receiver).getElementsByTagName(java_arguments.0); java_result }).item(0);
        let mut viewNodeList: Option<Rc<RefCell<NodeList>>> = jcall!(objectConfigNode).getChildNodes();
        let mut numberOfViews: i32 = jcall!(viewNodeList).getLength();
        for index in (0..numberOfViews) {
            let mut viewNode: Option<Rc<RefCell<Node>>> = jcall!(viewNodeList).item(index.clone());
            let mut viewAttributes: Option<Rc<RefCell<NamedNodeMap>>> = jcall!(viewNode).getAttributes();
            let mut attrNode: Option<Rc<RefCell<Attr>>> = (jcall!(viewAttributes).getNamedItem(jcall!(TransformInfoData::getInstance()).NAME.clone())).clone() /* as Attr */;
            if jcall!(jcall!(transformInfoInterface).getName()).compareTo(jcall!(attrNode).getValue()) == 0 {
                return true;
            }
        }
        return false;
    }
    

    // throws Exception
    pub fn getTemplateAttributes(&mut self) -> Option<Rc<RefCell<NamedNodeMap>>> {
        let mut componentNode: Option<Rc<RefCell<Node>>> = jcall!({ let java_receiver = self.document.clone(); let java_arguments = (jcall!(TransformInfoObjectConfigData::TransformInfoObjectConfigData::getInstance()).NAME.clone(),); let java_result = jcall!(java_receiver).getElementsByTagName(java_arguments.0); java_result }).item(0);
        return jcall!(componentNode).getAttributes();
    }
    

    // throws Exception
    pub fn getName(&mut self) -> String {
        let mut attrNode: Option<Rc<RefCell<Attr>>> = (jcall!(self.getTemplateAttributes()).getNamedItem(jcall!(TransformInfoObjectConfigData::TransformInfoObjectConfigData::getInstance()).NAME.clone())).clone() /* as Attr */;
        return jcall!(attrNode).getValue();
    }
    

    // throws Exception
    pub fn setName(&mut self, mut name: String) {
        let mut attrNode: Option<Rc<RefCell<Attr>>> = (jcall!(self.getTemplateAttributes()).getNamedItem(jcall!(TransformInfoObjectConfigData::TransformInfoObjectConfigData::getInstance()).NAME.clone())).clone() /* as Attr */;
        jcall!(attrNode).setValue(name.clone());
    }
    

    // throws Exception
    pub fn getNodeVector(&mut self, mut nodeName: String) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        let mut componentsNodeList: Option<Rc<RefCell<NodeList>>> = jcall!(self.document).getElementsByTagName(nodeName.clone());
        if componentsNodeList.is_some() && jcall!(componentsNodeList).getLength() > 0 {
            let mut viewNodeVector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = DomSearchHelper::getAllNodes(jcall!(TransformInfoData::getInstance()).NAME.clone(), jcall!(jcall!(componentsNodeList).item(0)).getChildNodes());
            let mut numberOfViews: i32 = jcall!(viewNodeVector).size();
            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).VIEW.clone()) {
                let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
                jcall!(stringBuffer).append(String::from("Number Of "));
                jcall!(stringBuffer).append(nodeName.clone());
                jcall!(stringBuffer).append(String::from(" Nodes: "));
                jcall!(stringBuffer).appendint(numberOfViews.clone());
                { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(stringBuffer).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("getNodeVector(nodename)"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
            }
            return viewNodeVector.clone();
        }
        return BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
    }
    

    // throws Exception
    pub fn getTransformDomNodes1(&mut self, mut nodeName: String) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        let mut viewVector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        let mut viewNodeVector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = self.getNodeVector(nodeName.clone());
        let mut size: i32 = jcall!(viewNodeVector).size();
        for index in (0..size) {
            let mut viewNode: Option<Rc<RefCell<Node>>> = (jcall!(viewNodeVector).get(index.clone())).clone() /* as Node */;
            jcall!(viewVector).add(crate::java_into_object(TransformInfoDomNode::new1(viewNode.clone()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::visual::transform::info::TransformInfoDomNode::TransformInfoDomNodeDyn>>)));
        }
        return viewVector.clone();
    }
    

    // throws Exception
    pub fn getTransforms1(&mut self, mut nodeName: String) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        let mut viewVector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        let mut viewNodeVector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = self.getNodeVector(nodeName.clone());
        let mut size: i32 = jcall!(viewNodeVector).size();
        for index in (0..size) {
            let mut viewNode: Option<Rc<RefCell<Node>>> = (jcall!(viewNodeVector).get(index.clone())).clone() /* as Node */;
            jcall!(viewVector).add(crate::java_into_object(jcall!(TransformInfoDomNode::new1(viewNode.clone()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::visual::transform::info::TransformInfoDomNode::TransformInfoDomNodeDyn>>)).getTransformInfoInterface()));
        }
        return viewVector.clone();
    }
    

    // throws Exception
    pub fn getTransformsGroup(&mut self, mut group: String) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).VIEW.clone()) {
            jcall!(self.logUtil).putF(format!("{}{}", String::from("Started: "), group), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("getTransformsGroup()"));
        }
        let mut viewVector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        let mut GROUP: String = jcall!(TransformInfosData::getInstance()).GROUP.clone();
        let mut componentsNodeList: Option<Rc<RefCell<NodeList>>> = jcall!(self.document).getElementsByTagName(GROUP.clone());
        if componentsNodeList.is_some() && jcall!(componentsNodeList).getLength() > 0 {
            let mut componentsNode: Option<Rc<RefCell<Node>>> = jcall!(componentsNodeList).item(0);
            let mut length: i32 = jcall!(componentsNodeList).getLength();
            for index in (0..length) {
                let mut node: Option<Rc<RefCell<Node>>> = jcall!(componentsNodeList).item(index.clone());
                let mut attributes: Option<Rc<RefCell<NamedNodeMap>>> = jcall!(node).getAttributes();
                let mut attrNode: Option<Rc<RefCell<Attr>>> = (jcall!(attributes).getNamedItem(GROUP.clone())).clone() /* as Attr */;
                let mut value: String = jcall!(attrNode).getValue();
                if (value.cmp(&group.clone()) as i32) == 0 {
                    componentsNode = node.clone();
                    break;
                }
            }
            let mut viewNodeVector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = DomSearchHelper::getAllNodes(jcall!(TransformInfoData::getInstance()).NAME.clone(), jcall!(componentsNode).getChildNodes());
            let mut numberOfViews: i32 = jcall!(viewNodeVector).size();
            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).VIEW.clone()) {
                jcall!(self.logUtil).putF(format!("{}{}", String::from("Number Of Nodes: "), numberOfViews), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("getTransformsGroup()"));
            }
            let mut size: i32 = jcall!(viewNodeVector).size();
            for index in (0..size) {
                let mut viewNode: Option<Rc<RefCell<Node>>> = (jcall!(viewNodeVector).get(index.clone())).clone() /* as Node */;
                jcall!(viewVector).add(crate::java_into_object(TransformInfoDomNode::new1(viewNode.clone()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::visual::transform::info::TransformInfoDomNode::TransformInfoDomNodeDyn>>)));
            }
        } else {
            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).VIEW.clone()) {
                if componentsNodeList.is_none() {
                    jcall!(self.logUtil).putF(String::from("Number Of Nodes: NULL"), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("getTransformsGroup()"));
                } else {
                    jcall!(self.logUtil).putF(String::from("Number Of Nodes: 0"), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("getTransformsGroup()"));
                }
            }
        }
        return viewVector.clone();
    }
    

    // throws Exception
    // Java overloaded method: Rust needs a different name
    pub fn getTransformDomNodes(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        return { let java_arguments = (jcall!(TransformInfosData::getInstance()).GROUP.clone(),); self.getTransformDomNodes1(java_arguments.0) };
    }
    

    // throws Exception
    // Java overloaded method: Rust needs a different name
    pub fn getTransforms(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        return { let java_arguments = (jcall!(TransformInfosData::getInstance()).NAME.clone(),); self.getTransforms1(java_arguments.0) };
    }
    

    // throws Exception
    pub fn getGroupTransforms(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        return { let java_arguments = (jcall!(TransformInfosData::getInstance()).GROUP.clone(),); self.getTransforms1(java_arguments.0) };
    }
    

    // throws Exception
    pub fn getParentTransforms(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        return { let java_arguments = (jcall!(TransformInfoData::getInstance()).PARENT.clone(),); self.getTransforms1(java_arguments.0) };
    }
    

    pub fn getRootNode(&mut self) -> Option<Rc<RefCell<Node>>> {
        return jcall!({ let java_receiver = self.document.clone(); let java_arguments = (jcall!(TransformInfoObjectConfigData::TransformInfoObjectConfigData::getInstance()).NAME.clone(),); let java_result = jcall!(java_receiver).getElementsByTagName(java_arguments.0); java_result }).item(0);
    }
    

    pub fn setOutputTypeName(&mut self, mut outputTypeName: String) {
        self.outputTypeName = outputTypeName.clone();
    }
    

    // throws Exception
    pub fn getOutputTypeName(&mut self) -> String {
        return self.outputTypeName.clone();
    }
    

    // throws Exception
    pub fn getInputOutputTypeName(&mut self) -> String {
        let mut node: Option<Rc<RefCell<Node>>> = DomSearchHelper::getNode(jcall!(InputOutputTypeData::getInstance()).NAME.clone(), jcall!(self.getRootNode()).getChildNodes());
        return DomNodeHelper::getTextNodeValue(node.clone());
    }
    

    // throws Exception
    pub fn getInputOutputTypeFile(&mut self) -> String {
        let mut node: Option<Rc<RefCell<Node>>> = DomSearchHelper::getNode(jcall!(InputOutputTypeData::getInstance()).FILE.clone(), jcall!(self.getRootNode()).getChildNodes());
        return DomNodeHelper::getTextNodeValue(node.clone());
    }
    

    // throws Exception
    pub fn getImportUriPath(&mut self) -> String {
        let mut node: Option<Rc<RefCell<Node>>> = DomSearchHelper::getNode(jcall!(XslData::getInstance()).ROOT_IMPORT_URI.clone(), jcall!(self.getRootNode()).getChildNodes());
        return DomNodeHelper::getTextNodeValue(node.clone());
    }
    

    pub fn toString(&mut self) -> String {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            if self.document.is_some() {
                return XmlDocumentHelper::toString(self.document.clone());
            } else {
                return None;
            }
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).VIEWERROR.clone()) {
                jcall!(self.logUtil).putF(String::from("DOM Document error"), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).TOSTRING.clone());
            }
            return None;
            }
        }
    }
}
pub trait TransformInfoObjectConfigDyn {
    fn as_TransformInfoObjectConfig(&self) -> &TransformInfoObjectConfig;
    fn as_TransformInfoObjectConfig_mut(&mut self) -> &mut TransformInfoObjectConfig;
    fn createDocument(&mut self) { TransformInfoObjectConfig::createDocument(self.as_TransformInfoObjectConfig_mut()) }
    fn getTransformInfoInterface(&mut self) -> Option<Rc<RefCell<dyn TransformInfoInterface>>> { TransformInfoObjectConfig::getTransformInfoInterface(self.as_TransformInfoObjectConfig_mut()) }
    fn toXmlDoc(&mut self) -> Option<Rc<RefCell<Document>>> { TransformInfoObjectConfig::toXmlDoc(self.as_TransformInfoObjectConfig_mut()) }
    fn setDocument(&mut self, mut document: Option<Rc<RefCell<Document>>>) { TransformInfoObjectConfig::setDocument(self.as_TransformInfoObjectConfig_mut(), document) }
    fn containsView(&mut self, mut transformInfoInterface: Option<Rc<RefCell<dyn TransformInfoInterface>>>) -> bool { TransformInfoObjectConfig::containsView(self.as_TransformInfoObjectConfig_mut(), transformInfoInterface) }
    fn getTemplateAttributes(&mut self) -> Option<Rc<RefCell<NamedNodeMap>>> { TransformInfoObjectConfig::getTemplateAttributes(self.as_TransformInfoObjectConfig_mut()) }
    fn getName(&mut self) -> String { TransformInfoObjectConfig::getName(self.as_TransformInfoObjectConfig_mut()) }
    fn setName(&mut self, mut name: String) { TransformInfoObjectConfig::setName(self.as_TransformInfoObjectConfig_mut(), name) }
    fn getNodeVector(&mut self, mut nodeName: String) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> { TransformInfoObjectConfig::getNodeVector(self.as_TransformInfoObjectConfig_mut(), nodeName) }
    fn getTransformDomNodes1(&mut self, mut nodeName: String) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> { TransformInfoObjectConfig::getTransformDomNodes1(self.as_TransformInfoObjectConfig_mut(), nodeName) }
    fn getTransforms1(&mut self, mut nodeName: String) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> { TransformInfoObjectConfig::getTransforms1(self.as_TransformInfoObjectConfig_mut(), nodeName) }
    fn getTransformsGroup(&mut self, mut group: String) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> { TransformInfoObjectConfig::getTransformsGroup(self.as_TransformInfoObjectConfig_mut(), group) }
    fn getTransformDomNodes(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> { TransformInfoObjectConfig::getTransformDomNodes(self.as_TransformInfoObjectConfig_mut()) }
    fn getTransforms(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> { TransformInfoObjectConfig::getTransforms(self.as_TransformInfoObjectConfig_mut()) }
    fn getGroupTransforms(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> { TransformInfoObjectConfig::getGroupTransforms(self.as_TransformInfoObjectConfig_mut()) }
    fn getParentTransforms(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> { TransformInfoObjectConfig::getParentTransforms(self.as_TransformInfoObjectConfig_mut()) }
    fn getRootNode(&mut self) -> Option<Rc<RefCell<Node>>> { TransformInfoObjectConfig::getRootNode(self.as_TransformInfoObjectConfig_mut()) }
    fn setOutputTypeName(&mut self, mut outputTypeName: String) { TransformInfoObjectConfig::setOutputTypeName(self.as_TransformInfoObjectConfig_mut(), outputTypeName) }
    fn getOutputTypeName(&mut self) -> String { TransformInfoObjectConfig::getOutputTypeName(self.as_TransformInfoObjectConfig_mut()) }
    fn getInputOutputTypeName(&mut self) -> String { TransformInfoObjectConfig::getInputOutputTypeName(self.as_TransformInfoObjectConfig_mut()) }
    fn getInputOutputTypeFile(&mut self) -> String { TransformInfoObjectConfig::getInputOutputTypeFile(self.as_TransformInfoObjectConfig_mut()) }
    fn getImportUriPath(&mut self) -> String { TransformInfoObjectConfig::getImportUriPath(self.as_TransformInfoObjectConfig_mut()) }
    fn toString(&mut self) -> String { TransformInfoObjectConfig::toString(self.as_TransformInfoObjectConfig_mut()) }
}
impl TransformInfoObjectConfigDyn for TransformInfoObjectConfig {
    fn as_TransformInfoObjectConfig(&self) -> &TransformInfoObjectConfig { self }
    fn as_TransformInfoObjectConfig_mut(&mut self) -> &mut TransformInfoObjectConfig { self }
}


