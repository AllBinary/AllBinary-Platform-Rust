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
// package org.allbinary.util
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::jsinterop::annotations::JsType;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::string::CommonLabels::CommonLabels;
use crate::jsinterop::annotations::JsMethod;
use crate::jsinterop::annotations::JsConstructor;
use crate::jsinterop::annotations::JsProperty;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct CircularIndexUtil {
    pub index: i32,
    pub lastIndex: i32,
    pub size: i32,
}

thread_local! { pub static CircularIndexUtil_NULL_CIRCULAR_INDEX_UTIL: Option<Rc<RefCell<CircularIndexUtil>>> = CircularIndexUtil::new(0, 0); }

impl CircularIndexUtil {
    

    pub fn new_struct(mut index: i32, mut size: i32) -> Self {
        let mut this = Self {
            index: 0,
            lastIndex: 0,
            size: 0,
        };
        this.index = 0;
        this.lastIndex = 0;
        this.size = 0;
        this.setSize(size.clone());
        this.setIndex(index.clone());
        this
    }

    pub fn new(mut index: i32, mut size: i32) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(index, size))))
    }
    

    pub fn next(&mut self) -> i32 {
        if self.index >= self.lastIndex {
            self.index = 0;
        } else {
            self.index += 1;
        }
        return self.index.clone();
    }
    

    pub fn previous(&mut self) -> i32 {
        if self.index < 1 {
            self.index = self.lastIndex.clone();
        } else {
            self.index -= 1;
        }
        return self.index.clone();
    }
    

    pub fn setIndex(&mut self, mut index: i32) {
        if index > self.lastIndex {
            return;
        }
        self.index = index.clone();
    }
    

    pub fn getIndex(&mut self) -> i32 {
        return self.index.clone();
    }
    

    pub fn createInstance(mut max: i32) -> Option<Rc<RefCell<CircularIndexUtil>>> {
        return CircularIndexUtil::new(0, max.clone());
    }
    

    pub fn createInstanceAt(mut index: i32, mut max: i32) -> Option<Rc<RefCell<CircularIndexUtil>>> {
        return CircularIndexUtil::new(index.clone(), max.clone());
    }
    

    pub fn setSize(&mut self, mut size: i32) {
        self.size = size.clone();
        self.lastIndex = size - 1;
        self.index = 0;
    }
    

    pub fn getSize(&mut self) -> i32 {
        return self.size.clone();
    }
    

    pub fn toString(&mut self) -> String {
        return jcall!(StringMaker::new()).append(jcall!(CommonLabels::getInstance()).INDEX_LABEL.clone()).appendint(self.index.clone()).append(String::from("lastIndex: ")).appendint(self.lastIndex.clone()).append(String::from("size: ")).appendint(self.size.clone()).toString();
    }
}

