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
// use crate::java::lang::reflect::Method::Method;
use crate::org::allbinary::business::init::db::DbConnectionInfo::DbConnectionInfo;
use crate::org::allbinary::logic::system::security::licensing::LicensingException::LicensingException;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

//Warning you must have sql root access
pub struct InitDb {
    pub anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>>,
    pub METHOD_CALL_GET_HOST_NAME: String,
    pub METHOD_CALL_ADD_USERS: String,
    pub METHOD_CALL_ADD_DATABASES: String,
    pub METHOD_CALL_ADD_TABLES: String,
    pub METHOD_CALL_TEMP_MAIN_PATH: String,
    pub METHOD_CALL_MAIN_PATH: String,
}


impl InitDb {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            anyType: None,
            METHOD_CALL_GET_HOST_NAME: String::new(),
            METHOD_CALL_ADD_USERS: String::new(),
            METHOD_CALL_ADD_DATABASES: String::new(),
            METHOD_CALL_ADD_TABLES: String::new(),
            METHOD_CALL_TEMP_MAIN_PATH: String::new(),
            METHOD_CALL_MAIN_PATH: String::new(),
        };
        this.METHOD_CALL_GET_HOST_NAME = String::from("getHostName");
        this.METHOD_CALL_ADD_USERS = String::from("addUsers");
        this.METHOD_CALL_ADD_DATABASES = String::from("addDatabases");
        this.METHOD_CALL_ADD_TABLES = String::from("addTables");
        this.METHOD_CALL_TEMP_MAIN_PATH = String::from("useTemporaryMainPath");
        this.METHOD_CALL_MAIN_PATH = String::from("useNormalMainPath");
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn new_struct1(mut dbConnectionInfo: Option<Rc<RefCell<dyn crate::org::allbinary::business::init::db::DbConnectionInfo::DbConnectionInfoDyn>>>) -> Self {
        let mut this = Self {
            anyType: None,
            METHOD_CALL_GET_HOST_NAME: String::new(),
            METHOD_CALL_ADD_USERS: String::new(),
            METHOD_CALL_ADD_DATABASES: String::new(),
            METHOD_CALL_ADD_TABLES: String::new(),
            METHOD_CALL_TEMP_MAIN_PATH: String::new(),
            METHOD_CALL_MAIN_PATH: String::new(),
        };
        this.METHOD_CALL_GET_HOST_NAME = String::from("getHostName");
        this.METHOD_CALL_ADD_USERS = String::from("addUsers");
        this.METHOD_CALL_ADD_DATABASES = String::from("addDatabases");
        this.METHOD_CALL_ADD_TABLES = String::from("addTables");
        this.METHOD_CALL_TEMP_MAIN_PATH = String::from("useTemporaryMainPath");
        this.METHOD_CALL_MAIN_PATH = String::from("useNormalMainPath");
        this
    }

    pub fn new1(mut dbConnectionInfo: Option<Rc<RefCell<dyn crate::org::allbinary::business::init::db::DbConnectionInfo::DbConnectionInfoDyn>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct1(dbConnectionInfo))))
    }
    

    pub fn setHelper(&mut self, mut anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) {
        self.anyType = anyType.clone();
    }
    

    pub fn getHelper(&mut self) -> Option<Rc<RefCell<Box<dyn std::any::Any>>>> {
        return self.anyType.clone();
    }
    

    pub fn getHostName(&mut self) -> String {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut dynamicClass: &'static str = std::any::type_name_of_val(&self.anyType);
            let mut method: Option<Rc<RefCell<Method>>> = jcall!(dynamicClass).getMethod(self.METHOD_CALL_GET_HOST_NAME.clone(), None);
            let mut result: String = (jcall!(method).invoke(self.anyType.clone(), None)).clone() /* as String */;
            return result.clone();
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            return None;
            }
        }
    }
    

    pub fn addUsers(&mut self) -> bool {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut dynamicClass: &'static str = std::any::type_name_of_val(&self.anyType);
            let mut method: Option<Rc<RefCell<Method>>> = jcall!(dynamicClass).getMethod(self.METHOD_CALL_ADD_USERS.clone(), None);
            let mut result: Option<bool> = (jcall!(method).invoke(self.anyType.clone(), None)).clone() /* as Boolean */;
            return result.unwrap();
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            return false;
            }
        }
    }
    

    pub fn addDatabases(&mut self) -> bool {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut dynamicClass: &'static str = std::any::type_name_of_val(&self.anyType);
            let mut method: Option<Rc<RefCell<Method>>> = jcall!(dynamicClass).getMethod(self.METHOD_CALL_ADD_DATABASES.clone(), None);
            let mut result: Option<bool> = (jcall!(method).invoke(self.anyType.clone(), None)).clone() /* as Boolean */;
            return result.unwrap();
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            return false;
            }
        }
    }
    

    pub fn addTables(&mut self) -> bool {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut dynamicClass: &'static str = std::any::type_name_of_val(&self.anyType);
            let mut method: Option<Rc<RefCell<Method>>> = jcall!(dynamicClass).getMethod(self.METHOD_CALL_ADD_TABLES.clone(), None);
            let mut result: Option<bool> = (jcall!(method).invoke(self.anyType.clone(), None)).clone() /* as Boolean */;
            return result.unwrap();
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            return false;
            }
        }
    }
    

    pub fn useTemporaryMainPath(&mut self) -> bool {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut dynamicClass: &'static str = std::any::type_name_of_val(&self.anyType);
            let mut method: Option<Rc<RefCell<Method>>> = jcall!(dynamicClass).getMethod(self.METHOD_CALL_TEMP_MAIN_PATH.clone(), None);
            let mut result: Option<bool> = (jcall!(method).invoke(self.anyType.clone(), None)).clone() /* as Boolean */;
            return result.unwrap();
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            return false;
            }
        }
    }
    

    pub fn useNormalMainPath(&mut self) -> bool {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut dynamicClass: &'static str = std::any::type_name_of_val(&self.anyType);
            let mut method: Option<Rc<RefCell<Method>>> = jcall!(dynamicClass).getMethod(self.METHOD_CALL_MAIN_PATH.clone(), None);
            let mut result: Option<bool> = (jcall!(method).invoke(self.anyType.clone(), None)).clone() /* as Boolean */;
            return result.unwrap();
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            return false;
            }
        }
    }
}
pub trait InitDbDyn {
    fn as_InitDb(&self) -> &InitDb;
    fn as_InitDb_mut(&mut self) -> &mut InitDb;
    fn setHelper(&mut self, mut anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) { InitDb::setHelper(self.as_InitDb_mut(), anyType) }
    fn getHelper(&mut self) -> Option<Rc<RefCell<Box<dyn std::any::Any>>>> { InitDb::getHelper(self.as_InitDb_mut()) }
    fn getHostName(&mut self) -> String { InitDb::getHostName(self.as_InitDb_mut()) }
    fn addUsers(&mut self) -> bool { InitDb::addUsers(self.as_InitDb_mut()) }
    fn addDatabases(&mut self) -> bool { InitDb::addDatabases(self.as_InitDb_mut()) }
    fn addTables(&mut self) -> bool { InitDb::addTables(self.as_InitDb_mut()) }
    fn useTemporaryMainPath(&mut self) -> bool { InitDb::useTemporaryMainPath(self.as_InitDb_mut()) }
    fn useNormalMainPath(&mut self) -> bool { InitDb::useNormalMainPath(self.as_InitDb_mut()) }
}
impl InitDbDyn for InitDb {
    fn as_InitDb(&self) -> &InitDb { self }
    fn as_InitDb_mut(&mut self) -> &mut InitDb { self }
}


