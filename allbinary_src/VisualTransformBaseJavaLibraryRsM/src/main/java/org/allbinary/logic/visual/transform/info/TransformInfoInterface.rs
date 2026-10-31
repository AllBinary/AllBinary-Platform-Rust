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
// package org.allbinary.logic.visual.transform.info
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;
use super::*;
// use crate::java::util::HashMap::HashMap;
use crate::org::allbinary::data::tables::TableMappingInterface::TableMappingInterface;
use crate::org::allbinary::logic::io::path::AbPath::AbPath;
use crate::org::allbinary::logic::visual::transform::info::objectConfig::TransformInfoObjectConfigInterface::TransformInfoObjectConfigInterface;
use crate::org::w3c::dom::Document::Document;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub trait TransformInfoInterface: TableMappingInterface {
    

    fn log(&mut self) -> String;
    

    // throws Exception
    fn r#override(&mut self, hashMap: Option<Rc<RefCell<HashMap>>>);
    

    fn getName(&mut self) -> String;
    

    fn setStoreName(&mut self, value: String);
    

    fn getStoreName(&mut self) -> String;
    

    fn getObjectFile(&mut self) -> String;
    

    fn getObject(&mut self) -> Option<Rc<RefCell<Box<dyn std::any::Any>>>>;
    

    // throws Exception
    fn getTemplateFilePath(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>>;
    

    fn getTemplateFile(&mut self) -> String;
    

    fn getTemplate(&mut self) -> String;
    

    // throws Exception
    fn getDataFilePath(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>>;
    

    fn getDataFile(&mut self) -> String;
    

    // throws Exception
    fn getDataDocument(&mut self) -> Option<Rc<RefCell<Document>>>;
    

    fn setName(&mut self, value: String);
    

    fn setObjectFile(&mut self, value: String);
    

    fn setObject(&mut self, anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>>);
    

    // throws Exception
    fn setObjectConfigInterface(&mut self, transformInfoObjectConfigInterface: Option<Rc<RefCell<dyn TransformInfoObjectConfigInterface>>>);
    

    // throws Exception
    fn getObjectConfigInterface(&mut self) -> Option<Rc<RefCell<dyn TransformInfoObjectConfigInterface>>>;
    

    fn setTemplateFile(&mut self, value: String);
    

    fn setTemplate(&mut self, value: String);
    

    fn setDataFile(&mut self, value: String);
    

    fn setData(&mut self, value: String);
    

    fn setChild(&mut self);
    

    fn isChild(&mut self) -> bool;
}


