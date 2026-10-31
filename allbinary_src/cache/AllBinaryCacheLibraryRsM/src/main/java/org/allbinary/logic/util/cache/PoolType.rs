// 
// AllBinary Open License Version 1 
// Copyright (c) 2007 AllBinary 
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
// package org.allbinary.logic.util.cache
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use super::CacheType::CacheType;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct PoolType {
    pub cacheType: Option<Rc<RefCell<CacheType>>>,
}


impl PoolType {
    

    pub fn new_struct(mut cacheType: Option<Rc<RefCell<CacheType>>>) -> Self {
        let mut this = Self {
            cacheType: None,
        };
        this.cacheType = cacheType.clone();
        this
    }

    pub fn new(mut cacheType: Option<Rc<RefCell<CacheType>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(cacheType))))
    }
    

    pub fn getCacheType(&mut self) -> Option<Rc<RefCell<CacheType>>> {
        return self.cacheType.clone();
    }
    

    pub fn toString(&mut self) -> String {
        let mut POOL_TYPE: String = String::from("PoolType: ");
        return format!("{}{}", POOL_TYPE, jcall!(self.getCacheType()).toString());
    }
}

