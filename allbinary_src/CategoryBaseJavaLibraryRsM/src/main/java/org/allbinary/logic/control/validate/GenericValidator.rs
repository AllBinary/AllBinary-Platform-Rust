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
// package org.allbinary.logic.control.validate
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::w3c::dom::Document::Document;
use crate::org::w3c::dom::Node::Node;
use super::ValidatorBase::ValidatorBase;
use super::dom::dom;
use super::w3c::w3c;
use super::org::org;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct GenericValidator {
    pub base: ValidatorBase,
}


impl GenericValidator {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: ValidatorBase::new_struct(),
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn toValidationInfoDoc(&mut self) -> Option<Rc<RefCell<Document>>> {
        return None;
    }
    

    pub fn toValidationInfoNode(&mut self, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Node>>> {
        return None;
    }
}
impl std::ops::Deref for GenericValidator {
    type Target = ValidatorBase;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for GenericValidator {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::logic::control::validate::ValidatorBase::ValidatorBaseDyn for GenericValidator {
    fn as_ValidatorBase(&self) -> &crate::org::allbinary::logic::control::validate::ValidatorBase::ValidatorBase { &self.base }
    fn as_ValidatorBase_mut(&mut self) -> &mut crate::org::allbinary::logic::control::validate::ValidatorBase::ValidatorBase { &mut self.base }
}


