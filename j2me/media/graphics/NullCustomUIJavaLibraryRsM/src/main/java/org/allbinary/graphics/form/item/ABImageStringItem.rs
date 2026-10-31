// Generated Code Do Not Modify
// package org.allbinary.graphics.form.item
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

pub struct ABImageStringItem {
}


impl ABImageStringItem {

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
}
pub trait ABImageStringItemDyn {
    fn as_ABImageStringItem(&self) -> &ABImageStringItem;
    fn as_ABImageStringItem_mut(&mut self) -> &mut ABImageStringItem;
}
impl ABImageStringItemDyn for ABImageStringItem {
    fn as_ABImageStringItem(&self) -> &ABImageStringItem { self }
    fn as_ABImageStringItem_mut(&mut self) -> &mut ABImageStringItem { self }
}


