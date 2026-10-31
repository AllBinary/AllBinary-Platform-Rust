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
use crate::org::allbinary::logic::io::path::AbPath::AbPath;
use crate::org::allbinary::logic::system::security::licensing::AbeClientInformationInterface::AbeClientInformationInterface;
use crate::org::allbinary::logic::visual::transform::info::TransformInfoInterface::TransformInfoInterface;
use crate::org::w3c::dom::Document::Document;
use super::TransformInfoObjectConfigInterface::TransformInfoObjectConfigInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub trait TransformInfoObjectConfigAndManipulatorFactoryInterface {
    

    // throws Exception
    fn getInstance3(&mut self, abeClientInformation: Option<Rc<RefCell<dyn AbeClientInformationInterface>>>, transformInfoInterface: Option<Rc<RefCell<dyn TransformInfoInterface>>>, objectConfigFileAbPath: Option<Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>>) -> Option<Rc<RefCell<dyn TransformInfoObjectConfigInterface>>>;
    

    // throws Exception
    // Java overloaded method: Rust needs a different name
    fn getInstance(&mut self, abeClientInformation: Option<Rc<RefCell<dyn AbeClientInformationInterface>>>, transformInfoInterface: Option<Rc<RefCell<dyn TransformInfoInterface>>>) -> Option<Rc<RefCell<dyn TransformInfoObjectConfigInterface>>>;
    

    // throws Exception
    // Java overloaded method: Rust needs a different name
    fn getInstance3(&mut self, abeClientInformation: Option<Rc<RefCell<dyn AbeClientInformationInterface>>>, transformInfoInterface: Option<Rc<RefCell<dyn TransformInfoInterface>>>, document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<dyn TransformInfoObjectConfigInterface>>>;

    fn toString(&mut self) -> String { String::from(std::any::type_name::<Self>()) }
}


