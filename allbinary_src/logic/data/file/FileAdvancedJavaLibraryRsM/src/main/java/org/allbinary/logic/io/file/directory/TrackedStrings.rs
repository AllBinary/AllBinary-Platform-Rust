// 
// AllBinary Open License Version 1 
// Copyright (c) 2026 AllBinary 
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
// package org.allbinary.logic.io.file.directory
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

pub struct TrackedStrings {
    pub TARGET_PATH_WINDOWS: String,
    pub TARGET_PATH_UNIX: String,
    pub HTML_TEMP: String,
    pub APPLICATION: String,
    pub APP: String,
    pub GIT_COMMAND: String,
    pub CHANGE_DIRECTORY_OPTION: String,
    pub LIST_FILES_COMMAND: String,
    pub ERROR_UNMATCH_OPTION: String,
    pub PATHSPEC_SEPARATOR: String,
    pub SEP_BY_NULL_CHAR_INSTEAD_OF_NEW_LINE: String,
}

thread_local! { pub static TrackedStrings_instance: Option<Rc<RefCell<TrackedStrings>>> = TrackedStrings::new(); }

impl TrackedStrings {

    pub fn new_struct() -> Self {
        let mut this = Self {
            TARGET_PATH_WINDOWS: String::new(),
            TARGET_PATH_UNIX: String::new(),
            HTML_TEMP: String::new(),
            APPLICATION: String::new(),
            APP: String::new(),
            GIT_COMMAND: String::new(),
            CHANGE_DIRECTORY_OPTION: String::new(),
            LIST_FILES_COMMAND: String::new(),
            ERROR_UNMATCH_OPTION: String::new(),
            PATHSPEC_SEPARATOR: String::new(),
            SEP_BY_NULL_CHAR_INSTEAD_OF_NEW_LINE: String::new(),
        };
        this.TARGET_PATH_WINDOWS = String::from("\\target\\");
        this.TARGET_PATH_UNIX = String::from("/target/");
        this.HTML_TEMP = String::from("HTMLTemp");
        this.APPLICATION = String::from("Application");
        this.APP = String::from("\\app");
        this.GIT_COMMAND = String::from("git");
        this.CHANGE_DIRECTORY_OPTION = String::from("-C");
        this.LIST_FILES_COMMAND = String::from("ls-files");
        this.ERROR_UNMATCH_OPTION = String::from("--error-unmatch");
        this.PATHSPEC_SEPARATOR = String::from("--");
        this.SEP_BY_NULL_CHAR_INSTEAD_OF_NEW_LINE = String::from("-z");
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<TrackedStrings>>> {
        return TrackedStrings_instance.with(Clone::clone).clone();
    }
}

