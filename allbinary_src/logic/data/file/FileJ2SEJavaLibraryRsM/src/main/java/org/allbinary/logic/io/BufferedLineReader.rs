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
// use crate::java::io::BufferedReader::BufferedReader;
// use crate::java::io::FileReader::FileReader;
use crate::org::allbinary::logic::io::file::AbFile::AbFile;
use crate::org::allbinary::logic::io::file::AbFileNativeUtil::AbFileNativeUtil;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct BufferedLineReader {
    pub tmpIn: Option<Rc<RefCell<BufferedReader>>>,
}


impl BufferedLineReader {
    

    pub fn new_struct(mut file: Option<Rc<RefCell<AbFile>>>) -> Self {
        let mut this = Self {
            tmpIn: None,
        };
        this.tmpIn = BufferedReader::new(FileReader::new(AbFileNativeUtil::get(file.clone())));
        this
    }

    pub fn new(mut file: Option<Rc<RefCell<AbFile>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(file))))
    }
    

    pub fn readUpToLines(&mut self, mut readAheadIndex: i64) {
    }
    

    // throws Exception
    pub fn readLine(&mut self) -> String {
        return jcall!(self.tmpIn).readLine();
    }
    

    pub fn getSize(&mut self) -> i64 {
        return 0_i64;
    }
    

    pub fn getCurrent(&mut self) -> i32 {
        return 0;
    }
}

