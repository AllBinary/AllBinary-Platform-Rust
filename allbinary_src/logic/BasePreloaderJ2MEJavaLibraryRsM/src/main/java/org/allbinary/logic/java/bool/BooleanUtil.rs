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
// package org.allbinary.logic.java.bool
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::TsUtil::TsUtil;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::string::StringValidationUtil::StringValidationUtil;
use super::BooleanFactory::BooleanFactory;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct BooleanUtil {
    pub tsUtil: Option<Rc<RefCell<TsUtil>>>,
}

thread_local! { pub static BooleanUtil_instance: Option<Rc<RefCell<BooleanUtil>>> = BooleanUtil::new(); }

impl BooleanUtil {
    

    pub fn getInstance() -> Option<Rc<RefCell<BooleanUtil>>> {
        return BooleanUtil_instance.with(Clone::clone).clone();
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            tsUtil: None,
        };
        this.tsUtil = TsUtil::getInstance();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    // throws Exception
    pub fn getFromString(&mut self, mut booleanString: String) -> bool {
        let mut stringValidationUtil: Option<Rc<RefCell<StringValidationUtil>>> = StringValidationUtil::getInstance();
        let mut booleanFactory: Option<Rc<RefCell<BooleanFactory>>> = BooleanFactory::getInstance();
        if jcall!(stringValidationUtil).isEmpty(booleanString.clone()) {
            return false;
        }
        let mut booleanStringLowerCase: String = booleanString.to_lowercase();
        if { let left = &(booleanStringLowerCase); let right = &(jcall!(booleanFactory).TRUE_STRING); let mut left_units = left.encode_utf16(); let mut right_units = right.encode_utf16(); loop { match (left_units.next(), right_units.next()) { (Some(a), Some(b)) if a != b => break a as i32 - b as i32, (Some(_), Some(_)) => {}, (None, None) => break 0, _ => break left.encode_utf16().count() as i32 - right.encode_utf16().count() as i32 } } } == 0 {
            return true;
        } else if { let left = &(booleanStringLowerCase); let right = &(jcall!(booleanFactory).FALSE_STRING); let mut left_units = left.encode_utf16(); let mut right_units = right.encode_utf16(); loop { match (left_units.next(), right_units.next()) { (Some(a), Some(b)) if a != b => break a as i32 - b as i32, (Some(_), Some(_)) => {}, (None, None) => break 0, _ => break left.encode_utf16().count() as i32 - right.encode_utf16().count() as i32 } } } == 0 {
            return false;
        } else {
            panic!("{:?}", format!("{}: {:?}", "Exception", (String::from("Invalid booleanString - Must be true or false"))));
        }
    }
    

    // throws Exception
    pub fn isStringBoolean(&mut self, mut booleanString: String) -> bool {
        let mut booleanFactory: Option<Rc<RefCell<BooleanFactory>>> = BooleanFactory::getInstance();
        let mut booleanStringLowerCase: String = booleanString.to_lowercase();
        if (booleanStringLowerCase.cmp(&jcall!(booleanFactory).TRUE_STRING.clone()) as i32) == 0 || (booleanStringLowerCase.cmp(&jcall!(booleanFactory).FALSE_STRING.clone()) as i32) == 0 {
            return true;
        }
        return false;
    }
    

    pub fn toStringFromBooleanArray(&mut self, mut booleanArray: Vec<bool>) -> String {
        let mut stringMaker: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        let mut size: i32 = (booleanArray.len() as i32).clone();
        for index in (0..size) {
            jcall!(stringMaker).appendboolean(booleanArray[(index) as usize].clone());
        }
        return jcall!(stringMaker).toString();
    }
}

