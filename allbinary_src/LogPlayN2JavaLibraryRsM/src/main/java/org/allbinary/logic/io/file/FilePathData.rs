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

pub struct FilePathData {
    pub SEPARATORCHAR: char,
    pub PATH_START: String,
}

thread_local! { pub static FilePathData_instance: Option<Rc<RefCell<FilePathData>>> = FilePathData::new(); }

impl FilePathData {
    

    pub fn getInstance() -> Option<Rc<RefCell<FilePathData>>> {
        return FilePathData_instance.with(Clone::clone).clone();
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            SEPARATORCHAR: '\0',
            PATH_START: String::new(),
        };
        this.SEPARATORCHAR = '\\';
        this.PATH_START = String::from("\\\\");
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
}

