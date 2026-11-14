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
use crate::java::io::File::File;
use crate::java::io::FileInputStream::FileInputStream;
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

impl crate::java::io::Closeable::Closeable for AbFileInputStream {
    fn close(&mut self) { crate::java::io::Closeable::Closeable::close(&mut self.base) }
}

impl AbFileInputStream {
    

    pub fn new_struct(mut name: String) -> Self {
        let mut this = Self {
            base: FileInputStream::new_struct(name.clone()),
        };
        // super(...) is in the struct literal above
        this
    }

    pub fn new(mut name: String) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(name))))
    }
    

    // throws FileNotFoundException
    pub fn createFromAbFile(mut file: Option<Rc<RefCell<AbFile>>>) -> Option<Rc<RefCell<dyn AbFileInputStreamDyn>>> {
        return AbFileInputStream::new(jcall!(AbFileNativeUtil::get(file.clone())).getPath()).map(|reference| reference as Rc<RefCell<dyn AbFileInputStreamDyn>>);
    }
    

    // throws FileNotFoundException
    pub fn createFromFile(mut file: Option<Rc<RefCell<File>>>) -> Option<Rc<RefCell<dyn AbFileInputStreamDyn>>> {
        return AbFileInputStream::new(jcall!(file).getPath()).map(|reference| reference as Rc<RefCell<dyn AbFileInputStreamDyn>>);
    }
}
impl std::ops::Deref for AbFileInputStream {
    type Target = FileInputStream;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for AbFileInputStream {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

pub trait AbFileInputStreamDyn: crate::java::io::FileInputStream::FileInputStreamDyn {
    fn as_AbFileInputStream(&self) -> &AbFileInputStream;
    fn as_AbFileInputStream_mut(&mut self) -> &mut AbFileInputStream;
}
impl AbFileInputStreamDyn for AbFileInputStream {
    fn as_AbFileInputStream(&self) -> &AbFileInputStream { self }
    fn as_AbFileInputStream_mut(&mut self) -> &mut AbFileInputStream { self }
}

impl crate::java::io::InputStream::InputStreamDyn for AbFileInputStream {
    fn as_InputStream(&self) -> &crate::java::io::InputStream::InputStream { &self.base.base }
    fn as_InputStream_mut(&mut self) -> &mut crate::java::io::InputStream::InputStream { &mut self.base.base }
}
impl crate::java::io::FileInputStream::FileInputStreamDyn for AbFileInputStream {
    fn as_FileInputStream(&self) -> &crate::java::io::FileInputStream::FileInputStream { &self.base }
    fn as_FileInputStream_mut(&mut self) -> &mut crate::java::io::FileInputStream::FileInputStream { &mut self.base }
}


