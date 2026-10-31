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
// package views.generic.inventory
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::logic::control::search::SearchRequest::SearchRequest;
use crate::org::w3c::dom::Document::Document;
use crate::org::w3c::dom::Node::Node;
use super::InventorySearch::InventorySearch;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct AutoSearchValidationInventoryView {
    pub base: InventorySearch,
}


impl AutoSearchValidationInventoryView {
    

    pub fn new_struct(mut searchRequest: Option<Rc<RefCell<SearchRequest>>>) -> Self {
        let mut this = Self {
            base: InventorySearch::new_struct(searchRequest.clone()),
        };
        // super(...) is in the struct literal above
        this
    }

    pub fn new(mut searchRequest: Option<Rc<RefCell<SearchRequest>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(searchRequest))))
    }
    

    pub fn isValid(&mut self) -> Option<bool> {
        return true;
    }
    

    pub fn toValidationInfoDoc(&mut self) -> Option<Rc<RefCell<Document>>> {
        return None;
    }
    

    pub fn toValidationInfoNode(&mut self, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Node>>> {
        return None;
    }
    

    pub fn validationInfo(&mut self) -> String {
        return None;
    }
}
impl std::ops::Deref for AutoSearchValidationInventoryView {
    type Target = InventorySearch;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for AutoSearchValidationInventoryView {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::views::generic::inventory::InventorySearch::InventorySearchDyn for AutoSearchValidationInventoryView {
    fn as_InventorySearch(&self) -> &crate::views::generic::inventory::InventorySearch::InventorySearch { &self.base }
    fn as_InventorySearch_mut(&mut self) -> &mut crate::views::generic::inventory::InventorySearch::InventorySearch { &mut self.base }
}


