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
use crate::org::allbinary::logic::io::file::AbFile::AbFile;
use crate::org::allbinary::logic::io::file::AbFileNativeUtil::AbFileNativeUtil;
use super::AbFileInputStream::AbFileInputStream;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct AbFileLocalInputStream {
    pub base: AbFileInputStream,
}

impl crate::java::io::Closeable::Closeable for AbFileLocalInputStream {
    fn close(&mut self) { self.base.close() }
}

impl AbFileLocalInputStream {
    

    pub fn new_struct(mut file: Option<Rc<RefCell<AbFile>>>) -> Self {
        let mut this = Self {
            base: AbFileInputStream::new_struct(jcall!(AbFileNativeUtil::get(file.clone())).getPath()),
        };
        // super(...) is in the struct literal above
        this
    }

    pub fn new(mut file: Option<Rc<RefCell<AbFile>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(file))))
    }
}
impl std::ops::Deref for AbFileLocalInputStream {
    type Target = AbFileInputStream;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for AbFileLocalInputStream {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::logic::io::AbFileInputStream::AbFileInputStreamDyn for AbFileLocalInputStream {
    fn as_AbFileInputStream(&self) -> &crate::org::allbinary::logic::io::AbFileInputStream::AbFileInputStream { &self.base }
    fn as_AbFileInputStream_mut(&mut self) -> &mut crate::org::allbinary::logic::io::AbFileInputStream::AbFileInputStream { &mut self.base }
}
impl crate::java::io::InputStream::InputStreamDyn for AbFileLocalInputStream {
    fn as_InputStream(&self) -> &crate::java::io::InputStream::InputStream { &self.base.base.base }
    fn as_InputStream_mut(&mut self) -> &mut crate::java::io::InputStream::InputStream { &mut self.base.base.base }
}
impl crate::java::io::FileInputStream::FileInputStreamDyn for AbFileLocalInputStream {
    fn as_FileInputStream(&self) -> &crate::java::io::FileInputStream::FileInputStream { &self.base.base }
    fn as_FileInputStream_mut(&mut self) -> &mut crate::java::io::FileInputStream::FileInputStream { &mut self.base.base }
}


