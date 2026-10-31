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

pub struct ImageItem {
}


impl ImageItem {

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
}
pub trait ImageItemDyn {
    fn as_ImageItem(&self) -> &ImageItem;
    fn as_ImageItem_mut(&mut self) -> &mut ImageItem;
}
impl ImageItemDyn for ImageItem {
    fn as_ImageItem(&self) -> &ImageItem { self }
    fn as_ImageItem_mut(&mut self) -> &mut ImageItem { self }
}


