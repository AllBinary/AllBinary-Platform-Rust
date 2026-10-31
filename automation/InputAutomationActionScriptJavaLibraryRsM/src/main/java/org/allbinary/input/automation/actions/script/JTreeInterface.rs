// Generated Code Do Not Modify
// package org.allbinary.input.automation.actions.script
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use super::JTree::JTree;
use super::swing::swing;
use super::javax::javax;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub trait JTreeInterface {
    

    fn updateJTree(&mut self);
    

    fn getActionScriptJTree(&mut self) -> Option<Rc<RefCell<JTree>>>;

    fn toString(&mut self) -> String { String::from(std::any::type_name::<Self>()) }
}


