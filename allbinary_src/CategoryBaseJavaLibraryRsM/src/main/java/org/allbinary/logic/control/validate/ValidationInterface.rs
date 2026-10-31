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

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub trait ValidationInterface {
    

    // throws Exception
    fn isValid(&mut self) -> Option<bool>;
    

    // throws Exception
    fn toValidationInfoNode(&mut self, document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Node>>>;
    

    // throws Exception
    fn toValidationInfoDoc(&mut self) -> Option<Rc<RefCell<Document>>>;
    

    // throws Exception
    fn validationInfo(&mut self) -> String;

    fn toString(&mut self) -> String { String::from(std::any::type_name::<Self>()) }
}


