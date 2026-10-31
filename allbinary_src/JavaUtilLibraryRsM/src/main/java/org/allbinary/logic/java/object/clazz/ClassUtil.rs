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
// package org.allbinary.logic.java.anyType.clazz
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::logic::java::anyType::ConstructorUtil::ConstructorUtil;
use crate::org::allbinary::logic::java::anyType::InterfaceUtil::InterfaceUtil;
use crate::org::allbinary::logic::java::anyType::MethodUtil::MethodUtil;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct ClassUtil {
}


impl ClassUtil {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn viewAll(mut anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>>, mut lineBreak: String) -> String {
        if anyType.is_some() {
            return ClassUtil::viewAll(std::any::type_name_of_val(&anyType), lineBreak.clone());
        } else {
            return String::from("Object Null");
        }
    }
    

    // Java overloaded method: Rust needs a different name
    pub fn viewAll(mut myClass: &'static str, mut lineBreak: String) -> String {
        if myClass.is_some() {
            let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
            let mut classes: Vec<&'static str> = jcall!(myClass).getClasses();
            jcall!(stringBuffer).append(lineBreak.clone());
            jcall!(stringBuffer).append(String::from("Class: "));
            jcall!(stringBuffer).append(myClass.to_string());
            jcall!(stringBuffer).append(String::from(" uses "));
            jcall!(stringBuffer).appendint((classes.len() as i32).clone());
            jcall!(stringBuffer).append(String::from(" other classes"));
            jcall!(stringBuffer).append(lineBreak.clone());
            jcall!(stringBuffer).append(lineBreak.clone());
            jcall!(stringBuffer).append(String::from("Classes: "));
            jcall!(stringBuffer).append(lineBreak.clone());
            for index in (0..(classes.len() as i32)) {
                jcall!(stringBuffer).append(String::from("Class: "));
                jcall!(stringBuffer).append(classes[(index) as usize].to_string());
                jcall!(stringBuffer).append(lineBreak.clone());
            }
            jcall!(stringBuffer).append(InterfaceUtil::viewAll(myClass.clone(), lineBreak.clone()));
            jcall!(stringBuffer).append(lineBreak.clone());
            jcall!(stringBuffer).append(ConstructorUtil::viewAll(myClass.clone(), lineBreak.clone()));
            jcall!(stringBuffer).append(lineBreak.clone());
            jcall!(stringBuffer).append(MethodUtil::viewAll(myClass.clone(), lineBreak.clone()));
            return jcall!(stringBuffer).toString();
        } else {
            return String::from("Class Null");
        }
    }
}

