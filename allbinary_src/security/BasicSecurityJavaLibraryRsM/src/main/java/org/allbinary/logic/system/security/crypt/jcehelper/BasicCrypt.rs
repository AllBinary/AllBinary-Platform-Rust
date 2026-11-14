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
use crate::java::lang::Exception::*;

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

    pub fn toString(&mut self) -> String {
        String::from(std::any::type_name::<Self>())
    }
    

    pub fn new_struct(mut keyAsString: String) -> Self {
        let mut this = Self {
            byteUtil: None,
            key: Vec::new(),
        };
        this.byteUtil = ByteUtil::getInstance();
        let mut key: Vec<i8> = { let java_field = jcall!(NullUtil::getInstance()).NULL_BYTE_ARRAY.clone(); java_field }.clone();
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            key = { let java_value = (jcall!(TsUtil::getInstance()).getByteArray(keyAsString.clone())).clone(); java_value };
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            let mut commonStrings: Option<Rc<RefCell<CommonStrings>>> = CommonStrings::getInstance();
            PreLogUtil::putOE({ let java_field = jcall!(commonStrings).EXCEPTION.clone(); java_field }.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("AbCrypt(alg,key)"), crate::java_into_object(e.clone()));
            }
        }
        this.key = key.clone();
        this
    }

    pub fn new(mut keyAsString: String) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(keyAsString))))
    }
    

    pub fn encrypt(&mut self, mut array: Vec<i8>) -> Vec<i8> {
        let java_try_result = match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            return Some(self.mutilate(array.clone()));
        })) {
            Ok(java_return) => java_return,
            Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            PreLogUtil::putOE(String::from("Encrypt Failed"), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("encrypt"), crate::java_into_object(e.clone()));
            return { let java_field = jcall!(NullUtil::getInstance()).NULL_BYTE_ARRAY.clone(); java_field }.clone();
            None
            }
        };if let Some(java_return) = java_try_result { return java_return; }unreachable!("try statement")
    }
    

    pub fn decrypt(&mut self, mut array: Vec<i8>) -> Vec<i8> {
        let java_try_result = match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            return Some(self.mutilate(array.clone()));
        })) {
            Ok(java_return) => java_return,
            Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            PreLogUtil::putOE(String::from("decrypt Failed"), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("decrypt"), crate::java_into_object(e.clone()));
            return { let java_field = jcall!(NullUtil::getInstance()).NULL_BYTE_ARRAY.clone(); java_field }.clone();
            None
            }
        };if let Some(java_return) = java_try_result { return java_return; }unreachable!("try statement")
    }
    

    pub fn mutilate(&mut self, mut array: Vec<i8>) -> Vec<i8> {
        let mut value: i8 = 0;
        for index in (0..(self.key.len() as i32)) {
            value = { let java_value = (self.key[(index) as usize]).clone(); java_value }.clone();
            array = { let java_value = (jcall!(self.byteUtil).xor(array.clone(), value.clone())).clone(); java_value };
        }
        return array.clone();
    }
}

