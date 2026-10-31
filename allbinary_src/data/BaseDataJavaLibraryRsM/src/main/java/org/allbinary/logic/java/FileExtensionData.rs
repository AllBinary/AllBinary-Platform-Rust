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
// package org.allbinary.logic.java
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

pub struct FileExtensionData {
    pub DOT_JAVA_SOURCE: String,
    pub JAVA_SOURCE: String,
}

thread_local! { pub static FileExtensionData_instance: Option<Rc<RefCell<FileExtensionData>>> = FileExtensionData::new(); }

impl FileExtensionData {
    

    pub fn getInstance() -> Option<Rc<RefCell<FileExtensionData>>> {
        return FileExtensionData_instance.with(Clone::clone).clone();
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            DOT_JAVA_SOURCE: String::new(),
            JAVA_SOURCE: String::new(),
        };
        this.DOT_JAVA_SOURCE = String::from(".java");
        this.JAVA_SOURCE = String::from("java");
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
}

