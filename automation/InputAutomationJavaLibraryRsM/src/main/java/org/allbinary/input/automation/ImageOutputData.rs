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
// package org.allbinary.input.automation
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::logic::io::file::FilePathData::FilePathData;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct ImageOutputData {
}

thread_local! { pub static ImageOutputData_SAVE_PATH: String = jcall!(StringMaker::new()).append(match String::from("user.dir").as_str() { "os.name" => std::env::consts::OS.to_string(), "os.arch" => std::env::consts::ARCH.to_string(), "user.home" => std::env::var("USERPROFILE").or_else(|_| std::env::var("HOME")).unwrap_or_default(), _ => String::new() }).appendchar(jcall!(FilePathData::getInstance()).SEPARATORCHAR.clone()).append(String::from("saves")).appendchar(jcall!(FilePathData::getInstance()).SEPARATORCHAR.clone()).toString(); }

impl ImageOutputData {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
}

