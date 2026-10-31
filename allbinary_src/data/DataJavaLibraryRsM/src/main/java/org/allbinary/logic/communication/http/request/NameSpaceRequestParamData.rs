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
// package org.allbinary.logic.communication.http.request
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct NameSpaceRequestParamData {
}

thread_local! { pub static NameSpaceRequestParamData_NAME: String = String::from("NAMESPACE_REQUESTDATA_NAME"); }
thread_local! { pub static NameSpaceRequestParamData_VALUE: String = String::from("INPUT_VALUE"); }
thread_local! { pub static NameSpaceRequestParamData_PROPERTIES: String = String::from("_PROPERTIES_"); }
thread_local! { pub static NameSpaceRequestParamData_SEP: String = String::from("_DOT_"); }
thread_local! { pub static NameSpaceRequestParamData_PROPERTIESSEPARATOR: String = String::from("_PROPDOT_"); }
thread_local! { pub static NameSpaceRequestParamData_DOCUMENT: String = String::from("NAMESPACE_REQUESTDATA_DOCUMENT"); }

impl NameSpaceRequestParamData {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
}

