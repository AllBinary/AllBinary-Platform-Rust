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
// package org.allbinary.logic.system.loader
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::java::io::FileInputStream::FileInputStream;
// use crate::java::io::IOException::IOException;
// use crate::java::util::Map::Map;
use crate::org::allbinary::logic::StdUtil::StdUtil;
use crate::org::allbinary::logic::communication::log::LogBuffer::LogBuffer;
use crate::org::allbinary::logic::communication::log::LogFactory::LogFactory;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::io::StreamUtil::StreamUtil;
use crate::org::allbinary::logic::io::path::AbPathData::AbPathData;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use crate::org::allbinary::logic::system::security::AbCryptUtil::AbCryptUtil;
use super::ClassLoader::ClassLoader;
use super::NoClassDefFoundError::NoClassDefFoundError;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct AbeClassLoader {
    pub base: ClassLoader,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub abCryptUtil: Option<Rc<RefCell<AbCryptUtil>>>,
    pub LOAD_CLASS: String,
    pub ENCRYPTED_EXTENSION: String,
    pub key: String,
}

pub static mut AbeClassLoader_classes: std::cell::LazyCell<Option<Rc<RefCell<dyn Map>>>> = std::cell::LazyCell::new(|| jcall!(StdUtil::getInstance()).createHashMap());
pub static mut AbeClassLoader_PATH: std::cell::LazyCell<String> = std::cell::LazyCell::new(|| None);

impl AbeClassLoader {
    

    pub fn new_struct(mut parent: Option<Rc<RefCell<ClassLoader>>>, mut key: String) -> Self {
        let mut this = Self {
            base: ClassLoader::new_struct1(parent.clone()),
            logUtil: None,
            abCryptUtil: None,
            LOAD_CLASS: String::new(),
            ENCRYPTED_EXTENSION: String::new(),
            key: String::new(),
        };
        this.logUtil = LogUtil::getInstance();
        this.abCryptUtil = AbCryptUtil::getInstance();
        this.LOAD_CLASS = String::from("loadClass");
        this.ENCRYPTED_EXTENSION = format!("{}{}", jcall!(AbPathData::getInstance()).EXTENSION_SEP, String::from("abc"));
        // super(...) is in the struct literal above
        this.key = key.clone();
        unsafe { *AbeClassLoader_PATH = format!("{}{}", jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).globals).URLGLOBALS).getWebappPath(), String::from("WEB-INF/classes/")) };
        this
    }

    pub fn new(mut parent: Option<Rc<RefCell<ClassLoader>>>, mut key: String) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(parent, key))))
    }
    

    pub fn findLoadedClass1(&mut self, mut name: String) -> &'static str {
        return (jcall!(unsafe { AbeClassLoader_classes.clone() }).get(name.clone())).clone() /* as Class */;
    }
    

    // throws ClassNotFoundException
    // synchronized
    pub fn loadClass(&mut self, mut name: String) -> &'static str {
        return self.loadClass2(name.clone(), false);
    }
    

    // throws ClassNotFoundException
    // Java overloaded method: Rust needs a different name
    // synchronized
    pub fn loadClass2(&mut self, mut name: String, mut resolve: bool) -> &'static str {
        let mut loadedWith: String = String::from("findLoadedClass1");
        let mut logBuffer: Option<Rc<RefCell<LogBuffer>>> = LogBuffer::new();
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut myClass: &'static str = self.findLoadedClass1(name.clone());
            if myClass.is_none() {
                match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    loadedWith = String::from("super.loadClass");
                    let mut normalClass: &'static str = self.base.loadClass2(name.clone(), resolve.clone());
                    if normalClass.is_some() {
                        return normalClass.clone();
                    }
                })) {
                    Ok(_) => {}
                    Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

                    jcall!(logBuffer).add(LogFactory::getInstance(format!("{}{}", format!("{}{}", format!("{}{}", String::from("Failed to Load Class: "), name), String::from("\nwith: ")), loadedWith), crate::java_into_object(String::from(std::any::type_name::<Self>())), self.LOAD_CLASS.clone(), e.clone()));
                    }
                }
                let mut classBytes: Vec<i8> = self.loadClassBytesFromFile(name.clone());
                if classBytes.is_none() {
                    panic!("{:?}", format!("{}: {:?}", "ClassNotFoundException", (format!("{}{}", format!("{}{}", format!("{}{}", String::from("My Bytes not loaded for: "), name), String::from("\nwith: ")), String::from("loadClassBytes")))));
                }
                myClass = self.defineClass(name.clone(), classBytes.clone(), 0, (classBytes.len() as i32).clone());
                if myClass.is_none() {
                    panic!("{:?}", format!("{}: {:?}", "ClassNotFoundException", (format!("{}{}", format!("{}{}", format!("{}{}", String::from("My Class Not Defineable for: "), name), String::from("\nwith: ")), String::from("loadClassBytes")))));
                }
                jcall!(unsafe { AbeClassLoader_classes.clone() }).put(name.clone(), myClass.clone());
            } else {
                jcall!(logBuffer).add(LogFactory::getInstanceF(format!("{}{}", format!("{}{}", format!("{}{}", String::from("Already Loaded: "), name), String::from("\nwith: ")), loadedWith), crate::java_into_object(String::from(std::any::type_name::<Self>())), self.LOAD_CLASS.clone()));
            }
            self.resolveClass(myClass.clone());
            return myClass.clone();
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).LOADERERROR.clone()) {
                jcall!(logBuffer).logAll();
                jcall!(self.logUtil).put(format!("{}{}", format!("{}{}", format!("{}{}", String::from("Failure loading: "), name), String::from("\nwith: ")), loadedWith), crate::java_into_object(String::from(std::any::type_name::<Self>())), self.LOAD_CLASS.clone(), crate::java_into_object(e.clone()));
            }
            return None;
            }
        }
    }
    

    pub fn loadClassBytesFromFile(&mut self, mut name: String) -> Vec<i8> {
        let mut r#in: Option<Rc<RefCell<FileInputStream>>> = None;
        let mut cname: String = jcall!(StringUtil::getInstance()).EMPTY_STRING.clone();
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            cname = format!("{}{}", format!("{}{}", unsafe { AbeClassLoader_PATH.clone() }, name.replace(&'.', &jcall!(AbPathData::getInstance()).SEPARATORCHAR)), self.ENCRYPTED_EXTENSION);
            r#in = FileInputStream::new(cname.clone());
            let mut decrypted: Vec<i8> = jcall!(self.abCryptUtil).decrypt(r#in.clone(), self.key.clone());
            return decrypted.clone();
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).LOADERERROR.clone()) {
                jcall!(self.logUtil).put(format!("{}{}", format!("{}{}", format!("{}{}", String::from("(Before LogBuffer Output) Failure loading Encrypted: "), name), String::from(" File: ")), cname), crate::java_into_object(String::from("AbeClassLoader")), String::from("loadClassBytes"), crate::java_into_object(e.clone()));
            }
            jcall!(StreamUtil::getInstance()).close(r#in.clone());
            }
        }
        return None;
    }
    

    // throws ClassNotFoundException
    // synchronized
    pub fn oldLoadClass(&mut self, mut name: String, mut resolve: bool) -> &'static str {
        let mut loadedWith: String = String::from("findLoadedClass1");
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut myClass: &'static str = self.findLoadedClass1(name.clone());
            if myClass.is_none() {
                match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    loadedWith = String::from("super.loadClass");
                    let mut normalClass: &'static str = self.base.loadClass2(name.clone(), resolve.clone());
                    if normalClass.is_some() {
                        return normalClass.clone();
                    }
                })) {
                    Ok(_) => {}
                    Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

                    if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).LOADER.clone()) {
                        jcall!(self.logUtil).putF(format!("{}{}", format!("{}{}", format!("{}{}", String::from("Failed Loaded Class: "), name), String::from(" with: ")), loadedWith), crate::java_into_object(String::from(std::any::type_name::<Self>())), self.LOAD_CLASS.clone());
                    }
                    }
                }
                loadedWith = String::from("loadClassBytes");
                let mut classBytes: Vec<i8> = self.loadClassBytesFromFile(name.clone());
                if classBytes.is_none() {
                    panic!("{:?}", format!("{}: {:?}", "ClassNotFoundException", (format!("{}{}", String::from("Bytes not loaded for: "), name))));
                }
                myClass = self.defineClass(name.clone(), classBytes.clone(), 0, (classBytes.len() as i32).clone());
                if myClass.is_none() {
                    panic!("{:?}", format!("{}: {:?}", "ClassNotFoundException", (format!("{}{}", String::from("Class Not Defineable for: "), name))));
                }
                jcall!(unsafe { AbeClassLoader_classes.clone() }).put(name.clone(), myClass.clone());
            } else {
                if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).LOADER.clone()) {
                    jcall!(self.logUtil).putF(format!("{}{}", format!("{}{}", format!("{}{}", String::from("Already Loaded: "), name), String::from(" with: ")), loadedWith), crate::java_into_object(String::from(std::any::type_name::<Self>())), self.LOAD_CLASS.clone());
                }
            }
            self.resolveClass(myClass.clone());
            return myClass.clone();
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).LOADERERROR.clone()) {
                jcall!(self.logUtil).put(format!("{}{}", String::from("Failure loading: "), name), crate::java_into_object(String::from("AbeClassLoader")), self.LOAD_CLASS.clone(), crate::java_into_object(e.clone()));
            }
            return None;
            }
        }
    }
}
impl std::ops::Deref for AbeClassLoader {
    type Target = ClassLoader;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for AbeClassLoader {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}



