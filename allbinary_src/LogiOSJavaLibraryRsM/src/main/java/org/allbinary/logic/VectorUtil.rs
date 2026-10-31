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
// package org.allbinary.logic
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use crate::Vector;
use super::*;
// use crate::java::util::Vector::Vector;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct VectorUtil {
}

thread_local! { pub static VectorUtil_instance: Option<Rc<RefCell<VectorUtil>>> = VectorUtil::new(); }

impl VectorUtil {

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<VectorUtil>>> {
        return VectorUtil_instance.with(Clone::clone).clone();
    }
    

    pub fn getSize(&mut self, mut anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) -> i32 {
        let mut vector: Option<Rc<RefCell<Vector<Option<Rc<RefCell<Box<dyn std::any::Any>>>>>>>> = None /* erased Java Vector cast */;
        return jcall!(vector).size();
    }
    

    pub fn elementAt(&mut self, mut anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>>, mut index: i32) -> Option<Rc<RefCell<Box<dyn std::any::Any>>>> {
        let mut vector: Option<Rc<RefCell<Vector<Option<Rc<RefCell<Box<dyn std::any::Any>>>>>>>> = None /* erased Java Vector cast */;
        return jcall!(vector).elementAt(index.clone());
    }
}

