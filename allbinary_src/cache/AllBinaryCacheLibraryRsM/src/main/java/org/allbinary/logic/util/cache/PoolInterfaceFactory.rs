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
use super::PoolInterface::PoolInterface;
use super::CacheableInterfaceFactoryInterface::CacheableInterfaceFactoryInterface;
use super::PoolType::PoolType;
use super::CachePolicy::CachePolicy;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct PoolInterfaceFactory {
}


impl PoolInterfaceFactory {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    // throws Exception
    pub fn getInstance(mut cacheableInterfaceFactoryInterface: Option<Rc<RefCell<dyn CacheableInterfaceFactoryInterface>>>, mut poolType: Option<Rc<RefCell<PoolType>>>, mut cachePolicy: Option<Rc<RefCell<CachePolicy>>>) -> Option<Rc<RefCell<dyn PoolInterface>>> {
        panic!("{:?}", format!("{}: {:?}", "Exception", (format!("{}{}", String::from("No Such PoolType: "), jcall!(poolType).toString()))));
    }
}

