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
use std::collections::HashMap;
use super::*;
// use crate::java::util::HashMap::HashMap;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct ABHashMap<K, V> {
    pub base: HashMap<K, V>,
}


impl<K, V> ABHashMap<K, V> {

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: HashMap::new(),
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
}
impl<K, V> std::ops::Deref for ABHashMap<K, V> {
    type Target = HashMap<K, V>;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl<K, V> std::ops::DerefMut for ABHashMap<K, V> {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}


