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
// package org.allbinary.logic.math.permutations
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::TsUtil::TsUtil;
use super::Comparable::Comparable;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

fn java_string_hash(text: &str) -> i32 {
    text.chars().fold(0i32, |hash, character| hash.wrapping_mul(31).wrapping_add(character as i32))
}

pub struct ComparableObject {
}

// impl Comparable for ComparableObject with the methods of the trait

impl ComparableObject {

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn compareTo(&mut self, mut anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) -> i32 {
        if jcall!(TsUtil::getInstance()).hashCode(None /* Java this: a &mut self method has no Rc of itself */) < jcall!(TsUtil::getInstance()).hashCode(crate::java_into_object(anyType.clone())) {
            return -1;
        } else if jcall!(TsUtil::getInstance()).hashCode(None /* Java this: a &mut self method has no Rc of itself */) > jcall!(TsUtil::getInstance()).hashCode(crate::java_into_object(anyType.clone())) {
            return 1;
        }
        return 0;
    }
}

