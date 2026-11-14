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
use crate::java::io::Writer::Writer;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct NullWriter {
    pub base: Writer,
}

thread_local! { pub static NullWriter_NULL_WRITER: Option<Rc<RefCell<NullWriter>>> = NullWriter::new(); }
impl crate::java::io::Closeable::Closeable for NullWriter {
    fn close(&mut self) { Self::close(self) }
}

impl NullWriter {

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: Writer::new_struct(),
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    // throws IOException
    pub fn write(&mut self, mut c: i32) {
    }
    

    // throws IOException
    // Java overloaded method: Rust needs a different name
    pub fn write3(&mut self, mut cbuf: Vec<char>, mut off: i32, mut len: i32) {
    }
    

    // throws IOException
    // Java overloaded method: Rust needs a different name
    pub fn write_String(&mut self, mut str: String) {
    }
    

    // throws IOException
    // Java overloaded method: Rust needs a different name
    pub fn write3_String_int_int(&mut self, mut str: String, mut off: i32, mut len: i32) {
    }
    

    // throws IOException
    pub fn flush(&mut self) {
    }
    

    // throws IOException
    pub fn close(&mut self) {
    }
}
impl std::ops::Deref for NullWriter {
    type Target = Writer;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for NullWriter {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::java::io::Writer::WriterDyn for NullWriter {
    fn as_Writer(&self) -> &crate::java::io::Writer::Writer { &self.base }
    fn as_Writer_mut(&mut self) -> &mut crate::java::io::Writer::Writer { &mut self.base }
}


