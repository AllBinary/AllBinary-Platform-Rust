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
// package org.allbinary.logic.communication.smtp
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

pub struct EmailData {
}

thread_local! { pub static EmailData_SERVER: String = String::from("EMAIL_SERVER"); }
thread_local! { pub static EmailData_NAME: String = String::from("EMAIL_NAME"); }
thread_local! { pub static EmailData_FROM: String = String::from("EMAIL_FROM"); }
thread_local! { pub static EmailData_TO: String = String::from("EMAIL_TO"); }
thread_local! { pub static EmailData_CC: String = String::from("EMAIL_CC"); }
thread_local! { pub static EmailData_BCC: String = String::from("EMAIL_BCC"); }
thread_local! { pub static EmailData_SUBJECT: String = String::from("EMAIL_SUBJECT"); }
thread_local! { pub static EmailData_CONTENT: String = String::from("EMAIL_CONTENT"); }

impl EmailData {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
}

