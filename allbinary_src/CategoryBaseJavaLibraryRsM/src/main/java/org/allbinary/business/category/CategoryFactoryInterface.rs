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
// package org.allbinary.business.category
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;
use super::*;
// use crate::java::util::HashMap::HashMap;
use crate::org::allbinary::logic::io::path::AbPath::AbPath;
use crate::org::w3c::dom::Node::Node;
use super::CategoryInterface::CategoryInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub trait CategoryFactoryInterface {
    

    fn getRootInstance(&mut self) -> Option<Rc<RefCell<dyn CategoryInterface>>>;
    

    // Java overloaded method: Rust needs a different name
    fn getRootInstance1(&mut self, categoryPath: Option<Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>>) -> Option<Rc<RefCell<dyn CategoryInterface>>>;
    

    fn getRootInstanceFromNode(&mut self, node: Option<Rc<RefCell<Node>>>) -> Option<Rc<RefCell<dyn CategoryInterface>>>;
    

    fn getInstance(&mut self, rootCategoryInterface: Option<Rc<RefCell<dyn CategoryInterface>>>, parentCategoryInterface: Option<Rc<RefCell<dyn CategoryInterface>>>, name: String) -> Option<Rc<RefCell<dyn CategoryInterface>>>;
    

    // Java overloaded method: Rust needs a different name
    fn getInstance(&mut self, rootCategoryInterface: Option<Rc<RefCell<dyn CategoryInterface>>>, parentCategoryInterface: Option<Rc<RefCell<dyn CategoryInterface>>>, node: Option<Rc<RefCell<Node>>>) -> Option<Rc<RefCell<dyn CategoryInterface>>>;
    

    // Java overloaded method: Rust needs a different name
    fn getInstance(&mut self, rootCategoryInterface: Option<Rc<RefCell<dyn CategoryInterface>>>, parentCategoryInterface: Option<Rc<RefCell<dyn CategoryInterface>>>, categoryPropertiesHashMap: Option<Rc<RefCell<HashMap>>>) -> Option<Rc<RefCell<dyn CategoryInterface>>>;

    fn toString(&mut self) -> String { String::from(std::any::type_name::<Self>()) }
}


