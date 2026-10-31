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
// use crate::java::security::Provider::Provider;
// use crate::java::security::Security::Security;
// use crate::java::security::spec::KeySpec::KeySpec;
// use crate::javax::crypto::Cipher::Cipher;
// use crate::javax::crypto::SecretKey::SecretKey;
// use crate::javax::crypto::SecretKeyFactory::SecretKeyFactory;
use crate::org::allbinary::init::crypt::jcehelper::CryptInterface::CryptInterface;
use crate::org::allbinary::logic::communication::log::PreLogUtil::PreLogUtil;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use super::SunJCE::SunJCE;
use super::provider::provider;
use super::crypto::crypto;
use super::sun::sun;
use super::com::com;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct AbBasicCrypt {
    pub cipher: Option<Rc<RefCell<Cipher>>>,
    pub secretKey: Option<Rc<RefCell<SecretKey>>>,
    pub algorithm: String,
    pub key: Vec<i8>,
}

impl CryptInterface for AbBasicCrypt {
    fn encrypt(&mut self, mut array: Vec<i8>) -> Vec<i8> { Self::encrypt(self, array) }
    fn decrypt(&mut self, mut array: Vec<i8>) -> Vec<i8> { Self::decrypt(self, array) }
}

impl AbBasicCrypt {
    

    pub fn new_struct(mut algorithm: String, mut key: String) -> Self {
        let mut this = Self {
            cipher: None,
            secretKey: None,
            algorithm: String::new(),
            key: Vec::new(),
        };
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            this.algorithm = algorithm.clone();
            this.key = jcall!(key).getBytes();
            this.init();
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            let mut commonStrings: Option<Rc<RefCell<CommonStrings>>> = CommonStrings::getInstance();
            PreLogUtil::putOE(jcall!(commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("AbCrypt(alg,key)"), e.clone());
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
            let mut keySpec: Option<Rc<RefCell<KeySpec>>> = jcall!(KeySpecFactory::KeySpecFactory::getInstance()).getInstance2(self.algorithm.clone(), self.key.clone());
            let mut keyFactory: Option<Rc<RefCell<SecretKeyFactory>>> = SecretKeyFactory::getInstance(self.algorithm.clone());
            self.secretKey = jcall!(keyFactory).generateSecret(keySpec.clone());
            self.cipher = Cipher::getInstance(self.algorithm.clone());
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            let mut commonStrings: Option<Rc<RefCell<CommonStrings>>> = CommonStrings::getInstance();
            PreLogUtil::putOE(String::from("init Failed"), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(commonStrings).INIT.clone(), e.clone());
            }
        }
    }
    

    pub fn encrypt(&mut self, mut array: Vec<i8>) -> Vec<i8> {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            jcall!(self.cipher).init(unsafe { crate::javax::crypto::Cipher::Cipher_ENCRYPT_MODE.clone() }.clone(), self.secretKey.clone());
            let mut ivArray: Vec<i8> = jcall!(self.secretKey).getEncoded();
            let mut encrypted: Vec<i8> = jcall!(self.cipher).doFinal(array.clone());
            let mut result: Vec<i8> = vec![0; ((ivArray.len() as i32) + (encrypted.len() as i32)) as usize];
            PreLogUtil::put(format!("{}{}", String::from("ivArray Length: "), (ivArray.len() as i32)), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("encrypt"));
            for index in (0..(ivArray.len() as i32)) {
                result[(index) as usize] = ivArray[(index) as usize].clone();
            }
            for index in (0..(encrypted.len() as i32)) {
                result[(index + (ivArray.len() as i32)) as usize] = encrypted[(index) as usize].clone();
            }
            return result.clone();
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            PreLogUtil::putOE(String::from("Encrypt Failed"), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("encrypt"), e.clone());
            return None;
            }
        }
    }
    

    pub fn decrypt(&mut self, mut array: Vec<i8>) -> Vec<i8> {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            jcall!(self.cipher).init(unsafe { crate::javax::crypto::Cipher::Cipher_DECRYPT_MODE.clone() }.clone(), self.secretKey.clone());
            let mut ivArray: Vec<i8> = vec![0; (8) as usize];
            for index in (0..8) {
                ivArray[(index) as usize] = array[(index) as usize].clone();
            }
            PreLogUtil::put(format!("{}{}", String::from("ivArray Length: "), (ivArray.len() as i32)), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("encrypt"));
            let mut result: Vec<i8> = vec![0; ((array.len() as i32) - (ivArray.len() as i32)) as usize];
            for index in ((ivArray.len() as i32)..(array.len() as i32)) {
                result[(index - (ivArray.len() as i32)) as usize] = array[(index) as usize].clone();
            }
            let mut decrypted: Vec<i8> = jcall!(self.cipher).doFinal(result.clone());
            return result.clone();
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            PreLogUtil::putOE(String::from("decrypt Failed"), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("decrypt"), e.clone());
            return None;
            }
        }
    }
}

