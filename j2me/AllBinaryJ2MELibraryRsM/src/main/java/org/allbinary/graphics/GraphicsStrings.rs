// 
// AllBinary Open License Version 1 
// Copyright (c) 2025 AllBinary 
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
// package org.allbinary.graphics
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

pub struct GraphicsStrings {
    pub ANIMATION: String,
    pub ANGLE: String,
    pub MOVEMENT_ANGLE: String,
    pub ROTATION: String,
    pub OPACITY: String,
    pub HTML: String,
    pub WIDTH: String,
    pub HEIGHT: String,
}

thread_local! { pub static GraphicsStrings_instance: Option<Rc<RefCell<GraphicsStrings>>> = GraphicsStrings::new(); }

impl GraphicsStrings {

    pub fn new_struct() -> Self {
        let mut this = Self {
            ANIMATION: String::new(),
            ANGLE: String::new(),
            MOVEMENT_ANGLE: String::new(),
            ROTATION: String::new(),
            OPACITY: String::new(),
            HTML: String::new(),
            WIDTH: String::new(),
            HEIGHT: String::new(),
        };
        this.ANIMATION = String::from("animation");
        this.ANGLE = String::from("angle");
        this.MOVEMENT_ANGLE = String::from("movement_angle");
        this.ROTATION = String::from("rotation");
        this.OPACITY = String::from("opacity");
        this.HTML = String::from("HTML");
        this.WIDTH = String::from("width");
        this.HEIGHT = String::from("height");
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<GraphicsStrings>>> {
        return GraphicsStrings_instance.with(Clone::clone).clone();
    }
}

