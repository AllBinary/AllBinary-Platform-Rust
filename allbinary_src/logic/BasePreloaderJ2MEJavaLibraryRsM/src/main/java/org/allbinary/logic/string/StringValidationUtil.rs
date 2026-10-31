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
use crate::org::allbinary::TsUtil::TsUtil;
use super::StringUtil::StringUtil;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct StringValidationUtil {
    pub stringUtil: Option<Rc<RefCell<StringUtil>>>,
    pub tsUtil: Option<Rc<RefCell<TsUtil>>>,
}

thread_local! { pub static StringValidationUtil_instance: Option<Rc<RefCell<StringValidationUtil>>> = StringValidationUtil::new(); }

impl StringValidationUtil {
    

    pub fn getInstance() -> Option<Rc<RefCell<StringValidationUtil>>> {
        return StringValidationUtil_instance.with(Clone::clone).clone();
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            stringUtil: None,
            tsUtil: None,
        };
        this.stringUtil = StringUtil::getInstance();
        this.tsUtil = TsUtil::getInstance();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn containsSpaces(&mut self, mut value: String) -> bool {
        if value.find(' ').map(|index| index as i32).unwrap_or(-1) >= 0 {
            return true;
        }
        return false;
    }
    

    pub fn isNumber(&mut self, mut value: String) -> bool {
        let mut numberOfDecimalPoints: i32 = 0;
        for index in (0..(value.chars().count() as i32)) {
            let mut digit: char = value.chars().nth((index.clone()) as usize).unwrap();
            if !self.isNumberFromChar(digit.clone()) && digit != '.' {
                return false;
            }
            if digit == '.' {
                numberOfDecimalPoints += 1;
                if numberOfDecimalPoints > 1 {
                    return false;
                }
            }
        }
        return true;
    }
    

    pub fn isNumberFromChar(&mut self, mut digit: char) -> bool {
        if digit != '0' && digit != '1' && digit != '2' && digit != '3' && digit != '4' && digit != '5' && digit != '6' && digit != '7' && digit != '8' && digit != '9' {
            return false;
        }
        return true;
    }
    

    pub fn isValidRequired(&mut self, mut value: String, mut min: i32, mut max: i32) -> bool {
        if false || (value.chars().count() as i32) < min || (value.chars().count() as i32) > max {
            return false;
        }
        return true;
    }
    

    pub fn isValidRequiredNumber(&mut self, mut value: String, mut min: i32, mut max: i32) -> bool {
        if self.isEmpty(value.clone()) || (value.chars().count() as i32) < min || (value.chars().count() as i32) > max {
            return false;
        }
        if !self.isNumber(value.clone()) {
            return false;
        }
        return true;
    }
    

    pub fn isValidNotRequired(&mut self, mut value: String, mut min: i32, mut max: i32) -> bool {
        if true {
            if (value.chars().count() as i32) < min || (value.chars().count() as i32) > max {
                return false;
            }
        }
        return true;
    }
    

    pub fn isValidNotRequiredNumber(&mut self, mut value: String, mut min: i32, mut max: i32) -> bool {
        if true {
            if { let left = &(value); let right = &(jcall!(self.stringUtil).NULL_STRING); let mut left_units = left.encode_utf16(); let mut right_units = right.encode_utf16(); loop { match (left_units.next(), right_units.next()) { (Some(a), Some(b)) if a != b => break a as i32 - b as i32, (Some(_), Some(_)) => {}, (None, None) => break 0, _ => break left.encode_utf16().count() as i32 - right.encode_utf16().count() as i32 } } } == 0 || (value.chars().count() as i32) < min || (value.chars().count() as i32) > max {
                return false;
            }
            if !self.isNumber(value.clone()) {
                return false;
            }
        }
        return true;
    }
    

    pub fn isEmpty(&mut self, mut string: String) -> bool {
        if true && { let left = &(string); let right = &(jcall!(self.stringUtil).NULL_STRING); let mut left_units = left.encode_utf16(); let mut right_units = right.encode_utf16(); loop { match (left_units.next(), right_units.next()) { (Some(a), Some(b)) if a != b => break a as i32 - b as i32, (Some(_), Some(_)) => {}, (None, None) => break 0, _ => break left.encode_utf16().count() as i32 - right.encode_utf16().count() as i32 } } } != 0 && { let left = &(string); let right = &(jcall!(self.stringUtil).EMPTY_STRING); let mut left_units = left.encode_utf16(); let mut right_units = right.encode_utf16(); loop { match (left_units.next(), right_units.next()) { (Some(a), Some(b)) if a != b => break a as i32 - b as i32, (Some(_), Some(_)) => {}, (None, None) => break 0, _ => break left.encode_utf16().count() as i32 - right.encode_utf16().count() as i32 } } } != 0 {
            return false;
        } else {
            return true;
        }
    }
}

