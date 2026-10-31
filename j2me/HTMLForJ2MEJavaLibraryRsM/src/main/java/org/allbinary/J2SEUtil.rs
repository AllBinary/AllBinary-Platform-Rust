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
// package org.allbinary
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;
use super::*;
// use crate::java::util::HashMap::HashMap;
// use crate::java::util::Map::Map;
// use crate::java::util::Set::Set;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

//HTMLForJ2ME
pub struct J2SEUtil {
}

thread_local! { pub static J2SEUtil_instance: Option<Rc<RefCell<J2SEUtil>>> = J2SEUtil::new(); }

impl J2SEUtil {

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<J2SEUtil>>> {
        return J2SEUtil_instance.with(Clone::clone).clone();
    }
    

    pub fn getHashMapAsArray(&mut self, mut hashMap: Option<Rc<RefCell<HashMap<Option<Rc<RefCell<Box<dyn std::any::Any>>>>, Option<Rc<RefCell<Box<dyn std::any::Any>>>>>>>>) -> Vec<Option<Rc<RefCell<Box<dyn std::any::Any>>>>> {
        let mut objectArray: Vec<Option<Rc<RefCell<Box<dyn std::any::Any>>>>> = jcall!(jcall!(hashMap).keySet()).toArray();
        return objectArray.clone();
    }
    

    pub fn getMapAsArray(&mut self, mut map: Option<Rc<RefCell<dyn Map<Option<Rc<RefCell<Box<dyn std::any::Any>>>>, Option<Rc<RefCell<Box<dyn std::any::Any>>>>>>>>) -> Vec<Option<Rc<RefCell<Box<dyn std::any::Any>>>>> {
        let mut objectArray: Vec<Option<Rc<RefCell<Box<dyn std::any::Any>>>>> = jcall!(jcall!(map).keySet()).toArray();
        return objectArray.clone();
    }
    

    pub fn getSetAsArray(&mut self, mut set: Option<Rc<RefCell<dyn Set<String>>>>) -> Vec<Option<Rc<RefCell<Box<dyn std::any::Any>>>>> {
        let mut objectArray: Vec<Option<Rc<RefCell<Box<dyn std::any::Any>>>>> = jcall!(set).toArray();
        return objectArray.clone();
    }
}

