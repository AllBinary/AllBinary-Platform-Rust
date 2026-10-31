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
// package org.allbinary.input.automation
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::java::awt::*;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct PointHelper {
}

thread_local! { pub static PointHelper_two: f64 = 2.0; }

impl PointHelper {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getCenterPoint(mut rectangle: Option<Rc<RefCell<Rectangle>>>) -> Option<Rc<RefCell<Point>>> {
        let mut x: f64 = jcall!(rectangle).getX() + jcall!(rectangle).getWidth() / PointHelper_two.with(Clone::clone);
        let mut y: f64 = jcall!(rectangle).getY() + jcall!(rectangle).getHeight() / PointHelper_two.with(Clone::clone);
        return PointFactory::PointFactory::getInstance(((x) as i32), ((y) as i32));
    }
}

