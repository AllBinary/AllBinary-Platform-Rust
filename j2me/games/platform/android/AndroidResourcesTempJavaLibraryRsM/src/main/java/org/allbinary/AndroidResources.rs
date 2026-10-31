// Generated Code Do Not Modify
// package org.allbinary
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::game::testgamedemo::R::R;
use super::drawable::drawable;
use super::id::id;
use super::layout::layout;
use super::raw::raw;
use super::string::string;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct AndroidResources {
    pub drawable: Option<Rc<RefCell<drawable>>>,
    pub id: Option<Rc<RefCell<id>>>,
    pub layout: Option<Rc<RefCell<layout>>>,
    pub raw: Option<Rc<RefCell<raw>>>,
    pub string: Option<Rc<RefCell<string>>>,
}

thread_local! { pub static AndroidResources_instance: Option<Rc<RefCell<AndroidResources>>> = AndroidResources::new(); }

impl AndroidResources {

    pub fn new_struct() -> Self {
        let mut this = Self {
            drawable: None,
            id: None,
            layout: None,
            raw: None,
            string: None,
        };
        this.drawable = drawable::new();
        this.id = id::new();
        this.layout = layout::new();
        this.raw = raw::new();
        this.string = string::new();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<AndroidResources>>> {
        return AndroidResources_instance.with(Clone::clone).clone();
    }
}

