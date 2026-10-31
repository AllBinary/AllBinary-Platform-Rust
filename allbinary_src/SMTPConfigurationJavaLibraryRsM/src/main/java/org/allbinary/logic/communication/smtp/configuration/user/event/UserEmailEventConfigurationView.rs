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
// package org.allbinary.logic.communication.smtp.configuration.user.event
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;
use super::*;
// use crate::java::util::HashMap::HashMap;
use crate::org::allbinary::data::tree::dom::DomNodeHelper::DomNodeHelper;
use crate::org::allbinary::data::tree::dom::DomNodeInterface::DomNodeInterface;
use crate::org::allbinary::data::tree::dom::ModDomHelper::ModDomHelper;
use crate::org::allbinary::logic::StdUtil::StdUtil;
use crate::org::w3c::dom::Document::Document;
use crate::org::w3c::dom::Node::Node;
use crate::org::w3c::dom::NodeList::NodeList;
use super::UserEmailEventConfigurationInterface::UserEmailEventConfigurationInterface;
use super::UserEmailEventConfiguration::UserEmailEventConfiguration;
use super::UserEmailEventConfigurationMapping::UserEmailEventConfigurationMapping;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct UserEmailEventConfigurationView {
    pub userEmailEventConfigurationInterface: Option<Rc<RefCell<dyn UserEmailEventConfigurationInterface>>>,
}

impl DomNodeInterface for UserEmailEventConfigurationView {
    fn toXmlNode(&mut self, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Node>>> { Self::toXmlNode(self, document) }
}

impl UserEmailEventConfigurationView {
    

    pub fn new_struct1(mut node: Option<Rc<RefCell<Node>>>) -> Self {
        let mut this = Self {
            userEmailEventConfigurationInterface: None,
        };
        let mut childNodeList: Option<Rc<RefCell<NodeList>>> = jcall!(node).getChildNodes();
        let mut hashMap: Option<Rc<RefCell<HashMap>>> = jcall!(StdUtil::getInstance()).createHashMap();
        let mut name: String = DomNodeHelper::getTextNodeValue2(UserEmailEventConfigurationData_NAME.with(Clone::clone).clone(), childNodeList.clone());
        jcall!(hashMap).put(UserEmailEventConfigurationData_NAME.with(Clone::clone).clone(), name.clone());
        let mut eventListenerClassPath: String = DomNodeHelper::getTextNodeValue2(UserEmailEventConfigurationData_LISTENER_CLASSPATH.with(Clone::clone).clone(), childNodeList.clone());
        jcall!(hashMap).put(UserEmailEventConfigurationData_LISTENER_CLASSPATH.with(Clone::clone).clone(), eventListenerClassPath.clone());
        this.setUserEmailEventConfigurationInterface((UserEmailEventConfiguration::new1(hashMap.clone())).clone() /* as UserEmailEventConfigurationInterface */);
        this
    }

    pub fn new1(mut node: Option<Rc<RefCell<Node>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct1(node))))
    }
    

    pub fn new_struct1(mut userEmailEventConfigurationInterface: Option<Rc<RefCell<dyn UserEmailEventConfigurationInterface>>>) -> Self {
        let mut this = Self {
            userEmailEventConfigurationInterface: None,
        };
        this.setUserEmailEventConfigurationInterface(userEmailEventConfigurationInterface.clone());
        this
    }

    pub fn new1(mut userEmailEventConfigurationInterface: Option<Rc<RefCell<dyn UserEmailEventConfigurationInterface>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct1(userEmailEventConfigurationInterface))))
    }
    

    // throws Exception
    pub fn toXmlNode(&mut self, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Node>>> {
        let mut userEmailEventConfigurationMapping: Option<Rc<RefCell<UserEmailEventConfigurationMapping>>> = UserEmailEventConfigurationMapping::new(self.getUserEmailEventConfigurationInterface());
        let mut hashMap: Option<Rc<RefCell<HashMap>>> = jcall!(userEmailEventConfigurationMapping).toHashMap();
        let mut node: Option<Rc<RefCell<Node>>> = ModDomHelper::createNameValueNodes(document.clone(), UserEmailEventConfigurationData_NAME.with(Clone::clone).clone(), hashMap.clone());
        return node.clone();
    }
    

    pub fn getUserEmailEventConfigurationInterface(&mut self) -> Option<Rc<RefCell<dyn UserEmailEventConfigurationInterface>>> {
        return self.userEmailEventConfigurationInterface.clone();
    }
    

    pub fn setUserEmailEventConfigurationInterface(&mut self, mut userEmailEventConfigurationInterface: Option<Rc<RefCell<dyn UserEmailEventConfigurationInterface>>>) {
        self.userEmailEventConfigurationInterface = userEmailEventConfigurationInterface.clone();
    }
}

