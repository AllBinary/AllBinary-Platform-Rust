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
// package org.allbinary.business.init.db
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::logic::io::path::AbPathData::AbPathData;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::string::CommonSeps::CommonSeps;
use super::DatabaseConnectionInfoInterface::DatabaseConnectionInfoInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct DbConnectionInfo {
    pub jdbcDriver: String,
    pub name: String,
    pub userName: String,
    pub password: String,
    pub schema: String,
    pub server: String,
    pub port: String,
    pub url: String,
    pub host: String,
    pub SCHEMA_SEP: String,
    pub USER_NAME_KEY: String,
    pub PASSWORD_KEY: String,
}

impl DatabaseConnectionInfoInterface for DbConnectionInfo {
    fn getUrl(&mut self) -> String { Self::getUrl(self) }
    fn getJdbcDriver(&mut self) -> String { Self::getJdbcDriver(self) }
    fn getName(&mut self) -> String { Self::getName(self) }
    fn getUserName(&mut self) -> String { Self::getUserName(self) }
    fn getPassword(&mut self) -> String { Self::getPassword(self) }
    fn getHost(&mut self) -> String { Self::getHost(self) }
    fn getSchema(&mut self) -> String { Self::getSchema(self) }
    fn getServer(&mut self) -> String { Self::getServer(self) }
    fn getPort(&mut self) -> String { Self::getPort(self) }
    fn setJdbcDriver(&mut self, mut value: String) { Self::setJdbcDriver(self, value) }
    fn setName(&mut self, mut value: String) { Self::setName(self, value) }
    fn setUserName(&mut self, mut value: String) { Self::setUserName(self, value) }
    fn setPassword(&mut self, mut value: String) { Self::setPassword(self, value) }
    fn setSchema(&mut self, mut value: String) { Self::setSchema(self, value) }
    fn setServer(&mut self, mut value: String) { Self::setServer(self, value) }
    fn setPort(&mut self, mut value: String) { Self::setPort(self, value) }
    fn getUserNameKey(&mut self) -> String { Self::getUserNameKey(self) }
    fn getPasswordKey(&mut self) -> String { Self::getPasswordKey(self) }
}

impl DbConnectionInfo {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            jdbcDriver: String::new(),
            name: String::new(),
            userName: String::new(),
            password: String::new(),
            schema: String::new(),
            server: String::new(),
            port: String::new(),
            url: String::new(),
            host: String::new(),
            SCHEMA_SEP: String::new(),
            USER_NAME_KEY: String::new(),
            PASSWORD_KEY: String::new(),
        };
        this.SCHEMA_SEP = String::from("://");
        this.USER_NAME_KEY = String::from("?user=");
        this.PASSWORD_KEY = String::from("&password=");
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn updateUrl(&mut self) {
        self.updateHost();
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        jcall!(stringBuffer).append(self.getHost());
        jcall!(stringBuffer).append(self.getName());
        jcall!(stringBuffer).append(self.getUserNameKey());
        jcall!(stringBuffer).append(self.getUserName());
        jcall!(stringBuffer).append(self.getPasswordKey());
        jcall!(stringBuffer).append(self.getPassword());
        self.url = jcall!(stringBuffer).toString();
    }
    

    pub fn getUrl(&mut self) -> String {
        return self.url.clone();
    }
    

    pub fn getJdbcDriver(&mut self) -> String {
        return self.jdbcDriver.clone();
    }
    

    pub fn getName(&mut self) -> String {
        return self.name.clone();
    }
    

    pub fn getUserName(&mut self) -> String {
        return self.userName.clone();
    }
    

    pub fn getPassword(&mut self) -> String {
        return self.password.clone();
    }
    

    pub fn updateHost(&mut self) {
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        jcall!(stringBuffer).append(self.getSchema());
        jcall!(stringBuffer).append(self.SCHEMA_SEP.clone());
        jcall!(stringBuffer).append(self.getServer());
        if self.getPort().is_some() && jcall!(self.getPort()).length() > 1 {
            jcall!(stringBuffer).append(jcall!(CommonSeps::getInstance()).COLON.clone());
            jcall!(stringBuffer).append(self.getPort());
        }
        jcall!(stringBuffer).append(jcall!(AbPathData::getInstance()).SEPARATOR.clone());
        self.host = jcall!(stringBuffer).toString();
    }
    

    pub fn getHost(&mut self) -> String {
        return self.host.clone();
    }
    

    pub fn getSchema(&mut self) -> String {
        return self.schema.clone();
    }
    

    pub fn getServer(&mut self) -> String {
        return self.server.clone();
    }
    

    pub fn getPort(&mut self) -> String {
        return self.port.clone();
    }
    

    pub fn setJdbcDriver(&mut self, mut value: String) {
        self.jdbcDriver = value.clone();
    }
    

    pub fn setName(&mut self, mut value: String) {
        self.name = value.clone();
        self.updateUrl();
    }
    

    pub fn setUserName(&mut self, mut value: String) {
        self.userName = value.clone();
        self.updateUrl();
    }
    

    pub fn setPassword(&mut self, mut value: String) {
        self.password = value.clone();
        self.updateUrl();
    }
    

    pub fn setSchema(&mut self, mut value: String) {
        self.schema = value.clone();
        self.updateUrl();
    }
    

    pub fn setServer(&mut self, mut value: String) {
        self.server = value.clone();
        self.updateUrl();
    }
    

    pub fn setPort(&mut self, mut value: String) {
        self.port = value.clone();
        self.updateUrl();
    }
    

    pub fn getUserNameKey(&mut self) -> String {
        return self.USER_NAME_KEY.clone();
    }
    

    pub fn getPasswordKey(&mut self) -> String {
        return self.PASSWORD_KEY.clone();
    }
}
pub trait DbConnectionInfoDyn: crate::org::allbinary::business::init::db::DatabaseConnectionInfoInterface::DatabaseConnectionInfoInterface {
    fn as_DbConnectionInfo(&self) -> &DbConnectionInfo;
    fn as_DbConnectionInfo_mut(&mut self) -> &mut DbConnectionInfo;
    fn updateUrl(&mut self) { DbConnectionInfo::updateUrl(self.as_DbConnectionInfo_mut()) }
    fn updateHost(&mut self) { DbConnectionInfo::updateHost(self.as_DbConnectionInfo_mut()) }
}
impl DbConnectionInfoDyn for DbConnectionInfo {
    fn as_DbConnectionInfo(&self) -> &DbConnectionInfo { self }
    fn as_DbConnectionInfo_mut(&mut self) -> &mut DbConnectionInfo { self }
}


