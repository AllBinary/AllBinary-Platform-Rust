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
// package org.allbinary.logic.visual.transform.info.objectConfig
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::logic::io::file::CommonDataFileStrings::CommonDataFileStrings;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct TransformInfoObjectConfigData {
    pub NAME: String,
    pub VARKEY: String,
    pub UNCRYPTED_EXTENSION: String,
    pub ENCRYPTED_EXTENSION: String,
}

thread_local! { pub static TransformInfoObjectConfigData_instance: Option<Rc<RefCell<TransformInfoObjectConfigData>>> = TransformInfoObjectConfigData::new(); }

impl TransformInfoObjectConfigData {
    

    pub fn getInstance() -> Option<Rc<RefCell<TransformInfoObjectConfigData>>> {
        return TransformInfoObjectConfigData_instance.with(Clone::clone).clone();
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            NAME: String::new(),
            VARKEY: String::new(),
            UNCRYPTED_EXTENSION: String::new(),
            ENCRYPTED_EXTENSION: String::new(),
        };
        this.NAME = String::from("OBJECTCONFIG_NAME");
        this.VARKEY = String::from("$");
        this.UNCRYPTED_EXTENSION = jcall!(CommonDataFileStrings::getInstance()).UNCRYPTED_EXTENSION.clone();
        this.ENCRYPTED_EXTENSION = jcall!(CommonDataFileStrings::getInstance()).ENCRYPTED_EXTENSION.clone();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
}

