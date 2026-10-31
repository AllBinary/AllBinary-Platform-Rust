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
use crate::org::allbinary::logic::string::StringMaker::StringMaker;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct CacheType {
    pub vendor: String,
    pub r#type: String,
}


impl CacheType {
    

    pub fn new_struct(mut vendor: String, mut r#type: String) -> Self {
        let mut this = Self {
            vendor: String::new(),
            r#type: String::new(),
        };
        this.vendor = vendor.clone();
        this.r#type = r#type.clone();
        this
    }

    pub fn new(mut vendor: String, mut r#type: String) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(vendor, r#type))))
    }
    

    pub fn getVendor(&mut self) -> String {
        return self.vendor.clone();
    }
    

    pub fn getType(&mut self) -> String {
        return self.r#type.clone();
    }
    

    pub fn toString(&mut self) -> String {
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        jcall!(stringBuffer).append(String::from("CacheType: Vendor: "));
        jcall!(stringBuffer).append(self.getVendor());
        jcall!(stringBuffer).append(String::from(" Type: "));
        jcall!(stringBuffer).append(self.getType());
        return jcall!(stringBuffer).toString();
    }
}

