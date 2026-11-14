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
use crate::java::security::Provider::Provider;
use crate::java::security::Security::Security;
use crate::java::security::spec::KeySpec::KeySpec;
use crate::javax::crypto::Cipher::Cipher;
use crate::javax::crypto::SecretKey::SecretKey;
use crate::javax::crypto::SecretKeyFactory::SecretKeyFactory;
use crate::org::allbinary::init::crypt::jcehelper::CryptInterface::CryptInterface;
use crate::org::allbinary::logic::communication::log::PreLogUtil::PreLogUtil;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::java::lang::Exception::*;
use super::SunJCE::SunJCE;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct AbBasicCrypt {
    pub cipher: Option<Rc<RefCell<Cipher>>>,
    pub secretKey: Option<Rc<RefCell<dyn SecretKey>>>,
    pub algorithm: String,
    pub key: Vec<i8>,
}

impl CryptInterface for AbBasicCrypt {
    fn encrypt(&mut self, mut array: Vec<i8>) -> Vec<i8> { Self::encrypt(self, array) }
    fn decrypt(&mut self, mut array: Vec<i8>) -> Vec<i8> { Self::decrypt(self, array) }
}

impl AbBasicCrypt {

    pub fn toString(&mut self) -> String {
        String::from(std::any::type_name::<Self>())
    }
    

    pub fn new_struct(mut algorithm: String, mut key: String) -> Self {
        let mut this = Self {
            cipher: None,
            secretKey: None,
            algorithm: String::new(),
            key: Vec::new(),
        };
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            this.algorithm = algorithm.clone();
            this.key = { let java_value = (key.bytes().map(|value| value as i8).collect::<Vec<i8>>()).clone(); java_value };
            this.init();
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            let mut commonStrings: Option<Rc<RefCell<CommonStrings>>> = CommonStrings::getInstance();
            PreLogUtil::putOE({ let java_field = jcall!(commonStrings).EXCEPTION.clone(); java_field }.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("AbCrypt(alg,key)"), crate::java_into_object(e.clone()));
            }
        }
        this
    }

    pub fn new(mut algorithm: String, mut key: String) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(algorithm, key))))
    }
    

    pub fn init(&mut self) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut sunJce: Option<Rc<RefCell<Provider>>> = SunJCE::new();
            Security::addProvider(sunJce.clone());
            let mut keySpec: Option<Rc<RefCell<dyn KeySpec>>> = jcall!(KeySpecFactory::KeySpecFactory::getInstance()).getInstance2(self.algorithm.clone(), self.key.clone());
            let mut keyFactory: Option<Rc<RefCell<SecretKeyFactory>>> = SecretKeyFactory::getInstance(self.algorithm.clone());
            self.secretKey = { let java_value = (jcall!(keyFactory).generateSecret(keySpec.clone())).clone(); java_value };
            self.cipher = { let java_value = (Cipher::getInstance(self.algorithm.clone())).clone(); java_value };
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            let mut commonStrings: Option<Rc<RefCell<CommonStrings>>> = CommonStrings::getInstance();
            PreLogUtil::putOE(String::from("init Failed"), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(commonStrings).INIT.clone(); java_field }.clone(), crate::java_into_object(e.clone()));
            }
        }
    }
    

    pub fn encrypt(&mut self, mut array: Vec<i8>) -> Vec<i8> {
        let java_try_result = match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            jcall!(self.cipher).init(unsafe { crate::javax::crypto::Cipher::Cipher_ENCRYPT_MODE.clone() }.clone(), self.secretKey.clone());
            let mut ivArray: Vec<i8> = jcall!(self.secretKey).getEncoded();
            let mut encrypted: Vec<i8> = jcall!(self.cipher).doFinal(array.clone());
            let mut result: Vec<i8> = vec![0; ((ivArray.len() as i32) + (encrypted.len() as i32)) as usize];
            PreLogUtil::put(format!("{}{}", String::from("ivArray Length: "), (ivArray.len() as i32)), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("encrypt"));
            for index in (0..(ivArray.len() as i32)) {
                result[(index) as usize] = { let java_value = (ivArray[(index) as usize]).clone(); java_value }.clone();
            }
            for index in (0..(encrypted.len() as i32)) {
                result[(index + (ivArray.len() as i32)) as usize] = { let java_value = (encrypted[(index) as usize]).clone(); java_value }.clone();
            }
            return Some(result.clone());
        })) {
            Ok(java_return) => java_return,
            Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            PreLogUtil::putOE(String::from("Encrypt Failed"), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("encrypt"), crate::java_into_object(e.clone()));
            return None;
            None
            }
        };if let Some(java_return) = java_try_result { return java_return; }unreachable!("try statement")
    }
    

    pub fn decrypt(&mut self, mut array: Vec<i8>) -> Vec<i8> {
        let java_try_result = match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            jcall!(self.cipher).init(unsafe { crate::javax::crypto::Cipher::Cipher_DECRYPT_MODE.clone() }.clone(), self.secretKey.clone());
            let mut ivArray: Vec<i8> = vec![0; (8) as usize];
            for index in (0..8) {
                ivArray[(index) as usize] = { let java_value = (array[(index) as usize]).clone(); java_value }.clone();
            }
            PreLogUtil::put(format!("{}{}", String::from("ivArray Length: "), (ivArray.len() as i32)), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("encrypt"));
            let mut result: Vec<i8> = vec![0; ((array.len() as i32) - (ivArray.len() as i32)) as usize];
            for index in ((ivArray.len() as i32)..(array.len() as i32)) {
                result[(index - (ivArray.len() as i32)) as usize] = { let java_value = (array[(index) as usize]).clone(); java_value }.clone();
            }
            let mut decrypted: Vec<i8> = jcall!(self.cipher).doFinal(result.clone());
            return Some(result.clone());
        })) {
            Ok(java_return) => java_return,
            Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            PreLogUtil::putOE(String::from("decrypt Failed"), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("decrypt"), crate::java_into_object(e.clone()));
            return None;
            None
            }
        };if let Some(java_return) = java_try_result { return java_return; }unreachable!("try statement")
    }
}

