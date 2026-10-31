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
// package org.allbinary.logic.system.security
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::java::io::ByteArrayOutputStream::ByteArrayOutputStream;
// use crate::java::io::InputStream::InputStream;
use crate::org::allbinary::logic::io::StreamUtil::StreamUtil;
use crate::org::allbinary::logic::system::security::crypt::jcehelper::AbCrypt::AbCrypt;
use crate::org::allbinary::logic::system::security::crypt::jcehelper::KeySpecFactory::KeySpecFactory;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct AbCryptUtil {
    pub streamUtil: Option<Rc<RefCell<StreamUtil>>>,
}

thread_local! { pub static AbCryptUtil_instance: Option<Rc<RefCell<AbCryptUtil>>> = AbCryptUtil::new(); }

impl AbCryptUtil {
    

    pub fn getInstance() -> Option<Rc<RefCell<AbCryptUtil>>> {
        return AbCryptUtil_instance.with(Clone::clone).clone();
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            streamUtil: None,
        };
        this.streamUtil = StreamUtil::getInstance();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    // throws Exception
    pub fn decrypt(&mut self, mut inputStream: Option<Rc<RefCell<InputStream>>>, mut key: String) -> Vec<i8> {
        let mut outputStream: Vec<u8> = None;
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            outputStream = (jcall!(self.streamUtil).get(inputStream.clone(), Vec::new(), vec![0; (16384) as usize])).clone() /* as ByteArrayOutputStream */;
            let mut abCrypt: Option<Rc<RefCell<AbCrypt>>> = AbCrypt::new(jcall!(KeySpecFactory::getInstance()).DESEDE.clone());
            jcall!(abCrypt).init(key.clone());
            return jcall!(abCrypt).decrypt(jcall!(outputStream).toByteArray());
        })) {
            Ok(java_return) => return java_return,
            Err(error) => std::panic::resume_unwind(error),
        }
        { // finally
            jcall!(self.streamUtil).close(outputStream.clone());
            jcall!(self.streamUtil).close(inputStream.clone());
        }
    }
}

