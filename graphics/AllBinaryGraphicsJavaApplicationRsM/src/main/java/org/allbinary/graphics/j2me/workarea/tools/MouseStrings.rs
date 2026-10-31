//
// AllBinary Open License Version 1
// Copyright (c) 2011 AllBinary
// 
// By agreeing to this license you and any business entity you represent are
// legally bound to the AllBinary Open License Version 1 legal agreement.
// 
// You may obtain the AllBinary Open License Version 1 legal agreement from
// AllBinary or the root directory of AllBinary's AllBinary Platform repository.
// 
// Created By: Travis Berthelot
//  
// Generated Code Do Not Modify
// package org.allbinary.graphics.j2me.workarea.tools
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::string::CommonSeps::CommonSeps;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct MouseStrings {
    pub MOUSE_PRESSED: String,
    pub MOUSE_RELEASED: String,
    pub MOUSE_CLICKED: String,
    pub MOUSE_DRAGGED: String,
    pub MOUSE_MOVED: String,
    pub MOUSE_PRESSED_LABEL: String,
    pub MOUSE_RELEASED_LABEL: String,
    pub MOUSE_CLICKED_LABEL: String,
    pub MOUSE_DRAGGED_LABEL: String,
    pub MOUSE_MOVED_LABEL: String,
}

thread_local! { pub static MouseStrings_instance: Option<Rc<RefCell<MouseStrings>>> = MouseStrings::new(); }

impl MouseStrings {

    pub fn new_struct() -> Self {
        let mut this = Self {
            MOUSE_PRESSED: String::new(),
            MOUSE_RELEASED: String::new(),
            MOUSE_CLICKED: String::new(),
            MOUSE_DRAGGED: String::new(),
            MOUSE_MOVED: String::new(),
            MOUSE_PRESSED_LABEL: String::new(),
            MOUSE_RELEASED_LABEL: String::new(),
            MOUSE_CLICKED_LABEL: String::new(),
            MOUSE_DRAGGED_LABEL: String::new(),
            MOUSE_MOVED_LABEL: String::new(),
        };
        this.MOUSE_PRESSED = String::from("mousePressed");
        this.MOUSE_RELEASED = String::from("mouseReleased");
        this.MOUSE_CLICKED = String::from("mouseClicked");
        this.MOUSE_DRAGGED = String::from("mouseDragged");
        this.MOUSE_MOVED = String::from("mouseMoved");
        this.MOUSE_PRESSED_LABEL = format!("{}{}", String::from("MousePressed"), jcall!(CommonSeps::getInstance()).COLON_SEP);
        this.MOUSE_RELEASED_LABEL = format!("{}{}", String::from("MouseReleased"), jcall!(CommonSeps::getInstance()).COLON_SEP);
        this.MOUSE_CLICKED_LABEL = format!("{}{}", String::from("MouseClicked"), jcall!(CommonSeps::getInstance()).COLON_SEP);
        this.MOUSE_DRAGGED_LABEL = format!("{}{}", String::from("MouseDragged"), jcall!(CommonSeps::getInstance()).COLON_SEP);
        this.MOUSE_MOVED_LABEL = format!("{}{}", String::from("MouseMoved"), jcall!(CommonSeps::getInstance()).COLON_SEP);
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<MouseStrings>>> {
        return MouseStrings_instance.with(Clone::clone).clone();
    }
}

