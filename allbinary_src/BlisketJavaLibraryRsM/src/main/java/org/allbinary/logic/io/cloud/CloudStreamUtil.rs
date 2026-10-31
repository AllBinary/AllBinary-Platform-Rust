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
// package org.allbinary.logic.io
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::java::io::InputStream::InputStream;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::io::file::AbFile::AbFile;
use super::AbFileInputStream::AbFileInputStream;
use super::AbFileLocalInputStream::AbFileLocalInputStream;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct CloudStreamUtil {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
}

thread_local! { pub static CloudStreamUtil_instance: Option<Rc<RefCell<CloudStreamUtil>>> = CloudStreamUtil::new(); }

impl CloudStreamUtil {

    pub fn new_struct() -> Self {
        let mut this = Self {
            logUtil: None,
        };
        this.logUtil = LogUtil::getInstance();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<CloudStreamUtil>>> {
        return CloudStreamUtil_instance.with(Clone::clone).clone();
    }
    

    // throws Exception
    pub fn getFile(&mut self, mut file: Option<Rc<RefCell<AbFile>>>) -> Option<Rc<RefCell<InputStream>>> {
        let mut inputStream: Option<Rc<RefCell<InputStream>>> = AbFileInputStream::new(file.clone());
        let mut available: i32 = jcall!(inputStream).available();
        if available > 0 {
            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).VIEW.clone()) {
                jcall!(self.logUtil).putF(format!("{}{}", String::from("Cloud File Bytes: "), available), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("processRequest()"));
            }
        }
        return inputStream.clone();
    }
    

    // throws Exception
    pub fn getFileAnyWhere(&mut self, mut file: Option<Rc<RefCell<AbFile>>>) -> Option<Rc<RefCell<InputStream>>> {
        let mut inputStream: Option<Rc<RefCell<InputStream>>> = AbFileLocalInputStream::new(file.clone());
        let mut available: i32 = jcall!(inputStream).available();
        if available > 0 {
            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).VIEW.clone()) {
                jcall!(self.logUtil).putF(format!("{}{}", String::from("Local File Bytes: "), available), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("processRequest()"));
            }
        } else {
            inputStream = AbFileInputStream::new(file.clone());
            available = jcall!(inputStream).available();
            if available > 0 {
                if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).VIEWERROR.clone()) {
                    jcall!(self.logUtil).putF(format!("{}{}", String::from("Cloud File Bytes: "), available), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("processRequest()"));
                }
            }
        }
        return inputStream.clone();
    }
    

    // throws Exception
    pub fn getFileLocal(&mut self, mut file: Option<Rc<RefCell<AbFile>>>) -> Option<Rc<RefCell<InputStream>>> {
        let mut inputStream: Option<Rc<RefCell<InputStream>>> = AbFileLocalInputStream::new(file.clone());
        let mut available: i32 = jcall!(inputStream).available();
        if available > 0 {
            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).VIEW.clone()) {
                jcall!(self.logUtil).putF(format!("{}{}", String::from("Local File Bytes: "), available), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("processRequest()"));
            }
        }
        return inputStream.clone();
    }
}

