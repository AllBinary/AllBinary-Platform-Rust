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
// package org.allbinary.image
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::javax::microedition::khronos::opengles::GL::GL;
// use crate::javax::microedition::lcdui::Graphics::Graphics;
// use crate::javax::microedition::lcdui::Image::Image;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct PreResourceImageUtil {
}

thread_local! { pub static PreResourceImageUtil_instance: Option<Rc<RefCell<PreResourceImageUtil>>> = PreResourceImageUtil::new(); }

impl PreResourceImageUtil {

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<PreResourceImageUtil>>> {
        return PreResourceImageUtil_instance.with(Clone::clone).clone();
    }
    

    pub fn encapsulate(&mut self, mut image: Option<Rc<RefCell<Image>>>) -> Option<Rc<RefCell<Image>>> {
        return image.clone();
    }
    

    pub fn updateGL(&mut self, mut gl: Option<Rc<RefCell<dyn GL>>>, mut image: Option<Rc<RefCell<Image>>>) {
    }
    

    pub fn update(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>, mut image: Option<Rc<RefCell<Image>>>) {
    }
}

