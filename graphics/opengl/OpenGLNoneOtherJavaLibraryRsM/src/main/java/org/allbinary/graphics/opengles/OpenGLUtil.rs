// Generated Code Do Not Modify
// package org.allbinary.graphics.opengles
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use crate::Runnable;
use super::*;
// use crate::javax::microedition::lcdui::Image::Image;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct OpenGLUtil {
}

thread_local! { pub static OpenGLUtil_instance: Option<Rc<RefCell<OpenGLUtil>>> = OpenGLUtil::new(); }

impl OpenGLUtil {

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<OpenGLUtil>>> {
        return OpenGLUtil_instance.with(Clone::clone).clone();
    }
    

    pub fn addImage(&mut self, mut image: Option<Rc<RefCell<Image>>>) -> Option<Rc<RefCell<Image>>> {
        return image.clone();
    }
    

    pub fn clear(&mut self) {
    }
    

    pub fn addRunnable(&mut self, mut runnable: Option<Rc<RefCell<dyn Runnable>>>) {
    }
}

