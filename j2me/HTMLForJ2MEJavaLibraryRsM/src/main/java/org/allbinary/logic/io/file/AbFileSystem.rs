// 
// AllBinary Open License Version 1 
// Copyright (c) 2026 AllBinary 
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
// package org.allbinary.logic.io.file
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::java::io::InputStream::InputStream;
use crate::org::allbinary::logic::NullUtil::NullUtil;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::java::lang::Exception::*;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

//HTMLForJ2ME
pub struct AbFileSystem {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub commonStrings: Option<Rc<RefCell<CommonStrings>>>,
}

pub static AbFileSystem_instance: crate::JavaStatic<Option<Rc<RefCell<AbFileSystem>>>> = crate::JavaStatic::new(|| AbFileSystem::new());

impl AbFileSystem {

    pub fn toString(&mut self) -> String {
        String::from(std::any::type_name::<Self>())
    }

    pub fn new_struct() -> Self {
        let mut this = Self {
            logUtil: None,
            commonStrings: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.commonStrings = CommonStrings::getInstance();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<AbFileSystem>>> {
        return AbFileSystem_instance.with(Clone::clone).clone();
    }
    

    pub fn isDirectoryOrFile(&mut self, mut path: String) -> bool {
        return false;
    }
    

    pub fn isDirectory(&mut self, mut path: String) -> bool {
        return false;
    }
    

    pub fn getFilesAsStringArrayForPath(&mut self, mut currentDirPath: String) -> Vec<String> {
        return jcall!(StringUtil::getInstance()).getArrayInstance();
    }
    

    pub fn readAsString(&mut self, mut fileName: String) -> String {
        let mut bytes: Vec<i8> = vec![0; (1000000) as usize];
        return self.readAsString2(fileName.clone(), bytes.clone());
    }
    

    // Java overloaded method: Rust needs a different name
    pub fn readAsString2(&mut self, mut fileName: String, mut bytes: Vec<i8>) -> String {
        return { let java_field = jcall!(StringUtil::getInstance()).EMPTY_STRING.clone(); java_field }.clone();
    }
    

    pub fn close(&mut self, mut closeable: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) -> bool {
        let java_try_result = match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            if closeable.is_some() {
                let mut inputStream: Option<Rc<RefCell<dyn crate::java::io::InputStream::InputStreamDyn>>> = ({ let java_object = (closeable).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn crate::java::io::InputStream::InputStreamDyn>>>>().expect("ClassCastException").clone()) });
                jcall!(inputStream).as_InputStream_mut().close();
            }
            return Some(true);
        })) {
            Ok(java_return) => java_return,
            Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            jcall!(self.logUtil).put({ let java_field = jcall!(self.commonStrings).EXCEPTION.clone(); java_field }.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(self.commonStrings).CLOSE.clone(); java_field }.clone(), crate::java_into_object(e.clone()));
            return false;
            None
            }
        };if let Some(java_return) = java_try_result { return java_return; }unreachable!("try statement")
    }
}

