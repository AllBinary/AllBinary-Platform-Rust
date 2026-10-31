// Generated Code Do Not Modify
// package org.allbinary.graphics.form
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

pub struct ScreenInfo {
    pub SMALL_WIDTH: i32,
    pub SMALL_HEIGHT: i32,
    pub MEDIUM_WIDTH: i32,
    pub MEDIUM_HEIGHT: i32,
}

thread_local! { pub static ScreenInfo_instance: Option<Rc<RefCell<ScreenInfo>>> = ScreenInfo::new(); }

impl ScreenInfo {

    pub fn new_struct() -> Self {
        let mut this = Self {
            SMALL_WIDTH: 0,
            SMALL_HEIGHT: 0,
            MEDIUM_WIDTH: 0,
            MEDIUM_HEIGHT: 0,
        };
        this.SMALL_WIDTH = 320;
        this.SMALL_HEIGHT = 240;
        this.MEDIUM_WIDTH = 800;
        this.MEDIUM_HEIGHT = 600;
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<ScreenInfo>>> {
        return ScreenInfo_instance.with(Clone::clone).clone();
    }
}

