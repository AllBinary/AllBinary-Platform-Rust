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
use crate::org::allbinary::TsUtil::TsUtil;
use crate::org::allbinary::init::crypt::jcehelper::CryptInterface::CryptInterface;
use crate::org::allbinary::logic::NullUtil::NullUtil;
use crate::org::allbinary::logic::communication::log::PreLogUtil::PreLogUtil;
use crate::org::allbinary::logic::java::byteutil::ByteUtil::ByteUtil;
use crate::org::allbinary::string::CommonStrings::CommonStrings;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct BasicCrypt {
    pub byteUtil: Option<Rc<RefCell<ByteUtil>>>,
    pub key: Vec<i8>,
}

impl CryptInterface for BasicCrypt {
    fn encrypt(&mut self, mut array: Vec<i8>) -> Vec<i8> { Self::encrypt(self, array) }
    fn decrypt(&mut self, mut array: Vec<i8>) -> Vec<i8> { Self::decrypt(self, array) }
}

impl BasicCrypt {
    

    pub fn new_struct(mut keyAsString: String) -> Self {
        let mut this = Self {
            byteUtil: None,
            key: Vec::new(),
        };
        this.byteUtil = ByteUtil::getInstance();
        let mut key: Vec<i8> = jcall!(NullUtil::getInstance()).NULL_BYTE_ARRAY.clone();
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            key = jcall!(TsUtil::getInstance()).getByteArray(keyAsString.clone());
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            let mut commonStrings: Option<Rc<RefCell<CommonStrings>>> = CommonStrings::getInstance();
            PreLogUtil::putOE(jcall!(commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("AbCrypt(alg,key)"), e.clone());
            }
        }
        this.key = key.clone();
        this
    }

    pub fn new(mut keyAsString: String) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(keyAsString))))
    }
    

    pub fn encrypt(&mut self, mut array: Vec<i8>) -> Vec<i8> {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            return self.mutilate(array.clone());
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            PreLogUtil::putOE(String::from("Encrypt Failed"), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("encrypt"), e.clone());
            return jcall!(NullUtil::getInstance()).NULL_BYTE_ARRAY.clone();
            }
        }
    }
    

    pub fn decrypt(&mut self, mut array: Vec<i8>) -> Vec<i8> {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            return self.mutilate(array.clone());
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            PreLogUtil::putOE(String::from("decrypt Failed"), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("decrypt"), e.clone());
            return jcall!(NullUtil::getInstance()).NULL_BYTE_ARRAY.clone();
            }
        }
    }
    

    pub fn mutilate(&mut self, mut array: Vec<i8>) -> Vec<i8> {
        let mut value: i8 = 0;
        for index in (0..(self.key.len() as i32)) {
            value = self.key[(index) as usize].clone();
            array = jcall!(self.byteUtil).xor(array.clone(), value.clone());
        }
        return array.clone();
    }
}

