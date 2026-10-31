// Generated Code Do Not Modify
// package org.eclipse.swt.graphics
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

pub struct Resource {
}


impl Resource {

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn dispose(&mut self) {
    }
    

    pub fn isDisposed(&mut self) -> bool {
        return false;
    }
}
pub trait ResourceDyn {
    fn as_Resource(&self) -> &Resource;
    fn as_Resource_mut(&mut self) -> &mut Resource;
    fn dispose(&mut self) { Resource::dispose(self.as_Resource_mut()) }
    fn isDisposed(&mut self) -> bool { Resource::isDisposed(self.as_Resource_mut()) }
}
impl ResourceDyn for Resource {
    fn as_Resource(&self) -> &Resource { self }
    fn as_Resource_mut(&mut self) -> &mut Resource { self }
}


