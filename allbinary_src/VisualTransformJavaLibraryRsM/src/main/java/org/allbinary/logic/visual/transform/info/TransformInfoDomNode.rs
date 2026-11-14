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
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::w3c::dom::Attr::Attr;
use crate::org::w3c::dom::NamedNodeMap::NamedNodeMap;
use crate::org::w3c::dom::Node::Node;
use crate::org::w3c::dom::NodeList::NodeList;
use crate::java::lang::Exception::*;
use super::TransformInfoInterface::TransformInfoInterface;
use super::TransformInfoData::TransformInfoData;
use super::TransformInfo::TransformInfo;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct TransformInfoDomNode {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub transformInfoInterface: Option<Rc<RefCell<dyn TransformInfoInterface>>>,
    pub mappedName: String,
}


impl TransformInfoDomNode {

    pub fn toString(&mut self) -> String {
        String::from(std::any::type_name::<Self>())
    }
    

    pub fn new_struct1(mut node: Option<Rc<RefCell<Node>>>) -> Self {
        let mut this = Self {
            logUtil: None,
            transformInfoInterface: None,
            mappedName: String::new(),
        };
        this.logUtil = LogUtil::getInstance();
        if jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypes.clone(); java_field }).LOGGING.clone(); java_field }).contains({ let java_field = jcall!(jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypeFactory.clone(); java_field }).getInstance()).VIEW.clone(); java_field }.clone()) {
            { let java_receiver = this.logUtil.clone(); let java_arguments = (format!("{}{}", String::from("Constructing TransformInfo From TransformInfoDomNode: "), jcall!(node).getNodeName()), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("TransformInfoDomNode(Node node)"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        }
        let mut transformInfoData: Option<Rc<RefCell<TransformInfoData>>> = TransformInfoData::getInstance();
        let mut attributes: Option<Rc<RefCell<NamedNodeMap>>> = jcall!(node).getAttributes();
        let mut attrNode: Option<Rc<RefCell<Attr>>> = (jcall!(attributes).getNamedItem({ let java_field = jcall!(transformInfoData).NAME.clone(); java_field }.clone())).clone() /* as Attr */;
        let mut name: String = jcall!(attrNode).getValue();
        let mut nodeList: Option<Rc<RefCell<NodeList>>> = jcall!(node).getChildNodes();
        if jcall!(nodeList).getLength() > 1 {
            if jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypes.clone(); java_field }).LOGGING.clone(); java_field }).contains({ let java_field = jcall!(jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypeFactory.clone(); java_field }).getInstance()).VIEW.clone(); java_field }.clone()) {
                let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
                jcall!(stringBuffer).append(String::from("Constructing Complete TransformInfo Node For: "));
                jcall!(stringBuffer).append(name.clone());
                jcall!(stringBuffer).append(String::from(" with "));
                jcall!(stringBuffer).appendint(((jcall!(nodeList).getLength()) as i32));
                jcall!(stringBuffer).append(String::from(" children"));
                { let java_receiver = this.logUtil.clone(); let java_arguments = (jcall!(stringBuffer).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("TransformInfoObjectConfigComponent(Node node)"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
            }
            let mut objectFileNameNode: Option<Rc<RefCell<Node>>> = DomSearchHelper::getNode({ let java_field = jcall!(transformInfoData).OBJECTFILENAME.clone(); java_field }.clone(), nodeList.clone());
            let mut objectFileName: String = DomNodeHelper::getTextNodeValue(objectFileNameNode.clone());
            let mut objectConfigFileNameNode: Option<Rc<RefCell<Node>>> = DomSearchHelper::getNode({ let java_field = jcall!(transformInfoData).OBJECTCONFIGFILENAME.clone(); java_field }.clone(), nodeList.clone());
            let mut objectConfigFileName: String = DomNodeHelper::getTextNodeValue(objectConfigFileNameNode.clone());
            let mut templateFileNameNode: Option<Rc<RefCell<Node>>> = DomSearchHelper::getNode({ let java_field = jcall!(transformInfoData).TEMPLATEFILENAME.clone(); java_field }.clone(), nodeList.clone());
            let mut templateFileName: String = DomNodeHelper::getTextNodeValue(templateFileNameNode.clone());
            let mut dataFileNameNode: Option<Rc<RefCell<Node>>> = DomSearchHelper::getNode({ let java_field = jcall!(transformInfoData).DATAFILENAME.clone(); java_field }.clone(), nodeList.clone());
            let mut dataFileName: String = DomNodeHelper::getTextNodeValue(dataFileNameNode.clone());
            this.transformInfoInterface = (TransformInfo::new5(name.clone(), objectFileName.clone(), objectConfigFileName.clone(), templateFileName.clone(), dataFileName.clone()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::visual::transform::info::TransformInfo::TransformInfoDyn>>)).clone() /* as TransformInfoInterface */;
        } else {
            if jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypes.clone(); java_field }).LOGGING.clone(); java_field }).contains({ let java_field = jcall!(jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypeFactory.clone(); java_field }).getInstance()).VIEW.clone(); java_field }.clone()) {
                let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
                jcall!(stringBuffer).append(String::from("Constructing Partial TransformInfo Node For: "));
                jcall!(stringBuffer).append(name.clone());
                jcall!(stringBuffer).append(String::from(" with "));
                jcall!(stringBuffer).appendint(((jcall!(nodeList).getLength()) as i32));
                jcall!(stringBuffer).append(String::from(" children"));
                { let java_receiver = this.logUtil.clone(); let java_arguments = (jcall!(stringBuffer).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("TransformInfoObjectConfigComponent(Node node)"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
            }
            this.transformInfoInterface = (TransformInfo::new1(name.clone()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::visual::transform::info::TransformInfo::TransformInfoDyn>>)).clone() /* as TransformInfoInterface */;
        }
        let mut templateAttrNode: Option<Rc<RefCell<Attr>>> = (jcall!(attributes).getNamedItem({ let java_field = jcall!(transformInfoData).MAPPED.clone(); java_field }.clone())).clone() /* as Attr */;
        if templateAttrNode.is_some() {
            this.mappedName = { let java_value = (jcall!(templateAttrNode).getValue()).clone(); java_value };
        } else {
            this.mappedName = { let java_value = (jcall!(this.getTransformInfoInterface()).getName()).clone(); java_value };
        }
        this
    }

    pub fn new1(mut node: Option<Rc<RefCell<Node>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct1(node))))
    }
    

    pub fn new_struct1_TransformInfoInterface(mut transformInfoInterface: Option<Rc<RefCell<dyn TransformInfoInterface>>>) -> Self {
        let mut this = Self {
            logUtil: None,
            transformInfoInterface: None,
            mappedName: String::new(),
        };
        this.logUtil = LogUtil::getInstance();
        this.transformInfoInterface = transformInfoInterface.clone();
        this.mappedName = { let java_value = (jcall!(transformInfoInterface).getName()).clone(); java_value };
        this
    }

    pub fn new1_TransformInfoInterface(mut transformInfoInterface: Option<Rc<RefCell<dyn TransformInfoInterface>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct1_TransformInfoInterface(transformInfoInterface))))
    }
    

    pub fn new_struct2(mut transformInfoInterface: Option<Rc<RefCell<dyn TransformInfoInterface>>>, mut mappedName: String) -> Self {
        let mut this = Self {
            logUtil: None,
            transformInfoInterface: None,
            mappedName: String::new(),
        };
        this.logUtil = LogUtil::getInstance();
        this.transformInfoInterface = transformInfoInterface.clone();
        this.mappedName = mappedName.clone();
        this
    }

    pub fn new2(mut transformInfoInterface: Option<Rc<RefCell<dyn TransformInfoInterface>>>, mut mappedName: String) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct2(transformInfoInterface, mappedName))))
    }
    

    pub fn new_struct2_String_String(mut name: String, mut mappedName: String) -> Self {
        let mut this = Self {
            logUtil: None,
            transformInfoInterface: None,
            mappedName: String::new(),
        };
        this.logUtil = LogUtil::getInstance();
        this.transformInfoInterface = TransformInfo::new1(name.clone()).map(|reference| reference as Rc<RefCell<dyn TransformInfoInterface>>);
        this.mappedName = mappedName.clone();
        this
    }

    pub fn new2_String_String(mut name: String, mut mappedName: String) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct2_String_String(name, mappedName))))
    }
    

    pub fn getTransformInfoInterface(&mut self) -> Option<Rc<RefCell<dyn TransformInfoInterface>>> {
        return self.transformInfoInterface.clone();
    }
    

    pub fn getMappedName(&mut self) -> String {
        return self.mappedName.clone();
    }
    

    pub fn getReplaceKey(&mut self) -> String {
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        jcall!(stringBuffer).append(String::from("<transform:component name=\""));
        jcall!(stringBuffer).append(String::from("body"));
        jcall!(stringBuffer).append(String::from("\"/>"));
        let mut key: String = jcall!(stringBuffer).toString();
        if jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypes.clone(); java_field }).LOGGING.clone(); java_field }).contains({ let java_field = jcall!(jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypeFactory.clone(); java_field }).getInstance()).VIEW.clone(); java_field }.clone()) {
            jcall!(self.logUtil).putF(format!("{}{}", String::from("Component Key: "), key), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("getReplaceKey()"));
        }
        return key.clone();
    }
}
pub trait TransformInfoDomNodeDyn: std::any::Any {
    fn as_TransformInfoDomNode(&self) -> &TransformInfoDomNode;
    fn as_TransformInfoDomNode_mut(&mut self) -> &mut TransformInfoDomNode;
    fn getTransformInfoInterface(&mut self) -> Option<Rc<RefCell<dyn TransformInfoInterface>>> { TransformInfoDomNode::getTransformInfoInterface(self.as_TransformInfoDomNode_mut()) }
    fn getMappedName(&mut self) -> String { TransformInfoDomNode::getMappedName(self.as_TransformInfoDomNode_mut()) }
    fn getReplaceKey(&mut self) -> String { TransformInfoDomNode::getReplaceKey(self.as_TransformInfoDomNode_mut()) }
}
impl TransformInfoDomNodeDyn for TransformInfoDomNode {
    fn as_TransformInfoDomNode(&self) -> &TransformInfoDomNode { self }
    fn as_TransformInfoDomNode_mut(&mut self) -> &mut TransformInfoDomNode { self }
}


