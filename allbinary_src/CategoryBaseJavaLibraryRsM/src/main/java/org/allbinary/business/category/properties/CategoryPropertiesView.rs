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
// package org.allbinary.business.category.properties
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::business::category::CategoryData::CategoryData;
use crate::org::allbinary::data::tree::dom::DomNodeInterface::DomNodeInterface;
use crate::org::w3c::dom::Document::Document;
use crate::org::w3c::dom::Element::Element;
use crate::org::w3c::dom::Node::Node;
use super::CategoryPropertiesInterface::CategoryPropertiesInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct CategoryPropertiesView {
    pub categoryPropertiesInterface: Option<Rc<RefCell<dyn CategoryPropertiesInterface>>>,
}

impl DomNodeInterface for CategoryPropertiesView {
    fn toXmlNode(&mut self, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Node>>> { Self::toXmlNode(self, document) }
}

impl CategoryPropertiesView {
    

    pub fn new_struct(mut categoryPropertiesInterface: Option<Rc<RefCell<dyn CategoryPropertiesInterface>>>) -> Self {
        let mut this = Self {
            categoryPropertiesInterface: None,
        };
        this.categoryPropertiesInterface = categoryPropertiesInterface.clone();
        this
    }

    pub fn new(mut categoryPropertiesInterface: Option<Rc<RefCell<dyn CategoryPropertiesInterface>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(categoryPropertiesInterface))))
    }
    

    // throws Exception
    pub fn toXmlNode(&mut self, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Node>>> {
        let mut categoryNode: Option<Rc<RefCell<Node>>> = jcall!(document).createElement(jcall!(CategoryData::getInstance()).NAME.clone());
        let mut newCategoryNode: Option<Rc<RefCell<Element>>> = (categoryNode).clone() /* as Element */;
        jcall!(newCategoryNode).setAttribute(jcall!(CategoryData::getInstance()).LABEL.clone(), jcall!(self.categoryPropertiesInterface).getValue());
        return categoryNode.clone();
    }
}
pub trait CategoryPropertiesViewDyn {
    fn as_CategoryPropertiesView(&self) -> &CategoryPropertiesView;
    fn as_CategoryPropertiesView_mut(&mut self) -> &mut CategoryPropertiesView;
    fn toXmlNode(&mut self, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Node>>> { CategoryPropertiesView::toXmlNode(self.as_CategoryPropertiesView_mut(), document) }
}
impl CategoryPropertiesViewDyn for CategoryPropertiesView {
    fn as_CategoryPropertiesView(&self) -> &CategoryPropertiesView { self }
    fn as_CategoryPropertiesView_mut(&mut self) -> &mut CategoryPropertiesView { self }
}


