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
use crate::java::io::FileInputStream::FileInputStream;
// use crate::java::io::FileNotFoundException::FileNotFoundException;
use crate::java::io::IOException::IOException;
use crate::java::io::InputStream::InputStream;
// use crate::java::nio::channels::FileChannel::FileChannel;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct AbFileInputStream {
    pub base: InputStream,
    pub fileInputStream: Option<Rc<RefCell<dyn crate::java::io::FileInputStream::FileInputStreamDyn>>>,
}

impl crate::java::io::Closeable::Closeable for AbFileInputStream {
    fn close(&mut self) { Self::close(self) }
}

impl AbFileInputStream {
    

    pub fn new_struct(mut fileInputStream: Option<Rc<RefCell<dyn crate::java::io::FileInputStream::FileInputStreamDyn>>>) -> Self {
        let mut this = Self {
            base: InputStream::new_struct(),
            fileInputStream: None,
        };
        this.fileInputStream = fileInputStream.clone();
        this
    }

    pub fn new(mut fileInputStream: Option<Rc<RefCell<dyn crate::java::io::FileInputStream::FileInputStreamDyn>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(fileInputStream))))
    }
    

    pub fn mark(&mut self, mut readlimit: i32) {
        jcall!(self.fileInputStream).as_FileInputStream_mut().mark(readlimit.clone());
    }
    

    pub fn markSupported(&mut self) -> bool {
        return jcall!(self.fileInputStream).as_FileInputStream_mut().markSupported();
    }
    

    // throws IOException
    // synchronized
    pub fn reset(&mut self) {
        jcall!(self.fileInputStream).as_FileInputStream_mut().reset();
    }
    

    // throws IOException
    pub fn available(&mut self) -> i32 {
        return jcall!(self.fileInputStream).as_FileInputStream_mut().available();
    }
    

    // throws IOException
    pub fn close(&mut self) {
        jcall!(self.fileInputStream).as_FileInputStream_mut().close();
    }
    

    pub fn getChannel(&mut self) -> Option<Rc<RefCell<FileChannel>>> {
        return jcall!(self.fileInputStream).as_FileInputStream_mut().getChannel();
    }
    

    // throws IOException
    pub fn read(&mut self) -> i32 {
        return jcall!(self.fileInputStream).as_FileInputStream_mut().read();
    }
    

    // throws IOException
    // Java overloaded method: Rust needs a different name
    pub fn read1(&mut self, mut b: Vec<i8>) -> i32 {
        return jcall!(self.fileInputStream).as_FileInputStream_mut().read1(b.clone());
    }
    

    // throws IOException
    // Java overloaded method: Rust needs a different name
    pub fn read3(&mut self, mut buffer: Vec<i8>, mut offset: i32, mut count: i32) -> i32 {
        return jcall!(self.fileInputStream).as_FileInputStream_mut().read3(buffer.clone(), offset.clone(), count.clone());
    }
    

    // throws IOException
    pub fn skip(&mut self, mut n: i64) -> i64 {
        return jcall!(self.fileInputStream).as_FileInputStream_mut().skip(n.clone());
    }
}
impl std::ops::Deref for AbFileInputStream {
    type Target = InputStream;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for AbFileInputStream {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

pub trait AbFileInputStreamDyn: crate::java::io::InputStream::InputStreamDyn {
    fn as_AbFileInputStream(&self) -> &AbFileInputStream;
    fn as_AbFileInputStream_mut(&mut self) -> &mut AbFileInputStream;
    fn mark(&mut self, mut readlimit: i32) { AbFileInputStream::mark(self.as_AbFileInputStream_mut(), readlimit) }
    fn markSupported(&mut self) -> bool { AbFileInputStream::markSupported(self.as_AbFileInputStream_mut()) }
    fn reset(&mut self) { AbFileInputStream::reset(self.as_AbFileInputStream_mut()) }
    fn available(&mut self) -> i32 { AbFileInputStream::available(self.as_AbFileInputStream_mut()) }
    fn close(&mut self) { AbFileInputStream::close(self.as_AbFileInputStream_mut()) }
    fn getChannel(&mut self) -> Option<Rc<RefCell<FileChannel>>> { AbFileInputStream::getChannel(self.as_AbFileInputStream_mut()) }
    fn read(&mut self) -> i32 { AbFileInputStream::read(self.as_AbFileInputStream_mut()) }
    fn read1(&mut self, mut b: Vec<i8>) -> i32 { AbFileInputStream::read1(self.as_AbFileInputStream_mut(), b) }
    fn read3(&mut self, mut buffer: Vec<i8>, mut offset: i32, mut count: i32) -> i32 { AbFileInputStream::read3(self.as_AbFileInputStream_mut(), buffer, offset, count) }
    fn skip(&mut self, mut n: i64) -> i64 { AbFileInputStream::skip(self.as_AbFileInputStream_mut(), n) }
}
impl AbFileInputStreamDyn for AbFileInputStream {
    fn as_AbFileInputStream(&self) -> &AbFileInputStream { self }
    fn as_AbFileInputStream_mut(&mut self) -> &mut AbFileInputStream { self }
}

impl crate::java::io::InputStream::InputStreamDyn for AbFileInputStream {
    fn as_InputStream(&self) -> &crate::java::io::InputStream::InputStream { &self.base }
    fn as_InputStream_mut(&mut self) -> &mut crate::java::io::InputStream::InputStream { &mut self.base }
}


