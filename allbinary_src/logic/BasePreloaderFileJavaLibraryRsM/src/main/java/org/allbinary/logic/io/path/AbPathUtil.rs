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
// package org.allbinary.logic.io.path
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use super::AbPath::AbPath;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct AbPathUtil {
    pub NO_ABPATH: Option<Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>>,
}

thread_local! { pub static AbPathUtil_instance: Option<Rc<RefCell<AbPathUtil>>> = AbPathUtil::new(); }

impl AbPathUtil {

    pub fn new_struct() -> Self {
        let mut this = Self {
            NO_ABPATH: None,
        };
        this.NO_ABPATH = AbPath::createAbPath();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<AbPathUtil>>> {
        return AbPathUtil_instance.with(Clone::clone).clone();
    }
}

