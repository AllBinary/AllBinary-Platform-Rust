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
// package org.allbinary.logic.communication.http
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use super::AcceptableResponse::AcceptableResponse;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct AcceptableResponseFactory {
    pub HTML: Option<Rc<RefCell<AcceptableResponse>>>,
    pub WML: Option<Rc<RefCell<AcceptableResponse>>>,
    pub VXML: Option<Rc<RefCell<AcceptableResponse>>>,
}

thread_local! { pub static AcceptableResponseFactory_instance: Option<Rc<RefCell<AcceptableResponseFactory>>> = AcceptableResponseFactory::new(); }

impl AcceptableResponseFactory {

    pub fn new_struct() -> Self {
        let mut this = Self {
            HTML: None,
            WML: None,
            VXML: None,
        };
        this.HTML = AcceptableResponse::new(String::from("text/html"), String::from("html"), 0);
        this.WML = AcceptableResponse::new(String::from("text/wml"), String::from("wml"), 1);
        this.VXML = AcceptableResponse::new(String::from("text/vxml"), String::from("vxml"), 2);
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<AcceptableResponseFactory>>> {
        return AcceptableResponseFactory_instance.with(Clone::clone).clone();
    }
}

