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
// package org.allbinary.logic.math
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::string::CommonPhoneStrings::CommonPhoneStrings;
use crate::org::allbinary::string::CommonSeps::CommonSeps;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct MathData {
    pub EQUALS: String,
    pub PLUS: String,
    pub GREATER_THAN: String,
    pub MINUS: String,
    pub DIVIDE: String,
    pub MULTIPLY: String,
}

thread_local! { pub static MathData_instance: Option<Rc<RefCell<MathData>>> = MathData::new(); }

impl MathData {
    

    pub fn getInstance() -> Option<Rc<RefCell<MathData>>> {
        return MathData_instance.with(Clone::clone).clone();
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            EQUALS: String::new(),
            PLUS: String::new(),
            GREATER_THAN: String::new(),
            MINUS: String::new(),
            DIVIDE: String::new(),
            MULTIPLY: String::new(),
        };
        this.PLUS = String::from("+");
        this.GREATER_THAN = String::from(">");
        let mut commonSeps: Option<Rc<RefCell<CommonSeps>>> = CommonSeps::getInstance();
        this.EQUALS = jcall!(commonSeps).EQUALS.clone();
        this.MINUS = jcall!(commonSeps).DASH.clone();
        this.DIVIDE = jcall!(commonSeps).FORWARD_SLASH.clone();
        this.MULTIPLY = jcall!(CommonPhoneStrings::getInstance()).STAR.clone();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
}

