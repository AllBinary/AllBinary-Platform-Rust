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
use crate::java::io::DataInputStream::DataInputStream;
use crate::java::io::InputStream::InputStream;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct AbDataInputStream {
    pub base: DataInputStream,
}

impl crate::java::io::Closeable::Closeable for AbDataInputStream {
    fn close(&mut self) { self.base.close() }
}

impl AbDataInputStream {
    

    pub fn new_struct(mut out: Option<Rc<RefCell<dyn crate::java::io::InputStream::InputStreamDyn>>>) -> Self {
        let mut this = Self {
            base: DataInputStream::new_struct(out.clone()),
        };
        // super(...) is in the struct literal above
        this
    }

    pub fn new(mut out: Option<Rc<RefCell<dyn crate::java::io::InputStream::InputStreamDyn>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(out))))
    }
}
impl std::ops::Deref for AbDataInputStream {
    type Target = DataInputStream;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for AbDataInputStream {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::java::io::InputStream::InputStreamDyn for AbDataInputStream {
    fn as_InputStream(&self) -> &crate::java::io::InputStream::InputStream { &self.base.base }
    fn as_InputStream_mut(&mut self) -> &mut crate::java::io::InputStream::InputStream { &mut self.base.base }
}


