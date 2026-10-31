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
// package org.allbinary.logic.java.anyType
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct ParamsUtil {
}


impl ParamsUtil {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn viewParams(mut params: Vec<&'static str>) -> String {
        if params.is_some() {
            let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
            for index in (0..(params.len() as i32)) {
                jcall!(stringBuffer).append(String::from(" "));
                jcall!(stringBuffer).append(params[(index) as usize].to_string());
            }
            return jcall!(stringBuffer).toString();
        } else {
            return String::from("Params Null");
        }
    }
}

