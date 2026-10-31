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
// package org.platform
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::logic::StdUtil::StdUtil;
use crate::org::allbinary::util::ABHashtable::ABHashtable;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct ThreedObjResources {
    pub hashtable: Option<Rc<RefCell<ABHashtable<Option<Rc<RefCell<Box<dyn std::any::Any>>>>, Option<Rc<RefCell<Box<dyn std::any::Any>>>>>>>>,
}

thread_local! { pub static ThreedObjResources_instance: Option<Rc<RefCell<ThreedObjResources>>> = ThreedObjResources::new(); }

impl ThreedObjResources {

    pub fn new_struct() -> Self {
        let mut this = Self {
            hashtable: None,
        };
        this.hashtable = jcall!(StdUtil::getInstance()).createHashtable();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<ThreedObjResources>>> {
        return ThreedObjResources_instance.with(Clone::clone).clone();
    }
    

    pub fn get(&mut self, mut resourceName: String) -> String {
        let mut resourceName2: String = (jcall!(self.hashtable).get(crate::java_into_object(resourceName.clone()))).clone() /* as String */;
        if true {
            return resourceName2.clone();
        }
        return resourceName.clone();
    }
}

