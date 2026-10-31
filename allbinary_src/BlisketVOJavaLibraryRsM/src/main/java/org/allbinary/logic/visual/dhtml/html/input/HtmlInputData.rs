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
// package org.allbinary.logic.visual.dhtml.html.input
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

pub struct HtmlInputData {
    pub TEXT: String,
    pub RADIO: String,
    pub CHECKBOX: String,
    pub HIDDEN: String,
    pub IMAGE: String,
    pub PASSWORD: String,
    pub RESET: String,
    pub SUBMIT: String,
    pub TYPES: Vec<String>,
}

thread_local! { pub static HtmlInputData_instance: Option<Rc<RefCell<HtmlInputData>>> = HtmlInputData::new(); }

impl HtmlInputData {

    pub fn new_struct() -> Self {
        let mut this = Self {
            TEXT: String::new(),
            RADIO: String::new(),
            CHECKBOX: String::new(),
            HIDDEN: String::new(),
            IMAGE: String::new(),
            PASSWORD: String::new(),
            RESET: String::new(),
            SUBMIT: String::new(),
            TYPES: Vec::new(),
        };
        this.TEXT = String::from("text");
        this.RADIO = String::from("radio");
        this.CHECKBOX = String::from("checkbox");
        this.HIDDEN = String::from("hidden");
        this.IMAGE = String::from("image");
        this.PASSWORD = String::from("password");
        this.RESET = String::from("reset");
        this.SUBMIT = String::from("submit");
        this.TYPES = vec![this.TEXT.clone(), this.RADIO.clone(), this.CHECKBOX.clone(), this.HIDDEN.clone(), this.IMAGE.clone(), this.PASSWORD.clone(), this.RADIO.clone(), this.RESET.clone(), this.SUBMIT.clone()];
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<HtmlInputData>>> {
        return HtmlInputData_instance.with(Clone::clone).clone();
    }
}

