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
// package org.allbinary.graphics.opengles.shader
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

pub struct ShaderStrings {
    pub NO_PROGRAM_HANDLE: String,
    pub UNIFORM_BLOCK_HANDLE: String,
    pub UNIFORM_HANDLE: String,
    pub UNIFORM_ATTRIBUTE_HANDLE: String,
}

thread_local! { pub static ShaderStrings_instance: Option<Rc<RefCell<ShaderStrings>>> = ShaderStrings::new(); }

impl ShaderStrings {

    pub fn new_struct() -> Self {
        let mut this = Self {
            NO_PROGRAM_HANDLE: String::new(),
            UNIFORM_BLOCK_HANDLE: String::new(),
            UNIFORM_HANDLE: String::new(),
            UNIFORM_ATTRIBUTE_HANDLE: String::new(),
        };
        this.NO_PROGRAM_HANDLE = String::from("No program handle");
        this.UNIFORM_BLOCK_HANDLE = String::from("Uniform Block Handle: ");
        this.UNIFORM_HANDLE = String::from("Uniform Handle: ");
        this.UNIFORM_ATTRIBUTE_HANDLE = String::from("Uniform Attribute Handle: ");
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<ShaderStrings>>> {
        return ShaderStrings_instance.with(Clone::clone).clone();
    }
}

