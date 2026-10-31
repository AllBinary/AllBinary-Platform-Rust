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
use super::*;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
use crate::org::allbinary::data::tree::dom::DomNodeHelper::DomNodeHelper;
use crate::org::allbinary::data::tree::dom::ModDomHelper::ModDomHelper;
use crate::org::allbinary::input::automation::module::generic::configuration::profile::actions::GenericProfileActions::GenericProfileActions;
use crate::org::allbinary::logic::StdUtil::StdUtil;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::w3c::dom::Document::Document;
use crate::org::w3c::dom::Node::Node;
use crate::org::w3c::dom::NodeList::NodeList;
use super::GenericProfileDataWorkerType::GenericProfileDataWorkerType;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct GenericProfile {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub name: String,
    pub vector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>,
    pub genericProfileActions: Option<Rc<RefCell<GenericProfileActions>>>,
}


impl GenericProfile {
    

    pub fn new_struct1(mut node: Option<Rc<RefCell<Node>>>) -> Self {
        let mut this = Self {
            logUtil: None,
            name: String::new(),
            vector: None,
            genericProfileActions: None,
        };
        this.logUtil = LogUtil::getInstance();
        { let java_arguments = (DomNodeHelper::getTextNodeValue(node.clone()),); this.setName(java_arguments.0) };
        this.vector = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        let mut nodeList: Option<Rc<RefCell<NodeList>>> = jcall!(node).getChildNodes();
        { let java_receiver = this.logUtil.clone(); let java_arguments = (format!("{}{}", format!("{}{}", format!("{}{}", String::from("Name: "), this.getName()), String::from(" Child Nodes: ")), jcall!(nodeList).getLength()), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("Contructor"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        for index in (0..jcall!(nodeList).getLength()) {
            let mut profileNode: Option<Rc<RefCell<Node>>> = jcall!(nodeList).item(index.clone());
            if jcall!(jcall!(profileNode).getNodeName()).compareTo(unsafe { GenericProfileDataWorkerData_NAME.clone() }.clone()) == 0 {
                let mut genericProfileDataWorkerType: Option<Rc<RefCell<dyn crate::org::allbinary::input::automation::module::generic::configuration::profile::GenericProfileDataWorkerType::GenericProfileDataWorkerTypeDyn>>> = GenericProfileDataWorkerType::getInstance(profileNode.clone());
                { let java_receiver = this.logUtil.clone(); let java_arguments = (format!("{}{}", String::from("Adding GenericProfileDataWorkerType: "), jcall!(genericProfileDataWorkerType).toString()), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("Contructor"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
                jcall!(this.vector).add(crate::java_into_object(genericProfileDataWorkerType.clone()));
            }
        }
        this
    }

    pub fn new1(mut node: Option<Rc<RefCell<Node>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct1(node))))
    }
    

    pub fn new_struct1(mut name: String) -> Self {
        let mut this = Self {
            logUtil: None,
            name: String::new(),
            vector: None,
            genericProfileActions: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.setName(name.clone());
        this.vector = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        this
    }

    pub fn new1(mut name: String) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct1(name))))
    }
    

    pub fn getGenericProfileDataWorkerTypeVector(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        return self.vector.clone();
    }
    

    pub fn add(&mut self, mut genericProfileDataWorkerType: Option<Rc<RefCell<dyn crate::org::allbinary::input::automation::module::generic::configuration::profile::GenericProfileDataWorkerType::GenericProfileDataWorkerTypeDyn>>>) {
        if !jcall!(self.vector).contains(crate::java_into_object(genericProfileDataWorkerType.clone())) {
            jcall!(self.vector).add(crate::java_into_object(genericProfileDataWorkerType.clone()));
        }
    }
    

    pub fn remove(&mut self, mut genericProfileDataWorkerType: Option<Rc<RefCell<dyn crate::org::allbinary::input::automation::module::generic::configuration::profile::GenericProfileDataWorkerType::GenericProfileDataWorkerTypeDyn>>>) {
        jcall!(self.vector).remove(crate::java_into_object(genericProfileDataWorkerType.clone()));
    }
    

    // throws Exception
    pub fn toXmlNode(&mut self, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Node>>> {
        let mut node: Option<Rc<RefCell<Node>>> = ModDomHelper::createTextNode(document.clone(), unsafe { GenericProfileData_NAME.clone() }.clone(), self.name.clone());
        let mut size: i32 = jcall!(self.vector).size();
        for index in (0..size) {
            let mut genericProfileDataWorkerType: Option<Rc<RefCell<dyn crate::org::allbinary::input::automation::module::generic::configuration::profile::GenericProfileDataWorkerType::GenericProfileDataWorkerTypeDyn>>> = { let java_object = (jcall!(self.vector).get(index.clone())).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn crate::org::allbinary::input::automation::module::generic::configuration::profile::GenericProfileDataWorkerType::GenericProfileDataWorkerTypeDyn>>>>().expect("ClassCastException").clone()) };
            jcall!(node).appendChild(jcall!(genericProfileDataWorkerType).toXmlNode(document.clone()));
        }
        return node.clone();
    }
    

    pub fn getName(&mut self) -> String {
        return self.name.clone();
    }
    

    pub fn setName(&mut self, mut name: String) {
        self.name = name.clone();
    }
    

    pub fn getGenericProfileActions(&mut self) -> Option<Rc<RefCell<GenericProfileActions>>> {
        return self.genericProfileActions.clone();
    }
    

    pub fn setGenericProfileActions(&mut self, mut genericProfileActions: Option<Rc<RefCell<GenericProfileActions>>>) {
        self.genericProfileActions = genericProfileActions.clone();
    }
}

