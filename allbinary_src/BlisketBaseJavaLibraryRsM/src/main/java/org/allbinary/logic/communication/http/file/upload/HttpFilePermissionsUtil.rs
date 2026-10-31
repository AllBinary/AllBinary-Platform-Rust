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
use crate::org::allbinary::logic::io::file::AbFile::AbFile;
use crate::org::allbinary::logic::io::path::AbPath::AbPath;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct HttpFilePermissionsUtil {
}

thread_local! { pub static HttpFilePermissionsUtil_instance: Option<Rc<RefCell<HttpFilePermissionsUtil>>> = HttpFilePermissionsUtil::new(); }

impl HttpFilePermissionsUtil {

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<HttpFilePermissionsUtil>>> {
        return HttpFilePermissionsUtil_instance.with(Clone::clone).clone();
    }
    

    // throws Exception
    pub fn create(&mut self, mut path: Option<Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>>) {
        let mut file: Option<Rc<RefCell<AbFile>>> = AbFile::createAbFileFromAbPath(path.clone());
        if !jcall!(file).isFile() {
            jcall!(file).createNewFile();
        }
    }
}

