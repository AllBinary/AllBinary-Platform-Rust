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
// package org.allbinary.logic.visual.transform.template.customizer.widgets.logo
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

pub struct LogoData {
    pub NAME: String,
    pub IMAGEPATH: String,
    pub IMAGEFILENAME: String,
    pub IMAGE: String,
}

thread_local! { pub static LogoData_instance: Option<Rc<RefCell<LogoData>>> = LogoData::new(); }

impl LogoData {
    

    pub fn getInstance() -> Option<Rc<RefCell<LogoData>>> {
        return LogoData_instance.with(Clone::clone).clone();
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            NAME: String::new(),
            IMAGEPATH: String::new(),
            IMAGEFILENAME: String::new(),
            IMAGE: String::new(),
        };
        this.NAME = String::from("LOGO_NAME");
        this.IMAGEPATH = String::from("LOGO_IMAGE_PATH");
        this.IMAGEFILENAME = String::from("LOGO_IMAGE_FILE_NAME");
        this.IMAGE = String::from("LOGO_IMAGE");
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
}

