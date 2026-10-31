// 
// AllBinary Open License Version 1 
// Copyright (c) 2025 AllBinary 
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
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::org::w3c::dom::Document::Document;
use crate::org::w3c::dom::Node::Node;
use super::ValidationInterface::ValidationInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct Validation {
    pub commonStrings: Option<Rc<RefCell<CommonStrings>>>,
}

impl ValidationInterface for Validation {
    fn isValid(&mut self) -> Option<bool> { Self::isValid(self) }
    fn toValidationInfoNode(&mut self, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Node>>> { Self::toValidationInfoNode(self, document) }
    fn toValidationInfoDoc(&mut self) -> Option<Rc<RefCell<Document>>> { Self::toValidationInfoDoc(self) }
    fn validationInfo(&mut self) -> String { Self::validationInfo(self) }
}

impl Validation {

    pub fn new_struct() -> Self {
        let mut this = Self {
            commonStrings: None,
        };
        this.commonStrings = CommonStrings::getInstance();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    // throws Exception
    pub fn isValid(&mut self) -> Option<bool> {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    // throws Exception
    pub fn toValidationInfoNode(&mut self, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Node>>> {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    // throws Exception
    pub fn toValidationInfoDoc(&mut self) -> Option<Rc<RefCell<Document>>> {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    // throws Exception
    pub fn validationInfo(&mut self) -> String {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
}
pub trait ValidationDyn {
    fn as_Validation(&self) -> &Validation;
    fn as_Validation_mut(&mut self) -> &mut Validation;
    fn isValid(&mut self) -> Option<bool> { Validation::isValid(self.as_Validation_mut()) }
    fn toValidationInfoNode(&mut self, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Node>>> { Validation::toValidationInfoNode(self.as_Validation_mut(), document) }
    fn toValidationInfoDoc(&mut self) -> Option<Rc<RefCell<Document>>> { Validation::toValidationInfoDoc(self.as_Validation_mut()) }
    fn validationInfo(&mut self) -> String { Validation::validationInfo(self.as_Validation_mut()) }
}
impl ValidationDyn for Validation {
    fn as_Validation(&self) -> &Validation { self }
    fn as_Validation_mut(&mut self) -> &mut Validation { self }
}


