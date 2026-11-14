// Generated Code Do Not Modify
// package org.allbinary.image.opengles
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::javax::microedition::khronos::opengles::GL::GL;
use crate::java::lang::Exception::*;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub trait OpenGLSurfaceChangedInterface: std::any::Any + crate::java::lang::Object::JavaObjectToString {
    

    // throws Exception
    fn set(&mut self, gl: Option<Rc<RefCell<dyn GL>>>);
}


