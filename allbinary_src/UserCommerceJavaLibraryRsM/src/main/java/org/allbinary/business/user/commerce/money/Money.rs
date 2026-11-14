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
// package org.allbinary.business.user.commerce.money
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::logic::io::path::AbPathData::AbPathData;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use crate::org::allbinary::logic::string::StringValidationUtil::StringValidationUtil;
use crate::org::allbinary::string::CommonPhoneStrings::CommonPhoneStrings;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct Money {
    pub units: Option<i64>,
    pub DEFAULT_CURRENCY: String,
}


impl Money {
    

    pub fn new_struct1(mut money: Option<Rc<RefCell<Money>>>) -> Self {
        let mut this = Self {
            units: None,
            DEFAULT_CURRENCY: String::new(),
        };
        this.DEFAULT_CURRENCY = String::from("USD");
        this.units = { let java_value = (Some((jcall!(money).getUnits()) as i64)).clone(); java_value };
        this
    }

    pub fn new1(mut money: Option<Rc<RefCell<Money>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct1(money))))
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            units: None,
            DEFAULT_CURRENCY: String::new(),
        };
        this.DEFAULT_CURRENCY = String::from("USD");
        this.units = Some((0) as i64);
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn new_struct1_String(mut usDollarStr: String) -> Self {
        let mut this = Self {
            units: None,
            DEFAULT_CURRENCY: String::new(),
        };
        this.DEFAULT_CURRENCY = String::from("USD");
        if true && jcall!(StringValidationUtil::getInstance()).isNumber(usDollarStr.clone()) {
            this.units = Some((this.convertUsdToUnits(usDollarStr.clone())) as i64);
        }
        this
    }

    pub fn new1_String(mut usDollarStr: String) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct1_String(usDollarStr))))
    }
    

    pub fn new_struct1_long(mut units: i64) -> Self {
        let mut this = Self {
            units: None,
            DEFAULT_CURRENCY: String::new(),
        };
        this.DEFAULT_CURRENCY = String::from("USD");
        this.units = Some((units.clone()) as i64);
        this
    }

    pub fn new1_long(mut units: i64) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct1_long(units))))
    }
    

    pub fn isValid(&mut self) -> bool {
        if self.units.is_some() && ((self.units).unwrap() as i64) > 0_i64 && ((self.units).unwrap() as i64) < i64::MAX {
            return true;
        } else {
            return false;
        }
    }
    

    pub fn add(&mut self, mut moreMoney: Option<Rc<RefCell<Money>>>) {
        { let java_arguments = (jcall!(moreMoney).toString(),); self.add(java_arguments.0) };
    }
    

    // Java overloaded method: Rust needs a different name
    pub fn add_String(&mut self, mut usDollarStr: String) {
        self.units = { let java_value = (Some(({ let java_value = ((self.units).unwrap() as i64); java_value } + self.convertUsdToUnits(usDollarStr.clone())) as i64)).clone(); java_value };
    }
    

    pub fn subtract(&mut self, mut usDollarStr: String) {
        self.units = { let java_value = (Some(({ let java_value = ((self.units).unwrap() as i64); java_value } - self.convertUsdToUnits(usDollarStr.clone())) as i64)).clone(); java_value };
    }
    

    pub fn multiply(&mut self, mut multiplier: Option<f32>) {
        self.units = { let java_value = (Some((((Some((((((self.units).unwrap() as i64)) as f32) * ((multiplier).unwrap() as f32)) as f32)).unwrap() as i64)) as i64)).clone(); java_value };
    }
    

    // Java overloaded method: Rust needs a different name
    pub fn multiply_int(&mut self, mut multiplier: i32) {
        self.units = { let java_value = (Some((((self.units).unwrap() as i64) * ((multiplier) as i64)) as i64)).clone(); java_value };
    }
    

    pub fn convertUnitsTo(&mut self, mut currency: String) -> String {
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        if (currency.cmp(&self.DEFAULT_CURRENCY.clone()) as i32) == 0 {
            let mut dollar: i64 = ((self.units).unwrap() as i64) / 100_i64;
            let mut cents: i64 = ((self.units).unwrap() as i64) - (dollar * 100_i64);
            jcall!(stringBuffer).append((Some((dollar.clone()) as i64)).expect("NullPointerException").to_string());
            jcall!(stringBuffer).append({ let java_field = jcall!(AbPathData::getInstance()).EXTENSION_SEP.clone(); java_field }.clone());
            if cents < 10_i64 {
                jcall!(stringBuffer).append({ let java_field = jcall!(CommonPhoneStrings::getInstance()).ZERO.clone(); java_field }.clone());
            }
            jcall!(stringBuffer).append((Some((cents.clone()) as i64)).expect("NullPointerException").to_string());
        }
        return jcall!(stringBuffer).toString();
    }
    

    pub fn convertUsdToUnits(&mut self, mut usDollarStr: String) -> i64 {
        let mut localUnit: i64 = 0_i64;
        let mut index: i32 = usDollarStr.find(&{ let java_field = jcall!(AbPathData::getInstance()).EXTENSION_SEP.clone(); java_field }.clone()).map(|index| index as i32).unwrap_or(-1);
        let mut stringUtil: Option<Rc<RefCell<StringUtil>>> = StringUtil::getInstance();
        let mut dollarStr: String = { let java_field = jcall!(stringUtil).EMPTY_STRING.clone(); java_field }.clone();
        let mut centsStr: String = { let java_field = jcall!(stringUtil).EMPTY_STRING.clone(); java_field }.clone();
        if index < 1 {
            dollarStr = usDollarStr.clone().clone();
            centsStr = String::from("00");
        } else {
            dollarStr = { let java_value = (usDollarStr[(0) as usize..(index) as usize].to_string().clone()).clone(); java_value };
            centsStr = { let java_value = (usDollarStr[(index + 1) as usize..].to_string().clone()).clone(); java_value };
        }
        localUnit = { let java_value = (((Some((((Some((dollarStr.clone()).trim().parse::<i64>().expect("NumberFormatException"))).unwrap() as i64) * 100_i64) as i64)).unwrap() as i64)).clone(); java_value };
        localUnit += ((Some((centsStr.clone()).trim().parse::<i64>().expect("NumberFormatException"))).unwrap() as i64);
        return localUnit.clone();
    }
    

    pub fn getUnits(&mut self) -> i64 {
        return ((self.units).unwrap() as i64);
    }
    

    pub fn getUnitsLong(&mut self) -> Option<i64> {
        return self.units.clone();
    }
    

    pub fn toString(&mut self) -> String {
        return { let java_arguments = (self.DEFAULT_CURRENCY.clone(),); self.convertUnitsTo(java_arguments.0) }.clone();
    }
}

