// Generated Code Do Not Modify
// package org.osgi.framework
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct BundleException {
    pub base: Exception,
}


impl BundleException {

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: Exception::new_struct(),
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
}
impl std::ops::Deref for BundleException {
    type Target = Exception;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for BundleException {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}



