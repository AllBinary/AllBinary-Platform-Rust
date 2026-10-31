// 
// AllBinary Open License Version 1 
// Copyright (c) 2025 AllBinary 
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
// package org.allbinary.data.tree.dom
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

pub struct URIResolverStrings {
    pub IMPORT_URL: String,
    pub ATTEMPT: String,
    pub BASE: String,
    pub NEW_PATH: String,
    pub NOTE: String,
    pub URL_GLOBAL: String,
    pub REQUIRED_EXTENSION: String,
    pub RESOLVE: String,
}

thread_local! { pub static URIResolverStrings_instance: Option<Rc<RefCell<URIResolverStrings>>> = URIResolverStrings::new(); }

impl URIResolverStrings {

    pub fn new_struct() -> Self {
        let mut this = Self {
            IMPORT_URL: String::new(),
            ATTEMPT: String::new(),
            BASE: String::new(),
            NEW_PATH: String::new(),
            NOTE: String::new(),
            URL_GLOBAL: String::new(),
            REQUIRED_EXTENSION: String::new(),
            RESOLVE: String::new(),
        };
        this.IMPORT_URL = String::from("/{import url}");
        this.ATTEMPT = String::from("attempt to use xsl:import: href=");
        this.BASE = String::from("\nBase= ");
        this.NEW_PATH = String::from("\nNew path= ");
        this.NOTE = String::from("\nNote: ");
        this.URL_GLOBAL = String::from(" is a urlglobal");
        this.REQUIRED_EXTENSION = String::from("\nRequired Extension: ");
        this.RESOLVE = String::from("resolve");
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<URIResolverStrings>>> {
        return URIResolverStrings_instance.with(Clone::clone).clone();
    }
}

