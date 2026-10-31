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
// use crate::java::security::spec::KeySpec::KeySpec;
// use crate::javax::crypto::spec::DESKeySpec::DESKeySpec;
// use crate::javax::crypto::spec::DESedeKeySpec::DESedeKeySpec;
use crate::org::allbinary::TsUtil::TsUtil;
use crate::org::allbinary::logic::communication::log::PreLogUtil::PreLogUtil;
use crate::org::allbinary::string::CommonStrings::CommonStrings;

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

thread_local! { pub static KeySpecFactory_instance: Option<Rc<RefCell<KeySpecFactory>>> = KeySpecFactory::new(); }

impl KeySpecFactory {
    

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
    pub fn getInstance2(&mut self, mut algorithm: String, mut keyData: Vec<i8>) -> Option<Rc<RefCell<KeySpec>>> {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut tsUtil: Option<Rc<RefCell<TsUtil>>> = TsUtil::getInstance();
            if { let left = &(algorithm); let right = &(self.DES); let mut left_units = left.encode_utf16(); let mut right_units = right.encode_utf16(); loop { match (left_units.next(), right_units.next()) { (Some(a), Some(b)) if a != b => break a as i32 - b as i32, (Some(_), Some(_)) => {}, (None, None) => break 0, _ => break left.encode_utf16().count() as i32 - right.encode_utf16().count() as i32 } } } == 0 {
                return DESKeySpec::new(keyData.clone());
            } else if { let left = &(algorithm); let right = &(self.DESEDE); let mut left_units = left.encode_utf16(); let mut right_units = right.encode_utf16(); loop { match (left_units.next(), right_units.next()) { (Some(a), Some(b)) if a != b => break a as i32 - b as i32, (Some(_), Some(_)) => {}, (None, None) => break 0, _ => break left.encode_utf16().count() as i32 - right.encode_utf16().count() as i32 } } } == 0 {
                return DESedeKeySpec::new(keyData.clone());
            } else {
                panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
            }
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            let mut commonStrings: Option<Rc<RefCell<CommonStrings>>> = CommonStrings::getInstance();
            PreLogUtil::putOE(jcall!(commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(commonStrings).GET_INSTANCE.clone(), e.clone());
            panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
            }
        }
    }
}

