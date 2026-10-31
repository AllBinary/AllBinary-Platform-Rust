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
// package org.allbinary.logic.communication.ftp.configuration
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct FtpConfigurationData {
}

thread_local! { pub static FtpConfigurationData_NAME: String = String::from("FTP_NAME"); }
thread_local! { pub static FtpConfigurationData_SERVER: String = String::from("FTP_SERVER"); }
thread_local! { pub static FtpConfigurationData_PATH: String = String::from("FTP_PATH"); }
thread_local! { pub static FtpConfigurationData_USERNAME: String = String::from("FTP_USERNAME"); }
thread_local! { pub static FtpConfigurationData_PASSWORD: String = String::from("FTP_PASSWORD"); }

impl FtpConfigurationData {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
}

