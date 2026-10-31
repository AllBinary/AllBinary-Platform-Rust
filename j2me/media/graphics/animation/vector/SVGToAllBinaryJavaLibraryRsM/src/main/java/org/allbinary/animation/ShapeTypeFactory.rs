// 
// AllBinary Open License Version 1 
// Copyright (c) 2026 AllBinary 
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
// package org.allbinary.animation
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use super::ShapeType::ShapeType;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct ShapeTypeFactory {
    pub KEY_LINE: String,
    pub KEY_CIRCLE: String,
    pub KEY_RECTANGLE: String,
    pub KEY_RECT: String,
    pub LINE: Option<Rc<RefCell<ShapeType>>>,
    pub CIRCLE: Option<Rc<RefCell<ShapeType>>>,
    pub RECTANGLE: Option<Rc<RefCell<ShapeType>>>,
}

thread_local! { pub static ShapeTypeFactory_instance: Option<Rc<RefCell<ShapeTypeFactory>>> = ShapeTypeFactory::new(); }

impl ShapeTypeFactory {

    pub fn new_struct() -> Self {
        let mut this = Self {
            KEY_LINE: String::new(),
            KEY_CIRCLE: String::new(),
            KEY_RECTANGLE: String::new(),
            KEY_RECT: String::new(),
            LINE: None,
            CIRCLE: None,
            RECTANGLE: None,
        };
        this.KEY_LINE = String::from("line");
        this.KEY_CIRCLE = String::from("circle");
        this.KEY_RECTANGLE = String::from("rectangle");
        this.KEY_RECT = String::from("rect");
        this.LINE = ShapeType::new(this.KEY_LINE.clone());
        this.CIRCLE = ShapeType::new(this.KEY_CIRCLE.clone());
        this.RECTANGLE = ShapeType::new(this.KEY_RECTANGLE.clone());
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<ShapeTypeFactory>>> {
        return ShapeTypeFactory_instance.with(Clone::clone).clone();
    }
}

