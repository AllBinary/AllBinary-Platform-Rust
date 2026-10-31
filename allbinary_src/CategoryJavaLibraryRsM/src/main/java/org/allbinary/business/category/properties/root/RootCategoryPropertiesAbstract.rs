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
// package org.allbinary.business.category.properties.root
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;
use super::*;
// use crate::java::util::HashMap::HashMap;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
use crate::org::allbinary::business::category::CategoryData::CategoryData;
use crate::org::allbinary::business::category::hierarchy::CategoryHierarchyInterface::CategoryHierarchyInterface;
use crate::org::allbinary::logic::StdUtil::StdUtil;
use crate::org::allbinary::logic::io::path::AbPath::AbPath;
use crate::org::allbinary::logic::io::path::AbPathData::AbPathData;
use crate::org::w3c::dom::Document::Document;
use crate::org::w3c::dom::Node::Node;
use super::RootCategoryPropertiesInterface::RootCategoryPropertiesInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct RootCategoryPropertiesAbstract {
    pub category: String,
    pub fileName: String,
}

// impl RootCategoryPropertiesInterface for RootCategoryPropertiesAbstract with the methods of the trait

impl RootCategoryPropertiesAbstract {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            category: String::new(),
            fileName: String::new(),
        };
        this.category = jcall!(CategoryData::getInstance()).ROOTCATEGORY.clone();
        this.fileName = format!("{}{}", format!("{}{}", this.category, jcall!(AbPathData::getInstance()).EXTENSION_SEP), jcall!(CategoryData::getInstance()).UNCRYPTED_EXTENSION);
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getKey(&mut self) -> Option<Rc<RefCell<Box<dyn std::any::Any>>>> {
        return (self.getValue()).clone() /* as Any */;
    }
    

    pub fn getValue(&mut self) -> String {
        return self.category.clone();
    }
    

    // throws Exception
    pub fn setValue(&mut self, mut value: String) {
        panic!("{:?}", format!("{}: {:?}", "Exception", (String::from("No Value Allowed"))));
    }
    

    // throws Exception
    pub fn setRootFilePath(&mut self, mut value: Option<Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>>) {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    // throws Exception
    // Java overloaded method: Rust needs a different name
    pub fn setRootFilePath(&mut self, mut value: String) {
        panic!("{:?}", format!("{}: {:?}", "Exception", (String::from("No Value Allowed"))));
    }
    

    // throws Exception
    pub fn getRootFilePath(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>> {
        panic!("{:?}", format!("{}: {:?}", "Exception", (String::from("No Value Allowed"))));
    }
    

    // throws Exception
    pub fn isRealRoot(&mut self) -> bool {
        panic!("{:?}", format!("{}: {:?}", "Exception", (String::from("No A Real Root"))));
    }
    

    // throws Exception
    pub fn isRoot(&mut self) -> bool {
        panic!("{:?}", format!("{}: {:?}", "Exception", (String::from("Root but not implemented"))));
    }
    

    // throws Exception
    pub fn getWebAppPath(&mut self, mut categoryHierarchyInterface: Option<Rc<RefCell<dyn CategoryHierarchyInterface>>>) -> String {
        panic!("{:?}", format!("{}: {:?}", "Exception", (String::from("Not Root"))));
    }
    

    pub fn getPath(&mut self, mut categoryHierarchyInterface: Option<Rc<RefCell<dyn CategoryHierarchyInterface>>>) -> Option<Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>> {
        return AbPath::createAbPath();
    }
    

    pub fn getFileName(&mut self) -> String {
        return self.fileName.clone();
    }
    

    pub fn isValid(&mut self) -> Option<bool> {
        return true;
    }
    

    pub fn toHashMap(&mut self) -> Option<Rc<RefCell<HashMap>>> {
        let mut categoryHashMap: Option<Rc<RefCell<HashMap>>> = jcall!(StdUtil::getInstance()).createHashMap();
        jcall!(categoryHashMap).put(jcall!(CategoryData::getInstance()).NAME.clone(), self.category.clone());
        return categoryHashMap.clone();
    }
    

    pub fn toVector(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        let mut categoryVector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        jcall!(categoryVector).add(crate::java_into_object(self.category.clone()));
        return categoryVector.clone();
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
        return None;
    }
}

