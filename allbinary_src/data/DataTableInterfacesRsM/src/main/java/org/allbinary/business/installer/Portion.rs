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
// package org.allbinary.business.installer
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;
use super::*;
// use crate::java::util::HashMap::HashMap;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct Portion {
    pub current: Option<i32>,
    pub total: Option<i32>,
}


impl Portion {
    

    pub fn new_struct(mut hashMap: Option<Rc<RefCell<HashMap>>>) -> Self {
        let mut this = Self {
            current: None,
            total: None,
        };
        this.current = (jcall!(hashMap).get(String::from("current"))).clone() /* as Integer */;
        this.total = (jcall!(hashMap).get(String::from("total"))).clone() /* as Integer */;
        this
    }

    pub fn new(mut hashMap: Option<Rc<RefCell<HashMap>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(hashMap))))
    }
    

    pub fn getCurrent(&mut self) -> Option<i32> {
        return self.current.clone();
    }
    

    pub fn getTotal(&mut self) -> Option<i32> {
        return self.total.clone();
    }
}

