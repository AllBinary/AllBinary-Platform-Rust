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
// use crate::java::io::FileNotFoundException::FileNotFoundException;
use crate::android::content::Context::Context;
use crate::org::allbinary::data::resource::ResourceUtil::ResourceUtil;
use super::AbFile::AbFile;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct FileFactory {
    pub context: Option<Rc<RefCell<Context>>>,
}

thread_local! { pub static FileFactory_SINGLETON: Option<Rc<RefCell<FileFactory>>> = FileFactory::new(jcall!(ResourceUtil::getInstance()).getContext()); }

impl FileFactory {
    

    pub fn new_struct(mut context: Option<Rc<RefCell<Context>>>) -> Self {
        let mut this = Self {
            context: None,
        };
        this.context = context.clone();
        this
    }

    pub fn new(mut context: Option<Rc<RefCell<Context>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(context))))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<FileFactory>>> {
        return FileFactory_SINGLETON.with(Clone::clone).clone();
    }
    

    // throws Exception
    pub fn isFile(&mut self, mut path: String) -> bool {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            jcall!(self.context).openFileInput(path.clone());
            return true;
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            return false;
            }
        }
    }
    

    pub fn getContext(&mut self) -> Option<Rc<RefCell<Context>>> {
        return self.context.clone();
    }
    

    // throws Exception
    pub fn getFile(&mut self, mut filePath: String) -> Option<Rc<RefCell<AbFile>>> {
        return AbFile::createAbFileFromRawPath(filePath.clone());
    }
    

    // throws Exception
    // Java overloaded method: Rust needs a different name
    pub fn getInstance2(&mut self, mut file: Option<Rc<RefCell<AbFile>>>, mut childFilePath: String) -> Option<Rc<RefCell<AbFile>>> {
        return AbFile::createAbFileWithChild(file.clone(), childFilePath.clone());
    }
}

