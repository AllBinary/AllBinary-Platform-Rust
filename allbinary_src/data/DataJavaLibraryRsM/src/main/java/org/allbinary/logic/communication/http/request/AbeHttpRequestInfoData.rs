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
// package org.allbinary.logic.communication.http.request
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

pub struct AbeHttpRequestInfoData {
    pub HTTP_USER_AGENT: String,
    pub REMOTE_ADDRESS: String,
    pub REMOTE_HOST: String,
    pub REMOTE_HOST_BY_ADDRESS: String,
    pub REMOTE_PORT: String,
    pub REQUEST_FILE_PATH: String,
}

thread_local! { pub static AbeHttpRequestInfoData_instance: Option<Rc<RefCell<AbeHttpRequestInfoData>>> = AbeHttpRequestInfoData::new(); }

impl AbeHttpRequestInfoData {
    

    pub fn getInstance() -> Option<Rc<RefCell<AbeHttpRequestInfoData>>> {
        return AbeHttpRequestInfoData_instance.with(Clone::clone).clone();
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            HTTP_USER_AGENT: String::new(),
            REMOTE_ADDRESS: String::new(),
            REMOTE_HOST: String::new(),
            REMOTE_HOST_BY_ADDRESS: String::new(),
            REMOTE_PORT: String::new(),
            REQUEST_FILE_PATH: String::new(),
        };
        this.HTTP_USER_AGENT = String::from("HTTP_USER_AGENT");
        this.REMOTE_ADDRESS = String::from("REMOTE_ADDRESS");
        this.REMOTE_HOST = String::from("REMOTE_HOST");
        this.REMOTE_HOST_BY_ADDRESS = String::from("REMOTE_HOST_BY_ADDRESS");
        this.REMOTE_PORT = String::from("REMOTE_PORT");
        this.REQUEST_FILE_PATH = String::from("REQUEST_FILE_PATH");
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
}

