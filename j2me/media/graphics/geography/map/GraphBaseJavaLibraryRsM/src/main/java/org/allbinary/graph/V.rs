// Generated Code Do Not Modify
// package org.allbinary.graph
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

pub struct V {
}


impl V {

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getId(&mut self) -> i32 {
        return 0;
    }
}
pub trait VDyn {
    fn as_V(&self) -> &V;
    fn as_V_mut(&mut self) -> &mut V;
    fn getId(&mut self) -> i32 { V::getId(self.as_V_mut()) }
}
impl VDyn for V {
    fn as_V(&self) -> &V { self }
    fn as_V_mut(&mut self) -> &mut V { self }
}


