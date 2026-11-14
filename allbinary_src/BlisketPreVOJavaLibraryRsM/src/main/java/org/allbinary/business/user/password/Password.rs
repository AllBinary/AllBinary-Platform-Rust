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
use super::*;
use crate::java::util::HashMap::HashMap;
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
use crate::java::lang::Exception::*;

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

    pub fn toString(&mut self) -> String {
        String::from(std::any::type_name::<Self>())
    }
    

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
        let java_try_result = match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut valid: Option<bool> = true;
            if jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypes.clone(); java_field }).LOGGING.clone(); java_field }).contains({ let java_field = jcall!(jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypeFactory.clone(); java_field }).getInstance()).VALIDATION.clone(); java_field }.clone()) {
                jcall!(self.logUtil).putF(format!("{}{}", String::from("Password: "), self.password), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(commonStrings).IS_VALID.clone(); java_field }.clone());
            }
            if !jcall!(StringValidationUtil::getInstance()).isValidRequired(self.password.clone(), 6, unsafe { crate::org::allbinary::business::user::UserData::UserData_MAXLEN.clone() }.clone()) {
                if jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypes.clone(); java_field }).LOGGING.clone(); java_field }).contains({ let java_field = jcall!(jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypeFactory.clone(); java_field }).getInstance()).VALIDATION.clone(); java_field }.clone()) {
                    jcall!(self.logUtil).putF(String::from("Password is invalid"), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(commonStrings).IS_VALID.clone(); java_field }.clone());
                }
                valid = { let java_value = (false).clone(); java_value }.clone();
            }
            return Some(valid.clone());
        })) {
            Ok(java_return) => java_return,
            Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            if jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypes.clone(); java_field }).LOGGING.clone(); java_field }).contains({ let java_field = jcall!(jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypeFactory.clone(); java_field }).getInstance()).VALIDATIONERROR.clone(); java_field }.clone()) {
                jcall!(self.logUtil).put(String::from("Failed to validate form"), crate::java_into_object(String::from("Password")), { let java_field = jcall!(commonStrings).IS_VALID.clone(); java_field }.clone(), crate::java_into_object(e.clone()));
            }
            return false;
            None
            }
        };if let Some(java_return) = java_try_result { return java_return; }unreachable!("try statement")
    }
    

    pub fn getValidationInfo(&mut self) -> String {
        if !jcall!(StringValidationUtil::getInstance()).isValidRequired(self.password.clone(), 6, unsafe { crate::org::allbinary::business::user::UserData::UserData_MAXLEN.clone() }.clone()) {
            return String::from("Please enter a Password with more than 6 characters.<br />");
        }
        let mut stringUtil: Option<Rc<RefCell<StringUtil>>> = StringUtil::getInstance();
        return { let java_field = jcall!(stringUtil).EMPTY_STRING.clone(); java_field }.clone();
    }
    

    pub fn toVector(&mut self, mut secret: String) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        self.password = { let java_value = (jcall!(StringUtil::getInstance()).getNonNull(self.password.clone())).clone(); java_value };
        let mut random: i32 = jcall!(Random::new()).nextInt(unsafe { crate::org::allbinary::logic::control::crypt::SuperCrypt::SuperCrypt_KEYMAX.clone() }.clone());
        let mut vector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        jcall!(vector).add(crate::java_into_object((Some((random.clone()) as i32)).expect("NullPointerException").to_string()));
        jcall!(vector).add(crate::java_into_object(secret.clone()));
        jcall!(vector).add(crate::java_into_object(jcall!(SuperCrypt::new(random.clone())).encrypt(self.password.clone())));
        return vector.clone();
    }
    

    pub fn toHashMap(&mut self, mut secret: String) -> Option<Rc<RefCell<HashMap<crate::JavaObject, crate::JavaObject>>>> {
        self.password = { let java_value = (jcall!(StringUtil::getInstance()).getNonNull(self.password.clone())).clone(); java_value };
        let mut values: Option<Rc<RefCell<HashMap<crate::JavaObject, crate::JavaObject>>>> = jcall!(StdUtil::getInstance()).createHashMap();
        let mut random: i32 = jcall!(Random::new()).nextInt(unsafe { crate::org::allbinary::logic::control::crypt::SuperCrypt::SuperCrypt_KEYMAX.clone() }.clone());
        jcall!(values).put(crate::java_into_object({ let java_field = jcall!(EntryData::getInstance()).ENCRYPTION.clone(); java_field }.clone()), crate::java_into_object((Some((random.clone()) as i32)).expect("NullPointerException").to_string()));
        jcall!(values).put(crate::java_into_object(crate::org::allbinary::business::user::UserData::UserData_SECRET.with(Clone::clone).clone()), crate::java_into_object(secret.clone()));
        jcall!(values).put(crate::java_into_object(crate::org::allbinary::business::user::UserData::UserData_PASSWORD.with(Clone::clone).clone()), crate::java_into_object(jcall!(SuperCrypt::new(random.clone())).encrypt(self.password.clone())));
        return values.clone();
    }
}

