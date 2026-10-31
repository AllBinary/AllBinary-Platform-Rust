// 
// AllBinary Open License Version 1 
// Copyright (c) 2002 AllBinary 
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
// package org.allbinary.logic.control.crypt.jcehelper
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

pub struct AbCrypt {
}


impl AbCrypt {
    

    pub fn new_struct(mut algorithm: String, mut key: String) -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new(mut algorithm: String, mut key: String) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(algorithm, key))))
    }
    

    pub fn encrypt(&mut self, mut array: Vec<i8>) -> Vec<i8> {
        return None;
    }
    

    pub fn decrypt(&mut self, mut array: Vec<i8>) -> Vec<i8> {
        return None;
    }
    

    pub fn mutilate(&mut self, mut array: Vec<i8>) -> Vec<i8> {
        return None;
    }
}

