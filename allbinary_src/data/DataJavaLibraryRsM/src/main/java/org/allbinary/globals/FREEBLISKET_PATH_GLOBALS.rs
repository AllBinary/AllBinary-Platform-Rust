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

pub static FREEBLISKET_PATH_GLOBALS_instance: crate::JavaStatic<Option<Rc<RefCell<FREEBLISKET_PATH_GLOBALS>>>> = crate::JavaStatic::new(|| FREEBLISKET_PATH_GLOBALS::new());

impl FREEBLISKET_PATH_GLOBALS {

    pub fn toString(&mut self) -> String {
        String::from(std::any::type_name::<Self>())
    }
    

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
        this.TEMPLATEPATH = format!("{}{}", String::from("template"), { let java_field = jcall!(AbPathData::getInstance()).SEPARATOR.clone(); java_field });
        this.STYLEPATH = format!("{}{}", format!("{}{}", this.TEMPLATEPATH, String::from("style")), { let java_field = jcall!(AbPathData::getInstance()).SEPARATOR.clone(); java_field });
        this.THEMEPATH = format!("{}{}", format!("{}{}", this.STYLEPATH, String::from("theme")), { let java_field = jcall!(AbPathData::getInstance()).SEPARATOR.clone(); java_field });
        this.DBINITPATH = format!("{}{}", format!("{}{}", { let java_field = jcall!(PATH_GLOBALS::PATH_GLOBALS::getInstance()).INIT_PATH.clone(); java_field }, String::from("db")), { let java_field = jcall!(AbPathData::getInstance()).SEPARATOR.clone(); java_field });
        this.LINEDATAPATH = format!("{}{}", format!("{}{}", { let java_field = jcall!(PATH_GLOBALS::PATH_GLOBALS::getInstance()).INIT_PATH.clone(); java_field }, String::from("lineData")), { let java_field = jcall!(AbPathData::getInstance()).SEPARATOR.clone(); java_field });
        this.LICENSEERRORPAGE = String::from("LicensingError.jsp");
        this.ERRORPAGE = String::from("Error.jsp");
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        jcall!(stringBuffer).append(String::from("install"));
        jcall!(stringBuffer).append({ let java_field = jcall!(AbPathData::getInstance()).SEPARATOR.clone(); java_field }.clone());
        this.INSTALLPATH = { let java_value = (jcall!(stringBuffer).toString()).clone(); java_value };
        jcall!(stringBuffer).delete(0, jcall!(stringBuffer).length());
        jcall!(stringBuffer).append({ let java_field = jcall!(PATH_GLOBALS::PATH_GLOBALS::getInstance()).INIT_PATH.clone(); java_field }.clone());
        jcall!(stringBuffer).append(String::from("views"));
        jcall!(stringBuffer).append({ let java_field = jcall!(AbPathData::getInstance()).SEPARATOR.clone(); java_field }.clone());
        this.XSLPATH = { let java_value = (jcall!(stringBuffer).toString()).clone(); java_value };
        jcall!(stringBuffer).delete(0, jcall!(stringBuffer).length());
        jcall!(stringBuffer).append(this.XSLPATH.clone());
        jcall!(stringBuffer).append(String::from("configuration"));
        jcall!(stringBuffer).append({ let java_field = jcall!(AbPathData::getInstance()).SEPARATOR.clone(); java_field }.clone());
        this.CONFIGURATIONPATH = { let java_value = (jcall!(stringBuffer).toString()).clone(); java_value };
        jcall!(stringBuffer).delete(0, jcall!(stringBuffer).length());
        jcall!(stringBuffer).append(this.CONFIGURATIONPATH.clone());
        jcall!(stringBuffer).append(String::from("user"));
        jcall!(stringBuffer).append({ let java_field = jcall!(AbPathData::getInstance()).SEPARATOR.clone(); java_field }.clone());
        this.USERCONFIGURATIONPATH = { let java_value = (jcall!(stringBuffer).toString()).clone(); java_value };
        jcall!(stringBuffer).delete(0, jcall!(stringBuffer).length());
        jcall!(stringBuffer).append(this.CONFIGURATIONPATH.clone());
        jcall!(stringBuffer).append(String::from("context"));
        jcall!(stringBuffer).append({ let java_field = jcall!(AbPathData::getInstance()).SEPARATOR.clone(); java_field }.clone());
        this.CONTEXTCONFIGURATIONPATH = { let java_value = (jcall!(stringBuffer).toString()).clone(); java_value };
        jcall!(stringBuffer).delete(0, jcall!(stringBuffer).length());
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
}

