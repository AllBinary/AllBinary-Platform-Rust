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
// package org.allbinary.logic.java.exception
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

//NoPlatform
pub struct ExceptionUtil {
    pub PRETEND_EXCEPTION: String,
}

thread_local! { pub static ExceptionUtil_instance: Option<Rc<RefCell<ExceptionUtil>>> = ExceptionUtil::new(); }

impl ExceptionUtil {
    

    pub fn getInstance() -> Option<Rc<RefCell<ExceptionUtil>>> {
        return ExceptionUtil_instance.with(Clone::clone).clone();
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            PRETEND_EXCEPTION: String::new(),
        };
        this.PRETEND_EXCEPTION = format!("{}: {:?}", "Exception", (String::from("Not Really An Exception")));
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getStackTrace(&mut self, mut e: String) -> String {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
}

