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
// package org.allbinary.ag.game.vector
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

pub struct HealthVectorData {
}

pub static mut HealthVectorData_points: std::cell::LazyCell<Vec<Vec<Vec<i32>>>> = std::cell::LazyCell::new(|| vec![vec![vec![2, 1], vec![3, 2], vec![3, 2], vec![4, 1], vec![4, 1], vec![5, 1], vec![5, 1], vec![6, 2], vec![6, 2], vec![6, 3], vec![6, 3], vec![3, 6], vec![3, 6], vec![0, 3], vec![0, 3], vec![0, 2], vec![0, 2], vec![1, 1], vec![1, 1], vec![2, 1], vec![1000, 1000]]]);

impl HealthVectorData {

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
}

