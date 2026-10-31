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
// package org.allbinary.logic.io
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::logic::io::file::AbFile::AbFile;
use super::AbFileOutputStream::AbFileOutputStream;
use super::AbDataOutputStream::AbDataOutputStream;
use super::FileStreamFactory::FileStreamFactory;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct DataOutputStreamFactory {
}

thread_local! { pub static DataOutputStreamFactory_instance: Option<Rc<RefCell<DataOutputStreamFactory>>> = DataOutputStreamFactory::new(); }

impl DataOutputStreamFactory {

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<DataOutputStreamFactory>>> {
        return DataOutputStreamFactory_instance.with(Clone::clone).clone();
    }
    

    // throws Exception
    pub fn getInstanceForAbFile(&mut self, mut file: Option<Rc<RefCell<AbFile>>>) -> Option<Rc<RefCell<AbDataOutputStream>>> {
        let mut idFileOutputStream: Option<Rc<RefCell<AbFileOutputStream>>> = AbFileOutputStream::createFromAbFile(file.clone());
        let mut idOutData: Option<Rc<RefCell<AbDataOutputStream>>> = AbDataOutputStream::new(idFileOutputStream.clone());
        return idOutData.clone();
    }
    

    // throws Exception
    // Java overloaded method: Rust needs a different name
    pub fn getInstance2(&mut self, mut filePath: String, mut fileName: String) -> Option<Rc<RefCell<AbDataOutputStream>>> {
        let mut fileStreamFactory: Option<Rc<RefCell<FileStreamFactory>>> = FileStreamFactory::getInstance();
        let mut fileOutputStream: Option<Rc<RefCell<AbFileOutputStream>>> = jcall!(fileStreamFactory).getFileOutputStreamInstance(filePath.clone(), fileName.clone());
        let mut idOutData: Option<Rc<RefCell<AbDataOutputStream>>> = AbDataOutputStream::new(fileOutputStream.clone());
        return idOutData.clone();
    }
}

