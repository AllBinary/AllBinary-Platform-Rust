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
// package org.allbinary.media.image
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::java::awt::Color::Color;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct ImageJ2SEUtil {
    pub TRANSPARENT_COLOR: Option<Rc<RefCell<Color>>>,
}

thread_local! { pub static ImageJ2SEUtil_instance: Option<Rc<RefCell<ImageJ2SEUtil>>> = ImageJ2SEUtil::new(); }

impl ImageJ2SEUtil {

    pub fn new_struct() -> Self {
        let mut this = Self {
            TRANSPARENT_COLOR: None,
        };
        this.TRANSPARENT_COLOR = Color::new(0, 0, 0, 0);
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<ImageJ2SEUtil>>> {
        return ImageJ2SEUtil_instance.with(Clone::clone).clone();
    }
}

