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
// package org.allbinary.logic.visual.transform.info.objectConfig
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
use crate::org::allbinary::data::tree::dom::document::mapping::DomDocumentMappingInterface::DomDocumentMappingInterface;
use crate::org::allbinary::logic::visual::transform::info::TransformInfoInterface::TransformInfoInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub trait TransformInfoObjectConfigInterface: DomDocumentMappingInterface {
    

    fn containsView(&mut self, transformInfoInterface: Option<Rc<RefCell<dyn TransformInfoInterface>>>) -> bool;
    

    // throws Exception
    fn getName(&mut self) -> String;
    

    // throws Exception
    fn getTransformsGroup(&mut self, group: String) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>;
    

    // throws Exception
    fn getTransforms(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>;
    

    // throws Exception
    fn getParentTransforms(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>;
    

    // throws Exception
    fn getGroupTransforms(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>;
    

    // throws Exception
    fn getOutputTypeName(&mut self) -> String;
    

    fn setOutputTypeName(&mut self, value: String);
    

    // throws Exception
    fn getInputOutputTypeName(&mut self) -> String;
    

    // throws Exception
    fn getInputOutputTypeFile(&mut self) -> String;
    

    // throws Exception
    fn getImportUriPath(&mut self) -> String;
    

    fn toString(&mut self) -> String;
}


