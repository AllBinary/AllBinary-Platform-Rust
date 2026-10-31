// 
// AllBinary Open License Version 1 
// Copyright (c) 2022 AllBinary 
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
// package org.allbinary.graphics.opengles.renderer
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

pub struct RendererStrings {
    pub ON_SURFACE_CREATED: String,
    pub ON_SURFACE_CHANGED: String,
    pub ON_DRAW_FRAME: String,
    pub ON_DRAW_FRAME_THREED: String,
    pub ON_DRAW_FRAME_2D: String,
}

thread_local! { pub static RendererStrings_instance: Option<Rc<RefCell<RendererStrings>>> = RendererStrings::new(); }

impl RendererStrings {

    pub fn new_struct() -> Self {
        let mut this = Self {
            ON_SURFACE_CREATED: String::new(),
            ON_SURFACE_CHANGED: String::new(),
            ON_DRAW_FRAME: String::new(),
            ON_DRAW_FRAME_THREED: String::new(),
            ON_DRAW_FRAME_2D: String::new(),
        };
        this.ON_SURFACE_CREATED = String::from("onSurfaceCreated");
        this.ON_SURFACE_CHANGED = String::from("onSurfaceChanged");
        this.ON_DRAW_FRAME = String::from("onDrawFrame");
        this.ON_DRAW_FRAME_THREED = String::from("onDrawFrameThreed");
        this.ON_DRAW_FRAME_2D = String::from("onDrawFrame2d");
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<RendererStrings>>> {
        return RendererStrings_instance.with(Clone::clone).clone();
    }
}

