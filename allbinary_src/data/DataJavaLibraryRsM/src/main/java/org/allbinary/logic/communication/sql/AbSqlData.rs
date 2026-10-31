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
// package org.allbinary.logic.communication.sql
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

pub struct AbSqlData {
}

thread_local! { pub static AbSqlData_ANYSINGLECHARACTERMATCH: String = String::from("_"); }
thread_local! { pub static AbSqlData_ANYMULTICHARACTERMATCH: String = String::from("%"); }
thread_local! { pub static AbSqlData_MAXNUM: i32 = 19; }
thread_local! { pub static AbSqlData_MINSTRING: i32 = 1; }
thread_local! { pub static AbSqlData_MAXSTRING: i32 = 255; }
thread_local! { pub static AbSqlData_MAXBLOB: i32 = 32000; }

impl AbSqlData {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
}

