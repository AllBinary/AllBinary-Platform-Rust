// Generated Code Do Not Modify
// package org.allbinary.logic.control.crypt.file
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::java::io::InputStream::InputStream;
use crate::org::allbinary::logic::io::path::AbPath::AbPath;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct CryptFileReader {
}


impl CryptFileReader {
    

    pub fn new_struct(mut decryptedExtension: String, mut encryptedExtension: String) -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new(mut decryptedExtension: String, mut encryptedExtension: String) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(decryptedExtension, encryptedExtension))))
    }
    

    // throws Exception
    pub fn get(&mut self, mut fileAbPath: Option<Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>>) -> String {
        return None;
    }
    

    // throws Exception
    pub fn getInputStream(&mut self, mut fileAbPath: Option<Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>>) -> Option<Rc<RefCell<InputStream>>> {
        return None;
    }
}

