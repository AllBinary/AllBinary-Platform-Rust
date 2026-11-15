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
// package org.allbinary.logic.system.security.crypt.jcehelper
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::java::security::spec::KeySpec::KeySpec;
use crate::javax::crypto::spec::DESKeySpec::DESKeySpec;
use crate::javax::crypto::spec::DESedeKeySpec::DESedeKeySpec;
use crate::org::allbinary::TsUtil::TsUtil;
use crate::org::allbinary::logic::communication::log::PreLogUtil::PreLogUtil;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::java::lang::Exception::*;
use crate::java::lang::RuntimeException::*;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct KeySpecFactory {
    pub DES: String,
    pub DESEDE: String,
    pub BLOWFISH: String,
}

pub static KeySpecFactory_instance: crate::JavaStatic<Option<Rc<RefCell<KeySpecFactory>>>> = crate::JavaStatic::new(|| KeySpecFactory::new());

impl KeySpecFactory {

    pub fn toString(&mut self) -> String {
        String::from(std::any::type_name::<Self>())
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<KeySpecFactory>>> {
        return KeySpecFactory_instance.with(Clone::clone).clone();
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            DES: String::new(),
            DESEDE: String::new(),
            BLOWFISH: String::new(),
        };
        this.DES = String::from("DES");
        this.DESEDE = String::from("DESede");
        this.BLOWFISH = String::from("Blowfish");
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    // Java overloaded method: Rust needs a different name
    pub fn getInstance2(&mut self, mut algorithm: String, mut keyData: Vec<i8>) -> Option<Rc<RefCell<dyn KeySpec>>> {
        let java_try_result = match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut tsUtil: Option<Rc<RefCell<TsUtil>>> = TsUtil::getInstance();
            if { let left = (algorithm).clone(); let right = (self.DES).clone(); let mut left_units = left.encode_utf16(); let mut right_units = right.encode_utf16(); loop { match (left_units.next(), right_units.next()) { (Some(a), Some(b)) if a != b => break a as i32 - b as i32, (Some(_), Some(_)) => {}, (None, None) => break 0, _ => break left.encode_utf16().count() as i32 - right.encode_utf16().count() as i32 } } } == 0 {
                return Some(DESKeySpec::new(keyData.clone()).map(|reference| reference as Rc<RefCell<dyn KeySpec>>).map(|reference| reference as Rc<RefCell<dyn KeySpec>>));
            } else if { let left = (algorithm).clone(); let right = (self.DESEDE).clone(); let mut left_units = left.encode_utf16(); let mut right_units = right.encode_utf16(); loop { match (left_units.next(), right_units.next()) { (Some(a), Some(b)) if a != b => break a as i32 - b as i32, (Some(_), Some(_)) => {}, (None, None) => break 0, _ => break left.encode_utf16().count() as i32 - right.encode_utf16().count() as i32 } } } == 0 {
                return Some(DESedeKeySpec::new(keyData.clone()).map(|reference| reference as Rc<RefCell<dyn KeySpec>>).map(|reference| reference as Rc<RefCell<dyn KeySpec>>));
            } else {
                panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
            }None
        })) {
            Ok(java_return) => java_return,
            Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            let mut commonStrings: Option<Rc<RefCell<CommonStrings>>> = CommonStrings::getInstance();
            PreLogUtil::putOE({ let java_field = jcall!(commonStrings).EXCEPTION.clone(); java_field }.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(commonStrings).GET_INSTANCE.clone(); java_field }.clone(), crate::java_into_object(e.clone()));
            panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
            None
            }
        };if let Some(java_return) = java_try_result { return java_return; }unreachable!("try statement")
    }
}

