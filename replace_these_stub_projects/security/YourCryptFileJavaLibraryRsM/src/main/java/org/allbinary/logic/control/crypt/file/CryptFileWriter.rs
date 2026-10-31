// Generated Code Do Not Modify
// package org.allbinary.logic.control.crypt.file
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::logic::io::path::AbPath::AbPath;
use crate::org::w3c::dom::Document::Document;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct CryptFileWriter {
}


impl CryptFileWriter {
    

    pub fn new_struct(mut decryptedExtension: String, mut encryptedExtension: String) -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new(mut decryptedExtension: String, mut encryptedExtension: String) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(decryptedExtension, encryptedExtension))))
    }
    

    // throws Exception
    pub fn write(&mut self, mut fileAbPath: Option<Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>>, mut document: Option<Rc<RefCell<Document>>>) {
    }
}

