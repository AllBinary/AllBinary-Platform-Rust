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
// package org.allbinary.logic.communication.log
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::logic::io::path::AbPathData::AbPathData;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct LogData {
    pub MAX: i64,
    pub extension: String,
    pub backupFileExt: String,
    pub ALLBINARY: String,
    pub NAME: String,
    pub DESCRIPTION: String,
}


impl LogData {

    pub fn new_struct() -> Self {
        let mut this = Self {
            MAX: 0_i64,
            extension: String::new(),
            backupFileExt: String::new(),
            ALLBINARY: String::new(),
            NAME: String::new(),
            DESCRIPTION: String::new(),
        };
        this.MAX = 5000000_i64;
        this.extension = String::from("log");
        this.backupFileExt = format!("{}{}", format!("{}{}", format!("{}{}", jcall!(AbPathData::getInstance()).EXTENSION_SEP, String::from("bak")), jcall!(AbPathData::getInstance()).EXTENSION_SEP), String::from("txt"));
        this.ALLBINARY = String::from("allbinary");
        this.NAME = String::from("LOG_NAME");
        this.DESCRIPTION = String::from("LOG_DESCRIPTION");
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
}

