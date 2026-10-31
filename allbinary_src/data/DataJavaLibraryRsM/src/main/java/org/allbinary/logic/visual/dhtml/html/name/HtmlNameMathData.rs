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
// package org.allbinary.logic.visual.dhtml.html.name
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

pub struct HtmlNameMathData {
    pub EQUALS: String,
    pub PLUS: String,
    pub MINUS: String,
    pub DIVIDE: String,
    pub MULTIPLY: String,
}

thread_local! { pub static HtmlNameMathData_instance: Option<Rc<RefCell<HtmlNameMathData>>> = HtmlNameMathData::new(); }

impl HtmlNameMathData {
    

    pub fn getInstance() -> Option<Rc<RefCell<HtmlNameMathData>>> {
        return HtmlNameMathData_instance.with(Clone::clone).clone();
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            EQUALS: String::new(),
            PLUS: String::new(),
            MINUS: String::new(),
            DIVIDE: String::new(),
            MULTIPLY: String::new(),
        };
        this.EQUALS = String::from("_EQUALS_");
        this.PLUS = String::from("_PLUS_");
        this.MINUS = String::from("_MINUS_");
        this.DIVIDE = String::from("_DIVIDE_");
        this.MULTIPLY = String::from("_MULTIPLY_");
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
}

