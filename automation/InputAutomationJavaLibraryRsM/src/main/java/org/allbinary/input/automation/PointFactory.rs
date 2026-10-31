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

pub struct PointFactory {
}

pub static mut PointFactory_pointArray: std::cell::LazyCell<Vec<Vec<Option<Rc<RefCell<Point>>>>>> = std::cell::LazyCell::new(|| vec![vec![None; (3000) as usize]; (3000) as usize]);

impl PointFactory {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance(mut x: i32, mut y: i32) -> Option<Rc<RefCell<Point>>> {
        let mut point: Option<Rc<RefCell<Point>>> = unsafe { PointFactory_pointArray.clone() }[(x) as usize][(y) as usize].clone();
        if point.is_none() {
            unsafe { *PointFactory_pointArray = Point::new(x.clone(), y.clone()) };
            point = unsafe { PointFactory_pointArray.clone() }[(x) as usize][(y) as usize];
        }
        return point.clone();
    }
}

