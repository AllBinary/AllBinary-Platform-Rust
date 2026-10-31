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
use crate::org::allbinary::logic::control::validate::ValidationInterface::ValidationInterface;
use crate::org::w3c::dom::Document::Document;
use crate::org::w3c::dom::Node::Node;
use super::StoreThemeCategoryView::StoreThemeCategoryView;
use super::StoreThemeCategoryInterface::StoreThemeCategoryInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct StoreThemeCategoryValidationView {
    pub base: StoreThemeCategoryView,
}

impl ValidationInterface for StoreThemeCategoryValidationView {
    fn isValid(&mut self) -> Option<bool> { Self::isValid(self) }
    fn toValidationInfoNode(&mut self, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Node>>> { Self::toValidationInfoNode(self, document) }
    fn toValidationInfoDoc(&mut self) -> Option<Rc<RefCell<Document>>> { Self::toValidationInfoDoc(self) }
    fn validationInfo(&mut self) -> String { Self::validationInfo(self) }
}

impl StoreThemeCategoryValidationView {
    

    pub fn new_struct(mut categoryInterface: Option<Rc<RefCell<dyn StoreThemeCategoryInterface>>>) -> Self {
        let mut this = Self {
            base: StoreThemeCategoryView::new_struct((categoryInterface).clone() /* as StoreThemeCategoryInterface */),
        };
        // super(...) is in the struct literal above
        this
    }

    pub fn new(mut categoryInterface: Option<Rc<RefCell<dyn StoreThemeCategoryInterface>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(categoryInterface))))
    }
    

    // throws Exception
    pub fn isValid(&mut self) -> Option<bool> {
        return true;
    }
    

    // throws Exception
    pub fn toValidationInfoDoc(&mut self) -> Option<Rc<RefCell<Document>>> {
        return None;
    }
    

    // throws Exception
    pub fn toValidationInfoNode(&mut self, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Node>>> {
        return None;
    }
    

    // throws Exception
    pub fn validationInfo(&mut self) -> String {
        return String::from("Error");
    }
}
impl std::ops::Deref for StoreThemeCategoryValidationView {
    type Target = StoreThemeCategoryView;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for StoreThemeCategoryValidationView {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::business::category::store::theme::StoreThemeCategoryView::StoreThemeCategoryViewDyn for StoreThemeCategoryValidationView {
    fn as_StoreThemeCategoryView(&self) -> &crate::org::allbinary::business::category::store::theme::StoreThemeCategoryView::StoreThemeCategoryView { &self.base }
    fn as_StoreThemeCategoryView_mut(&mut self) -> &mut crate::org::allbinary::business::category::store::theme::StoreThemeCategoryView::StoreThemeCategoryView { &mut self.base }
}
impl crate::org::allbinary::business::category::CategoryView::CategoryViewDyn for StoreThemeCategoryValidationView {
    fn as_CategoryView(&self) -> &crate::org::allbinary::business::category::CategoryView::CategoryView { &self.base.base }
    fn as_CategoryView_mut(&mut self) -> &mut crate::org::allbinary::business::category::CategoryView::CategoryView { &mut self.base.base }
}


