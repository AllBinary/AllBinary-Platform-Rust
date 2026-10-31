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
use super::*;
use crate::jsinterop::annotations::JsType;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct ABSystemWrapper {
}

thread_local! { pub static ABSystemWrapper_instance: Option<Rc<RefCell<ABSystemWrapper>>> = ABSystemWrapper::new(); }

impl ABSystemWrapper {

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<ABSystemWrapper>>> {
        return ABSystemWrapper_instance.with(Clone::clone).clone();
    }
    

    pub fn arraycopy<T: Clone>(&mut self, src: &[T], srcPos: i32, dest: &mut [T], destPos: i32, length: i32) {
        
                { let java_copy_source = src[(srcPos) as usize..(srcPos + length) as usize].to_vec(); dest[(destPos) as usize..(destPos + length) as usize].clone_from_slice(&java_copy_source); };
    }
    

    pub fn currentTimeMillis(&mut self) -> i64 {
        return (std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis() as i64);
    }
}

