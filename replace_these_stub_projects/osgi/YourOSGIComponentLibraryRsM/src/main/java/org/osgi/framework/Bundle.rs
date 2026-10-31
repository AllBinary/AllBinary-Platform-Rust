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

pub trait Bundle {
    

    fn getSymbolicName(&mut self) -> String;
    

    // throws BundleException
    fn start(&mut self);
    

    // throws BundleException
    // Java overloaded method: Rust needs a different name
    fn start1(&mut self, options: i32);

    fn toString(&mut self) -> String { String::from(std::any::type_name::<Self>()) }
}


