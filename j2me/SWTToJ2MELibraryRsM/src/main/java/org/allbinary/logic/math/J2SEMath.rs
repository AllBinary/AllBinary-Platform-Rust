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
// package org.allbinary.logic.math
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

pub struct J2SEMath {
}

thread_local! { pub static J2SEMath_instance: Option<Rc<RefCell<J2SEMath>>> = J2SEMath::new(); }

impl J2SEMath {

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<J2SEMath>>> {
        return J2SEMath_instance.with(Clone::clone).clone();
    }
    

    pub fn round(&mut self, mut value: f32) -> i32 {
        return ((value.clone()) as f64).round() as i64;
    }
    

    pub fn abs(&mut self, mut value: f32) -> f32 {
        return (if (value < 0.0_f32) { -value } else { value });
    }
    

    pub fn absdouble(&mut self, mut value: f64) -> f64 {
        return (if (value < 0.0) { -value } else { value });
    }
}

