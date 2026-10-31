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
// package org.allbinary.string
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::jsinterop::annotations::JsType;
use crate::jsinterop::annotations::JsMethod;
use crate::jsinterop::annotations::JsProperty;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct CommonLabels {
    pub WIDTH_LABEL: String,
    pub HEIGHT_LABEL: String,
    pub END_LABEL: String,
    pub TOTAL_LABEL: String,
    pub INDEX_LABEL: String,
    pub START_LABEL: String,
    pub COMMAND_LABEL: String,
    pub NAME_LABEL: String,
    pub START: String,
    pub ELAPSED: String,
    pub CURRENT: String,
    pub NEXT: String,
    pub ITEM_LABEL: String,
    pub RESULT_: String,
    pub COLON_SEP: String,
}

thread_local! { pub static CommonLabels_instance: Option<Rc<RefCell<CommonLabels>>> = CommonLabels::new(); }

impl CommonLabels {

    pub fn new_struct() -> Self {
        let mut this = Self {
            WIDTH_LABEL: String::new(),
            HEIGHT_LABEL: String::new(),
            END_LABEL: String::new(),
            TOTAL_LABEL: String::new(),
            INDEX_LABEL: String::new(),
            START_LABEL: String::new(),
            COMMAND_LABEL: String::new(),
            NAME_LABEL: String::new(),
            START: String::new(),
            ELAPSED: String::new(),
            CURRENT: String::new(),
            NEXT: String::new(),
            ITEM_LABEL: String::new(),
            RESULT_: String::new(),
            COLON_SEP: String::new(),
        };
        this.WIDTH_LABEL = String::from(" Width: ");
        this.HEIGHT_LABEL = String::from(" Height: ");
        this.END_LABEL = String::from("End: ");
        this.TOTAL_LABEL = String::from("Total: ");
        this.INDEX_LABEL = String::from("index: ");
        this.START_LABEL = String::from("Start: ");
        this.COMMAND_LABEL = String::from("Command: ");
        this.NAME_LABEL = String::from("Name: ");
        this.START = this.START_LABEL.clone();
        this.ELAPSED = String::from(" Elapsed: ");
        this.CURRENT = String::from(" Current: ");
        this.NEXT = String::from("Next");
        this.ITEM_LABEL = String::from("Item: ");
        this.RESULT_ = String::from("Result: ");
        this.COLON_SEP = String::from(": ");
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<CommonLabels>>> {
        return CommonLabels_instance.with(Clone::clone).clone();
    }
}

