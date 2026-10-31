// Generated Code Do Not Modify
// package org.allbinary.game.displayable.canvas
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::javax::microedition::lcdui::Canvas::Canvas;
// use crate::javax::microedition::lcdui::Graphics::Graphics;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

//Temp should not be in any build
pub struct NullDisplayable {
    pub base: Canvas,
}

thread_local! { pub static NullDisplayable_SINGLETON: Option<Rc<RefCell<NullDisplayable>>> = NullDisplayable::new(); }

impl NullDisplayable {
    

    pub fn getInstance() -> Option<Rc<RefCell<NullDisplayable>>> {
        return NullDisplayable_SINGLETON.with(Clone::clone).clone();
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: Canvas::new_struct(),
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn paint(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) {
    }
}
impl std::ops::Deref for NullDisplayable {
    type Target = Canvas;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for NullDisplayable {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}



