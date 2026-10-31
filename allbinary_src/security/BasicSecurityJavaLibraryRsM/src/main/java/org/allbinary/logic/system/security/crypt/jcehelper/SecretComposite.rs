// 
// AllBinary Open License Version 1 
// Copyright (c) 2025 AllBinary 
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
// use crate::javax::crypto::Cipher::Cipher;
// use crate::javax::crypto::SecretKey::SecretKey;
use crate::org::allbinary::logic::java::byteutil::ByteUtil::ByteUtil;
use super::BaseSecretComposite::BaseSecretComposite;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct SecretComposite {
    pub base: BaseSecretComposite,
    pub byteUtil: Option<Rc<RefCell<ByteUtil>>>,
    pub cipher: Option<Rc<RefCell<Cipher>>>,
    pub secretKey: Option<Rc<RefCell<SecretKey>>>,
    pub key: Vec<i8>,
}


impl SecretComposite {
    

    pub fn new_struct(mut secretKey: Option<Rc<RefCell<SecretKey>>>, mut cipher: Option<Rc<RefCell<Cipher>>>, mut key: Vec<i8>) -> Self {
        let mut this = Self {
            base: BaseSecretComposite::new_struct(),
            byteUtil: None,
            cipher: None,
            secretKey: None,
            key: Vec::new(),
        };
        this.byteUtil = ByteUtil::getInstance();
        this.secretKey = secretKey.clone();
        this.cipher = cipher.clone();
        this.key = key.clone();
        this
    }

    pub fn new(mut secretKey: Option<Rc<RefCell<SecretKey>>>, mut cipher: Option<Rc<RefCell<Cipher>>>, mut key: Vec<i8>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(secretKey, cipher, key))))
    }
    

    // throws Exception
    pub fn encrypt(&mut self, mut array: Vec<i8>) -> Vec<i8> {
        array = self.mutilate(array.clone());
        jcall!(self.cipher).init(unsafe { crate::javax::crypto::Cipher::Cipher_ENCRYPT_MODE.clone() }.clone(), self.secretKey.clone());
        return jcall!(self.cipher).doFinal(array.clone());
    }
    

    // throws Exception
    pub fn decrypt(&mut self, mut array: Vec<i8>) -> Vec<i8> {
        jcall!(self.cipher).init(unsafe { crate::javax::crypto::Cipher::Cipher_DECRYPT_MODE.clone() }.clone(), self.secretKey.clone());
        return { let java_arguments = (jcall!(self.cipher).doFinal(array.clone()),); self.mutilate(java_arguments.0) };
    }
    

    pub fn mutilate(&mut self, mut array: Vec<i8>) -> Vec<i8> {
        for index in (0..(self.key.len() as i32)) {
            let mut value: i8 = self.key[(index) as usize].clone();
            if ((value) as i32) < 8 && ((value) as i32) > 0 {
                array = jcall!(self.byteUtil).xorByte(array.clone(), ((value) as i32));
            }
        }
        return array.clone();
    }
}
impl std::ops::Deref for SecretComposite {
    type Target = BaseSecretComposite;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for SecretComposite {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::logic::system::security::crypt::jcehelper::BaseSecretComposite::BaseSecretCompositeDyn for SecretComposite {
    fn as_BaseSecretComposite(&self) -> &crate::org::allbinary::logic::system::security::crypt::jcehelper::BaseSecretComposite::BaseSecretComposite { &self.base }
    fn as_BaseSecretComposite_mut(&mut self) -> &mut crate::org::allbinary::logic::system::security::crypt::jcehelper::BaseSecretComposite::BaseSecretComposite { &mut self.base }
    fn encrypt(&mut self, mut array: Vec<i8>) -> Vec<i8> { Self::encrypt(self, array) }
    fn decrypt(&mut self, mut array: Vec<i8>) -> Vec<i8> { Self::decrypt(self, array) }
}


