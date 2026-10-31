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
// package org.allbinary.logic.io.path
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::logic::io::file::FilePathData::FilePathData;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use crate::org::allbinary::string::CommonSeps::CommonSeps;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct AbPathData {
    pub EXTENSION_SEP: String,
    pub SEPARATORCHAR: char,
    pub SEPARATOR: String,
}

thread_local! { pub static AbPathData_instance: Option<Rc<RefCell<AbPathData>>> = AbPathData::new(); }

impl AbPathData {
    

    pub fn getInstance() -> Option<Rc<RefCell<AbPathData>>> {
        return AbPathData_instance.with(Clone::clone).clone();
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            EXTENSION_SEP: String::new(),
            SEPARATORCHAR: '\0',
            SEPARATOR: String::new(),
        };
        this.EXTENSION_SEP = jcall!(CommonSeps::getInstance()).PERIOD.clone();
        this.SEPARATORCHAR = '/';
        this.SEPARATOR = jcall!(CommonSeps::getInstance()).FORWARD_SLASH.clone();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
}

