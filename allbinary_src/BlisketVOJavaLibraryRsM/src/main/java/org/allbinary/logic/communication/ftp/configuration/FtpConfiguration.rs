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
use std::collections::HashMap;
use super::*;
// use crate::java::util::HashMap::HashMap;
use super::FtpConfigurationInterface::FtpConfigurationInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct FtpConfiguration {
    pub ftpServer: String,
    pub ftpPath: String,
    pub ftpUserName: String,
    pub ftpPassword: String,
}

impl FtpConfigurationInterface for FtpConfiguration {
    fn getServer(&mut self) -> String { Self::getServer(self) }
    fn getPath(&mut self) -> String { Self::getPath(self) }
    fn getUserName(&mut self) -> String { Self::getUserName(self) }
    fn getPassword(&mut self) -> String { Self::getPassword(self) }
    fn setServer(&mut self, mut value: String) { Self::setServer(self, value) }
    fn setPath(&mut self, mut value: String) { Self::setPath(self, value) }
    fn setUserName(&mut self, mut value: String) { Self::setUserName(self, value) }
    fn setPassword(&mut self, mut value: String) { Self::setPassword(self, value) }
}

impl FtpConfiguration {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            ftpServer: String::new(),
            ftpPath: String::new(),
            ftpUserName: String::new(),
            ftpPassword: String::new(),
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn new_struct1(mut hashMap: Option<Rc<RefCell<HashMap>>>) -> Self {
        let mut this = Self {
            ftpServer: String::new(),
            ftpPath: String::new(),
            ftpUserName: String::new(),
            ftpPassword: String::new(),
        };
        this.ftpServer = (jcall!(hashMap).get(FtpConfigurationData_SERVER.with(Clone::clone).clone())).clone() /* as String */;
        this.ftpPath = (jcall!(hashMap).get(FtpConfigurationData_PATH.with(Clone::clone).clone())).clone() /* as String */;
        this.ftpUserName = (jcall!(hashMap).get(FtpConfigurationData_USERNAME.with(Clone::clone).clone())).clone() /* as String */;
        this.ftpPassword = (jcall!(hashMap).get(FtpConfigurationData_PASSWORD.with(Clone::clone).clone())).clone() /* as String */;
        this
    }

    pub fn new1(mut hashMap: Option<Rc<RefCell<HashMap>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct1(hashMap))))
    }
    

    pub fn getServer(&mut self) -> String {
        return self.ftpServer.clone();
    }
    

    pub fn getPath(&mut self) -> String {
        return self.ftpPath.clone();
    }
    

    pub fn getUserName(&mut self) -> String {
        return self.ftpUserName.clone();
    }
    

    pub fn getPassword(&mut self) -> String {
        return self.ftpPassword.clone();
    }
    

    pub fn setServer(&mut self, mut value: String) {
        self.ftpServer = value.clone();
    }
    

    pub fn setPath(&mut self, mut value: String) {
        self.ftpPath = value.clone();
    }
    

    pub fn setUserName(&mut self, mut value: String) {
        self.ftpUserName = value.clone();
    }
    

    pub fn setPassword(&mut self, mut value: String) {
        self.ftpPassword = value.clone();
    }
}

