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
// package org.allbinary.logic.communication.smtp.configuration.server
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;
use super::*;
// use crate::java::util::HashMap::HashMap;
use crate::org::allbinary::data::tree::dom::DomNodeHelper::DomNodeHelper;
use crate::org::allbinary::data::tree::dom::DomNodeInterface::DomNodeInterface;
use crate::org::allbinary::data::tree::dom::ModDomHelper::ModDomHelper;
use crate::org::w3c::dom::Document::Document;
use crate::org::w3c::dom::Node::Node;
use crate::org::w3c::dom::NodeList::NodeList;
use super::EmailServerConfigurationInterface::EmailServerConfigurationInterface;
use super::EmailServerConfiguration::EmailServerConfiguration;
use super::EmailServerConfigurationMapping::EmailServerConfigurationMapping;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct EmailServerConfigurationView {
    pub emailServerConfigurationInterface: Option<Rc<RefCell<dyn EmailServerConfigurationInterface>>>,
}

impl DomNodeInterface for EmailServerConfigurationView {
    fn toXmlNode(&mut self, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Node>>> { Self::toXmlNode(self, document) }
}

impl EmailServerConfigurationView {
    

    pub fn new_struct1(mut node: Option<Rc<RefCell<Node>>>) -> Self {
        let mut this = Self {
            emailServerConfigurationInterface: None,
        };
        let mut childNodeList: Option<Rc<RefCell<NodeList>>> = jcall!(node).getChildNodes();
        let mut server: String = DomNodeHelper::getTextNodeValue2(EmailServerConfigurationData_SERVER.with(Clone::clone).clone(), childNodeList.clone());
        let mut accountName: String = DomNodeHelper::getTextNodeValue2(EmailServerConfigurationData_ACCOUNT.with(Clone::clone).clone(), childNodeList.clone());
        let mut password: String = DomNodeHelper::getTextNodeValue2(EmailServerConfigurationData_PASSWORD.with(Clone::clone).clone(), childNodeList.clone());
        this.emailServerConfigurationInterface = (EmailServerConfiguration::new3(accountName.clone(), password.clone(), server.clone())).clone() /* as EmailServerConfigurationInterface */;
        this
    }

    pub fn new1(mut node: Option<Rc<RefCell<Node>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct1(node))))
    }
    

    pub fn new_struct1(mut emailServerConfigurationInterface: Option<Rc<RefCell<dyn EmailServerConfigurationInterface>>>) -> Self {
        let mut this = Self {
            emailServerConfigurationInterface: None,
        };
        this.emailServerConfigurationInterface = emailServerConfigurationInterface.clone();
        this
    }

    pub fn new1(mut emailServerConfigurationInterface: Option<Rc<RefCell<dyn EmailServerConfigurationInterface>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct1(emailServerConfigurationInterface))))
    }
    

    pub fn getEmailConfigurationInterface(&mut self) -> Option<Rc<RefCell<dyn EmailServerConfigurationInterface>>> {
        return self.emailServerConfigurationInterface.clone();
    }
    

    // throws Exception
    pub fn toXmlNode(&mut self, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Node>>> {
        let mut emailConfigurationMapping: Option<Rc<RefCell<EmailServerConfigurationMapping>>> = EmailServerConfigurationMapping::new(self.emailServerConfigurationInterface.clone());
        let mut hashMap: Option<Rc<RefCell<HashMap>>> = jcall!(emailConfigurationMapping).toHashMap();
        let mut node: Option<Rc<RefCell<Node>>> = ModDomHelper::createNodeWithValueNodes(document.clone(), EmailServerConfigurationData_NAME.with(Clone::clone).clone(), hashMap.clone());
        return node.clone();
    }
}

