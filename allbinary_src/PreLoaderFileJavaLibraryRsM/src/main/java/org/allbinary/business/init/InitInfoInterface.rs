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
// package org.allbinary.business.init
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::logic::io::path::AbPath::AbPath;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub trait InitInfoInterface {
    

    fn isTesting(&mut self) -> bool;
    

    fn getTesting(&mut self) -> String;
    

    fn getTestHtmlPath(&mut self) -> String;
    

    fn getMainPath(&mut self) -> String;
    

    fn setTesting(&mut self, value: String);
    

    fn setTestHtmlPath(&mut self, value: Option<Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>>);
    

    fn setMainPath(&mut self, value: Option<Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>>);
    

    // throws Exception
    fn isTestingValid(&mut self, testing: String) -> bool;
    

    fn isTestHtmlPathValid(&mut self, value: Option<Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>>) -> bool;
    

    fn isMainPathValid(&mut self, value: Option<Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>>) -> bool;

    fn toString(&mut self) -> String { String::from(std::any::type_name::<Self>()) }
}


