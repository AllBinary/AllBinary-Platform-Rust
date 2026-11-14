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
use crate::java::util::HashMap::HashMap;
use crate::java::util::Map::Map;
use crate::java::util::Set::Set;
use crate::org::allbinary::business::init::db::DatabaseConnectionInfoInterface::DatabaseConnectionInfoInterface;
use crate::org::allbinary::business::init::db::DbConnectionInfo::DbConnectionInfo;
use crate::org::allbinary::logic::StdUtil::StdUtil;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use crate::org::allbinary::logic::string::StringValidationUtil::StringValidationUtil;
use crate::org::allbinary::logic::system::security::licensing::AbeClientInformationInterface::AbeClientInformationInterface;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::java::lang::Exception::*;
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

    pub fn toString(&mut self) -> String {
        String::from(std::any::type_name::<Self>())
    }
    

    pub fn new_struct2(mut abeClientInformation: Option<Rc<RefCell<dyn AbeClientInformationInterface>>>, mut map: Option<Rc<RefCell<dyn Map<crate::JavaObject, crate::JavaObject>>>>) -> Self {
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
        let mut hashMap: Option<Rc<RefCell<HashMap<crate::JavaObject, crate::JavaObject>>>> = jcall!(StdUtil::getInstance()).createHashMap();
        let mut keys: Option<Rc<RefCell<dyn Set>>> = jcall!(map).keySet();
        let mut keyArray: Vec<Option<Rc<RefCell<Box<dyn std::any::Any>>>>> = jcall!(keys).toArray();
        let mut size: i32 = (keyArray.len() as i32).clone();
        for index in (0..size) {
            let mut key: String = { let java_object = (keyArray[(index) as usize]).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() };
            let mut values: Vec<String> = { let java_object = (jcall!(map).get(crate::java_into_object(key.clone()))).clone(); java_object.map(|object| object.borrow().downcast_ref::<Vec<String>>().cloned().expect("ClassCastException")).unwrap_or_default() };
            jcall!(hashMap).put(crate::java_into_object(key.clone().clone()), crate::java_into_object(values[(0) as usize].clone().clone()));
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

    pub fn new2(mut abeClientInformation: Option<Rc<RefCell<dyn AbeClientInformationInterface>>>, mut map: Option<Rc<RefCell<dyn Map<crate::JavaObject, crate::JavaObject>>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct2(abeClientInformation, map))))
    }
    

    pub fn new_struct2_AbeClientInformationInterface_HashMap(mut abeClientInformation: Option<Rc<RefCell<dyn AbeClientInformationInterface>>>, mut initHashMap: Option<Rc<RefCell<HashMap<crate::JavaObject, crate::JavaObject>>>>) -> Self {
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

    pub fn new2_AbeClientInformationInterface_HashMap(mut abeClientInformation: Option<Rc<RefCell<dyn AbeClientInformationInterface>>>, mut initHashMap: Option<Rc<RefCell<HashMap<crate::JavaObject, crate::JavaObject>>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct2_AbeClientInformationInterface_HashMap(abeClientInformation, initHashMap))))
    }
    

    pub fn getFormData(&mut self, mut abeClientInformation: Option<Rc<RefCell<dyn AbeClientInformationInterface>>>, mut hashMap: Option<Rc<RefCell<HashMap<crate::JavaObject, crate::JavaObject>>>>) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut initializerData: Option<Rc<RefCell<InitializerData>>> = InitializerData::getInstance();
            { let java_arguments = ({ let java_object = (jcall!(hashMap).get(crate::java_into_object({ let java_field = jcall!(initializerData).DBUSER.clone(); java_field }.clone()))).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() },); self.setAdminDbUserName(java_arguments.0) };
            { let java_arguments = ({ let java_object = (jcall!(hashMap).get(crate::java_into_object({ let java_field = jcall!(initializerData).DBPASSWORD.clone(); java_field }.clone()))).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() },); self.setAdminDbPassword(java_arguments.0) };
            { let java_arguments = ({ let java_object = (jcall!(hashMap).get(crate::java_into_object({ let java_field = jcall!(initializerData).ADMINJDBCDRIVER.clone(); java_field }.clone()))).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() },); self.setAdminJdbcDriver(java_arguments.0) };
            { let java_arguments = ({ let java_object = (jcall!(hashMap).get(crate::java_into_object({ let java_field = jcall!(initializerData).ADMINSCHEMA.clone(); java_field }.clone()))).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() },); self.setAdminSchema(java_arguments.0) };
            { let java_arguments = ({ let java_object = (jcall!(hashMap).get(crate::java_into_object({ let java_field = jcall!(initializerData).ADMINSERVER.clone(); java_field }.clone()))).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() },); self.setAdminServer(java_arguments.0) };
            { let java_arguments = ({ let java_object = (jcall!(hashMap).get(crate::java_into_object({ let java_field = jcall!(initializerData).ADMINPORT.clone(); java_field }.clone()))).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() },); self.setAdminPort(java_arguments.0) };
            let mut dbConnectionInfo: Option<Rc<RefCell<dyn crate::org::allbinary::business::init::db::DbConnectionInfo::DbConnectionInfoDyn>>> = DbConnectionInfo::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::business::init::db::DbConnectionInfo::DbConnectionInfoDyn>>);
            let mut adminDbName: String = { let java_field = jcall!(StringUtil::getInstance()).EMPTY_STRING.clone(); java_field }.clone();
            jcall!(dbConnectionInfo).as_DbConnectionInfo_mut().setJdbcDriver(self.getAdminJdbcDriver());
            jcall!(dbConnectionInfo).as_DbConnectionInfo_mut().setName(adminDbName.clone());
            jcall!(dbConnectionInfo).as_DbConnectionInfo_mut().setUserName(self.getAdminDbUserName());
            jcall!(dbConnectionInfo).as_DbConnectionInfo_mut().setPassword(self.getAdminDbPassword());
            jcall!(dbConnectionInfo).as_DbConnectionInfo_mut().setSchema(self.getAdminSchema());
            jcall!(dbConnectionInfo).as_DbConnectionInfo_mut().setServer(self.getAdminServer());
            jcall!(dbConnectionInfo).as_DbConnectionInfo_mut().setPort(self.getAdminPort());
            self.initDb = DynamicInitDb::new2(abeClientInformation.clone(), (|| -> Option<Rc<RefCell<dyn DatabaseConnectionInfoInterface>>> { panic!("ClassCastException: a Rust dyn DbConnectionInfo cannot be cast to DatabaseConnectionInfoInterface") })());
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            jcall!(self.logUtil).put(String::from("Unable to get form data"), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("getFormData()"), crate::java_into_object(e.clone()));
            }
        }
    }
    

    pub fn isJdbcDriverValid(&mut self, mut jdbcDriverClassPathString: String) -> bool {
        let java_try_result = match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            jcall!(Class::Class::forName(jdbcDriverClassPathString.clone())).newInstance();
            return Some(true);
        })) {
            Ok(java_return) => java_return,
            Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            if jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypes.clone(); java_field }).LOGGING.clone(); java_field }).contains({ let java_field = jcall!(jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypeFactory.clone(); java_field }).getInstance()).PRELOADERERROR.clone(); java_field }.clone()) {
                let mut commonStrings: Option<Rc<RefCell<CommonStrings>>> = CommonStrings::getInstance();
                jcall!(self.logUtil).put({ let java_field = jcall!(commonStrings).EXCEPTION.clone(); java_field }.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(commonStrings).IS_VALID.clone(); java_field }.clone(), crate::java_into_object(e.clone()));
            }
            return false;
            None
            }
        };if let Some(java_return) = java_try_result { return java_return; }unreachable!("try statement")
    }
    

    pub fn isValid(&mut self) -> bool {
        let mut isValid: bool = true;
        if !{ let java_arguments = (self.adminJdbcDriver.clone(),); self.isJdbcDriverValid(java_arguments.0) } {
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
        if !{ let java_arguments = (self.adminJdbcDriver.clone(),); self.isJdbcDriverValid(java_arguments.0) } {
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
        let java_try_result = match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            jcall!(self.initDb).addUsers();
            return Some(true);
        })) {
            Ok(java_return) => java_return,
            Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            jcall!(self.logUtil).put(String::from("Unable to Create Users"), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("createUsers()"), crate::java_into_object(e.clone()));
            return false;
            None
            }
        };if let Some(java_return) = java_try_result { return java_return; }unreachable!("try statement")
    }
    

    pub fn createDatabases(&mut self) -> bool {
        let java_try_result = match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            jcall!(self.initDb).addDatabases();
            return Some(true);
        })) {
            Ok(java_return) => java_return,
            Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            jcall!(self.logUtil).put(String::from("Unable to Create Databases"), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("createDatabases()"), crate::java_into_object(e.clone()));
            return false;
            None
            }
        };if let Some(java_return) = java_try_result { return java_return; }unreachable!("try statement")
    }
    

    pub fn createTables(&mut self) -> bool {
        let java_try_result = match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            jcall!(self.initDb).addTables();
            return Some(true);
        })) {
            Ok(java_return) => java_return,
            Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            jcall!(self.logUtil).put(String::from("Unable to Create Tables"), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("createTables()"), crate::java_into_object(e.clone()));
            return false;
            None
            }
        };if let Some(java_return) = java_try_result { return java_return; }unreachable!("try statement")
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

