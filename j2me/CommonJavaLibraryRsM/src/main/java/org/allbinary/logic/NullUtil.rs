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

//ActualPlatform
pub struct NullUtil {
    pub NULL_OBJECT: Option<Rc<RefCell<Box<dyn std::any::Any>>>>,
    pub NULL_CLASS: &'static str,
    pub NULL_OBJECT_ARRAY: Vec<Option<Rc<RefCell<Box<dyn std::any::Any>>>>>,
    pub NULL_CHAR_ARRAY: Vec<char>,
    pub NULL_BYTE_ARRAY: Vec<i8>,
    pub NULL_SHORT_ARRAY_ARRAY: Vec<Vec<i16>>,
    pub NULL_INT_ARRAY: Vec<i32>,
    pub NULL_INT_ARRAY_ARRAY: Vec<Vec<i32>>,
    pub NULL_INT_ARRAY_ARRAY_ARRAY: Vec<Vec<Vec<i32>>>,
    pub NULL_FLOAT_ARRAY: Vec<f32>,
}

thread_local! { pub static NullUtil_instance: Option<Rc<RefCell<NullUtil>>> = NullUtil::new(); }

impl NullUtil {

    pub fn new_struct() -> Self {
        let mut this = Self {
            NULL_OBJECT: None,
            NULL_CLASS: "",
            NULL_OBJECT_ARRAY: Vec::new(),
            NULL_CHAR_ARRAY: Vec::new(),
            NULL_BYTE_ARRAY: Vec::new(),
            NULL_SHORT_ARRAY_ARRAY: Vec::new(),
            NULL_INT_ARRAY: Vec::new(),
            NULL_INT_ARRAY_ARRAY: Vec::new(),
            NULL_INT_ARRAY_ARRAY_ARRAY: Vec::new(),
            NULL_FLOAT_ARRAY: Vec::new(),
        };
        this.NULL_OBJECT = Some(Rc::new(RefCell::new(Box::new(()) as Box<dyn std::any::Any>)));
        this.NULL_CLASS = std::any::type_name_of_val(&this.NULL_OBJECT);
        this.NULL_OBJECT_ARRAY = vec![None; (0) as usize];
        this.NULL_CHAR_ARRAY = vec!['\0'; (0) as usize];
        this.NULL_BYTE_ARRAY = vec![0; (0) as usize];
        this.NULL_SHORT_ARRAY_ARRAY = vec![vec![0; (0) as usize]; (0) as usize];
        this.NULL_INT_ARRAY = vec![0; (0) as usize];
        this.NULL_INT_ARRAY_ARRAY = vec![vec![0; (0) as usize]; (0) as usize];
        this.NULL_INT_ARRAY_ARRAY_ARRAY = vec![vec![vec![0; (0) as usize]; (0) as usize]; (0) as usize];
        this.NULL_FLOAT_ARRAY = vec![0.0_f32; (0) as usize];
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<NullUtil>>> {
        return NullUtil_instance.with(Clone::clone).clone();
    }
}

