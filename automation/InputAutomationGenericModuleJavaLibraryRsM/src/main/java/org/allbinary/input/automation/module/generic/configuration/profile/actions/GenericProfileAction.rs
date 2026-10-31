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
// package org.allbinary.input.automation.module.generic.configuration.profile.actions
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;
use super::*;
// use crate::java::util::HashMap::HashMap;
use crate::org::allbinary::data::tree::dom::DomNodeHelper::DomNodeHelper;
use crate::org::allbinary::data::tree::dom::DomNodeInterface::DomNodeInterface;
use crate::org::allbinary::data::tree::dom::DomSearchHelper::DomSearchHelper;
use crate::org::allbinary::data::tree::dom::ModDomHelper::ModDomHelper;
use crate::org::allbinary::input::automation::module::generic::configuration::profile::actions::script::GenericProfileActionScript::GenericProfileActionScript;
use crate::org::allbinary::logic::StdUtil::StdUtil;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::w3c::dom::Document::Document;
use crate::org::w3c::dom::Node::Node;
use super::GenericProfileActionJPanel::GenericProfileActionJPanel;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct GenericProfileAction {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub name: String,
    pub genericProfileActionScript: Option<Rc<RefCell<GenericProfileActionScript>>>,
    pub genericProfileActionJPanel: Option<Rc<RefCell<GenericProfileActionJPanel>>>,
}

impl DomNodeInterface for GenericProfileAction {
    fn toXmlNode(&mut self, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Node>>> { Self::toXmlNode(self, document) }
}

impl GenericProfileAction {
    

    pub fn new_struct2(mut genericProfileActionJPanel: Option<Rc<RefCell<GenericProfileActionJPanel>>>, mut node: Option<Rc<RefCell<Node>>>) -> Self {
        let mut this = Self {
            logUtil: None,
            name: String::new(),
            genericProfileActionScript: None,
            genericProfileActionJPanel: None,
        };
        this.logUtil = LogUtil::getInstance();
        let mut actionNameNode: Option<Rc<RefCell<Node>>> = DomSearchHelper::getNode(unsafe { GenericProfileActionData_NAME.clone() }.clone(), jcall!(node).getChildNodes());
        if actionNameNode.is_some() {
            this.name = DomNodeHelper::getTextNodeValue(actionNameNode.clone());
        } else {
            panic!("{:?}", format!("{}: {:?}", "Exception", (String::from("Profile Action Name Node Null"))));
        }
        this.setGenericProfileActionScript(GenericProfileActionScript::new2(this.genericProfileActionJPanel.clone(), node.clone()));
        this
    }

    pub fn new2(mut genericProfileActionJPanel: Option<Rc<RefCell<GenericProfileActionJPanel>>>, mut node: Option<Rc<RefCell<Node>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct2(genericProfileActionJPanel, node))))
    }
    

    pub fn new_struct1(mut name: String) -> Self {
        let mut this = Self {
            logUtil: None,
            name: String::new(),
            genericProfileActionScript: None,
            genericProfileActionJPanel: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.name = name.clone();
        this.setGenericProfileActionScript(GenericProfileActionScript::new1(this.genericProfileActionJPanel.clone()));
        this
    }

    pub fn new1(mut name: String) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct1(name))))
    }
    

    pub fn getName(&mut self) -> String {
        return self.name.clone();
    }
    

    pub fn setName(&mut self, mut name: String) {
        self.name = name.clone();
    }
    

    pub fn toHashMap(&mut self) -> Option<Rc<RefCell<HashMap>>> {
        let mut hashMap: Option<Rc<RefCell<HashMap>>> = jcall!(StdUtil::getInstance()).createHashMap();
        jcall!(hashMap).put(unsafe { GenericProfileActionData_NAME.clone() }.clone(), self.name.clone());
        { let java_receiver = self.logUtil.clone(); let java_arguments = (format!("{}{}", String::from("HashMap: "), jcall!(hashMap).toString()), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("toHashMap()"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        return hashMap.clone();
    }
    

    // throws Exception
    pub fn toXmlNode(&mut self, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Node>>> {
        let mut node: Option<Rc<RefCell<Node>>> = ModDomHelper::createNodeWithValueNodes(document.clone(), unsafe { GenericProfileActionData_NAME.clone() }.clone(), self.toHashMap());
        jcall!(node).appendChild(jcall!(self.getGenericProfileActionScript()).toXmlNode(document.clone()));
        return node.clone();
    }
    

    pub fn getGenericProfileActionScript(&mut self) -> Option<Rc<RefCell<GenericProfileActionScript>>> {
        return self.genericProfileActionScript.clone();
    }
    

    pub fn setGenericProfileActionScript(&mut self, mut genericProfileActionScript: Option<Rc<RefCell<GenericProfileActionScript>>>) {
        self.genericProfileActionScript = genericProfileActionScript.clone();
    }
}

