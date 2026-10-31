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
// use crate::java::io::File::File;
use crate::org::allbinary::logic::communication::log::PreLogUtil::PreLogUtil;
use super::AbFile::AbFile;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct FileWrapperUtil {
}


impl FileWrapperUtil {

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn wrapFiles(mut files: Vec<Option<Rc<RefCell<Box<dyn std::any::Any>>>>>) -> Vec<Option<Rc<RefCell<AbFile>>>> {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut abFileArray: Vec<Option<Rc<RefCell<AbFile>>>> = vec![None; ((files.len() as i32)) as usize];
            for index in (0..=(files.len() as i32) - 1).rev() {
                abFileArray[(index) as usize] = AbFile::new((files[(index) as usize]).clone() /* as File */);
            }
            return abFileArray.clone();
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            PreLogUtil::putSE(String::from("Exception Wrapping Files"), String::from("FileWrapperUtil"), String::from("wrapFiles"), e.clone());
            return vec![None; (0) as usize];
            }
        }
    }
    

    pub fn wrapFile(mut file: Option<Rc<RefCell<File>>>) -> Option<Rc<RefCell<AbFile>>> {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            return AbFile::new(file.clone());
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            PreLogUtil::putSE(String::from("Exception Wrapping File"), String::from("FileWrapperUtil"), String::from("wrapFile"), e.clone());
            return AbFile_NULL_FILE.with(Clone::clone).clone();
            }
        }
    }
}

