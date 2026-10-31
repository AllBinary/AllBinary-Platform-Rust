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
// package org.allbinary.business.quoterequest
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

pub struct QuoteRequestData {
    pub ID: String,
    pub PROJECT_INFO: String,
    pub CUSTOMER_COMMENTS: String,
    pub BUDGET: String,
    pub TIMEFRAME: String,
    pub COMMENTS: String,
}

thread_local! { pub static QuoteRequestData_instance: Option<Rc<RefCell<QuoteRequestData>>> = QuoteRequestData::new(); }

impl QuoteRequestData {
    

    pub fn getInstance() -> Option<Rc<RefCell<QuoteRequestData>>> {
        return QuoteRequestData_instance.with(Clone::clone).clone();
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            ID: String::new(),
            PROJECT_INFO: String::new(),
            CUSTOMER_COMMENTS: String::new(),
            BUDGET: String::new(),
            TIMEFRAME: String::new(),
            COMMENTS: String::new(),
        };
        this.ID = String::from("QUOTEREQUEST_ID");
        this.PROJECT_INFO = String::from("QUOTEREQUEST_PROJECT_INFO");
        this.CUSTOMER_COMMENTS = String::from("QUOTEREQUEST_CUSTOMER_COMMENTS");
        this.BUDGET = String::from("QUOTEREQUEST_BUDGET");
        this.TIMEFRAME = String::from("QUOTEREQUEST_TIMEFRAME");
        this.COMMENTS = String::from("QUOTEREQUEST_COMMENTS");
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
}

