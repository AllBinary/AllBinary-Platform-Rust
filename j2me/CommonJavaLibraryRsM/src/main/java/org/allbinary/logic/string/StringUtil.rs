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
// package org.allbinary.logic.string
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::jsinterop::annotations::JsType;
use crate::jsinterop::annotations::JsMethod;
use crate::jsinterop::annotations::JsConstructor;
use crate::jsinterop::annotations::JsProperty;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct StringUtil {
    pub INIT_STRING: String,
    pub NULL_STRING: String,
    pub EMPTY_STRING: String,
    pub ONE_EMPTY_STRING_ARRAY: Vec<String>,
    pub stringArray: Vec<String>,
}

thread_local! { pub static StringUtil_instance: Option<Rc<RefCell<StringUtil>>> = StringUtil::new(); }

impl StringUtil {
    

    pub fn getInstance() -> Option<Rc<RefCell<StringUtil>>> {
        return StringUtil_instance.with(Clone::clone).clone();
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            INIT_STRING: String::new(),
            NULL_STRING: String::new(),
            EMPTY_STRING: String::new(),
            ONE_EMPTY_STRING_ARRAY: Vec::new(),
            stringArray: Vec::new(),
        };
        this.INIT_STRING = String::from("").clone();
        this.NULL_STRING = String::from("null");
        this.EMPTY_STRING = String::from("");
        this.ONE_EMPTY_STRING_ARRAY = vec![this.EMPTY_STRING.clone()];
        this.stringArray = vec![String::new(); (0) as usize];
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getNonNull(&mut self, mut string: String) -> String {
        if false {
            return self.EMPTY_STRING.clone();
        } else {
            return string.clone();
        }
    }
    

    pub fn getArrayInstance(&mut self) -> Vec<String> {
        return self.stringArray.clone();
    }
    

    pub fn count(&mut self, mut string: String, mut aChar: char) -> i32 {
        let mut count: i32 = 0;
        let mut size: i32 = (string.chars().count() as i32);
        for index in (0..size) {
            if string.chars().nth((index.clone()) as usize).unwrap() == aChar {
                count += 1;
            }
        }
        return count.clone();
    }
    

    pub fn toString(&mut self, mut anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) -> String {
        if anyType.is_some() {
            return { let value = jcall!(anyType); value.as_ref().downcast_ref::<String>().cloned().unwrap_or_else(|| format!("{:?}", value.as_ref().type_id())) };
        } else {
            return self.NULL_STRING.clone();
        }
    }
}

