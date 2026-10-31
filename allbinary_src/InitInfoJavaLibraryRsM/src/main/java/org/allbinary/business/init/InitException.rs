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
// package org.allbinary.business.init
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

pub struct InitException {
    pub base: Exception,
}


impl InitException {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: Exception::new_struct(),
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn new_struct1(mut msg: String) -> Self {
        let mut this = Self {
            base: Exception::new_struct1(msg.clone()),
        };
        // super(...) is in the struct literal above
        this
    }

    pub fn new1(mut msg: String) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct1(msg))))
    }
}
impl std::ops::Deref for InitException {
    type Target = Exception;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for InitException {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}



