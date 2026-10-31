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
// package org.allbinary.logic.visual.transform.template.customizer.widgets.title
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

pub struct TitleData {
    pub NAME: String,
    pub TEXT: String,
    pub VIEWNAMEKEY: String,
    pub MAXLEN: i32,
}

thread_local! { pub static TitleData_instance: Option<Rc<RefCell<TitleData>>> = TitleData::new(); }

impl TitleData {
    

    pub fn getInstance() -> Option<Rc<RefCell<TitleData>>> {
        return TitleData_instance.with(Clone::clone).clone();
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            NAME: String::new(),
            TEXT: String::new(),
            VIEWNAMEKEY: String::new(),
            MAXLEN: 0,
        };
        this.NAME = String::from("TITLE_NAME");
        this.TEXT = String::from("TITLE_TEXT");
        this.VIEWNAMEKEY = String::from("Title");
        this.MAXLEN = 255;
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
}

