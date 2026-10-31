// Generated Code Do Not Modify
// package org.allbinary.device
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use super::OpenGLESGraphics::OpenGLESGraphics;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct OpenGLESGraphicsFactory {
}


impl OpenGLESGraphicsFactory {

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getOpenGLESGraphicsInstance(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::device::OpenGLESGraphics::OpenGLESGraphicsDyn>>> {
        return OpenGLESNullGraphicsFactory::OpenGLESNullGraphicsFactory::getInstance();
    }
}
pub trait OpenGLESGraphicsFactoryDyn {
    fn as_OpenGLESGraphicsFactory(&self) -> &OpenGLESGraphicsFactory;
    fn as_OpenGLESGraphicsFactory_mut(&mut self) -> &mut OpenGLESGraphicsFactory;
    fn getOpenGLESGraphicsInstance(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::device::OpenGLESGraphics::OpenGLESGraphicsDyn>>> { OpenGLESGraphicsFactory::getOpenGLESGraphicsInstance(self.as_OpenGLESGraphicsFactory_mut()) }
}
impl OpenGLESGraphicsFactoryDyn for OpenGLESGraphicsFactory {
    fn as_OpenGLESGraphicsFactory(&self) -> &OpenGLESGraphicsFactory { self }
    fn as_OpenGLESGraphicsFactory_mut(&mut self) -> &mut OpenGLESGraphicsFactory { self }
}


