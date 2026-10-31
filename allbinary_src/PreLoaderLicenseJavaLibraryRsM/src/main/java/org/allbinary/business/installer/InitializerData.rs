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
// package org.allbinary.business.installer
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct InitializerData {
    pub DBUSER: String,
    pub DBPASSWORD: String,
    pub ADMINJDBCDRIVER: String,
    pub ADMINSCHEMA: String,
    pub ADMINSERVER: String,
    pub ADMINPORT: String,
    pub CUSTOMERJDBCDRIVER: String,
    pub CUSTOMERNAME: String,
    pub CUSTOMERUSERNAME: String,
    pub CUSTOMERPASSWORD: String,
    pub CUSTOMERSCHEMA: String,
    pub CUSTOMERSERVER: String,
    pub CUSTOMERPORT: String,
    pub HISTORYJDBCDRIVER: String,
    pub HISTORYNAME: String,
    pub HISTORYUSERNAME: String,
    pub HISTORYPASSWORD: String,
    pub HISTORYSCHEMA: String,
    pub HISTORYSERVER: String,
    pub HISTORYPORT: String,
    pub LOGJDBCDRIVER: String,
    pub LOGNAME: String,
    pub LOGUSERNAME: String,
    pub LOGPASSWORD: String,
    pub LOGSCHEMA: String,
    pub LOGSERVER: String,
    pub LOGPORT: String,
    pub INVENTORYJDBCDRIVER: String,
    pub INVENTORYNAME: String,
    pub INVENTORYUSERNAME: String,
    pub INVENTORYPASSWORD: String,
    pub INVENTORYSCHEMA: String,
    pub INVENTORYSERVER: String,
    pub INVENTORYPORT: String,
    pub STATICPAGESJDBCDRIVER: String,
    pub STATICPAGESNAME: String,
    pub STATICPAGESUSERNAME: String,
    pub STATICPAGESPASSWORD: String,
    pub STATICPAGESSCHEMA: String,
    pub STATICPAGESSERVER: String,
    pub STATICPAGESPORT: String,
}

thread_local! { pub static InitializerData_instance: Option<Rc<RefCell<InitializerData>>> = InitializerData::new(); }

impl InitializerData {

    pub fn new_struct() -> Self {
        let mut this = Self {
            DBUSER: String::new(),
            DBPASSWORD: String::new(),
            ADMINJDBCDRIVER: String::new(),
            ADMINSCHEMA: String::new(),
            ADMINSERVER: String::new(),
            ADMINPORT: String::new(),
            CUSTOMERJDBCDRIVER: String::new(),
            CUSTOMERNAME: String::new(),
            CUSTOMERUSERNAME: String::new(),
            CUSTOMERPASSWORD: String::new(),
            CUSTOMERSCHEMA: String::new(),
            CUSTOMERSERVER: String::new(),
            CUSTOMERPORT: String::new(),
            HISTORYJDBCDRIVER: String::new(),
            HISTORYNAME: String::new(),
            HISTORYUSERNAME: String::new(),
            HISTORYPASSWORD: String::new(),
            HISTORYSCHEMA: String::new(),
            HISTORYSERVER: String::new(),
            HISTORYPORT: String::new(),
            LOGJDBCDRIVER: String::new(),
            LOGNAME: String::new(),
            LOGUSERNAME: String::new(),
            LOGPASSWORD: String::new(),
            LOGSCHEMA: String::new(),
            LOGSERVER: String::new(),
            LOGPORT: String::new(),
            INVENTORYJDBCDRIVER: String::new(),
            INVENTORYNAME: String::new(),
            INVENTORYUSERNAME: String::new(),
            INVENTORYPASSWORD: String::new(),
            INVENTORYSCHEMA: String::new(),
            INVENTORYSERVER: String::new(),
            INVENTORYPORT: String::new(),
            STATICPAGESJDBCDRIVER: String::new(),
            STATICPAGESNAME: String::new(),
            STATICPAGESUSERNAME: String::new(),
            STATICPAGESPASSWORD: String::new(),
            STATICPAGESSCHEMA: String::new(),
            STATICPAGESSERVER: String::new(),
            STATICPAGESPORT: String::new(),
        };
        this.DBUSER = String::from("DBUSER");
        this.DBPASSWORD = String::from("DBPASSWORD");
        this.ADMINJDBCDRIVER = String::from("ADMIN_JDBCDRIVER");
        this.ADMINSCHEMA = String::from("ADMINSCHEMA");
        this.ADMINSERVER = String::from("ADMINSERVER");
        this.ADMINPORT = String::from("ADMINPORT");
        this.CUSTOMERJDBCDRIVER = String::from("CUSTOMER_JDBCDRIVER");
        this.CUSTOMERNAME = String::from("CUSTOMERNAME");
        this.CUSTOMERUSERNAME = String::from("CUSTOMERUSERNAME");
        this.CUSTOMERPASSWORD = String::from("CUSTOMERPASSWORD");
        this.CUSTOMERSCHEMA = String::from("CUSTOMERSCHEMA");
        this.CUSTOMERSERVER = String::from("CUSTOMERSERVER");
        this.CUSTOMERPORT = String::from("CUSTOMERPORT");
        this.HISTORYJDBCDRIVER = String::from("HISTORY_JDBCDRIVER");
        this.HISTORYNAME = String::from("HISTORYNAME");
        this.HISTORYUSERNAME = String::from("HISTORYUSERNAME");
        this.HISTORYPASSWORD = String::from("HISTORYPASSWORD");
        this.HISTORYSCHEMA = String::from("HISTORYSCHEMA");
        this.HISTORYSERVER = String::from("HISTORYSERVER");
        this.HISTORYPORT = String::from("HISTORYPORT");
        this.LOGJDBCDRIVER = String::from("LOG_JDBCDRIVER");
        this.LOGNAME = String::from("LOGNAME");
        this.LOGUSERNAME = String::from("LOGUSERNAME");
        this.LOGPASSWORD = String::from("LOGPASSWORD");
        this.LOGSCHEMA = String::from("LOGSCHEMA");
        this.LOGSERVER = String::from("LOGSERVER");
        this.LOGPORT = String::from("LOGPORT");
        this.INVENTORYJDBCDRIVER = String::from("INVENTORY_JDBCDRIVER");
        this.INVENTORYNAME = String::from("INVENTORYNAME");
        this.INVENTORYUSERNAME = String::from("INVENTORYUSERNAME");
        this.INVENTORYPASSWORD = String::from("INVENTORYPASSWORD");
        this.INVENTORYSCHEMA = String::from("INVENTORYSCHEMA");
        this.INVENTORYSERVER = String::from("INVENTORYSERVER");
        this.INVENTORYPORT = String::from("INVENTORYPORT");
        this.STATICPAGESJDBCDRIVER = String::from("STATICPAGES_JDBCDRIVER");
        this.STATICPAGESNAME = String::from("STATICPAGESNAME");
        this.STATICPAGESUSERNAME = String::from("STATICPAGESUSERNAME");
        this.STATICPAGESPASSWORD = String::from("STATICPAGESPASSWORD");
        this.STATICPAGESSCHEMA = String::from("STATICPAGESSCHEMA");
        this.STATICPAGESSERVER = String::from("STATICPAGESSERVER");
        this.STATICPAGESPORT = String::from("STATICPAGESPORT");
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<InitializerData>>> {
        return InitializerData_instance.with(Clone::clone).clone();
    }
    

    pub fn getJdbcDriverSolutionInfo(&mut self) -> String {
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        jcall!(stringBuffer).append(String::from("The following describes the possible problems and solutions regarding the Jdbc Driver error(s):<p/>"));
        jcall!(stringBuffer).append(String::from("1. A JDBC driver you specified is not in your classpath.<br/>"));
        jcall!(stringBuffer).append(String::from("Solution 1: Move the JDBC driver into any directory specified in the existing classpath. <br/>"));
        jcall!(stringBuffer).append(String::from("Solution 2: Add the directory that contains the JDBC driver to the classpath.<br/>"));
        jcall!(stringBuffer).append(String::from("Solution 3: Add the JDBC driver to the WEB-INF/lib directory where you installed this webapp.<br/>"));
        jcall!(stringBuffer).append(String::from("2. The JDBC driver you specified does not exit.<br/>"));
        jcall!(stringBuffer).append(String::from("Solution: Get a JDBC driver. <br/>"));
        jcall!(stringBuffer).append(String::from("3. The JDBC driver you specified is not valid.<br/>"));
        jcall!(stringBuffer).append(String::from("Solution: Use a valid JDBC driver.<p/>"));
        return jcall!(stringBuffer).toString();
    }
}

