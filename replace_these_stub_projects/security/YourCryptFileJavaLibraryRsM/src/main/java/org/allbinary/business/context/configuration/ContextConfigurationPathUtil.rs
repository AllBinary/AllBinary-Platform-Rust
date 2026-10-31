// Generated Code Do Not Modify
// package org.allbinary.business.context.configuration
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::logic::io::file::AbFile::AbFile;
use crate::org::allbinary::logic::io::path::AbPath::AbPath;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct ContextConfigurationPathUtil {
}


impl ContextConfigurationPathUtil {

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    // throws Exception
    pub fn getAbFile(mut contextName: String) -> Option<Rc<RefCell<AbFile>>> {
        return None;
    }
    

    // throws Exception
    pub fn getAbPath(mut contextName: String) -> Option<Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>> {
        return None;
    }
    

    // throws Exception
    pub fn getPath() -> String {
        return None;
    }
    

    pub fn getFileName(mut contextName: String) -> String {
        return None;
    }
}

