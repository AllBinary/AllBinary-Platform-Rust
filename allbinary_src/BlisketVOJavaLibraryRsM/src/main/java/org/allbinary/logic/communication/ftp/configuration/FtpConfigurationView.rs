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
// package org.allbinary.logic.communication.ftp.configuration
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
use super::FtpConfigurationInterface::FtpConfigurationInterface;
use super::FtpConfiguration::FtpConfiguration;
use super::FtpConfigurationMapping::FtpConfigurationMapping;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct FtpConfigurationView {
    pub ftpConfigurationInterface: Option<Rc<RefCell<dyn FtpConfigurationInterface>>>,
}

impl DomNodeInterface for FtpConfigurationView {
    fn toXmlNode(&mut self, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Node>>> { Self::toXmlNode(self, document) }
}

impl FtpConfigurationView {
    

    pub fn new_struct1(mut node: Option<Rc<RefCell<Node>>>) -> Self {
        let mut this = Self {
            ftpConfigurationInterface: None,
        };
        this.setFtpConfigurationInterface((FtpConfiguration::new()).clone() /* as FtpConfigurationInterface */);
        let mut chileNodeList: Option<Rc<RefCell<NodeList>>> = jcall!(node).getChildNodes();
        let mut server: String = DomNodeHelper::getTextNodeValue2(FtpConfigurationData_SERVER.with(Clone::clone).clone(), chileNodeList.clone());
        jcall!(this.getFtpConfigurationInterface()).setServer(server.clone());
        let mut userName: String = DomNodeHelper::getTextNodeValue2(FtpConfigurationData_USERNAME.with(Clone::clone).clone(), chileNodeList.clone());
        jcall!(this.getFtpConfigurationInterface()).setUserName(userName.clone());
        let mut password: String = DomNodeHelper::getTextNodeValue2(FtpConfigurationData_PASSWORD.with(Clone::clone).clone(), chileNodeList.clone());
        jcall!(this.getFtpConfigurationInterface()).setPassword(password.clone());
        let mut path: String = DomNodeHelper::getTextNodeValue2(FtpConfigurationData_PATH.with(Clone::clone).clone(), chileNodeList.clone());
        jcall!(this.getFtpConfigurationInterface()).setPath(path.clone());
        this
    }

    pub fn new1(mut node: Option<Rc<RefCell<Node>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct1(node))))
    }
    

    pub fn new_struct1(mut ftpConfigurationInterface: Option<Rc<RefCell<dyn FtpConfigurationInterface>>>) -> Self {
        let mut this = Self {
            ftpConfigurationInterface: None,
        };
        this.setFtpConfigurationInterface(ftpConfigurationInterface.clone());
        this
    }

    pub fn new1(mut ftpConfigurationInterface: Option<Rc<RefCell<dyn FtpConfigurationInterface>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct1(ftpConfigurationInterface))))
    }
    

    // throws Exception
    pub fn toXmlNode(&mut self, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Node>>> {
        let mut ftpConfigurationMapping: Option<Rc<RefCell<FtpConfigurationMapping>>> = FtpConfigurationMapping::new(self.getFtpConfigurationInterface());
        let mut hashMap: Option<Rc<RefCell<HashMap>>> = jcall!(ftpConfigurationMapping).toHashMap();
        let mut node: Option<Rc<RefCell<Node>>> = ModDomHelper::createNameValueNodes(document.clone(), FtpConfigurationData_NAME.with(Clone::clone).clone(), hashMap.clone());
        return node.clone();
    }
    

    pub fn getFtpConfigurationInterface(&mut self) -> Option<Rc<RefCell<dyn FtpConfigurationInterface>>> {
        return self.ftpConfigurationInterface.clone();
    }
    

    pub fn setFtpConfigurationInterface(&mut self, mut ftpConfigurationInterface: Option<Rc<RefCell<dyn FtpConfigurationInterface>>>) {
        self.ftpConfigurationInterface = ftpConfigurationInterface.clone();
    }
}

