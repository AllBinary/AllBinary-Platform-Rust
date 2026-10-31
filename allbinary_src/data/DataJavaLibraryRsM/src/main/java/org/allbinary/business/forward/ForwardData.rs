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
// package org.allbinary.business.forward
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

pub struct ForwardData {
    pub PAGE: String,
    pub NAME: String,
    pub VALUE: String,
}

thread_local! { pub static ForwardData_instance: Option<Rc<RefCell<ForwardData>>> = ForwardData::new(); }

impl ForwardData {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            PAGE: String::new(),
            NAME: String::new(),
            VALUE: String::new(),
        };
        this.PAGE = String::from("FORWARD_PAGE");
        this.NAME = String::from("FORWARD_NAME");
        this.VALUE = String::from("FORWARD_VALUE");
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<ForwardData>>> {
        return ForwardData_instance.with(Clone::clone).clone();
    }
}

