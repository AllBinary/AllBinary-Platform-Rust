// Generated Code Do Not Modify
// package org.allbinary.logic.control.crypt
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

pub struct SuperCrypt {
}

pub static mut SuperCrypt_KEYMAX: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| 0);

impl SuperCrypt {
    

    pub fn new_struct(mut key: i32) -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new(mut key: i32) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(key))))
    }
    

    pub fn encrypt(&mut self, mut value: String) -> String {
        return None;
    }
    

    pub fn decrypt(&mut self, mut value: String) -> String {
        return None;
    }
}

