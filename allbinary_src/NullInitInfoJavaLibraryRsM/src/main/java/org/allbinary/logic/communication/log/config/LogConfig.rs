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
// package org.allbinary.logic.communication.log.config
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::business::entry::EntryData::EntryData;
use crate::org::allbinary::data::tree::dom::DomNodeHelper::DomNodeHelper;
use crate::org::allbinary::data::tree::dom::DomSearchHelper::DomSearchHelper;
use crate::org::allbinary::logic::java::bool::BooleanUtil::BooleanUtil;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
use crate::org::w3c::dom::Node::Node;
use super::LogConfigData::LogConfigData;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct LogConfig {
    pub isEnabled: bool,
    pub name: String,
    pub description: String,
    pub path: String,
    pub fileName: String,
}


impl LogConfig {
    

    pub fn new_struct4(mut aName: String, mut aDescription: String, mut aPath: String, mut aFileName: String) -> Self {
        let mut this = Self {
            isEnabled: false,
            name: String::new(),
            description: String::new(),
            path: String::new(),
            fileName: String::new(),
        };
        this.isEnabled = false;
        this.name = aName.clone();
        this.description = aDescription.clone();
        this.path = aPath.clone();
        this.fileName = aFileName.clone();
        this
    }

    pub fn new4(mut aName: String, mut aDescription: String, mut aPath: String, mut aFileName: String) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct4(aName, aDescription, aPath, aFileName))))
    }
    

    pub fn new_struct1(mut node: Option<Rc<RefCell<Node>>>) -> Self {
        let mut this = Self {
            isEnabled: false,
            name: String::new(),
            description: String::new(),
            path: String::new(),
            fileName: String::new(),
        };
        let mut enabledValueNode: Option<Rc<RefCell<Node>>> = DomSearchHelper::getNode(jcall!(EntryData::getInstance()).ENABLE.clone(), jcall!(node).getChildNodes());
        let mut isEnabledString: String = DomNodeHelper::getTextNodeValue(enabledValueNode.clone());
        this.isEnabled = jcall!(BooleanUtil::getInstance()).getFromString(isEnabledString.clone());
        let mut logConfigData: Option<Rc<RefCell<LogConfigData>>> = LogConfigData::getInstance();
        let mut nameValueNode: Option<Rc<RefCell<Node>>> = DomSearchHelper::getNode(jcall!(logConfigData).NAME.clone(), jcall!(node).getChildNodes());
        this.name = DomNodeHelper::getTextNodeValue(nameValueNode.clone());
        let mut descriptionValueNode: Option<Rc<RefCell<Node>>> = DomSearchHelper::getNode(jcall!(logConfigData).DESCRIPTION.clone(), jcall!(node).getChildNodes());
        this.description = DomNodeHelper::getTextNodeValue(descriptionValueNode.clone());
        let mut fileValueNode: Option<Rc<RefCell<Node>>> = DomSearchHelper::getNode(jcall!(logConfigData).FILE.clone(), jcall!(node).getChildNodes());
        this.fileName = DomNodeHelper::getTextNodeValue(fileValueNode.clone());
        this
    }

    pub fn new1(mut node: Option<Rc<RefCell<Node>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct1(node))))
    }
    

    pub fn getName(&mut self) -> String {
        return self.name.clone();
    }
    

    pub fn isEnabled(&mut self) -> bool {
        return self.isEnabled.clone();
    }
    

    pub fn enable(&mut self) {
        self.isEnabled = true;
    }
    

    pub fn disable(&mut self) {
        self.isEnabled = false;
    }
    

    pub fn getDescription(&mut self) -> String {
        return self.description.clone();
    }
    

    pub fn getFileName(&mut self) -> String {
        return self.fileName.clone();
    }
    

    pub fn setName(&mut self, mut value: String) {
        self.name = value.clone();
    }
    

    pub fn setDescription(&mut self, mut value: String) {
        self.description = value.clone();
    }
    

    pub fn setPath(&mut self, mut value: String) {
        self.path = value.clone();
    }
    

    pub fn setFileName(&mut self, mut value: String) {
        self.fileName = value.clone();
    }
    

    // throws Exception
    pub fn getTypeVector(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        let mut logKeyVector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        return logKeyVector.clone();
    }
}

