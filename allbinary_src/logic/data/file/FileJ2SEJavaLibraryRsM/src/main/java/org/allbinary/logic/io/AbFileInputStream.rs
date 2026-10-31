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
// use crate::java::io::File::File;
// use crate::java::io::FileInputStream::FileInputStream;
// use crate::java::io::FileNotFoundException::FileNotFoundException;
use crate::org::allbinary::logic::io::file::AbFile::AbFile;
use crate::org::allbinary::logic::io::file::AbFileNativeUtil::AbFileNativeUtil;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct AbFileInputStream {
    pub base: FileInputStream,
}


impl AbFileInputStream {
    

    pub fn new_struct1(mut name: String) -> Self {
        let mut this = Self {
            base: FileInputStream::new_struct1(name.clone()),
        };
        // super(...) is in the struct literal above
        this
    }

    pub fn new1(mut name: String) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct1(name))))
    }
    

    pub fn new_struct1(mut file: Option<Rc<RefCell<AbFile>>>) -> Self {
        let mut this = Self {
            base: FileInputStream::new_struct1(AbFileNativeUtil::get(file.clone())),
        };
        // super(...) is in the struct literal above
        this
    }

    pub fn new1(mut file: Option<Rc<RefCell<AbFile>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct1(file))))
    }
    

    pub fn new_struct1(mut file: Option<Rc<RefCell<File>>>) -> Self {
        let mut this = Self {
            base: FileInputStream::new_struct1(file.clone()),
        };
        // super(...) is in the struct literal above
        this
    }

    pub fn new1(mut file: Option<Rc<RefCell<File>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct1(file))))
    }
}
impl std::ops::Deref for AbFileInputStream {
    type Target = FileInputStream;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for AbFileInputStream {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

pub trait AbFileInputStreamDyn {
    fn as_AbFileInputStream(&self) -> &AbFileInputStream;
    fn as_AbFileInputStream_mut(&mut self) -> &mut AbFileInputStream;
}
impl AbFileInputStreamDyn for AbFileInputStream {
    fn as_AbFileInputStream(&self) -> &AbFileInputStream { self }
    fn as_AbFileInputStream_mut(&mut self) -> &mut AbFileInputStream { self }
}



