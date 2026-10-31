// 
// AllBinary Open License Version 1 
// Copyright (c) 2025 AllBinary 
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
// package org.apache.xmlrpc
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
use crate::org::allbinary::logic::NullUtil::NullUtil;
use super::XmlRpcHandler::XmlRpcHandler;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct NullXmlRpcHandler {
}

thread_local! { pub static NullXmlRpcHandler_NULL_XML_RPC_HANDLER: Option<Rc<RefCell<NullXmlRpcHandler>>> = NullXmlRpcHandler::new(); }
impl XmlRpcHandler for NullXmlRpcHandler {
    fn execute(&mut self, mut method: String, mut params: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) -> Option<Rc<RefCell<Box<dyn std::any::Any>>>> { Self::execute(self, method, params) }
}

impl NullXmlRpcHandler {

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    // throws Exception
    pub fn execute(&mut self, mut method: String, mut params: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) -> Option<Rc<RefCell<Box<dyn std::any::Any>>>> {
        return jcall!(NullUtil::getInstance()).NULL_OBJECT.clone();
    }
}

