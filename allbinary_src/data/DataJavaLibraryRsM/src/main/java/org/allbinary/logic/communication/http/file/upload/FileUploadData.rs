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
// package org.allbinary.logic.communication.http.file.upload
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

pub struct FileUploadData {
    pub FILE: String,
    pub UPLOAD_TO_FILE_PATH: String,
    pub FILE_DATA: String,
}

thread_local! { pub static FileUploadData_instance: Option<Rc<RefCell<FileUploadData>>> = FileUploadData::new(); }

impl FileUploadData {

    pub fn new_struct() -> Self {
        let mut this = Self {
            FILE: String::new(),
            UPLOAD_TO_FILE_PATH: String::new(),
            FILE_DATA: String::new(),
        };
        this.FILE = String::from("download_file_permissions.xml");
        this.UPLOAD_TO_FILE_PATH = String::from("FILE_UPLOAD_UPLOAD_TO_FILE_PATH");
        this.FILE_DATA = String::from("FILE_UPLOAD_FILE_DATA");
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<FileUploadData>>> {
        return FileUploadData_instance.with(Clone::clone).clone();
    }
}

