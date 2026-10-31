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
// package org.allbinary.logic.visual.dhtml.html.table
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::string::CommonStrings::CommonStrings;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct HtmlTableData {
    pub LEFT: String,
    pub RIGHT: String,
    pub NONE: String,
    pub TOP: String,
    pub BOTTOM: String,
    pub TOPBOT: String,
    pub SIDES: String,
    pub ALL: String,
    pub BORDERED: String,
    pub BASIC: String,
    pub ROWS: String,
    pub FRAMES: Vec<String>,
}

thread_local! { pub static HtmlTableData_instance: Option<Rc<RefCell<HtmlTableData>>> = HtmlTableData::new(); }

impl HtmlTableData {

    pub fn new_struct() -> Self {
        let mut this = Self {
            LEFT: String::new(),
            RIGHT: String::new(),
            NONE: String::new(),
            TOP: String::new(),
            BOTTOM: String::new(),
            TOPBOT: String::new(),
            SIDES: String::new(),
            ALL: String::new(),
            BORDERED: String::new(),
            BASIC: String::new(),
            ROWS: String::new(),
            FRAMES: Vec::new(),
        };
        this.LEFT = jcall!(CommonStrings::getInstance()).LEFT.clone();
        this.RIGHT = jcall!(CommonStrings::getInstance()).RIGHT.clone();
        this.NONE = String::from("none");
        this.TOP = String::from("top");
        this.BOTTOM = String::from("bottom");
        this.TOPBOT = String::from("topbot");
        this.SIDES = String::from("sides");
        this.ALL = String::from("all");
        this.BORDERED = String::from("border");
        this.BASIC = String::from("basic");
        this.ROWS = String::from("rows");
        this.FRAMES = vec![this.NONE.clone(), this.TOP.clone(), this.BOTTOM.clone(), this.TOPBOT.clone(), this.SIDES.clone(), this.ALL.clone(), this.BORDERED.clone()];
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<HtmlTableData>>> {
        return HtmlTableData_instance.with(Clone::clone).clone();
    }
}

