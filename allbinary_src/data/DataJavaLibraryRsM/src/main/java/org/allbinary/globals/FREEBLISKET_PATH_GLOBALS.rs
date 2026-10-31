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

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct FREEBLISKET_PATH_GLOBALS {
    pub INSTALLPATH: String,
    pub XSLPATH: String,
    pub TEMPLATEPATH: String,
    pub STYLEPATH: String,
    pub THEMEPATH: String,
    pub DBINITPATH: String,
    pub LINEDATAPATH: String,
    pub CONFIGURATIONPATH: String,
    pub USERCONFIGURATIONPATH: String,
    pub CONTEXTCONFIGURATIONPATH: String,
    pub LICENSEERRORPAGE: String,
    pub ERRORPAGE: String,
}

thread_local! { pub static FREEBLISKET_PATH_GLOBALS_instance: Option<Rc<RefCell<FREEBLISKET_PATH_GLOBALS>>> = FREEBLISKET_PATH_GLOBALS::new(); }

impl FREEBLISKET_PATH_GLOBALS {
    

    pub fn getInstance() -> Option<Rc<RefCell<FREEBLISKET_PATH_GLOBALS>>> {
        return FREEBLISKET_PATH_GLOBALS_instance.with(Clone::clone).clone();
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            INSTALLPATH: String::new(),
            XSLPATH: String::new(),
            TEMPLATEPATH: String::new(),
            STYLEPATH: String::new(),
            THEMEPATH: String::new(),
            DBINITPATH: String::new(),
            LINEDATAPATH: String::new(),
            CONFIGURATIONPATH: String::new(),
            USERCONFIGURATIONPATH: String::new(),
            CONTEXTCONFIGURATIONPATH: String::new(),
            LICENSEERRORPAGE: String::new(),
            ERRORPAGE: String::new(),
        };
        this.TEMPLATEPATH = format!("{}{}", String::from("template"), jcall!(AbPathData::getInstance()).SEPARATOR);
        this.STYLEPATH = format!("{}{}", format!("{}{}", this.TEMPLATEPATH, String::from("style")), jcall!(AbPathData::getInstance()).SEPARATOR);
        this.THEMEPATH = format!("{}{}", format!("{}{}", this.STYLEPATH, String::from("theme")), jcall!(AbPathData::getInstance()).SEPARATOR);
        this.DBINITPATH = format!("{}{}", format!("{}{}", jcall!(jcall!(PATH_GLOBALS).getInstance()).INIT_PATH, String::from("db")), jcall!(AbPathData::getInstance()).SEPARATOR);
        this.LINEDATAPATH = format!("{}{}", format!("{}{}", jcall!(jcall!(PATH_GLOBALS).getInstance()).INIT_PATH, String::from("lineData")), jcall!(AbPathData::getInstance()).SEPARATOR);
        this.LICENSEERRORPAGE = String::from("LicensingError.jsp");
        this.ERRORPAGE = String::from("Error.jsp");
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        jcall!(stringBuffer).append(String::from("install"));
        jcall!(stringBuffer).append(jcall!(AbPathData::getInstance()).SEPARATOR.clone());
        this.INSTALLPATH = jcall!(stringBuffer).toString();
        jcall!(stringBuffer).delete(0, jcall!(stringBuffer).length());
        jcall!(stringBuffer).append(jcall!(jcall!(PATH_GLOBALS).getInstance()).INIT_PATH.clone());
        jcall!(stringBuffer).append(String::from("views"));
        jcall!(stringBuffer).append(jcall!(AbPathData::getInstance()).SEPARATOR.clone());
        this.XSLPATH = jcall!(stringBuffer).toString();
        jcall!(stringBuffer).delete(0, jcall!(stringBuffer).length());
        jcall!(stringBuffer).append(this.XSLPATH.clone());
        jcall!(stringBuffer).append(String::from("configuration"));
        jcall!(stringBuffer).append(jcall!(AbPathData::getInstance()).SEPARATOR.clone());
        this.CONFIGURATIONPATH = jcall!(stringBuffer).toString();
        jcall!(stringBuffer).delete(0, jcall!(stringBuffer).length());
        jcall!(stringBuffer).append(this.CONFIGURATIONPATH.clone());
        jcall!(stringBuffer).append(String::from("user"));
        jcall!(stringBuffer).append(jcall!(AbPathData::getInstance()).SEPARATOR.clone());
        this.USERCONFIGURATIONPATH = jcall!(stringBuffer).toString();
        jcall!(stringBuffer).delete(0, jcall!(stringBuffer).length());
        jcall!(stringBuffer).append(this.CONFIGURATIONPATH.clone());
        jcall!(stringBuffer).append(String::from("context"));
        jcall!(stringBuffer).append(jcall!(AbPathData::getInstance()).SEPARATOR.clone());
        this.CONTEXTCONFIGURATIONPATH = jcall!(stringBuffer).toString();
        jcall!(stringBuffer).delete(0, jcall!(stringBuffer).length());
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
}

