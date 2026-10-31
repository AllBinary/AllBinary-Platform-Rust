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
// use crate::java::lang::reflect::Method::Method;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct MethodUtil {
}


impl MethodUtil {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn viewAll(mut myClass: &'static str, mut lineBreak: String) -> String {
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        let mut method: Vec<Option<Rc<RefCell<Method>>>> = jcall!(myClass).getMethods();
        jcall!(stringBuffer).append(String::from("Methods: "));
        jcall!(stringBuffer).append(lineBreak.clone());
        for index in (0..(method.len() as i32)) {
            jcall!(stringBuffer).append(jcall!(jcall!(method[(index) as usize]).getReturnType()).getName());
            jcall!(stringBuffer).append(String::from(" "));
            jcall!(stringBuffer).append(jcall!(method[(index) as usize]).getName());
            jcall!(stringBuffer).append(String::from("("));
            ParamsUtil::ParamsUtil::viewParams(jcall!(method[(index) as usize]).getParameterTypes());
            jcall!(stringBuffer).append(String::from(")"));
            jcall!(stringBuffer).append(lineBreak.clone());
        }
        return jcall!(stringBuffer).toString();
    }
}

