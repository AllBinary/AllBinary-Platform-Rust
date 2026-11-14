// 
// AllBinary Open License Version 1 
// Copyright (c) 2025 AllBinary 
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
use crate::java::io::IOException::IOException;
use crate::java::io::Reader::Reader;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct NullReader {
    pub base: Reader,
}

thread_local! { pub static NullReader_NULL_READER: Option<Rc<RefCell<NullReader>>> = NullReader::new(); }
impl crate::java::io::Closeable::Closeable for NullReader {
    fn close(&mut self) { Self::close(self) }
}

impl NullReader {

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: Reader::new_struct(),
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    // throws IOException
    pub fn read(&mut self) -> i32 {
        return -1;
    }
    

    // throws IOException
    // Java overloaded method: Rust needs a different name
    pub fn read3(&mut self, mut cbuf: Vec<char>, mut off: i32, mut len: i32) -> i32 {
        return -1;
    }
    

    // throws IOException
    pub fn ready(&mut self) -> bool {
        return false;
    }
    

    // throws IOException
    pub fn skip(&mut self, mut n: i64) -> i64 {
        return 0_i64;
    }
    

    pub fn close(&mut self) {
    }
}
impl std::ops::Deref for NullReader {
    type Target = Reader;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for NullReader {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::java::io::Reader::ReaderDyn for NullReader {
    fn as_Reader(&self) -> &crate::java::io::Reader::Reader { &self.base }
    fn as_Reader_mut(&mut self) -> &mut crate::java::io::Reader::Reader { &mut self.base }
}


