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
// package org.allbinary.logic.system.security.crypt
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::TsUtil::TsUtil;
use crate::org::allbinary::logic::communication::log::PreLogUtil::PreLogUtil;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use crate::org::allbinary::logic::system::security::crypt::jcehelper::AbCrypt::AbCrypt;
use crate::org::allbinary::logic::system::security::crypt::jcehelper::KeySpecFactory::KeySpecFactory;
use crate::org::allbinary::string::CommonStrings::CommonStrings;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct WeakCrypt {
    pub tsUtil: Option<Rc<RefCell<TsUtil>>>,
    pub abCrypt: Option<Rc<RefCell<AbCrypt>>>,
}

pub static mut WeakCrypt_keys: std::cell::LazyCell<Vec<String>> = std::cell::LazyCell::new(|| vec![String::from("qwres6qewrupoi1132790jfahleavi89398"), String::from("143uk32lngf8675gt1tv9214uvt924ut9u9"), String::from("12kvjlvlkv11445v12951v512v9m51295v1"), String::from("gampipoiupg4g43y5u238tu2485u20u0932"), String::from("a93u5uu02s2d2g3259u4395u409u093u9vu"), String::from("nlkxzj90je3mln9l8l89qjf0j3v50j23r9j"), String::from("2lk5fn5j23nr8f0233vq35n23jn58vhy843"), String::from("9q24334m598t092tkn2iv0982456n425j9g"), String::from("5m9uvmt9436243m5v34g09d43i52ki35g99"), String::from("n123lr1iubiu2br32lv32rbu23b32ub2rlb"), String::from("va0u0iovji2ojpvj2oij22poiu4ov2uv28v"), String::from("bkk80gn2v9uc902340ut209pcup0u09utc9"), String::from("mn32m32neee23c42fn32nf3mf2n3fn2m3fm"), String::from("zcxpozo3423voizupovsijevnv3n33o33n3"), String::from("v0345324thv93h03893893vh893v8h3vhzz"), String::from("v8na898v8wyef9ty39yv98yaw39yeeyf8ye"), String::from("45zd36d34jg09434hfshp8uj9sy8s7cyr78"), String::from("b098yy0908vdvh983b2lb327980a76s5e5e"), String::from("v90y9v9p8y3r98pagsdgsagyr6y3985yhfh"), String::from("vblkzhhzp8yvzi8u4t9856456zy3rv8a388"), String::from("da7696023rgf28dssfa98asd879hg32hewe"), String::from("afo7d6ffkjb3jr3i34gb24ytuvu3jr3uryf"), String::from("4gijvqo8vq984hv984hj5j3511h832hf833")]);
pub static mut WeakCrypt_KEYMAX: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| (unsafe { WeakCrypt_keys.clone() }.len() as i32).clone());

impl WeakCrypt {
    

    pub fn new_struct(mut key: i32) -> Self {
        let mut this = Self {
            tsUtil: None,
            abCrypt: None,
        };
        this.tsUtil = TsUtil::getInstance();
        this.abCrypt = AbCrypt::new(jcall!(KeySpecFactory::getInstance()).DES.clone());
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            jcall!(this.abCrypt).init(unsafe { WeakCrypt_keys.clone() }[(key) as usize].clone());
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            let mut commonStrings: Option<Rc<RefCell<CommonStrings>>> = CommonStrings::getInstance();
            PreLogUtil::putOE(jcall!(commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("SuperCrypt(key)"), e.clone());
            }
        }
        this
    }

    pub fn new(mut key: i32) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(key))))
    }
    

    pub fn encrypt(&mut self, mut value: String) -> String {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut crypted: Vec<i8> = { let java_receiver = self.abCrypt.clone(); let java_arguments = (jcall!(self.tsUtil).getByteArray(value.clone()),); let java_result = jcall!(java_receiver).encrypt(java_arguments.0); java_result };
            return DatabaseEncoder::DatabaseEncoder::encode(crypted.clone());
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            PreLogUtil::putOE(String::from("Encrypt Failed"), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("encrypt"), e.clone());
            return jcall!(StringUtil::getInstance()).EMPTY_STRING.clone();
            }
        }
    }
    

    pub fn decrypt(&mut self, mut value: String) -> String {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut decrypted: Vec<i8> = { let java_receiver = self.abCrypt.clone(); let java_arguments = (DatabaseEncoder::DatabaseEncoder::decode(value.clone()),); let java_result = jcall!(java_receiver).decrypt(java_arguments.0); java_result };
            return String::from_utf8(decrypted.clone().iter().map(|value| *value as u8).collect()).unwrap();
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            PreLogUtil::putOE(String::from("decrypt Failed"), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("decrypt"), e.clone());
            return jcall!(StringUtil::getInstance()).EMPTY_STRING.clone();
            }
        }
    }
}

