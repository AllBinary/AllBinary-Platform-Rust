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
use super::AbFile::AbFile;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct FileFactory {
}

thread_local! { pub static FileFactory_instance: Option<Rc<RefCell<FileFactory>>> = FileFactory::new(); }

impl FileFactory {

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<FileFactory>>> {
        return FileFactory_instance.with(Clone::clone).clone();
    }
    

    // throws Exception
    // Java overloaded method: Rust needs a different name
    pub fn getInstance1(&mut self, mut filePath: String) -> Option<Rc<RefCell<AbFile>>> {
        return AbFile::createAbFileFromRawPath(filePath.clone());
    }
    

    // throws Exception
    // Java overloaded method: Rust needs a different name
    pub fn getInstance2(&mut self, mut file: Option<Rc<RefCell<AbFile>>>, mut childFilePath: String) -> Option<Rc<RefCell<AbFile>>> {
        return AbFile::createAbFileWithChild(file.clone(), childFilePath.clone());
    }
    

    // throws Exception
    pub fn isFile(&mut self, mut path: String) -> bool {
        let mut file: Option<Rc<RefCell<AbFile>>> = AbFile::createAbFile(path.clone());
        if jcall!(file).exists() {
            return true;
        } else {
            return false;
        }
    }
}

