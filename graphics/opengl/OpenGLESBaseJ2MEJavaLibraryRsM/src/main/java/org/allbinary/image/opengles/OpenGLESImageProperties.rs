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
// package org.allbinary.image.opengles
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

pub struct OpenGLESImageProperties {
    pub textureID: i32,
    pub scaleX: f32,
    pub scaleY: f32,
    pub scaleX2: f32,
    pub scaleY2: f32,
    pub angle: i32,
    pub redf: f32,
    pub greenf: f32,
    pub bluef: f32,
    pub alphaf: f32,
}


impl OpenGLESImageProperties {

    pub fn new_struct() -> Self {
        let mut this = Self {
            textureID: 0,
            scaleX: 0.0_f32,
            scaleY: 0.0_f32,
            scaleX2: 0.0_f32,
            scaleY2: 0.0_f32,
            angle: 0,
            redf: 0.0_f32,
            greenf: 0.0_f32,
            bluef: 0.0_f32,
            alphaf: 0.0_f32,
        };
        this.textureID = -1;
        this.scaleX = 1.0_f32;
        this.scaleY = 1.0_f32;
        this.scaleX2 = 1.0_f32;
        this.scaleY2 = 1.0_f32;
        this.redf = 1.0_f32;
        this.greenf = 1.0_f32;
        this.bluef = 1.0_f32;
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
}

