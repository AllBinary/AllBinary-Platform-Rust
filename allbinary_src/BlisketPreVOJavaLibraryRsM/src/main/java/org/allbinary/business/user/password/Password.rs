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
// package org.allbinary.business.user.password
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;
use super::*;
// use crate::java::util::HashMap::HashMap;
// use crate::java::util::Random::Random;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
use crate::org::allbinary::business::entry::EntryData::EntryData;
use crate::org::allbinary::business::user::UserData::UserData;
use crate::org::allbinary::logic::StdUtil::StdUtil;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::control::crypt::SuperCrypt::SuperCrypt;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use crate::org::allbinary::logic::string::StringValidationUtil::StringValidationUtil;
use crate::org::allbinary::string::CommonStrings::CommonStrings;

// java.util.Random's seeded 48-bit linear congruential generator.
pub struct Random { seed: u64 }
impl Random {
    pub fn new(seed: i64) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self { seed: (seed as u64 ^ 0x5DEECE66D) & ((1u64 << 48) - 1) })))
    }
    pub fn nextInt(&mut self) -> i32 {
        self.seed = self.seed.wrapping_mul(0x5DEECE66D).wrapping_add(11) & ((1u64 << 48) - 1);
        (self.seed >> 16) as u32 as i32
    }
}

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct Password {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub password: String,
}


impl Password {
    

    pub fn new_struct(mut password: String) -> Self {
        let mut this = Self {
            logUtil: None,
            password: String::new(),
        };
        this.logUtil = LogUtil::getInstance();
        this.password = password.clone();
        this
    }

    pub fn new(mut password: String) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(password))))
    }
    

    pub fn set(&mut self, mut value: String) {
        self.password = value.clone();
    }
    

    pub fn get(&mut self) -> String {
        return self.password.clone();
    }
    

    pub fn isValid(&mut self) -> Option<bool> {
        let mut commonStrings: Option<Rc<RefCell<CommonStrings>>> = CommonStrings::getInstance();
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut valid: Option<bool> = true;
            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).VALIDATION.clone()) {
                jcall!(self.logUtil).putF(format!("{}{}", String::from("Password: "), self.password), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(commonStrings).IS_VALID.clone());
            }
            if !jcall!(StringValidationUtil::getInstance()).isValidRequired(self.password.clone(), 6, unsafe { crate::org::allbinary::business::user::UserData::UserData_MAXLEN.clone() }.clone()) {
                if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).VALIDATION.clone()) {
                    jcall!(self.logUtil).putF(String::from("Password is invalid"), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(commonStrings).IS_VALID.clone());
                }
                valid = false;
            }
            return valid.clone();
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).VALIDATIONERROR.clone()) {
                jcall!(self.logUtil).put(String::from("Failed to validate form"), crate::java_into_object(String::from("Password")), jcall!(commonStrings).IS_VALID.clone(), crate::java_into_object(e.clone()));
            }
            return false;
            }
        }
    }
    

    pub fn getValidationInfo(&mut self) -> String {
        if !jcall!(StringValidationUtil::getInstance()).isValidRequired(self.password.clone(), 6, unsafe { crate::org::allbinary::business::user::UserData::UserData_MAXLEN.clone() }.clone()) {
            return String::from("Please enter a Password with more than 6 characters.<br />");
        }
        let mut stringUtil: Option<Rc<RefCell<StringUtil>>> = StringUtil::getInstance();
        return jcall!(stringUtil).EMPTY_STRING.clone();
    }
    

    pub fn toVector(&mut self, mut secret: String) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        self.password = jcall!(StringUtil::getInstance()).getNonNull(self.password.clone());
        let mut random: i32 = jcall!(Random::new()).nextInt(unsafe { crate::org::allbinary::logic::control::crypt::SuperCrypt::SuperCrypt_KEYMAX.clone() }.clone());
        let mut vector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        jcall!(vector).add(crate::java_into_object(jcall!(Some((random.clone()) as i32)).toString()));
        jcall!(vector).add(crate::java_into_object(secret.clone()));
        jcall!(vector).add(crate::java_into_object(jcall!(SuperCrypt::new(random.clone())).encrypt(self.password.clone())));
        return vector.clone();
    }
    

    pub fn toHashMap(&mut self, mut secret: String) -> Option<Rc<RefCell<HashMap>>> {
        self.password = jcall!(StringUtil::getInstance()).getNonNull(self.password.clone());
        let mut values: Option<Rc<RefCell<HashMap>>> = jcall!(StdUtil::getInstance()).createHashMap();
        let mut random: i32 = jcall!(Random::new()).nextInt(unsafe { crate::org::allbinary::logic::control::crypt::SuperCrypt::SuperCrypt_KEYMAX.clone() }.clone());
        jcall!(values).put(jcall!(EntryData::getInstance()).ENCRYPTION.clone(), jcall!(Some((random.clone()) as i32)).toString());
        jcall!(values).put(crate::org::allbinary::business::user::UserData::UserData_SECRET.with(Clone::clone).clone(), secret.clone());
        jcall!(values).put(crate::org::allbinary::business::user::UserData::UserData_PASSWORD.with(Clone::clone).clone(), jcall!(SuperCrypt::new(random.clone())).encrypt(self.password.clone()));
        return values.clone();
    }
}

