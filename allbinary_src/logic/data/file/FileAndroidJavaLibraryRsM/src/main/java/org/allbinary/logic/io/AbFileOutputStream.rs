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
// use crate::java::io::FileNotFoundException::FileNotFoundException;
// use crate::java::io::FileOutputStream::FileOutputStream;
// use crate::java::io::IOException::IOException;
// use crate::java::io::OutputStream::OutputStream;
// use crate::java::nio::channels::FileChannel::FileChannel;
use crate::org::allbinary::logic::io::file::AbFile::AbFile;
use crate::org::allbinary::logic::io::file::AbFileNativeUtil::AbFileNativeUtil;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct AbFileOutputStream {
    pub base: OutputStream,
    pub fileOutputStream: Option<Rc<RefCell<FileOutputStream>>>,
}


impl AbFileOutputStream {
    

    // throws FileNotFoundException
    pub fn createFromAbFile(mut file: Option<Rc<RefCell<AbFile>>>) -> Option<Rc<RefCell<AbFileOutputStream>>> {
        return AbFileOutputStream::new(FileOutputStream::new(AbFileNativeUtil::get(file.clone())));
    }
    

    // throws FileNotFoundException
    pub fn createFromAbFileAppend(mut file: Option<Rc<RefCell<AbFile>>>, mut append: bool) -> Option<Rc<RefCell<AbFileOutputStream>>> {
        return AbFileOutputStream::new(FileOutputStream::new(jcall!(file).getPath()));
    }
    

    // throws FileNotFoundException
    pub fn createFromFilePath(mut name: String) -> Option<Rc<RefCell<AbFileOutputStream>>> {
        return AbFileOutputStream::new(FileOutputStream::new(name.clone()));
    }
    

    // throws FileNotFoundException
    pub fn createFromFilePathAppend(mut name: String, mut append: bool) -> Option<Rc<RefCell<AbFileOutputStream>>> {
        return AbFileOutputStream::new(FileOutputStream::new(name.clone()));
    }
    

    pub fn new_struct(mut fileOutputStream: Option<Rc<RefCell<FileOutputStream>>>) -> Self {
        let mut this = Self {
            base: OutputStream::new_struct(),
            fileOutputStream: None,
        };
        this.fileOutputStream = fileOutputStream.clone();
        this
    }

    pub fn new(mut fileOutputStream: Option<Rc<RefCell<FileOutputStream>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(fileOutputStream))))
    }
    

    // throws IOException
    pub fn close(&mut self) {
        jcall!(self.fileOutputStream).close();
    }
    

    pub fn getChannel(&mut self) -> Option<Rc<RefCell<FileChannel>>> {
        return jcall!(self.fileOutputStream).getChannel();
    }
    

    // throws IOException
    pub fn write(&mut self, mut buffer: Vec<i8>) {
        jcall!(self.fileOutputStream).write(buffer.clone());
    }
    

    // throws IOException
    // Java overloaded method: Rust needs a different name
    pub fn write3(&mut self, mut buffer: Vec<i8>, mut offset: i32, mut count: i32) {
        jcall!(self.fileOutputStream).write(buffer.clone(), offset.clone(), count.clone());
    }
    

    // throws IOException
    // Java overloaded method: Rust needs a different name
    pub fn write(&mut self, mut b: i32) {
        jcall!(self.fileOutputStream).write(b.clone());
    }
}
impl std::ops::Deref for AbFileOutputStream {
    type Target = OutputStream;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for AbFileOutputStream {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}



