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
// package org.allbinary.logic.java.array
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

pub struct IntArrayUtil {
}


impl IntArrayUtil {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn toString(mut array: Vec<i32>) -> String {
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        for index in (0..(array.len() as i32)) {
            jcall!(stringBuffer).appendint(array[(index) as usize].clone());
            if index < (array.len() as i32) {
                jcall!(stringBuffer).append(String::from(", "));
            }
        }
        return jcall!(stringBuffer).toString();
    }
    

    // Java overloaded method: Rust needs a different name
    pub fn toString(mut array: Vec<Vec<i32>>) -> String {
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        for index in (0..(array.len() as i32)) {
            jcall!(stringBuffer).append(String::from("{"));
            for index2 in (0..(array[(index) as usize].len() as i32)) {
                jcall!(stringBuffer).appendint(((array[(index) as usize][(index2) as usize]) as i32).clone());
                if index2 < (array[(index) as usize].len() as i32) - 1 {
                    jcall!(stringBuffer).append(String::from(", "));
                }
            }
            jcall!(stringBuffer).append(String::from("}"));
            if index < (array.len() as i32) - 1 {
                jcall!(stringBuffer).append(String::from(", \n"));
            }
        }
        return jcall!(stringBuffer).toString();
    }
    

    // Java overloaded method: Rust needs a different name
    pub fn toString(mut array: Vec<Vec<Vec<i32>>>) -> String {
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        for index in (0..(array.len() as i32)) {
            jcall!(stringBuffer).append(String::from("{"));
            for index2 in (0..(array[(index) as usize].len() as i32)) {
                jcall!(stringBuffer).append(String::from("{"));
                for index3 in (0..(array[(index) as usize][(index2) as usize].len() as i32)) {
                    jcall!(stringBuffer).appendint(((array[(index) as usize][(index2) as usize][(index3) as usize]) as i32).clone());
                    if index3 < (array[(index) as usize][(index2) as usize].len() as i32) - 1 {
                        jcall!(stringBuffer).append(String::from(", "));
                    }
                }
                jcall!(stringBuffer).append(String::from("}"));
                if index2 < (array[(index) as usize].len() as i32) - 1 {
                    jcall!(stringBuffer).append(String::from(", "));
                }
            }
            jcall!(stringBuffer).append(String::from("}"));
            if index < (array.len() as i32) - 1 {
                jcall!(stringBuffer).append(String::from(", \n"));
            }
        }
        return jcall!(stringBuffer).toString();
    }
}

