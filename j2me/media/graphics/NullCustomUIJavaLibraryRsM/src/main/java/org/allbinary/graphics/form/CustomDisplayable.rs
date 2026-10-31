// Generated Code Do Not Modify
// package org.allbinary.graphics.form
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

pub struct CustomDisplayable {
}


impl CustomDisplayable {

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn setTitle(&mut self, mut title: String) {
    }
}
pub trait CustomDisplayableDyn {
    fn as_CustomDisplayable(&self) -> &CustomDisplayable;
    fn as_CustomDisplayable_mut(&mut self) -> &mut CustomDisplayable;
    fn setTitle(&mut self, mut title: String) { CustomDisplayable::setTitle(self.as_CustomDisplayable_mut(), title) }
}
impl CustomDisplayableDyn for CustomDisplayable {
    fn as_CustomDisplayable(&self) -> &CustomDisplayable { self }
    fn as_CustomDisplayable_mut(&mut self) -> &mut CustomDisplayable { self }
}


