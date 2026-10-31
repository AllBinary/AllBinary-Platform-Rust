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
// package org.allbinary.graphics.color
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::logic::StdUtil::StdUtil;
use crate::org::allbinary::util::ABHashtable::ABHashtable;
use super::BasicColor::BasicColor;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct BasicColorCacheFactory {
    pub hashtable: Option<Rc<RefCell<ABHashtable<Option<Rc<RefCell<Box<dyn std::any::Any>>>>, Option<Rc<RefCell<Box<dyn std::any::Any>>>>>>>>,
}

thread_local! { pub static BasicColorCacheFactory_instance: Option<Rc<RefCell<BasicColorCacheFactory>>> = BasicColorCacheFactory::new(); }

impl BasicColorCacheFactory {
    

    pub fn getInstance() -> Option<Rc<RefCell<BasicColorCacheFactory>>> {
        return BasicColorCacheFactory_instance.with(Clone::clone).clone();
    }
    

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
    

    pub fn add(&mut self, mut basicDefaultColor: Option<Rc<RefCell<BasicColor>>>) {
        { let java_receiver = self.hashtable.clone(); let java_arguments = (crate::java_into_object((Some((jcall!(basicDefaultColor).intValue()) as i32)).expect("NullPointerException")), crate::java_into_object(basicDefaultColor.clone()),); jcall!(java_receiver).put(java_arguments.0, java_arguments.1); };
    }
    

    // synchronized
    pub fn getAndOrCreate(&mut self, mut integer: Option<i32>) -> Option<Rc<RefCell<BasicColor>>> {
        let mut basicColor: Option<Rc<RefCell<BasicColor>>> = { let java_object = (jcall!(self.hashtable).get(crate::java_into_object((integer.clone()).expect("NullPointerException")))).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<BasicColor>>>>().expect("ClassCastException").clone()) };
        if basicColor.is_none() {
            basicColor = jcall!(BasicColorFactory::BasicColorFactory::getInstance()).createInstance(((integer).unwrap() as i32));
            jcall!(self.hashtable).put(crate::java_into_object((integer.clone()).expect("NullPointerException")), crate::java_into_object(basicColor.clone()));
        }
        return basicColor.clone();
    }
}

