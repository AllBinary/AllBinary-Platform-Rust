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
use crate::java::io::OutputStream::OutputStream;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct NullOutputStream {
    pub base: OutputStream,
}

thread_local! { pub static NullOutputStream_NULL_OUTPUT_STREAM: Option<Rc<RefCell<NullOutputStream>>> = NullOutputStream::new(); }
impl crate::java::io::Closeable::Closeable for NullOutputStream {
    fn close(&mut self) { Self::close(self) }
}

impl NullOutputStream {

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: OutputStream::new_struct(),
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    // throws IOException
    pub fn write(&mut self, mut b: i32) {
    }
    

    // throws IOException
    // Java overloaded method: Rust needs a different name
    pub fn write3(&mut self, mut b: Vec<i8>, mut off: i32, mut len: i32) {
    }
    

    pub fn close(&mut self) {
    }
}
impl std::ops::Deref for NullOutputStream {
    type Target = OutputStream;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for NullOutputStream {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::java::io::OutputStream::OutputStreamDyn for NullOutputStream {
    fn as_OutputStream(&self) -> &crate::java::io::OutputStream::OutputStream { &self.base }
    fn as_OutputStream_mut(&mut self) -> &mut crate::java::io::OutputStream::OutputStream { &mut self.base }
}


