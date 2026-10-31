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
// package org.allbinary.logic
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::util::ABHashMap::ABHashMap;
use crate::org::allbinary::util::ABHashtable::ABHashtable;
use crate::org::allbinary::util::ABStack::ABStack;
use crate::org::allbinary::util::ABVector::ABVector;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

//ActualPlatform
pub struct StdUtil {
    pub EMPTY_VECTOR: Option<Rc<RefCell<ABVector<Option<Rc<RefCell<Box<dyn std::any::Any>>>>>>>>,
    pub NULL_TABLE: Option<Rc<RefCell<ABHashtable<Option<Rc<RefCell<Box<dyn std::any::Any>>>>, Option<Rc<RefCell<Box<dyn std::any::Any>>>>>>>>,
    pub NULL_MAP: Option<Rc<RefCell<ABHashMap<Option<Rc<RefCell<Box<dyn std::any::Any>>>>, Option<Rc<RefCell<Box<dyn std::any::Any>>>>>>>>,
}

thread_local! { pub static StdUtil_instance: Option<Rc<RefCell<StdUtil>>> = StdUtil::new(); }

impl StdUtil {

    pub fn new_struct() -> Self {
        let mut this = Self {
            EMPTY_VECTOR: None,
            NULL_TABLE: None,
            NULL_MAP: None,
        };
        this.EMPTY_VECTOR = this.createVector();
        this.NULL_TABLE = this.createHashtable();
        this.NULL_MAP = this.createHashMap();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<StdUtil>>> {
        return StdUtil_instance.with(Clone::clone).clone();
    }
    

    pub fn createStack(&mut self) -> Option<Rc<RefCell<ABStack<Option<Rc<RefCell<Box<dyn std::any::Any>>>>>>>> {
        return ABStack::new();
    }
    

    pub fn createVector(&mut self) -> Option<Rc<RefCell<ABVector<Option<Rc<RefCell<Box<dyn std::any::Any>>>>>>>> {
        return ABVector::new();
    }
    

    pub fn createHashtable(&mut self) -> Option<Rc<RefCell<ABHashtable<Option<Rc<RefCell<Box<dyn std::any::Any>>>>, Option<Rc<RefCell<Box<dyn std::any::Any>>>>>>>> {
        return ABHashtable::new();
    }
    

    pub fn createHashMap(&mut self) -> Option<Rc<RefCell<ABHashMap<Option<Rc<RefCell<Box<dyn std::any::Any>>>>, Option<Rc<RefCell<Box<dyn std::any::Any>>>>>>>> {
        return ABHashMap::new();
    }
}

