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
use crate::org::allbinary::data::tree::dom::DomSearchHelper::DomSearchHelper;
use crate::org::allbinary::data::tree::dom::document::DomDocumentHelper::DomDocumentHelper;
use crate::org::allbinary::logic::control::crypt::file::CryptFileReader::CryptFileReader;
use crate::org::allbinary::logic::io::path::AbPath::AbPath;
use crate::org::w3c::dom::Document::Document;
use crate::org::w3c::dom::Node::Node;
use super::ContextConfigurationData::ContextConfigurationData;
use super::ContextConfigurationView::ContextConfigurationView;
use super::ContextConfigurationInterface::ContextConfigurationInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct ContextConfigurationInterfaceFactory {
}

thread_local! { pub static ContextConfigurationInterfaceFactory_instance: Option<Rc<RefCell<ContextConfigurationInterfaceFactory>>> = ContextConfigurationInterfaceFactory::new(); }

impl ContextConfigurationInterfaceFactory {
    

    pub fn getInstance() -> Option<Rc<RefCell<ContextConfigurationInterfaceFactory>>> {
        return ContextConfigurationInterfaceFactory_instance.with(Clone::clone).clone();
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    // throws Exception
    // Java overloaded method: Rust needs a different name
    pub fn getInstance1(&mut self, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<dyn ContextConfigurationInterface>>> {
        let mut contextConfigurationData: Option<Rc<RefCell<ContextConfigurationData>>> = ContextConfigurationData::getInstance();
        let mut contextConfigurationNode: Option<Rc<RefCell<Node>>> = DomSearchHelper::getNode(jcall!(contextConfigurationData).NAME.clone(), jcall!(document).getChildNodes());
        return (jcall!(ContextConfigurationView::new1(contextConfigurationNode.clone())).getContextConfigurationInterface()).clone() /* as ContextConfigurationInterface */;
    }
    

    // throws Exception
    // Java overloaded method: Rust needs a different name
    pub fn getInstance1(&mut self, mut contextName: String) -> Option<Rc<RefCell<dyn ContextConfigurationInterface>>> {
        let mut abPath: Option<Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>> = ContextConfigurationPathUtil::ContextConfigurationPathUtil::getAbPath(contextName.clone());
        let mut contextConfigurationData: Option<Rc<RefCell<ContextConfigurationData>>> = ContextConfigurationData::getInstance();
        let mut documentString: String = jcall!(CryptFileReader::new(jcall!(contextConfigurationData).UNCRYPTED_EXTENSION.clone(), jcall!(contextConfigurationData).ENCRYPTED_EXTENSION.clone())).get(abPath.clone());
        let mut document: Option<Rc<RefCell<Document>>> = DomDocumentHelper::create1(documentString.clone());
        return self.getInstance1(document.clone());
    }
}

