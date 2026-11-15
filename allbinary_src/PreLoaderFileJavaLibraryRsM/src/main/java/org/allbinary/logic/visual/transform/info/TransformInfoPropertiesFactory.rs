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
// package org.allbinary.logic.visual.transform.info
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::data::tree::dom::DomNodeHelper::DomNodeHelper;
use crate::org::allbinary::data::tree::dom::DomSearchHelper::DomSearchHelper;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::string::CommonLabels::CommonLabels;
use crate::org::w3c::dom::Attr::Attr;
use crate::org::w3c::dom::NamedNodeMap::NamedNodeMap;
use crate::org::w3c::dom::Node::Node;
use crate::org::w3c::dom::NodeList::NodeList;
use crate::java::lang::Exception::*;
use super::TransformInfoData::TransformInfoData;
use super::TransformInfoProperties::TransformInfoProperties;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct TransformInfoPropertiesFactory {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
}

pub static TransformInfoPropertiesFactory_instance: crate::JavaStatic<Option<Rc<RefCell<TransformInfoPropertiesFactory>>>> = crate::JavaStatic::new(|| TransformInfoPropertiesFactory::new());

impl TransformInfoPropertiesFactory {

    pub fn toString(&mut self) -> String {
        String::from(std::any::type_name::<Self>())
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<TransformInfoPropertiesFactory>>> {
        return TransformInfoPropertiesFactory_instance.with(Clone::clone).clone();
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
    pub fn getInstance1(&mut self, mut node: Option<Rc<RefCell<Node>>>) -> Option<Rc<RefCell<TransformInfoProperties>>> {
        let mut transformInfoChildNodeList: Option<Rc<RefCell<NodeList>>> = jcall!(node).getChildNodes();
        let mut transformInfoData: Option<Rc<RefCell<TransformInfoData>>> = TransformInfoData::getInstance();
        let mut attributes: Option<Rc<RefCell<NamedNodeMap>>> = jcall!(node).getAttributes();
        let mut attrNode: Option<Rc<RefCell<Attr>>> = (jcall!(attributes).getNamedItem({ let java_field = jcall!(transformInfoData).NAME.clone(); java_field }.clone())).clone() /* as Attr */;
        let mut name: String = jcall!(attrNode).getValue();
        if jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypes.clone(); java_field }).LOGGING.clone(); java_field }).contains({ let java_field = jcall!(jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypeFactory.clone(); java_field }).getInstance()).VIEW.clone(); java_field }.clone()) {
            { let java_receiver = self.logUtil.clone(); let java_arguments = (format!("{}{}", format!("{}{}", { let java_field = jcall!(CommonLabels::getInstance()).NEXT.clone(); java_field }, String::from(" View Name: ")), name), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("toTransformInfoPropertiesHashMap()"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        }
        let mut labelNode: Option<Rc<RefCell<Node>>> = DomSearchHelper::getNodeNoThrow({ let java_field = jcall!(transformInfoData).LABEL.clone(); java_field }.clone(), transformInfoChildNodeList.clone());
        let mut label: String = DomNodeHelper::getTextNodesValue(labelNode.clone());
        if false {
            label = name.clone();
        }
        let mut descriptionNode: Option<Rc<RefCell<Node>>> = DomSearchHelper::getNode({ let java_field = jcall!(transformInfoData).DESCRIPTION.clone(); java_field }.clone(), transformInfoChildNodeList.clone());
        let mut description: String = DomNodeHelper::getTextNodesValue(descriptionNode.clone());
        let mut objectFileNode: Option<Rc<RefCell<Node>>> = DomSearchHelper::getNode({ let java_field = jcall!(transformInfoData).OBJECTFILENAME.clone(); java_field }.clone(), transformInfoChildNodeList.clone());
        let mut objectFileName: String = DomNodeHelper::getTextNodeValue(objectFileNode.clone());
        let mut objectConfigFileNode: Option<Rc<RefCell<Node>>> = DomSearchHelper::getNode({ let java_field = jcall!(transformInfoData).OBJECTCONFIGFILENAME.clone(); java_field }.clone(), transformInfoChildNodeList.clone());
        let mut objectConfigFileName: String = DomNodeHelper::getTextNodeValue(objectConfigFileNode.clone());
        let mut templateFileNode: Option<Rc<RefCell<Node>>> = DomSearchHelper::getNode({ let java_field = jcall!(transformInfoData).TEMPLATEFILENAME.clone(); java_field }.clone(), transformInfoChildNodeList.clone());
        let mut templateFileName: String = DomNodeHelper::getTextNodeValue(templateFileNode.clone());
        let mut transformInfoProperties: Option<Rc<RefCell<TransformInfoProperties>>> = TransformInfoProperties::new(name.clone(), label.clone(), description.clone(), objectFileName.clone(), objectConfigFileName.clone(), templateFileName.clone());
        return transformInfoProperties.clone();
    }
}

