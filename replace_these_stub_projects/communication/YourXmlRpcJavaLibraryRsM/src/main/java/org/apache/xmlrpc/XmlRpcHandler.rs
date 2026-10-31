// Generated Code Do Not Modify
// package org.apache.xmlrpc
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub trait XmlRpcHandler {
    

    // throws Exception
    fn execute(&mut self, method: String, params: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) -> Option<Rc<RefCell<Box<dyn std::any::Any>>>>;

    fn toString(&mut self) -> String { String::from(std::any::type_name::<Self>()) }
}


