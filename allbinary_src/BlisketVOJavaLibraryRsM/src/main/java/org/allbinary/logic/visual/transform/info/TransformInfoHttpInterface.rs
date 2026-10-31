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
// use crate::javax::servlet::jsp::PageContext::PageContext;
use crate::org::allbinary::logic::communication::http::request::session::WeblisketSessionInterface::WeblisketSessionInterface;
use super::TransformInfoInterface::TransformInfoInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub trait TransformInfoHttpInterface: TransformInfoInterface {
    

    fn getWeblisketSession(&mut self) -> Option<Rc<RefCell<dyn WeblisketSessionInterface>>>;
    

    fn getPageContext(&mut self) -> Option<Rc<RefCell<PageContext>>>;
    

    fn getPropertiesHashMap(&mut self) -> Option<Rc<RefCell<HashMap>>>;
}


