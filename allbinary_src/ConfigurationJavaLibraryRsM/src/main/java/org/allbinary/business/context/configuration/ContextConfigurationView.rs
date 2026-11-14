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
// package org.allbinary.business.context.configuration
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::data::tree::dom::DomNodeInterface::DomNodeInterface;
use crate::org::allbinary::data::tree::dom::DomSearchHelper::DomSearchHelper;
use crate::org::allbinary::logic::communication::smtp::configuration::server::EmailServerConfigurationData::EmailServerConfigurationData;
use crate::org::allbinary::logic::communication::smtp::configuration::server::EmailServerConfigurationInterface::EmailServerConfigurationInterface;
use crate::org::allbinary::logic::communication::smtp::configuration::server::EmailServerConfigurationView::EmailServerConfigurationView;
use crate::org::w3c::dom::Document::Document;
use crate::org::w3c::dom::Node::Node;
use crate::org::w3c::dom::NodeList::NodeList;
use crate::java::lang::Exception::*;
use super::ContextConfigurationInterface::ContextConfigurationInterface;
use super::ContextConfiguration::ContextConfiguration;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct ContextConfigurationView {
    pub contextConfigurationInterface: Option<Rc<RefCell<dyn ContextConfigurationInterface>>>,
}

impl DomNodeInterface for ContextConfigurationView {
    fn toXmlNode(&mut self, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Node>>> { Self::toXmlNode(self, document) }
}

impl ContextConfigurationView {

    pub fn toString(&mut self) -> String {
        String::from(std::any::type_name::<Self>())
    }
    

    pub fn new_struct1(mut node: Option<Rc<RefCell<Node>>>) -> Self {
        let mut this = Self {
            contextConfigurationInterface: None,
        };
        let mut childNodeList: Option<Rc<RefCell<NodeList>>> = jcall!(node).getChildNodes();
        this.contextConfigurationInterface = ContextConfiguration::new().map(|reference| reference as Rc<RefCell<dyn ContextConfigurationInterface>>);
        let mut emailConfigurationNode: Option<Rc<RefCell<Node>>> = DomSearchHelper::getNode(crate::org::allbinary::logic::communication::smtp::configuration::server::EmailServerConfigurationData::EmailServerConfigurationData_NAME.with(Clone::clone).clone(), childNodeList.clone());
        let mut emailServerConfigurationView: Option<Rc<RefCell<EmailServerConfigurationView>>> = EmailServerConfigurationView::new1(emailConfigurationNode.clone());
        { let java_receiver = this.contextConfigurationInterface.clone(); let java_arguments = ((jcall!(emailServerConfigurationView).getEmailConfigurationInterface()).clone(),); jcall!(java_receiver).setEmailServerConfigurationInterface(java_arguments.0); };
        this
    }

    pub fn new1(mut node: Option<Rc<RefCell<Node>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct1(node))))
    }
    

    pub fn new_struct1_ContextConfigurationInterface(mut contextConfigurationInterface: Option<Rc<RefCell<dyn ContextConfigurationInterface>>>) -> Self {
        let mut this = Self {
            contextConfigurationInterface: None,
        };
        this.contextConfigurationInterface = contextConfigurationInterface.clone();
        this
    }

    pub fn new1_ContextConfigurationInterface(mut contextConfigurationInterface: Option<Rc<RefCell<dyn ContextConfigurationInterface>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct1_ContextConfigurationInterface(contextConfigurationInterface))))
    }
    

    pub fn getContextConfigurationInterface(&mut self) -> Option<Rc<RefCell<dyn ContextConfigurationInterface>>> {
        return self.contextConfigurationInterface.clone();
    }
    

    // throws Exception
    pub fn toXmlNode(&mut self, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Node>>> {
        let mut node: Option<Rc<RefCell<Node>>> = jcall!(document).createElement({ let java_field = jcall!(ContextConfigurationData::ContextConfigurationData::getInstance()).NAME.clone(); java_field }.clone());
        let mut emailServerConfigurationView: Option<Rc<RefCell<EmailServerConfigurationView>>> = EmailServerConfigurationView::new1({ let java_argument = jcall!(self.getContextConfigurationInterface()).getEmailServerConfigurationInterface(); java_argument });
        jcall!(node).appendChild(jcall!(emailServerConfigurationView).toXmlNode(document.clone()));
        return node.clone();
    }
}

