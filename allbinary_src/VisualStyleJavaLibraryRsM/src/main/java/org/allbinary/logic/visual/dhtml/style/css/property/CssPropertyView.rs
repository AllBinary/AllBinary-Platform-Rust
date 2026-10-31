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
// package org.allbinary.logic.visual.dhtml.style.css.property
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::data::tree::dom::DomNodeInterface::DomNodeInterface;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::org::w3c::dom::Document::Document;
use crate::org::w3c::dom::Node::Node;
use super::CssProperty::CssProperty;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct CssPropertyView {
    pub commonStrings: Option<Rc<RefCell<CommonStrings>>>,
    pub cssProperty: Option<Rc<RefCell<CssProperty>>>,
}

impl DomNodeInterface for CssPropertyView {
    fn toXmlNode(&mut self, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Node>>> { Self::toXmlNode(self, document) }
}

impl CssPropertyView {
    

    pub fn new_struct1(mut name: String) -> Self {
        let mut this = Self {
            commonStrings: None,
            cssProperty: None,
        };
        this.commonStrings = CommonStrings::getInstance();
        this.cssProperty = CssProperty::new1(name.clone());
        this
    }

    pub fn new1(mut name: String) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct1(name))))
    }
    

    pub fn new_struct1(mut node: Option<Rc<RefCell<Node>>>) -> Self {
        let mut this = Self {
            commonStrings: None,
            cssProperty: None,
        };
        this.commonStrings = CommonStrings::getInstance();
        this.cssProperty = CssProperty::new1(node.clone());
        this
    }

    pub fn new1(mut node: Option<Rc<RefCell<Node>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct1(node))))
    }
    

    pub fn getProperty(&mut self) -> Option<Rc<RefCell<CssProperty>>> {
        return self.cssProperty.clone();
    }
    

    // throws Exception
    pub fn toXmlNode(&mut self, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Node>>> {
        return jcall!(self.cssProperty).toXmlNode(document.clone());
    }
}
pub trait CssPropertyViewDyn {
    fn as_CssPropertyView(&self) -> &CssPropertyView;
    fn as_CssPropertyView_mut(&mut self) -> &mut CssPropertyView;
    fn getProperty(&mut self) -> Option<Rc<RefCell<CssProperty>>> { CssPropertyView::getProperty(self.as_CssPropertyView_mut()) }
    fn toXmlNode(&mut self, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Node>>> { CssPropertyView::toXmlNode(self.as_CssPropertyView_mut(), document) }
}
impl CssPropertyViewDyn for CssPropertyView {
    fn as_CssPropertyView(&self) -> &CssPropertyView { self }
    fn as_CssPropertyView_mut(&mut self) -> &mut CssPropertyView { self }
}


