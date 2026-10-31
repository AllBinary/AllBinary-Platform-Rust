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
// use crate::java::lang::reflect::Constructor::Constructor;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::string::CommonSeps::CommonSeps;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct ConstructorUtil {
}


impl ConstructorUtil {
    

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
        let mut constructor: Vec<Option<Rc<RefCell<Constructor>>>> = jcall!(myClass).getConstructors();
        jcall!(stringBuffer).append(String::from("Constructors: "));
        jcall!(stringBuffer).append(lineBreak.clone());
        for index in (0..(constructor.len() as i32)) {
            jcall!(stringBuffer).append(ConstructorUtil::view(constructor[(index) as usize].clone(), lineBreak.clone()));
        }
        return jcall!(stringBuffer).toString();
    }
    

    pub fn view(mut constructor: Option<Rc<RefCell<Constructor>>>, mut lineBreak: String) -> String {
        if constructor.is_some() {
            let mut commonSeps: Option<Rc<RefCell<CommonSeps>>> = CommonSeps::getInstance();
            let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
            jcall!(stringBuffer).append(jcall!(constructor).getName());
            let mut classes: Vec<&'static str> = jcall!(constructor).getParameterTypes();
            for index in (0..(classes.len() as i32)) {
                jcall!(stringBuffer).append(jcall!(commonSeps).SPACE.clone());
                jcall!(stringBuffer).append(classes[(index) as usize].to_string());
                jcall!(stringBuffer).append(jcall!(commonSeps).COLON.clone());
                jcall!(stringBuffer).appendint(index.clone());
            }
            jcall!(stringBuffer).append(lineBreak.clone());
            return jcall!(stringBuffer).toString();
        } else {
            return String::from("Constructor Null");
        }
    }
}

