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
use crate::Stack;
use super::*;
// use crate::java::util::Stack::Stack;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct ABStack<E> {
    pub base: Stack<E>,
}


impl<E> ABStack<E> {

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: Stack::new_struct(),
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
}
impl<E> std::ops::Deref for ABStack<E> {
    type Target = Stack<E>;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl<E> std::ops::DerefMut for ABStack<E> {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}


