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
use crate::org::allbinary::logic::NullUtil::NullUtil;
use crate::org::allbinary::logic::io::file::AbFile::AbFile;
use crate::org::allbinary::logic::io::path::AbPath::AbPath;
use super::AbFileInputStream::AbFileInputStream;
use super::AbFileOutputStream::AbFileOutputStream;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct FileStreamFactory {
}

pub static mut FileStreamFactory_SINGLETON: std::cell::LazyCell<Option<Rc<RefCell<Box<dyn std::any::Any>>>>> = std::cell::LazyCell::new(|| jcall!(NullUtil::getInstance()).NULL_OBJECT.clone());

impl FileStreamFactory {
    

    pub fn getInstance() -> Option<Rc<RefCell<FileStreamFactory>>> {
        if match (&unsafe { FileStreamFactory_SINGLETON.clone() }, &jcall!(NullUtil::getInstance()).NULL_OBJECT) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            unsafe { *FileStreamFactory_SINGLETON = FileStreamFactory::new() };
        }
        return (unsafe { FileStreamFactory_SINGLETON.clone() }).clone() /* as FileStreamFactory */;
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    // throws Exception
    pub fn getFileInputStreamInstance(&mut self, mut path: String, mut fileName: String) -> Option<Rc<RefCell<dyn crate::org::allbinary::logic::io::AbFileInputStream::AbFileInputStreamDyn>>> {
        let mut FILEABPATH: Option<Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>> = AbPath::new(path.clone(), fileName.clone()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>);
        let mut file: Option<Rc<RefCell<AbFile>>> = AbFile::createAbFileFromAbPath(FILEABPATH.clone());
        return AbFileInputStream::new1(file.clone()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::io::AbFileInputStream::AbFileInputStreamDyn>>);
    }
    

    // throws Exception
    pub fn getFileOutputStreamInstance(&mut self, mut path: String, mut fileName: String) -> Option<Rc<RefCell<AbFileOutputStream>>> {
        let mut FILEABPATH: Option<Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>> = AbPath::new(path.clone(), fileName.clone()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>);
        let mut file: Option<Rc<RefCell<AbFile>>> = AbFile::createAbFileFromAbPath(FILEABPATH.clone());
        if jcall!(file).exists() {
            jcall!(file).delete();
            jcall!(file).createNewFile();
        }
        return AbFileOutputStream::createFromAbFile(file.clone());
    }
    

    // throws Exception
    pub fn delete(&mut self, mut path: String, mut fileName: String) {
        let mut FILEABPATH: Option<Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>> = AbPath::new(path.clone(), fileName.clone()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>);
        let mut file: Option<Rc<RefCell<AbFile>>> = AbFile::createAbFileFromAbPath(FILEABPATH.clone());
        jcall!(file).delete();
    }
}

