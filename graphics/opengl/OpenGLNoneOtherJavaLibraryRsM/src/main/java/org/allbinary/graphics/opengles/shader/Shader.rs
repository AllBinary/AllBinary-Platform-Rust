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
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListUtil::BasicArrayListUtil;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct Shader {
    pub shaderName: String,
    pub shaderStringList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>,
    pub shaderAsString: String,
    pub shaderHandle: i32,
}


impl Shader {

    pub fn new_struct() -> Self {
        let mut this = Self {
            shaderName: String::new(),
            shaderStringList: None,
            shaderAsString: String::new(),
            shaderHandle: 0,
        };
        this.shaderName = jcall!(StringUtil::getInstance()).EMPTY_STRING.clone();
        this.shaderStringList = jcall!(BasicArrayListUtil::getInstance()).getImmutableInstance();
        this.shaderAsString = jcall!(StringUtil::getInstance()).EMPTY_STRING.clone();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
}

