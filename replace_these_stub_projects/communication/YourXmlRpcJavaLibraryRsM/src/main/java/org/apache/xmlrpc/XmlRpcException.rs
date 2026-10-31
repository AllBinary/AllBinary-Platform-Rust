// Generated Code Do Not Modify
// package org.apache.xmlrpc
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

pub struct XmlRpcException {
    pub base: Exception,
}


impl XmlRpcException {
    

    pub fn new_struct(mut code: i32, mut message: String) -> Self {
        let mut this = Self {
            base: Exception::new_struct1(message.clone()),
        };
        // super(...) is in the struct literal above
        this
    }

    pub fn new(mut code: i32, mut message: String) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(code, message))))
    }
}
impl std::ops::Deref for XmlRpcException {
    type Target = Exception;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for XmlRpcException {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}



