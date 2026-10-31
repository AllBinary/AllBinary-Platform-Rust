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
// package org.allbinary.util
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use crate::Hashtable;
use super::*;
// use crate::java::util::Hashtable::Hashtable;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct ABHashtable<K, V> {
    pub base: Hashtable<K, V>,
}


impl<K, V> ABHashtable<K, V> {
    

    pub fn new_struct1(mut initialCapacity: i32) -> Self {
        let mut this = Self {
            base: Hashtable::new_struct1(initialCapacity.clone()),
        };
        // super(...) is in the struct literal above
        this
    }

    pub fn new1(mut initialCapacity: i32) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct1(initialCapacity))))
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: Hashtable::new_struct(),
        };
        // super(...) is in the struct literal above
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
}
impl<K, V> std::ops::Deref for ABHashtable<K, V> {
    type Target = Hashtable<K, V>;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl<K, V> std::ops::DerefMut for ABHashtable<K, V> {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}


