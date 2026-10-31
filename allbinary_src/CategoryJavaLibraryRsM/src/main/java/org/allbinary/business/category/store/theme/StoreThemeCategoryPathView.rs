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
use crate::org::allbinary::business::category::CategoryView::CategoryView;
use crate::org::allbinary::data::tree::dom::DomNodeInterface::DomNodeInterface;
use crate::org::allbinary::logic::visual::theme::ThemeInterface::ThemeInterface;
use crate::org::allbinary::logic::visual::theme::ThemePropertiesView::ThemePropertiesView;
use crate::org::allbinary::logic::visual::theme::ThemeValidation::ThemeValidation;
use crate::org::w3c::dom::Document::Document;
use crate::org::w3c::dom::Node::Node;
use super::StoreThemeCategoryInterface::StoreThemeCategoryInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct StoreThemeCategoryPathView {
    pub base: CategoryView,
}

impl DomNodeInterface for StoreThemeCategoryPathView {
    fn toXmlNode(&mut self, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Node>>> { Self::toXmlNode(self, document) }
}

impl StoreThemeCategoryPathView {
    

    pub fn new_struct(mut categoryInterface: Option<Rc<RefCell<dyn StoreThemeCategoryInterface>>>) -> Self {
        let mut this = Self {
            base: CategoryView::new_struct((categoryInterface).clone() /* as StoreThemeCategoryInterface */),
        };
        // super(...) is in the struct literal above
        this
    }

    pub fn new(mut categoryInterface: Option<Rc<RefCell<dyn StoreThemeCategoryInterface>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(categoryInterface))))
    }
    

    // throws Exception
    pub fn toXmlNode(&mut self, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Node>>> {
        let mut node: Option<Rc<RefCell<Node>>> = self.base.toXmlNode(document.clone());
        let mut storeThemeCategoryInterface: Option<Rc<RefCell<dyn StoreThemeCategoryInterface>>> = (self.getCategoryInterface()).clone() /* as StoreThemeCategoryInterface */;
        let mut themeArray: Vec<Option<Rc<RefCell<Box<dyn std::any::Any>>>>> = jcall!(jcall!(storeThemeCategoryInterface).getThemes()).toArray();
        let mut size: i32 = (themeArray.len() as i32).clone();
        for i in (0..size) {
            let mut themeValidation: Option<Rc<RefCell<ThemeValidation>>> = (themeArray[(i) as usize]).clone() /* as ThemeValidation */;
            let mut domNodeInterface: Option<Rc<RefCell<dyn DomNodeInterface>>> = (ThemePropertiesView::new((themeValidation).clone() /* as ThemeInterface */)).clone() /* as DomNodeInterface */;
            jcall!(node).appendChild(jcall!(domNodeInterface).toXmlNode(document.clone()));
        }
        return node.clone();
    }
}
impl std::ops::Deref for StoreThemeCategoryPathView {
    type Target = CategoryView;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for StoreThemeCategoryPathView {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

pub trait StoreThemeCategoryPathViewDyn: crate::org::allbinary::business::category::CategoryView::CategoryViewDyn {
    fn as_StoreThemeCategoryPathView(&self) -> &StoreThemeCategoryPathView;
    fn as_StoreThemeCategoryPathView_mut(&mut self) -> &mut StoreThemeCategoryPathView;
}
impl StoreThemeCategoryPathViewDyn for StoreThemeCategoryPathView {
    fn as_StoreThemeCategoryPathView(&self) -> &StoreThemeCategoryPathView { self }
    fn as_StoreThemeCategoryPathView_mut(&mut self) -> &mut StoreThemeCategoryPathView { self }
}

impl crate::org::allbinary::business::category::CategoryView::CategoryViewDyn for StoreThemeCategoryPathView {
    fn as_CategoryView(&self) -> &crate::org::allbinary::business::category::CategoryView::CategoryView { &self.base }
    fn as_CategoryView_mut(&mut self) -> &mut crate::org::allbinary::business::category::CategoryView::CategoryView { &mut self.base }
    fn toXmlNode(&mut self, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Node>>> { Self::toXmlNode(self, document) }
}


