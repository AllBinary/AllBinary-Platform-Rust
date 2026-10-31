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
// package org.allbinary.globals
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::logic::io::path::AbPathData::AbPathData;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::string::CommonStrings::CommonStrings;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct PATH_GLOBALS {
    pub DATA_PATH: String,
    pub INIT_PATH: String,
    pub VIEWS_PATH: String,
    pub LOG_PATH: String,
    pub BACKUP_PATH: String,
}

thread_local! { pub static PATH_GLOBALS_instance: Option<Rc<RefCell<PATH_GLOBALS>>> = PATH_GLOBALS::new(); }

impl PATH_GLOBALS {
    

    pub fn getInstance() -> Option<Rc<RefCell<PATH_GLOBALS>>> {
        return PATH_GLOBALS_instance.with(Clone::clone).clone();
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            DATA_PATH: String::new(),
            INIT_PATH: String::new(),
            VIEWS_PATH: String::new(),
            LOG_PATH: String::new(),
            BACKUP_PATH: String::new(),
        };
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        let mut abPathData: Option<Rc<RefCell<AbPathData>>> = AbPathData::getInstance();
        jcall!(stringBuffer).append(String::from("data"));
        jcall!(stringBuffer).append(jcall!(abPathData).SEPARATOR.clone());
        this.DATA_PATH = jcall!(stringBuffer).toString();
        jcall!(stringBuffer).delete(0, jcall!(stringBuffer).length());
        jcall!(stringBuffer).append(this.DATA_PATH.clone());
        jcall!(stringBuffer).append(jcall!(CommonStrings::getInstance()).INIT.clone());
        jcall!(stringBuffer).append(jcall!(abPathData).SEPARATOR.clone());
        this.INIT_PATH = jcall!(stringBuffer).toString();
        jcall!(stringBuffer).delete(0, jcall!(stringBuffer).length());
        jcall!(stringBuffer).append(this.DATA_PATH.clone());
        jcall!(stringBuffer).append(String::from("log"));
        jcall!(stringBuffer).append(jcall!(abPathData).SEPARATOR.clone());
        this.LOG_PATH = jcall!(stringBuffer).toString();
        jcall!(stringBuffer).delete(0, jcall!(stringBuffer).length());
        jcall!(stringBuffer).append(this.DATA_PATH.clone());
        jcall!(stringBuffer).append(String::from("backup"));
        jcall!(stringBuffer).append(jcall!(abPathData).SEPARATOR.clone());
        this.BACKUP_PATH = jcall!(stringBuffer).toString();
        jcall!(stringBuffer).delete(0, jcall!(stringBuffer).length());
        jcall!(stringBuffer).append(this.INIT_PATH.clone());
        jcall!(stringBuffer).append(String::from("views"));
        jcall!(stringBuffer).append(jcall!(abPathData).SEPARATOR.clone());
        this.VIEWS_PATH = jcall!(stringBuffer).toString();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
}

