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
// package org.allbinary.logic.io.file
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

pub struct FileData {
    pub NAME: String,
    pub ISFILE: String,
    pub ISDIRECTORY: String,
    pub ISHIDDEN: String,
    pub ISABSOLUTE: String,
    pub ROOT_NAME: String,
    pub PATH: String,
    pub ABSOLUTE_PATH: String,
    pub CANONICAL_PATH: String,
    pub PARENT: String,
    pub MINLEN: i32,
    pub MAXLEN: i32,
    pub MAXIMAGEFILESIZE: i32,
    pub MINIMAGEFILESIZE: i32,
    pub MAXDOWNLOADABLEFILESIZE: i32,
    pub MINDOWNLOADABLEFILESIZE: i32,
}

thread_local! { pub static FileData_instance: Option<Rc<RefCell<FileData>>> = FileData::new(); }

impl FileData {
    

    pub fn getInstance() -> Option<Rc<RefCell<FileData>>> {
        return FileData_instance.with(Clone::clone).clone();
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            NAME: String::new(),
            ISFILE: String::new(),
            ISDIRECTORY: String::new(),
            ISHIDDEN: String::new(),
            ISABSOLUTE: String::new(),
            ROOT_NAME: String::new(),
            PATH: String::new(),
            ABSOLUTE_PATH: String::new(),
            CANONICAL_PATH: String::new(),
            PARENT: String::new(),
            MINLEN: 0,
            MAXLEN: 0,
            MAXIMAGEFILESIZE: 0,
            MINIMAGEFILESIZE: 0,
            MAXDOWNLOADABLEFILESIZE: 0,
            MINDOWNLOADABLEFILESIZE: 0,
        };
        this.NAME = String::from("FILE_NAME");
        this.ISFILE = String::from("FILE_ISFILE");
        this.ISDIRECTORY = String::from("FILE_ISDIRECTORY");
        this.ISHIDDEN = String::from("FILE_ISHIDDEN");
        this.ISABSOLUTE = String::from("FILE_ISABSOLUTE");
        this.ROOT_NAME = String::from("FILE_ROOT_NAME");
        this.PATH = String::from("FILE_PATH");
        this.ABSOLUTE_PATH = String::from("FILE_ABSOLUTE_PATH");
        this.CANONICAL_PATH = String::from("FILE_CANONICAL_PATH");
        this.PARENT = String::from("FILE_PARENT");
        this.MINLEN = 1;
        this.MAXLEN = 120;
        this.MAXIMAGEFILESIZE = 1048576;
        this.MINIMAGEFILESIZE = 32;
        this.MAXDOWNLOADABLEFILESIZE = 1048576 * 20;
        this.MINDOWNLOADABLEFILESIZE = 1;
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
}

