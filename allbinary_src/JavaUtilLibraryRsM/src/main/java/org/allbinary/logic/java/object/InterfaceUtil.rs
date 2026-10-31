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
use crate::org::allbinary::string::CommonSeps::CommonSeps;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct InterfaceUtil {
}


impl InterfaceUtil {
    

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
        let mut interfaces: Vec<&'static str> = jcall!(myClass).getInterfaces();
        jcall!(stringBuffer).append(lineBreak.clone());
        jcall!(stringBuffer).append(String::from("Interfaces: "));
        jcall!(stringBuffer).append(lineBreak.clone());
        for index in (0..(interfaces.len() as i32)) {
            jcall!(stringBuffer).append(String::from("Interface: "));
            jcall!(stringBuffer).append(interfaces[(index) as usize].to_string());
            jcall!(stringBuffer).append(lineBreak.clone());
        }
        return jcall!(stringBuffer).toString();
    }
    

    pub fn getClass(mut interfaceName: String, mut interfaces: Vec<&'static str>) -> &'static str {
        if true && interfaces.is_some() && (interfaces.len() as i32) > 0 {
            for index in (0..(interfaces.len() as i32)) {
                if jcall!(interfaces[(index) as usize].to_string()).indexOf(interfaceName.clone()) >= 0 {
                    return interfaces[(index) as usize].clone();
                }
            }
        }
        return None;
    }
    

    pub fn isImplemented(mut clazz: &'static str, mut anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) -> bool {
        let mut classes: Vec<&'static str> = jcall!(std::any::type_name_of_val(&anyType)).getInterfaces();
        for index in (0..(classes.len() as i32)) {
            if jcall!(clazz.to_string()).compareTo(classes[(index) as usize].to_string()) == 0 {
                return true;
            }
        }
        return false;
    }
    

    // Java overloaded method: Rust needs a different name
    pub fn isImplemented(mut className: String, mut anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) -> bool {
        let mut classes: Vec<&'static str> = jcall!(std::any::type_name_of_val(&anyType)).getInterfaces();
        for index in (0..(classes.len() as i32)) {
            if jcall!(classes[(index) as usize].to_string()).indexOf(className.clone()) == 0 {
                return true;
            }
        }
        return false;
    }
    

    pub fn isImplementedView(mut clazz: &'static str, mut anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) -> String {
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        let mut classes: Vec<&'static str> = jcall!(std::any::type_name_of_val(&anyType)).getInterfaces();
        jcall!(stringBuffer).append(String::from("isImplementedView: \n"));
        for index in (0..(classes.len() as i32)) {
            jcall!(stringBuffer).append(clazz.to_string());
            jcall!(stringBuffer).append(String::from(" should be = "));
            jcall!(stringBuffer).append(classes[(index) as usize].to_string());
            jcall!(stringBuffer).append(classes[(index) as usize].to_string());
            jcall!(stringBuffer).append(jcall!(CommonSeps::getInstance()).NEW_LINE.clone());
        }
        return jcall!(stringBuffer).toString();
    }
}

