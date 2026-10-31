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
use super::*;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
use crate::org::allbinary::business::category::hierarchy::CategoryHierarchyInterface::CategoryHierarchyInterface;
use crate::org::allbinary::business::category::properties::CategoryPropertiesInterface::CategoryPropertiesInterface;
use crate::org::allbinary::data::tables::TableMappingInterface::TableMappingInterface;
use crate::org::allbinary::logic::control::validate::ValidationInterface::ValidationInterface;
use crate::org::allbinary::logic::io::path::AbPath::AbPath;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub trait CategoryInterface: TableMappingInterface + ValidationInterface {
    

    fn getChildNodes(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>;
    

    fn addChildProperty(&mut self, categoryPropertiesInterface: Option<Rc<RefCell<dyn CategoryPropertiesInterface>>>) -> bool;
    

    fn addChild(&mut self, categoryInterface: Option<Rc<RefCell<dyn CategoryInterface>>>) -> bool;
    

    fn removeChild(&mut self, categoryInterface: Option<Rc<RefCell<dyn CategoryInterface>>>) -> bool;
    

    // throws Exception
    fn isLeaf(&mut self) -> bool;
    

    fn getProperties(&mut self) -> Option<Rc<RefCell<dyn CategoryPropertiesInterface>>>;
    

    fn setProperties(&mut self, categoryPropertiesInterface: Option<Rc<RefCell<dyn CategoryPropertiesInterface>>>);
    

    fn getHierarchy(&mut self) -> Option<Rc<RefCell<dyn CategoryHierarchyInterface>>>;
    

    fn setHierarchy(&mut self, categoryHierarchyInterface: Option<Rc<RefCell<dyn CategoryHierarchyInterface>>>);
    

    // throws Exception
    fn getPath(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>>;
    

    // throws Exception
    fn getFilePath(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>>;
    

    // throws Exception
    fn getWebAppPath(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>>;
    

    // throws Exception
    fn getRootFilePath(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>>;
    

    // throws Exception
    fn log(&mut self);
}


