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
use crate::org::allbinary::data::tree::dom::document::DomDocumentHelper::DomDocumentHelper;
use crate::org::allbinary::data::tree::dom::document::XmlDocumentHelper::XmlDocumentHelper;
use crate::org::allbinary::data::tree::dom::document::mapping::DomDocumentMappingInterface::DomDocumentMappingInterface;
use crate::org::w3c::dom::Document::Document;
use crate::org::w3c::dom::Node::Node;
use super::ContextConfigurationInterface::ContextConfigurationInterface;
use super::ContextConfigurationView::ContextConfigurationView;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct ContextConfigurationDomDocumentMapping {
    pub contextConfigurationInterface: Option<Rc<RefCell<dyn ContextConfigurationInterface>>>,
}

impl DomDocumentMappingInterface for ContextConfigurationDomDocumentMapping {
    fn toXmlDoc(&mut self) -> Option<Rc<RefCell<Document>>> { Self::toXmlDoc(self) }
}

impl ContextConfigurationDomDocumentMapping {
    

    pub fn new_struct(mut contextConfigurationInterface: Option<Rc<RefCell<dyn ContextConfigurationInterface>>>) -> Self {
        let mut this = Self {
            contextConfigurationInterface: None,
        };
        this.contextConfigurationInterface = contextConfigurationInterface.clone();
        this
    }

    pub fn new(mut contextConfigurationInterface: Option<Rc<RefCell<dyn ContextConfigurationInterface>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(contextConfigurationInterface))))
    }
    

    // throws Exception
    pub fn toXmlDoc(&mut self) -> Option<Rc<RefCell<Document>>> {
        let mut document: Option<Rc<RefCell<Document>>> = DomDocumentHelper::create();
        let mut node: Option<Rc<RefCell<Node>>> = jcall!(ContextConfigurationView::new1(self.contextConfigurationInterface.clone())).toXmlNode(document.clone());
        jcall!(document).appendChild(node.clone());
        return document.clone();
    }
    

    // throws Exception
    pub fn toDomDocumentString(&mut self) -> String {
        return XmlDocumentHelper::toString(self.toXmlDoc());
    }
}

