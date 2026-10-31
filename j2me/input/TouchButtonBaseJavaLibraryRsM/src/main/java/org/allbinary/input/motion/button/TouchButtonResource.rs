// 
// To change this license header, choose License Headers in Project Properties. 
// To change this template file, choose Tools | Templates 
// and open the template in the editor.  
// Generated Code Do Not Modify
// package org.allbinary.input.motion.button
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

pub struct TouchButtonResource {
    pub RESOURCE: String,
    pub HINT: String,
}


impl TouchButtonResource {
    

    pub fn new_struct(mut resource: String, mut hint: String) -> Self {
        let mut this = Self {
            RESOURCE: String::new(),
            HINT: String::new(),
        };
        this.RESOURCE = resource.clone();
        this.HINT = hint.clone();
        this
    }

    pub fn new(mut resource: String, mut hint: String) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(resource, hint))))
    }
}
pub trait TouchButtonResourceDyn {
    fn as_TouchButtonResource(&self) -> &TouchButtonResource;
    fn as_TouchButtonResource_mut(&mut self) -> &mut TouchButtonResource;
}
impl TouchButtonResourceDyn for TouchButtonResource {
    fn as_TouchButtonResource(&self) -> &TouchButtonResource { self }
    fn as_TouchButtonResource_mut(&mut self) -> &mut TouchButtonResource { self }
}


