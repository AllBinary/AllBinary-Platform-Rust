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
// use crate::java::awt::*;
use crate::org::allbinary::logic::util::cache::CacheableInterface::CacheableInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct ColorCacheable {
    pub key: Option<Rc<RefCell<Box<dyn std::any::Any>>>>,
    pub color: Option<Rc<RefCell<Color>>>,
}

impl CacheableInterface for ColorCacheable {
    fn getKey(&mut self) -> Option<Rc<RefCell<Box<dyn std::any::Any>>>> { Self::getKey(self) }
}

impl ColorCacheable {
    

    pub fn new_struct(mut key: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) -> Self {
        let mut this = Self {
            key: None,
            color: None,
        };
        this.key = key.clone();
        this.color = Color::new(((({ let java_object = key; java_object.as_ref().map(|object| { let object = object.borrow(); object.downcast_ref::<i32>().copied().or_else(|| object.downcast_ref::<Option<i32>>().copied().flatten()).expect("ClassCastException") }) })).unwrap() as i32));
        this
    }

    pub fn new(mut key: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(key))))
    }
    

    pub fn getKey(&mut self) -> Option<Rc<RefCell<Box<dyn std::any::Any>>>> {
        return self.key.clone();
    }
    

    pub fn getColor(&mut self) -> Option<Rc<RefCell<Color>>> {
        return self.color.clone();
    }
    

    pub fn setColor(&mut self, mut color: Option<Rc<RefCell<Color>>>) {
        self.color = color.clone();
    }
}

