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
// package org.allbinary.logic.java.number
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::string::CommonPhoneStrings::CommonPhoneStrings;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct LongUtil {
}


impl LongUtil {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn fillIn(mut end: String) -> String {
        let mut frameStringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        if jcall!(frameStringBuffer).length() < unsafe { LongData_MAX_LONG_LENGTH.clone() } {
            let mut commonPhoneStrings: Option<Rc<RefCell<CommonPhoneStrings>>> = CommonPhoneStrings::getInstance();
            let mut size: i32 = unsafe { LongData_MAX_LONG_LENGTH.clone() } - jcall!(frameStringBuffer).length();
            for index in (0..size) {
                jcall!(frameStringBuffer).append(jcall!(commonPhoneStrings).ZERO.clone());
            }
        }
        jcall!(frameStringBuffer).append(end.clone());
        return jcall!(frameStringBuffer).toString();
    }
}

