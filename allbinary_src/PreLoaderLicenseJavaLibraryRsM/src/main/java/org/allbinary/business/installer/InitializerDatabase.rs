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
use std::collections::HashMap;
use super::*;
// use crate::java::util::HashMap::HashMap;
// use crate::java::util::Map::Map;
// use crate::java::util::Set::Set;
use crate::org::allbinary::business::init::db::DatabaseConnectionInfoInterface::DatabaseConnectionInfoInterface;
use crate::org::allbinary::business::init::db::DbConnectionInfo::DbConnectionInfo;
use crate::org::allbinary::logic::StdUtil::StdUtil;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use crate::org::allbinary::logic::string::StringValidationUtil::StringValidationUtil;
use crate::org::allbinary::logic::system::security::licensing::AbeClientInformationInterface::AbeClientInformationInterface;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use super::DynamicInitDb::DynamicInitDb;
use super::InitializerData::InitializerData;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct InitializerDatabase {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub adminDbUserName: String,
    pub adminDbPassword: String,
    pub adminJdbcDriver: String,
    pub adminSchema: String,
    pub adminServer: String,
    pub adminPort: String,
    pub initDb: Option<Rc<RefCell<DynamicInitDb>>>,
}

thread_local! { pub static InitializerDatabase_MAXDB: i32 = 30; }
thread_local! { pub static InitializerDatabase_MAX: i32 = 16; }
thread_local! { pub static InitializerDatabase_MIN: i32 = 4; }
thread_local! { pub static InitializerDatabase_MINPASSWORD: i32 = 0; }

impl InitializerDatabase {
    

    pub fn new_struct2(mut abeClientInformation: Option<Rc<RefCell<dyn AbeClientInformationInterface>>>, mut map: Option<Rc<RefCell<dyn Map>>>) -> Self {
        let mut this = Self {
            logUtil: None,
            adminDbUserName: String::new(),
            adminDbPassword: String::new(),
            adminJdbcDriver: String::new(),
            adminSchema: String::new(),
            adminServer: String::new(),
            adminPort: String::new(),
            initDb: None,
        };
        this.logUtil = LogUtil::getInstance();
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        let mut hashMap: Option<Rc<RefCell<HashMap>>> = jcall!(StdUtil::getInstance()).createHashMap();
        let mut keys: Option<Rc<RefCell<dyn Set>>> = jcall!(map).keySet();
        let mut keyArray: Vec<Option<Rc<RefCell<Box<dyn std::any::Any>>>>> = jcall!(keys).toArray();
        let mut size: i32 = (keyArray.len() as i32).clone();
        for index in (0..size) {
            let mut key: String = (keyArray[(index) as usize]).clone() /* as String */;
            let mut values: Vec<String> = (jcall!(map).get(key.clone())).clone() /* as  */;
            jcall!(hashMap).put(key.clone().clone(), values[(0) as usize].clone().clone());
            jcall!(stringBuffer).delete(0, jcall!(stringBuffer).length());
            jcall!(stringBuffer).append(String::from("key: "));
            jcall!(stringBuffer).append(key.clone());
            jcall!(stringBuffer).append(String::from(" Value: "));
            jcall!(stringBuffer).append(values[(0) as usize].clone());
            { let java_receiver = this.logUtil.clone(); let java_arguments = (jcall!(stringBuffer).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("getFormData()"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        }
        this.getFormData(abeClientInformation.clone(), hashMap.clone());
        this
    }

    pub fn new2(mut abeClientInformation: Option<Rc<RefCell<dyn AbeClientInformationInterface>>>, mut map: Option<Rc<RefCell<dyn Map>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct2(abeClientInformation, map))))
    }
    

    pub fn new_struct2(mut abeClientInformation: Option<Rc<RefCell<dyn AbeClientInformationInterface>>>, mut initHashMap: Option<Rc<RefCell<HashMap>>>) -> Self {
        let mut this = Self {
            logUtil: None,
            adminDbUserName: String::new(),
            adminDbPassword: String::new(),
            adminJdbcDriver: String::new(),
            adminSchema: String::new(),
            adminServer: String::new(),
            adminPort: String::new(),
            initDb: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.getFormData(abeClientInformation.clone(), initHashMap.clone());
        this
    }

    pub fn new2(mut abeClientInformation: Option<Rc<RefCell<dyn AbeClientInformationInterface>>>, mut initHashMap: Option<Rc<RefCell<HashMap>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct2(abeClientInformation, initHashMap))))
    }
    

    pub fn getFormData(&mut self, mut abeClientInformation: Option<Rc<RefCell<dyn AbeClientInformationInterface>>>, mut hashMap: Option<Rc<RefCell<HashMap>>>) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut initializerData: Option<Rc<RefCell<InitializerData>>> = InitializerData::getInstance();
            { let java_arguments = ((jcall!(hashMap).get(jcall!(initializerData).DBUSER.clone())).clone() /* as String */,); self.setAdminDbUserName(java_arguments.0) };
            { let java_arguments = ((jcall!(hashMap).get(jcall!(initializerData).DBPASSWORD.clone())).clone() /* as String */,); self.setAdminDbPassword(java_arguments.0) };
            { let java_arguments = ((jcall!(hashMap).get(jcall!(initializerData).ADMINJDBCDRIVER.clone())).clone() /* as String */,); self.setAdminJdbcDriver(java_arguments.0) };
            { let java_arguments = ((jcall!(hashMap).get(jcall!(initializerData).ADMINSCHEMA.clone())).clone() /* as String */,); self.setAdminSchema(java_arguments.0) };
            { let java_arguments = ((jcall!(hashMap).get(jcall!(initializerData).ADMINSERVER.clone())).clone() /* as String */,); self.setAdminServer(java_arguments.0) };
            { let java_arguments = ((jcall!(hashMap).get(jcall!(initializerData).ADMINPORT.clone())).clone() /* as String */,); self.setAdminPort(java_arguments.0) };
            let mut dbConnectionInfo: Option<Rc<RefCell<dyn crate::org::allbinary::business::init::db::DbConnectionInfo::DbConnectionInfoDyn>>> = DbConnectionInfo::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::business::init::db::DbConnectionInfo::DbConnectionInfoDyn>>);
            let mut adminDbName: String = jcall!(StringUtil::getInstance()).EMPTY_STRING.clone();
            jcall!(dbConnectionInfo).setJdbcDriver(self.getAdminJdbcDriver());
            jcall!(dbConnectionInfo).setName(adminDbName.clone());
            jcall!(dbConnectionInfo).setUserName(self.getAdminDbUserName());
            jcall!(dbConnectionInfo).setPassword(self.getAdminDbPassword());
            jcall!(dbConnectionInfo).setSchema(self.getAdminSchema());
            jcall!(dbConnectionInfo).setServer(self.getAdminServer());
            jcall!(dbConnectionInfo).setPort(self.getAdminPort());
            self.initDb = DynamicInitDb::new2(abeClientInformation.clone(), (dbConnectionInfo).clone() /* as DatabaseConnectionInfoInterface */);
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            jcall!(self.logUtil).put(String::from("Unable to get form data"), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("getFormData()"), crate::java_into_object(e.clone()));
            }
        }
    }
    

    pub fn isJdbcDriverValid(&mut self, mut jdbcDriverClassPathString: String) -> bool {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            jcall!(Class::Class::forName(jdbcDriverClassPathString.clone())).newInstance();
            return true;
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).PRELOADERERROR.clone()) {
                let mut commonStrings: Option<Rc<RefCell<CommonStrings>>> = CommonStrings::getInstance();
                jcall!(self.logUtil).put(jcall!(commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(commonStrings).IS_VALID.clone(), crate::java_into_object(e.clone()));
            }
            return false;
            }
        }
    }
    

    pub fn isValid(&mut self) -> bool {
        let mut isValid: bool = true;
        if !self.isJdbcDriverValid(self.adminJdbcDriver.clone()) {
            isValid = false;
        }
        let mut stringValidationUtil: Option<Rc<RefCell<StringValidationUtil>>> = StringValidationUtil::getInstance();
        if !jcall!(stringValidationUtil).isValidRequired(self.adminDbUserName.clone(), InitializerDatabase_MIN.with(Clone::clone).clone(), InitializerDatabase_MAX.with(Clone::clone).clone()) {
            isValid = false;
        }
        if !jcall!(stringValidationUtil).isValidNotRequired(self.adminDbPassword.clone(), InitializerDatabase_MINPASSWORD.with(Clone::clone).clone(), InitializerDatabase_MAX.with(Clone::clone).clone()) {
            isValid = false;
        }
        return isValid.clone();
    }
    

    pub fn getJdbcDriverValidationInfo(&mut self, mut jdbcDriver: String) -> String {
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        jcall!(stringBuffer).append(String::from("The JDBC driver ("));
        jcall!(stringBuffer).append(jdbcDriver.clone());
        jcall!(stringBuffer).append(String::from(") you have provided is not valid.<br/>"));
        return jcall!(stringBuffer).toString();
    }
    

    pub fn getInvalidInfo(&mut self) -> String {
        let mut isValid: bool = true;
        let mut isJdbcDriversValid: bool = true;
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        if !self.isJdbcDriverValid(self.adminJdbcDriver.clone()) {
            isJdbcDriversValid = false;
            jcall!(stringBuffer).append({ let java_arguments = (self.getAdminJdbcDriver(),); self.getJdbcDriverValidationInfo(java_arguments.0) });
        }
        let mut stringValidationUtil: Option<Rc<RefCell<StringValidationUtil>>> = StringValidationUtil::getInstance();
        if !jcall!(stringValidationUtil).isValidRequired(self.adminDbUserName.clone(), InitializerDatabase_MIN.with(Clone::clone).clone(), InitializerDatabase_MAX.with(Clone::clone).clone()) {
            jcall!(stringBuffer).append(format!("{}{}", format!("{}{}", format!("{}{}", format!("{}{}", String::from("Admin username should be < "), InitializerDatabase_MAX.with(Clone::clone)), String::from(" and > ")), InitializerDatabase_MIN.with(Clone::clone)), String::from(" characters in length.<br />")));
        }
        if !jcall!(stringValidationUtil).isValidNotRequired(self.adminDbPassword.clone(), InitializerDatabase_MINPASSWORD.with(Clone::clone).clone(), InitializerDatabase_MAX.with(Clone::clone).clone()) {
            jcall!(stringBuffer).append(format!("{}{}", format!("{}{}", format!("{}{}", format!("{}{}", String::from("Admin password should be < "), InitializerDatabase_MAX.with(Clone::clone)), String::from(" and > ")), InitializerDatabase_MINPASSWORD.with(Clone::clone)), String::from(" characters in length.<br />")));
        }
        if !isJdbcDriversValid {
            jcall!(stringBuffer).append(jcall!(InitializerData::getInstance()).getJdbcDriverSolutionInfo());
        }
        return jcall!(stringBuffer).toString();
    }
    

    pub fn createUsers(&mut self) -> bool {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            jcall!(self.initDb).addUsers();
            return true;
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            jcall!(self.logUtil).put(String::from("Unable to Create Users"), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("createUsers()"), crate::java_into_object(e.clone()));
            return false;
            }
        }
    }
    

    pub fn createDatabases(&mut self) -> bool {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            jcall!(self.initDb).addDatabases();
            return true;
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            jcall!(self.logUtil).put(String::from("Unable to Create Databases"), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("createDatabases()"), crate::java_into_object(e.clone()));
            return false;
            }
        }
    }
    

    pub fn createTables(&mut self) -> bool {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            jcall!(self.initDb).addTables();
            return true;
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            jcall!(self.logUtil).put(String::from("Unable to Create Tables"), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("createTables()"), crate::java_into_object(e.clone()));
            return false;
            }
        }
    }
    

    pub fn getAdminDbUserName(&mut self) -> String {
        return self.adminDbUserName.clone();
    }
    

    pub fn setAdminDbUserName(&mut self, mut adminDbUserName: String) {
        self.adminDbUserName = adminDbUserName.clone();
    }
    

    pub fn getAdminDbPassword(&mut self) -> String {
        return self.adminDbPassword.clone();
    }
    

    pub fn setAdminDbPassword(&mut self, mut adminDbPassword: String) {
        self.adminDbPassword = adminDbPassword.clone();
    }
    

    pub fn getAdminJdbcDriver(&mut self) -> String {
        return self.adminJdbcDriver.clone();
    }
    

    pub fn setAdminJdbcDriver(&mut self, mut adminJdbcDriver: String) {
        self.adminJdbcDriver = adminJdbcDriver.clone();
    }
    

    pub fn getAdminSchema(&mut self) -> String {
        return self.adminSchema.clone();
    }
    

    pub fn setAdminSchema(&mut self, mut adminSchema: String) {
        self.adminSchema = adminSchema.clone();
    }
    

    pub fn getAdminServer(&mut self) -> String {
        return self.adminServer.clone();
    }
    

    pub fn setAdminServer(&mut self, mut adminServer: String) {
        self.adminServer = adminServer.clone();
    }
    

    pub fn getAdminPort(&mut self) -> String {
        return self.adminPort.clone();
    }
    

    pub fn setAdminPort(&mut self, mut adminPort: String) {
        self.adminPort = adminPort.clone();
    }
}

