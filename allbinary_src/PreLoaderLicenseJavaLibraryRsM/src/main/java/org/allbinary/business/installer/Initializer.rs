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
use crate::org::allbinary::business::init::db::HistoryDbInitInfo::HistoryDbInitInfo;
use crate::org::allbinary::business::init::db::InventoryDbInitInfo::InventoryDbInitInfo;
use crate::org::allbinary::business::init::db::LogDbInitInfo::LogDbInitInfo;
use crate::org::allbinary::business::init::db::StaticPagesDbInitInfo::StaticPagesDbInitInfo;
use crate::org::allbinary::business::init::db::UserDbInitInfo::UserDbInitInfo;
use crate::org::allbinary::logic::StdUtil::StdUtil;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::string::StringValidationUtil::StringValidationUtil;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use super::InitializerData::InitializerData;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct Initializer {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub userJdbcDriver: String,
    pub userName: String,
    pub userUserName: String,
    pub userPassword: String,
    pub userSchema: String,
    pub userServer: String,
    pub userPort: String,
    pub historyJdbcDriver: String,
    pub historyName: String,
    pub historyUserName: String,
    pub historyPassword: String,
    pub historySchema: String,
    pub historyServer: String,
    pub historyPort: String,
    pub logJdbcDriver: String,
    pub logName: String,
    pub logUserName: String,
    pub logPassword: String,
    pub logSchema: String,
    pub logServer: String,
    pub logPort: String,
    pub inventoryJdbcDriver: String,
    pub inventoryName: String,
    pub inventoryUserName: String,
    pub inventoryPassword: String,
    pub inventorySchema: String,
    pub inventoryServer: String,
    pub inventoryPort: String,
    pub staticPagesJdbcDriver: String,
    pub staticPagesName: String,
    pub staticPagesUserName: String,
    pub staticPagesPassword: String,
    pub staticPagesSchema: String,
    pub staticPagesServer: String,
    pub staticPagesPort: String,
}

thread_local! { pub static Initializer_MAXDB: i32 = 30; }
thread_local! { pub static Initializer_MAX: i32 = 16; }
thread_local! { pub static Initializer_MIN: i32 = 4; }
thread_local! { pub static Initializer_MINPASSWORD: i32 = 0; }

impl Initializer {
    

    pub fn new_struct1(mut map: Option<Rc<RefCell<dyn Map>>>) -> Self {
        let mut this = Self {
            logUtil: None,
            userJdbcDriver: String::new(),
            userName: String::new(),
            userUserName: String::new(),
            userPassword: String::new(),
            userSchema: String::new(),
            userServer: String::new(),
            userPort: String::new(),
            historyJdbcDriver: String::new(),
            historyName: String::new(),
            historyUserName: String::new(),
            historyPassword: String::new(),
            historySchema: String::new(),
            historyServer: String::new(),
            historyPort: String::new(),
            logJdbcDriver: String::new(),
            logName: String::new(),
            logUserName: String::new(),
            logPassword: String::new(),
            logSchema: String::new(),
            logServer: String::new(),
            logPort: String::new(),
            inventoryJdbcDriver: String::new(),
            inventoryName: String::new(),
            inventoryUserName: String::new(),
            inventoryPassword: String::new(),
            inventorySchema: String::new(),
            inventoryServer: String::new(),
            inventoryPort: String::new(),
            staticPagesJdbcDriver: String::new(),
            staticPagesName: String::new(),
            staticPagesUserName: String::new(),
            staticPagesPassword: String::new(),
            staticPagesSchema: String::new(),
            staticPagesServer: String::new(),
            staticPagesPort: String::new(),
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
        this.getFormData(hashMap.clone());
        this
    }

    pub fn new1(mut map: Option<Rc<RefCell<dyn Map>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct1(map))))
    }
    

    pub fn new_struct1(mut initHashMap: Option<Rc<RefCell<HashMap>>>) -> Self {
        let mut this = Self {
            logUtil: None,
            userJdbcDriver: String::new(),
            userName: String::new(),
            userUserName: String::new(),
            userPassword: String::new(),
            userSchema: String::new(),
            userServer: String::new(),
            userPort: String::new(),
            historyJdbcDriver: String::new(),
            historyName: String::new(),
            historyUserName: String::new(),
            historyPassword: String::new(),
            historySchema: String::new(),
            historyServer: String::new(),
            historyPort: String::new(),
            logJdbcDriver: String::new(),
            logName: String::new(),
            logUserName: String::new(),
            logPassword: String::new(),
            logSchema: String::new(),
            logServer: String::new(),
            logPort: String::new(),
            inventoryJdbcDriver: String::new(),
            inventoryName: String::new(),
            inventoryUserName: String::new(),
            inventoryPassword: String::new(),
            inventorySchema: String::new(),
            inventoryServer: String::new(),
            inventoryPort: String::new(),
            staticPagesJdbcDriver: String::new(),
            staticPagesName: String::new(),
            staticPagesUserName: String::new(),
            staticPagesPassword: String::new(),
            staticPagesSchema: String::new(),
            staticPagesServer: String::new(),
            staticPagesPort: String::new(),
        };
        this.logUtil = LogUtil::getInstance();
        this.getFormData(initHashMap.clone());
        this
    }

    pub fn new1(mut initHashMap: Option<Rc<RefCell<HashMap>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct1(initHashMap))))
    }
    

    pub fn getFormData(&mut self, mut hashMap: Option<Rc<RefCell<HashMap>>>) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut initializerData: Option<Rc<RefCell<InitializerData>>> = InitializerData::getInstance();
            self.userJdbcDriver = (jcall!(hashMap).get(jcall!(initializerData).CUSTOMERJDBCDRIVER.clone())).clone() /* as String */;
            self.userName = (jcall!(hashMap).get(jcall!(initializerData).CUSTOMERNAME.clone())).clone() /* as String */;
            self.userUserName = (jcall!(hashMap).get(jcall!(initializerData).CUSTOMERUSERNAME.clone())).clone() /* as String */;
            self.userPassword = (jcall!(hashMap).get(jcall!(initializerData).CUSTOMERPASSWORD.clone())).clone() /* as String */;
            self.userSchema = (jcall!(hashMap).get(jcall!(initializerData).CUSTOMERSCHEMA.clone())).clone() /* as String */;
            self.userServer = (jcall!(hashMap).get(jcall!(initializerData).CUSTOMERSERVER.clone())).clone() /* as String */;
            self.userPort = (jcall!(hashMap).get(jcall!(initializerData).CUSTOMERPORT.clone())).clone() /* as String */;
            self.historyJdbcDriver = (jcall!(hashMap).get(jcall!(initializerData).HISTORYJDBCDRIVER.clone())).clone() /* as String */;
            self.historyName = (jcall!(hashMap).get(jcall!(initializerData).HISTORYNAME.clone())).clone() /* as String */;
            self.historyUserName = (jcall!(hashMap).get(jcall!(initializerData).HISTORYUSERNAME.clone())).clone() /* as String */;
            self.historyPassword = (jcall!(hashMap).get(jcall!(initializerData).HISTORYPASSWORD.clone())).clone() /* as String */;
            self.historySchema = (jcall!(hashMap).get(jcall!(initializerData).HISTORYSCHEMA.clone())).clone() /* as String */;
            self.historyServer = (jcall!(hashMap).get(jcall!(initializerData).HISTORYSERVER.clone())).clone() /* as String */;
            self.historyPort = (jcall!(hashMap).get(jcall!(initializerData).HISTORYPORT.clone())).clone() /* as String */;
            self.logJdbcDriver = (jcall!(hashMap).get(jcall!(initializerData).LOGJDBCDRIVER.clone())).clone() /* as String */;
            self.logName = (jcall!(hashMap).get(jcall!(initializerData).LOGNAME.clone())).clone() /* as String */;
            self.logUserName = (jcall!(hashMap).get(jcall!(initializerData).LOGUSERNAME.clone())).clone() /* as String */;
            self.logPassword = (jcall!(hashMap).get(jcall!(initializerData).LOGPASSWORD.clone())).clone() /* as String */;
            self.logSchema = (jcall!(hashMap).get(jcall!(initializerData).LOGSCHEMA.clone())).clone() /* as String */;
            self.logServer = (jcall!(hashMap).get(jcall!(initializerData).LOGSERVER.clone())).clone() /* as String */;
            self.logPort = (jcall!(hashMap).get(jcall!(initializerData).LOGPORT.clone())).clone() /* as String */;
            self.inventoryJdbcDriver = (jcall!(hashMap).get(jcall!(initializerData).INVENTORYJDBCDRIVER.clone())).clone() /* as String */;
            self.inventoryName = (jcall!(hashMap).get(jcall!(initializerData).INVENTORYNAME.clone())).clone() /* as String */;
            self.inventoryUserName = (jcall!(hashMap).get(jcall!(initializerData).INVENTORYUSERNAME.clone())).clone() /* as String */;
            self.inventoryPassword = (jcall!(hashMap).get(jcall!(initializerData).INVENTORYPASSWORD.clone())).clone() /* as String */;
            self.inventorySchema = (jcall!(hashMap).get(jcall!(initializerData).INVENTORYSCHEMA.clone())).clone() /* as String */;
            self.inventoryServer = (jcall!(hashMap).get(jcall!(initializerData).INVENTORYSERVER.clone())).clone() /* as String */;
            self.inventoryPort = (jcall!(hashMap).get(jcall!(initializerData).INVENTORYPORT.clone())).clone() /* as String */;
            self.staticPagesJdbcDriver = (jcall!(hashMap).get(jcall!(initializerData).STATICPAGESJDBCDRIVER.clone())).clone() /* as String */;
            self.staticPagesName = (jcall!(hashMap).get(jcall!(initializerData).STATICPAGESNAME.clone())).clone() /* as String */;
            self.staticPagesUserName = (jcall!(hashMap).get(jcall!(initializerData).STATICPAGESUSERNAME.clone())).clone() /* as String */;
            self.staticPagesPassword = (jcall!(hashMap).get(jcall!(initializerData).STATICPAGESPASSWORD.clone())).clone() /* as String */;
            self.staticPagesSchema = (jcall!(hashMap).get(jcall!(initializerData).STATICPAGESSCHEMA.clone())).clone() /* as String */;
            self.staticPagesServer = (jcall!(hashMap).get(jcall!(initializerData).STATICPAGESSERVER.clone())).clone() /* as String */;
            self.staticPagesPort = (jcall!(hashMap).get(jcall!(initializerData).STATICPAGESPORT.clone())).clone() /* as String */;
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
        if !self.isJdbcDriverValid(self.userJdbcDriver.clone()) {
            isValid = false;
        }
        let mut stringValidationUtil: Option<Rc<RefCell<StringValidationUtil>>> = StringValidationUtil::getInstance();
        if !jcall!(stringValidationUtil).isValidRequired(self.userName.clone(), Initializer_MIN.with(Clone::clone).clone(), Initializer_MAXDB.with(Clone::clone).clone()) {
            isValid = false;
        }
        if !jcall!(stringValidationUtil).isValidRequired(self.userUserName.clone(), Initializer_MIN.with(Clone::clone).clone(), Initializer_MAX.with(Clone::clone).clone()) {
            isValid = false;
        }
        if !jcall!(stringValidationUtil).isValidNotRequired(self.userPassword.clone(), Initializer_MINPASSWORD.with(Clone::clone).clone(), Initializer_MAX.with(Clone::clone).clone()) {
            isValid = false;
        }
        if !self.isJdbcDriverValid(self.historyJdbcDriver.clone()) {
            isValid = false;
        }
        if !jcall!(stringValidationUtil).isValidRequired(self.historyName.clone(), Initializer_MIN.with(Clone::clone).clone(), Initializer_MAXDB.with(Clone::clone).clone()) {
            isValid = false;
        }
        if !jcall!(stringValidationUtil).isValidRequired(self.historyUserName.clone(), Initializer_MIN.with(Clone::clone).clone(), Initializer_MAX.with(Clone::clone).clone()) {
            isValid = false;
        }
        if !jcall!(stringValidationUtil).isValidNotRequired(self.historyPassword.clone(), Initializer_MINPASSWORD.with(Clone::clone).clone(), Initializer_MAX.with(Clone::clone).clone()) {
            isValid = false;
        }
        if !self.isJdbcDriverValid(self.logJdbcDriver.clone()) {
            isValid = false;
        }
        if !jcall!(stringValidationUtil).isValidRequired(self.logName.clone(), Initializer_MIN.with(Clone::clone).clone(), Initializer_MAXDB.with(Clone::clone).clone()) {
            isValid = false;
        }
        if !jcall!(stringValidationUtil).isValidRequired(self.logUserName.clone(), Initializer_MIN.with(Clone::clone).clone(), Initializer_MAX.with(Clone::clone).clone()) {
            isValid = false;
        }
        if !jcall!(stringValidationUtil).isValidNotRequired(self.logPassword.clone(), Initializer_MINPASSWORD.with(Clone::clone).clone(), Initializer_MAX.with(Clone::clone).clone()) {
            isValid = false;
        }
        if !self.isJdbcDriverValid(self.inventoryJdbcDriver.clone()) {
            isValid = false;
        }
        if !jcall!(stringValidationUtil).isValidRequired(self.inventoryName.clone(), Initializer_MIN.with(Clone::clone).clone(), Initializer_MAXDB.with(Clone::clone).clone()) {
            isValid = false;
        }
        if !jcall!(stringValidationUtil).isValidRequired(self.inventoryUserName.clone(), Initializer_MIN.with(Clone::clone).clone(), Initializer_MAX.with(Clone::clone).clone()) {
            isValid = false;
        }
        if !jcall!(stringValidationUtil).isValidNotRequired(self.inventoryPassword.clone(), Initializer_MINPASSWORD.with(Clone::clone).clone(), Initializer_MAX.with(Clone::clone).clone()) {
            isValid = false;
        }
        if !self.isJdbcDriverValid(self.staticPagesJdbcDriver.clone()) {
            isValid = false;
        }
        if !jcall!(stringValidationUtil).isValidRequired(self.staticPagesName.clone(), Initializer_MIN.with(Clone::clone).clone(), Initializer_MAXDB.with(Clone::clone).clone()) {
            isValid = false;
        }
        if !jcall!(stringValidationUtil).isValidRequired(self.staticPagesUserName.clone(), Initializer_MIN.with(Clone::clone).clone(), Initializer_MAX.with(Clone::clone).clone()) {
            isValid = false;
        }
        if !jcall!(stringValidationUtil).isValidNotRequired(self.staticPagesPassword.clone(), Initializer_MINPASSWORD.with(Clone::clone).clone(), Initializer_MAX.with(Clone::clone).clone()) {
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
    

    pub fn getInvalidInfo(&mut self) -> String {
        let mut isJdbcDriversValid: bool = true;
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        if !self.isJdbcDriverValid(self.userJdbcDriver.clone()) {
            isJdbcDriversValid = false;
            jcall!(stringBuffer).append(self.getJdbcDriverValidationInfo(self.userJdbcDriver.clone()));
        }
        let mut stringValidationUtil: Option<Rc<RefCell<StringValidationUtil>>> = StringValidationUtil::getInstance();
        if !jcall!(stringValidationUtil).isValidRequired(self.userName.clone(), Initializer_MIN.with(Clone::clone).clone(), Initializer_MAXDB.with(Clone::clone).clone()) {
            jcall!(stringBuffer).append(format!("{}{}", format!("{}{}", format!("{}{}", format!("{}{}", String::from("User db name should be < "), Initializer_MAXDB.with(Clone::clone)), String::from(" and > ")), Initializer_MIN.with(Clone::clone)), String::from(" characters in length.<br />")));
        }
        if !jcall!(stringValidationUtil).isValidRequired(self.userUserName.clone(), Initializer_MIN.with(Clone::clone).clone(), Initializer_MAX.with(Clone::clone).clone()) {
            jcall!(stringBuffer).append(format!("{}{}", format!("{}{}", format!("{}{}", format!("{}{}", String::from("User db username should be < "), Initializer_MAX.with(Clone::clone)), String::from(" and > ")), Initializer_MIN.with(Clone::clone)), String::from(" characters in length.<br />")));
        }
        if !jcall!(stringValidationUtil).isValidNotRequired(self.userPassword.clone(), Initializer_MINPASSWORD.with(Clone::clone).clone(), Initializer_MAX.with(Clone::clone).clone()) {
            jcall!(stringBuffer).append(format!("{}{}", format!("{}{}", format!("{}{}", format!("{}{}", String::from("User DB password should be < "), Initializer_MAX.with(Clone::clone)), String::from(" and > ")), Initializer_MINPASSWORD.with(Clone::clone)), String::from(" characters in length.<br />")));
        }
        if !self.isJdbcDriverValid(self.historyJdbcDriver.clone()) {
            isJdbcDriversValid = false;
            jcall!(stringBuffer).append(self.getJdbcDriverValidationInfo(self.historyJdbcDriver.clone()));
        }
        if !jcall!(stringValidationUtil).isValidRequired(self.historyName.clone(), Initializer_MIN.with(Clone::clone).clone(), Initializer_MAXDB.with(Clone::clone).clone()) {
            jcall!(stringBuffer).append(format!("{}{}", format!("{}{}", format!("{}{}", format!("{}{}", String::from("History db should be < "), Initializer_MAXDB.with(Clone::clone)), String::from(" and > ")), Initializer_MIN.with(Clone::clone)), String::from(" characters in length.<br />")));
        }
        if !jcall!(stringValidationUtil).isValidRequired(self.historyUserName.clone(), Initializer_MIN.with(Clone::clone).clone(), Initializer_MAX.with(Clone::clone).clone()) {
            jcall!(stringBuffer).append(format!("{}{}", format!("{}{}", format!("{}{}", format!("{}{}", String::from("History db username should be < "), Initializer_MAX.with(Clone::clone)), String::from(" and > ")), Initializer_MIN.with(Clone::clone)), String::from(" characters in length.<br />")));
        }
        if !jcall!(stringValidationUtil).isValidNotRequired(self.historyPassword.clone(), Initializer_MINPASSWORD.with(Clone::clone).clone(), Initializer_MAX.with(Clone::clone).clone()) {
            jcall!(stringBuffer).append(format!("{}{}", format!("{}{}", format!("{}{}", format!("{}{}", String::from("History db password should be < "), Initializer_MAX.with(Clone::clone)), String::from(" and > ")), Initializer_MINPASSWORD.with(Clone::clone)), String::from(" characters in length.<br />")));
        }
        if !self.isJdbcDriverValid(self.logJdbcDriver.clone()) {
            isJdbcDriversValid = false;
            jcall!(stringBuffer).append(self.getJdbcDriverValidationInfo(self.logJdbcDriver.clone()));
        }
        if !jcall!(stringValidationUtil).isValidRequired(self.logName.clone(), Initializer_MIN.with(Clone::clone).clone(), Initializer_MAXDB.with(Clone::clone).clone()) {
            jcall!(stringBuffer).append(format!("{}{}", format!("{}{}", format!("{}{}", format!("{}{}", String::from("Log db should be < "), Initializer_MAXDB.with(Clone::clone)), String::from(" and > ")), Initializer_MIN.with(Clone::clone)), String::from(" characters in length.<br />")));
        }
        if !jcall!(stringValidationUtil).isValidRequired(self.logUserName.clone(), Initializer_MIN.with(Clone::clone).clone(), Initializer_MAX.with(Clone::clone).clone()) {
            jcall!(stringBuffer).append(format!("{}{}", format!("{}{}", format!("{}{}", format!("{}{}", String::from("Log db username should be < "), Initializer_MAX.with(Clone::clone)), String::from(" and > ")), Initializer_MIN.with(Clone::clone)), String::from(" characters in length.<br />")));
        }
        if !jcall!(stringValidationUtil).isValidNotRequired(self.logPassword.clone(), Initializer_MINPASSWORD.with(Clone::clone).clone(), Initializer_MAX.with(Clone::clone).clone()) {
            jcall!(stringBuffer).append(format!("{}{}", format!("{}{}", format!("{}{}", format!("{}{}", String::from("Log db password should be < "), Initializer_MAX.with(Clone::clone)), String::from(" and > ")), Initializer_MINPASSWORD.with(Clone::clone)), String::from(" characters in length.<br />")));
        }
        if !self.isJdbcDriverValid(self.inventoryJdbcDriver.clone()) {
            isJdbcDriversValid = false;
            jcall!(stringBuffer).append(self.getJdbcDriverValidationInfo(self.inventoryJdbcDriver.clone()));
        }
        if !jcall!(stringValidationUtil).isValidRequired(self.inventoryName.clone(), Initializer_MIN.with(Clone::clone).clone(), Initializer_MAXDB.with(Clone::clone).clone()) {
            jcall!(stringBuffer).append(format!("{}{}", format!("{}{}", format!("{}{}", format!("{}{}", String::from("Inventory db should be < "), Initializer_MAXDB.with(Clone::clone)), String::from(" and > ")), Initializer_MIN.with(Clone::clone)), String::from(" characters in length.<br />")));
        }
        if !jcall!(stringValidationUtil).isValidRequired(self.inventoryUserName.clone(), Initializer_MIN.with(Clone::clone).clone(), Initializer_MAX.with(Clone::clone).clone()) {
            jcall!(stringBuffer).append(format!("{}{}", format!("{}{}", format!("{}{}", format!("{}{}", String::from("Inventory db username should be < "), Initializer_MAX.with(Clone::clone)), String::from(" and > ")), Initializer_MIN.with(Clone::clone)), String::from(" characters in length.<br />")));
        }
        if !jcall!(stringValidationUtil).isValidNotRequired(self.inventoryPassword.clone(), Initializer_MINPASSWORD.with(Clone::clone).clone(), Initializer_MAX.with(Clone::clone).clone()) {
            jcall!(stringBuffer).append(format!("{}{}", format!("{}{}", format!("{}{}", format!("{}{}", String::from("Inventory DB password should be < "), Initializer_MAX.with(Clone::clone)), String::from(" and > ")), Initializer_MINPASSWORD.with(Clone::clone)), String::from(" characters in length.<br />")));
        }
        if !self.isJdbcDriverValid(self.staticPagesJdbcDriver.clone()) {
            isJdbcDriversValid = false;
            jcall!(stringBuffer).append(self.getJdbcDriverValidationInfo(self.staticPagesJdbcDriver.clone()));
        }
        if !jcall!(stringValidationUtil).isValidRequired(self.staticPagesName.clone(), Initializer_MIN.with(Clone::clone).clone(), Initializer_MAXDB.with(Clone::clone).clone()) {
            jcall!(stringBuffer).append(format!("{}{}", format!("{}{}", format!("{}{}", format!("{}{}", String::from("Static Pages db should be < "), Initializer_MAXDB.with(Clone::clone)), String::from(" and > ")), Initializer_MIN.with(Clone::clone)), String::from(" characters in length.<br />")));
        }
        if !jcall!(stringValidationUtil).isValidRequired(self.staticPagesUserName.clone(), Initializer_MIN.with(Clone::clone).clone(), Initializer_MAX.with(Clone::clone).clone()) {
            jcall!(stringBuffer).append(format!("{}{}", format!("{}{}", format!("{}{}", format!("{}{}", String::from("Static Pages db username should be < "), Initializer_MAX.with(Clone::clone)), String::from(" and > ")), Initializer_MIN.with(Clone::clone)), String::from(" characters in length.<br />")));
        }
        if !jcall!(stringValidationUtil).isValidNotRequired(self.staticPagesPassword.clone(), Initializer_MINPASSWORD.with(Clone::clone).clone(), Initializer_MAX.with(Clone::clone).clone()) {
            jcall!(stringBuffer).append(format!("{}{}", format!("{}{}", format!("{}{}", format!("{}{}", String::from("Static Pages DB password should be < "), Initializer_MAX.with(Clone::clone)), String::from(" and > ")), Initializer_MINPASSWORD.with(Clone::clone)), String::from(" characters in length.<br />")));
        }
        if !isJdbcDriversValid {
            jcall!(stringBuffer).append(self.getJdbcDriverSolutionInfo());
        }
        return jcall!(stringBuffer).toString();
    }
    

    // throws Exception
    pub fn set(&mut self) {
        jcall!(self.logUtil).putF(String::from("Creating DB connection files"), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("set()"));
        let mut userDbInitInfo: Option<Rc<RefCell<UserDbInitInfo>>> = UserDbInitInfo::new1(false);
        jcall!(userDbInitInfo).setJdbcDriver(self.userJdbcDriver.clone());
        jcall!(userDbInitInfo).setName(self.userName.clone());
        jcall!(userDbInitInfo).setUserName(self.userUserName.clone());
        jcall!(userDbInitInfo).setPassword(self.userPassword.clone());
        jcall!(userDbInitInfo).setSchema(self.userSchema.clone());
        jcall!(userDbInitInfo).setServer(self.userServer.clone());
        jcall!(userDbInitInfo).setPort(self.userPort.clone());
        jcall!(userDbInitInfo).setHasRead(true);
        jcall!(userDbInitInfo).write();
        let mut historyDbInitInfo: Option<Rc<RefCell<HistoryDbInitInfo>>> = HistoryDbInitInfo::new1(false);
        jcall!(historyDbInitInfo).setJdbcDriver(self.historyJdbcDriver.clone());
        jcall!(historyDbInitInfo).setName(self.historyName.clone());
        jcall!(historyDbInitInfo).setUserName(self.historyUserName.clone());
        jcall!(historyDbInitInfo).setPassword(self.historyPassword.clone());
        jcall!(historyDbInitInfo).setSchema(self.historySchema.clone());
        jcall!(historyDbInitInfo).setServer(self.historyServer.clone());
        jcall!(historyDbInitInfo).setPort(self.historyPort.clone());
        jcall!(historyDbInitInfo).setHasRead(true);
        jcall!(historyDbInitInfo).write();
        let mut logDbInitInfo: Option<Rc<RefCell<LogDbInitInfo>>> = LogDbInitInfo::new1(false);
        jcall!(logDbInitInfo).setJdbcDriver(self.logJdbcDriver.clone());
        jcall!(logDbInitInfo).setName(self.logName.clone());
        jcall!(logDbInitInfo).setUserName(self.logUserName.clone());
        jcall!(logDbInitInfo).setPassword(self.logPassword.clone());
        jcall!(logDbInitInfo).setSchema(self.logSchema.clone());
        jcall!(logDbInitInfo).setServer(self.logServer.clone());
        jcall!(logDbInitInfo).setPort(self.logPort.clone());
        jcall!(logDbInitInfo).setHasRead(true);
        jcall!(logDbInitInfo).write();
        let mut staticPagesDbInitInfo: Option<Rc<RefCell<StaticPagesDbInitInfo>>> = StaticPagesDbInitInfo::new1(false);
        jcall!(staticPagesDbInitInfo).setJdbcDriver(self.staticPagesJdbcDriver.clone());
        jcall!(staticPagesDbInitInfo).setName(self.staticPagesName.clone());
        jcall!(staticPagesDbInitInfo).setUserName(self.staticPagesUserName.clone());
        jcall!(staticPagesDbInitInfo).setPassword(self.staticPagesPassword.clone());
        jcall!(staticPagesDbInitInfo).setSchema(self.staticPagesSchema.clone());
        jcall!(staticPagesDbInitInfo).setServer(self.staticPagesServer.clone());
        jcall!(staticPagesDbInitInfo).setPort(self.staticPagesPort.clone());
        jcall!(staticPagesDbInitInfo).setHasRead(true);
        jcall!(staticPagesDbInitInfo).write();
        let mut inventoryDbInitInfo: Option<Rc<RefCell<InventoryDbInitInfo>>> = InventoryDbInitInfo::new1(false);
        jcall!(inventoryDbInitInfo).setJdbcDriver(self.inventoryJdbcDriver.clone());
        jcall!(inventoryDbInitInfo).setName(self.inventoryName.clone());
        jcall!(inventoryDbInitInfo).setUserName(self.inventoryUserName.clone());
        jcall!(inventoryDbInitInfo).setPassword(self.inventoryPassword.clone());
        jcall!(inventoryDbInitInfo).setSchema(self.inventorySchema.clone());
        jcall!(inventoryDbInitInfo).setServer(self.inventoryServer.clone());
        jcall!(inventoryDbInitInfo).setPort(self.inventoryPort.clone());
        jcall!(inventoryDbInitInfo).setHasRead(true);
        jcall!(inventoryDbInitInfo).write();
        jcall!(self.logUtil).putF(String::from("Created DB connection files"), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("set()"));
    }
}

