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
// use crate::java::security::Security::Security;
// use crate::java::security::spec::KeySpec::KeySpec;
// use crate::javax::crypto::Cipher::Cipher;
// use crate::javax::crypto::SecretKey::SecretKey;
// use crate::javax::crypto::SecretKeyFactory::SecretKeyFactory;
use crate::org::allbinary::TsUtil::TsUtil;
use crate::org::allbinary::init::crypt::jcehelper::CryptInterface::CryptInterface;
use crate::org::allbinary::logic::NullUtil::NullUtil;
use crate::org::allbinary::logic::communication::log::PreLogUtil::PreLogUtil;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::org::bouncycastle::jce::provider::BouncyCastleProvider::BouncyCastleProvider;
use super::BaseSecretComposite::BaseSecretComposite;
use super::SecretComposite::SecretComposite;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct AbCrypt {
    pub tsUtil: Option<Rc<RefCell<TsUtil>>>,
    pub secretComposite: Option<Rc<RefCell<dyn crate::org::allbinary::logic::system::security::crypt::jcehelper::BaseSecretComposite::BaseSecretCompositeDyn>>>,
    pub algorithm: String,
}

impl CryptInterface for AbCrypt {
    fn encrypt(&mut self, mut array: Vec<i8>) -> Vec<i8> { Self::encrypt(self, array) }
    fn decrypt(&mut self, mut array: Vec<i8>) -> Vec<i8> { Self::decrypt(self, array) }
}

impl AbCrypt {
    

    pub fn new_struct(mut algorithm: String) -> Self {
        let mut this = Self {
            tsUtil: None,
            secretComposite: None,
            algorithm: String::new(),
        };
        this.tsUtil = TsUtil::getInstance();
        this.secretComposite = BaseSecretComposite_NULL_SECRET_COMPOSITE.with(Clone::clone).clone();
        this.algorithm = algorithm.clone();
        this
    }

    pub fn new(mut algorithm: String) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(algorithm))))
    }
    

    pub fn init(&mut self, mut keyAsString: String) {
        let mut commonStrings: Option<Rc<RefCell<CommonStrings>>> = CommonStrings::getInstance();
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                Security::addProvider(BouncyCastleProvider::new());
            })) {
                Ok(_) => {}
                Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

                PreLogUtil::putOE(jcall!(commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(commonStrings).INIT.clone(), e.clone());
                }
            }
            let mut key: Vec<i8> = jcall!(self.tsUtil).getByteArray(keyAsString.clone());
            let mut keySpec: Option<Rc<RefCell<KeySpec>>> = jcall!(KeySpecFactory::KeySpecFactory::getInstance()).getInstance2(self.algorithm.clone(), key.clone());
            let mut keyFactory: Option<Rc<RefCell<SecretKeyFactory>>> = SecretKeyFactory::getInstance(self.algorithm.clone());
            let mut secretKey: Option<Rc<RefCell<SecretKey>>> = jcall!(keyFactory).generateSecret(keySpec.clone());
            let mut cipher: Option<Rc<RefCell<Cipher>>> = Cipher::getInstance(self.algorithm.clone());
            self.secretComposite = SecretComposite::new(secretKey.clone(), cipher.clone(), key.clone()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::system::security::crypt::jcehelper::BaseSecretComposite::BaseSecretCompositeDyn>>);
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            PreLogUtil::putOE(String::from("init Failed"), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(commonStrings).INIT.clone(), e.clone());
            }
        }
    }
    

    pub fn encrypt(&mut self, mut array: Vec<i8>) -> Vec<i8> {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            return jcall!(self.secretComposite).encrypt(array.clone());
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
            return jcall!(self.secretComposite).decrypt(array.clone());
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            PreLogUtil::putOE(String::from("decrypt Failed"), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("decrypt"), e.clone());
            return jcall!(NullUtil::getInstance()).NULL_BYTE_ARRAY.clone();
            }
        }
    }
}

