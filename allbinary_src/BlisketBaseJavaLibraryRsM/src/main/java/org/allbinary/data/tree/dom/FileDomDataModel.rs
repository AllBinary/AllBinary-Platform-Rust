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
// package org.allbinary.data.tree.dom
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;
use super::*;
// use crate::java::util::HashMap::HashMap;
use crate::org::allbinary::data::tables::TableMappingInterface::TableMappingInterface;
use crate::org::allbinary::logic::io::file::AbFile::AbFile;
use crate::org::w3c::dom::Document::Document;
use crate::org::w3c::dom::Node::Node;
use super::DomNodeInterface::DomNodeInterface;
use super::FileTableMapping::FileTableMapping;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct FileDomDataModel {
    pub file: Option<Rc<RefCell<AbFile>>>,
}

impl DomNodeInterface for FileDomDataModel {
    fn toXmlNode(&mut self, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Node>>> { Self::toXmlNode(self, document) }
}

impl FileDomDataModel {
    

    pub fn new_struct(mut file: Option<Rc<RefCell<AbFile>>>) -> Self {
        let mut this = Self {
            file: None,
        };
        this.file = file.clone();
        this
    }

    pub fn new(mut file: Option<Rc<RefCell<AbFile>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(file))))
    }
    

    // throws Exception
    pub fn toXmlNode(&mut self, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Node>>> {
        let mut tableMappingInterface: Option<Rc<RefCell<dyn TableMappingInterface>>> = (FileTableMapping::new(self.file.clone())).clone() /* as TableMappingInterface */;
        let mut hashMap: Option<Rc<RefCell<HashMap>>> = jcall!(tableMappingInterface).toHashMap();
        return ModDomHelper::ModDomHelper::createNodeWithValueNodes(document.clone(), String::from("file"), hashMap.clone());
    }
}

