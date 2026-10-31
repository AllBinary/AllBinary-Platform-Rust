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

pub struct E {
}

thread_local! { pub static E_e: Option<Rc<RefCell<dyn EDyn>>> = E::new().map(|reference| reference as Rc<RefCell<dyn EDyn>>); }

impl E {

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn newInstance(&mut self) -> Option<Rc<RefCell<dyn EDyn>>> {
        return E_e.with(Clone::clone).clone();
    }
}
pub trait EDyn {
    fn as_E(&self) -> &E;
    fn as_E_mut(&mut self) -> &mut E;
    fn newInstance(&mut self) -> Option<Rc<RefCell<dyn EDyn>>> { E::newInstance(self.as_E_mut()) }
}
impl EDyn for E {
    fn as_E(&self) -> &E { self }
    fn as_E_mut(&mut self) -> &mut E { self }
}


