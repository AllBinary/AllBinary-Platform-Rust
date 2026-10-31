// Generated Code Do Not Modify
// package org.allbinary.graphics.form
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use super::CustomDisplayable::CustomDisplayable;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct CustomScreen {
    pub base: CustomDisplayable,
}


impl CustomScreen {

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: CustomDisplayable::new_struct(),
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
}
impl std::ops::Deref for CustomScreen {
    type Target = CustomDisplayable;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for CustomScreen {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

pub trait CustomScreenDyn: crate::org::allbinary::graphics::form::CustomDisplayable::CustomDisplayableDyn {
    fn as_CustomScreen(&self) -> &CustomScreen;
    fn as_CustomScreen_mut(&mut self) -> &mut CustomScreen;
}
impl CustomScreenDyn for CustomScreen {
    fn as_CustomScreen(&self) -> &CustomScreen { self }
    fn as_CustomScreen_mut(&mut self) -> &mut CustomScreen { self }
}

impl crate::org::allbinary::graphics::form::CustomDisplayable::CustomDisplayableDyn for CustomScreen {
    fn as_CustomDisplayable(&self) -> &crate::org::allbinary::graphics::form::CustomDisplayable::CustomDisplayable { &self.base }
    fn as_CustomDisplayable_mut(&mut self) -> &mut crate::org::allbinary::graphics::form::CustomDisplayable::CustomDisplayable { &mut self.base }
}


