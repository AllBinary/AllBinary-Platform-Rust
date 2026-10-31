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
// package org.allbinary.logic.visual.theme
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::data::tree::dom::DomNodeInterface::DomNodeInterface;
use crate::org::allbinary::data::tree::dom::ModDomHelper::ModDomHelper;
use crate::org::w3c::dom::Document::Document;
use crate::org::w3c::dom::Node::Node;
use super::ThemeInterface::ThemeInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct ThemePropertiesView {
    pub themeInterface: Option<Rc<RefCell<dyn ThemeInterface>>>,
}

impl DomNodeInterface for ThemePropertiesView {
    fn toXmlNode(&mut self, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Node>>> { Self::toXmlNode(self, document) }
}

impl ThemePropertiesView {
    

    pub fn new_struct(mut themeInterface: Option<Rc<RefCell<dyn ThemeInterface>>>) -> Self {
        let mut this = Self {
            themeInterface: None,
        };
        this.themeInterface = themeInterface.clone();
        this
    }

    pub fn new(mut themeInterface: Option<Rc<RefCell<dyn ThemeInterface>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(themeInterface))))
    }
    

    // throws Exception
    pub fn toXmlNode(&mut self, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Node>>> {
        let mut node: Option<Rc<RefCell<Node>>> = ModDomHelper::createNameValueNodes(document.clone(), jcall!(ThemeData::ThemeData::getInstance()).NAME.clone(), jcall!(self.themeInterface).toHashMap());
        return node.clone();
    }
}

