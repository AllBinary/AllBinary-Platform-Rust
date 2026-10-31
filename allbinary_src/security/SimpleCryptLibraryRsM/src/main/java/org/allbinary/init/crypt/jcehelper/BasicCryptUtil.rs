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
// package org.allbinary.init.crypt.jcehelper
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::java::io::ByteArrayInputStream::ByteArrayInputStream;
// use crate::java::io::ByteArrayOutputStream::ByteArrayOutputStream;
// use crate::java::io::InputStream::InputStream;
use crate::org::allbinary::TsUtil::TsUtil;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use super::CryptInterface::CryptInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct BasicCryptUtil {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub tsUtil: Option<Rc<RefCell<TsUtil>>>,
    pub XML_START: String,
}

thread_local! { pub static BasicCryptUtil_instance: Option<Rc<RefCell<BasicCryptUtil>>> = BasicCryptUtil::new(); }

impl BasicCryptUtil {

    pub fn new_struct() -> Self {
        let mut this = Self {
            logUtil: None,
            tsUtil: None,
            XML_START: String::new(),
        };
        this.logUtil = LogUtil::getInstance();
        this.tsUtil = TsUtil::getInstance();
        this.XML_START = String::from("<?xml");
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<BasicCryptUtil>>> {
        return BasicCryptUtil_instance.with(Clone::clone).clone();
    }
    

    // throws Exception
    pub fn getDecryptedInputStream(&mut self, mut inputStream: Option<Rc<RefCell<InputStream>>>, mut cryptInterface: Option<Rc<RefCell<dyn CryptInterface>>>) -> Option<Rc<RefCell<InputStream>>> {
        let mut buffer: Vec<u8> = Vec::new();
        let mut ch: i32 = 0;
        while true {
            ch = jcall!(inputStream).read();
            if ch == -1 {
                break;
            }
            jcall!(buffer).write(ch.clone());
        }
        let mut decrypted: Vec<i8> = jcall!(cryptInterface).decrypt(jcall!(buffer).toByteArray());
        let mut decryptedString: String = String::from_utf8(decrypted.clone().iter().map(|value| *value as u8).collect()).unwrap();
        let mut index: i32 = decryptedString.find(&self.XML_START.clone()).map(|index| index as i32).unwrap_or(-1);
        if index > 0 {
            decryptedString = decryptedString[(index.clone()) as usize..].to_string();
        }
        return ByteArrayInputStream::new(jcall!(self.tsUtil).getByteArray(decryptedString.clone()));
    }
}

