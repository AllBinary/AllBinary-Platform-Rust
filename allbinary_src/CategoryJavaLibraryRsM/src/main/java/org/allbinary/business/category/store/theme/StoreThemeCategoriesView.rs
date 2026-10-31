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
// package org.allbinary.business.category.store.theme
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::business::category::CategoryInterface::CategoryInterface;
use crate::org::allbinary::data::tree::dom::DomNodeInterface::DomNodeInterface;
use crate::org::w3c::dom::Document::Document;
use crate::org::w3c::dom::Node::Node;
use super::StoreThemeCategoryInterface::StoreThemeCategoryInterface;
use super::StoreThemeCategoryView::StoreThemeCategoryView;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct StoreThemeCategoriesView {
    pub categoryInterface: Option<Rc<RefCell<dyn StoreThemeCategoryInterface>>>,
}

impl DomNodeInterface for StoreThemeCategoriesView {
    fn toXmlNode(&mut self, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Node>>> { Self::toXmlNode(self, document) }
}

impl StoreThemeCategoriesView {
    

    pub fn new_struct(mut categoryInterface: Option<Rc<RefCell<dyn StoreThemeCategoryInterface>>>) -> Self {
        let mut this = Self {
            categoryInterface: None,
        };
        this.categoryInterface = categoryInterface.clone();
        this
    }

    pub fn new(mut categoryInterface: Option<Rc<RefCell<dyn StoreThemeCategoryInterface>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(categoryInterface))))
    }
    

    // throws Exception
    pub fn toXmlNode(&mut self, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Node>>> {
        let mut node: Option<Rc<RefCell<Node>>> = jcall!(StoreThemeCategoryView::new(self.categoryInterface.clone()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::business::category::store::theme::StoreThemeCategoryView::StoreThemeCategoryViewDyn>>)).toXmlNode(document.clone());
        let mut childArray: Vec<Option<Rc<RefCell<Box<dyn std::any::Any>>>>> = jcall!(jcall!(self.categoryInterface).getChildNodes()).toArray();
        let mut size: i32 = (childArray.len() as i32).clone();
        for i in (0..size) {
            let mut childCategoryInterface: Option<Rc<RefCell<dyn CategoryInterface>>> = (childArray[(i) as usize]).clone() /* as CategoryInterface */;
            let mut childCategoryNode: Option<Rc<RefCell<Node>>> = jcall!(StoreThemeCategoriesView::new(panic!("ClassCastException: a Rust dyn CategoryInterface cannot be cast to dyn StoreThemeCategoryInterface"))).toXmlNode(document.clone());
            jcall!(node).appendChild(childCategoryNode.clone());
        }
        return node.clone();
    }
}

