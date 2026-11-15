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

pub static NameSpaceRequestParamData_NAME: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("NAMESPACE_REQUESTDATA_NAME"));
pub static NameSpaceRequestParamData_VALUE: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("INPUT_VALUE"));
pub static NameSpaceRequestParamData_PROPERTIES: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("_PROPERTIES_"));
pub static NameSpaceRequestParamData_SEP: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("_DOT_"));
pub static NameSpaceRequestParamData_PROPERTIESSEPARATOR: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("_PROPDOT_"));
pub static NameSpaceRequestParamData_DOCUMENT: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("NAMESPACE_REQUESTDATA_DOCUMENT"));

impl NameSpaceRequestParamData {

    pub fn toString(&mut self) -> String {
        String::from(std::any::type_name::<Self>())
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
}

