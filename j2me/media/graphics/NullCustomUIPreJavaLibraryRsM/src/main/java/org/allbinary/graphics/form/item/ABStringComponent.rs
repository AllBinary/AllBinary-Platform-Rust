// Generated Code Do Not Modify
// package org.allbinary.graphics.form.item
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::graphics::color::BasicColor::BasicColor;
use crate::org::allbinary::graphics::color::BasicColorFactory::BasicColorFactory;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct ABStringComponent {
}

thread_local! { pub static ABStringComponent_NULL_STRING_COMPONENT: Option<Rc<RefCell<ABStringComponent>>> = ABStringComponent::new(jcall!(BasicColorFactory::getInstance()).BLACK.clone(), jcall!(BasicColorFactory::getInstance()).WHITE.clone()); }

impl ABStringComponent {
    

    pub fn getBackgroundBasicColor(&mut self) -> Option<Rc<RefCell<BasicColor>>> {
        return jcall!(BasicColorFactory::getInstance()).WHITE.clone();
    }
    

    pub fn new_struct(mut backgroundBasicColor: Option<Rc<RefCell<BasicColor>>>, mut foregroundBasicColor: Option<Rc<RefCell<BasicColor>>>) -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new(mut backgroundBasicColor: Option<Rc<RefCell<BasicColor>>>, mut foregroundBasicColor: Option<Rc<RefCell<BasicColor>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(backgroundBasicColor, foregroundBasicColor))))
    }
    

    pub fn getForegroundBasicColor(&mut self) -> Option<Rc<RefCell<BasicColor>>> {
        return jcall!(BasicColorFactory::getInstance()).WHITE.clone();
    }
    

    pub fn setForegroundBasicColor(&mut self, mut foregroundBasicColor: Option<Rc<RefCell<BasicColor>>>) {
    }
}

