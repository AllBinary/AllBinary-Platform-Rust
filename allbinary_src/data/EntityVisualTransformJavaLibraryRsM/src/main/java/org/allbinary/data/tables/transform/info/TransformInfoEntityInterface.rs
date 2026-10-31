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
// package org.allbinary.data.tables.transform.info
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;
use super::*;
// use crate::java::util::HashMap::HashMap;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
// use crate::javax::servlet::jsp::PageContext::PageContext;
use crate::org::allbinary::data::tables::BasicDataTableInterface::BasicDataTableInterface;
use crate::org::allbinary::logic::visual::transform::info::TransformInfoInterface::TransformInfoInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub trait TransformInfoEntityInterface: BasicDataTableInterface {
    

    // throws Exception
    fn get(&mut self, name: String, propertiesHashMap: Option<Rc<RefCell<HashMap>>>, pageContext: Option<Rc<RefCell<PageContext>>>) -> Option<Rc<RefCell<dyn TransformInfoInterface>>>;
    

    // throws Exception
    fn getObjectConfigs(&mut self, storeName: String) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>;
    

    // throws Exception
    fn getNames(&mut self, storeName: String) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>;
    

    fn delete(&mut self, name: String);
    

    fn insert(&mut self, values: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>);
    

    fn update(&mut self, updatedValues: Option<Rc<RefCell<HashMap>>>);
}


