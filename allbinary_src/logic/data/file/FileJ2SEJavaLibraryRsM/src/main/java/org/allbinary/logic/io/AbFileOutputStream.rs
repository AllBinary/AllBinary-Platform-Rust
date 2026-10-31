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
use crate::org::allbinary::logic::io::file::AbFile::AbFile;
use crate::org::allbinary::logic::io::file::AbFileNativeUtil::AbFileNativeUtil;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct AbFileOutputStream {
    pub base: FileOutputStream,
}


impl AbFileOutputStream {
    

    // throws FileNotFoundException
    pub fn createFromAbFile(mut file: Option<Rc<RefCell<AbFile>>>) -> Option<Rc<RefCell<AbFileOutputStream>>> {
        return AbFileOutputStream::new(jcall!(file).getPath(), false);
    }
    

    // throws FileNotFoundException
    pub fn createFromAbFileAppend(mut file: Option<Rc<RefCell<AbFile>>>, mut append: bool) -> Option<Rc<RefCell<AbFileOutputStream>>> {
        return AbFileOutputStream::new(jcall!(file).getPath(), append.clone());
    }
    

    // throws FileNotFoundException
    pub fn createFromFilePath(mut name: String) -> Option<Rc<RefCell<AbFileOutputStream>>> {
        return AbFileOutputStream::new(name.clone(), false);
    }
    

    // throws FileNotFoundException
    pub fn createFromFilePathAppend(mut name: String, mut append: bool) -> Option<Rc<RefCell<AbFileOutputStream>>> {
        return AbFileOutputStream::new(name.clone(), append.clone());
    }
    

    pub fn new_struct(mut name: String, mut append: bool) -> Self {
        let mut this = Self {
            base: FileOutputStream::new_struct2(name.clone(), append.clone()),
        };
        // super(...) is in the struct literal above
        this
    }

    pub fn new(mut name: String, mut append: bool) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(name, append))))
    }
}
impl std::ops::Deref for AbFileOutputStream {
    type Target = FileOutputStream;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for AbFileOutputStream {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}



