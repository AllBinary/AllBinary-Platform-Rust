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
use crate::java::lang::Exception::*;
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

    pub fn toString(&mut self) -> String {
        String::from(std::any::type_name::<Self>())
    }
    

    pub fn new_struct1(mut map: Option<Rc<RefCell<dyn Map<crate::JavaObject, crate::JavaObject>>>>) -> Self {
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
        this.getFormData(hashMap.clone());
        this
    }

    pub fn new1(mut map: Option<Rc<RefCell<dyn Map<crate::JavaObject, crate::JavaObject>>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct1(map))))
    }
    

    pub fn new_struct1_HashMap(mut initHashMap: Option<Rc<RefCell<HashMap<crate::JavaObject, crate::JavaObject>>>>) -> Self {
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

    pub fn new1_HashMap(mut initHashMap: Option<Rc<RefCell<HashMap<crate::JavaObject, crate::JavaObject>>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct1_HashMap(initHashMap))))
    }
    

    pub fn getFormData(&mut self, mut hashMap: Option<Rc<RefCell<HashMap<crate::JavaObject, crate::JavaObject>>>>) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut initializerData: Option<Rc<RefCell<InitializerData>>> = InitializerData::getInstance();
            self.userJdbcDriver = { let java_value = ({ let java_object = (jcall!(hashMap).get(crate::java_into_object({ let java_field = jcall!(initializerData).CUSTOMERJDBCDRIVER.clone(); java_field }.clone()))).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() }).clone(); java_value };
            self.userName = { let java_value = ({ let java_object = (jcall!(hashMap).get(crate::java_into_object({ let java_field = jcall!(initializerData).CUSTOMERNAME.clone(); java_field }.clone()))).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() }).clone(); java_value };
            self.userUserName = { let java_value = ({ let java_object = (jcall!(hashMap).get(crate::java_into_object({ let java_field = jcall!(initializerData).CUSTOMERUSERNAME.clone(); java_field }.clone()))).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() }).clone(); java_value };
            self.userPassword = { let java_value = ({ let java_object = (jcall!(hashMap).get(crate::java_into_object({ let java_field = jcall!(initializerData).CUSTOMERPASSWORD.clone(); java_field }.clone()))).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() }).clone(); java_value };
            self.userSchema = { let java_value = ({ let java_object = (jcall!(hashMap).get(crate::java_into_object({ let java_field = jcall!(initializerData).CUSTOMERSCHEMA.clone(); java_field }.clone()))).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() }).clone(); java_value };
            self.userServer = { let java_value = ({ let java_object = (jcall!(hashMap).get(crate::java_into_object({ let java_field = jcall!(initializerData).CUSTOMERSERVER.clone(); java_field }.clone()))).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() }).clone(); java_value };
            self.userPort = { let java_value = ({ let java_object = (jcall!(hashMap).get(crate::java_into_object({ let java_field = jcall!(initializerData).CUSTOMERPORT.clone(); java_field }.clone()))).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() }).clone(); java_value };
            self.historyJdbcDriver = { let java_value = ({ let java_object = (jcall!(hashMap).get(crate::java_into_object({ let java_field = jcall!(initializerData).HISTORYJDBCDRIVER.clone(); java_field }.clone()))).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() }).clone(); java_value };
            self.historyName = { let java_value = ({ let java_object = (jcall!(hashMap).get(crate::java_into_object({ let java_field = jcall!(initializerData).HISTORYNAME.clone(); java_field }.clone()))).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() }).clone(); java_value };
            self.historyUserName = { let java_value = ({ let java_object = (jcall!(hashMap).get(crate::java_into_object({ let java_field = jcall!(initializerData).HISTORYUSERNAME.clone(); java_field }.clone()))).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() }).clone(); java_value };
            self.historyPassword = { let java_value = ({ let java_object = (jcall!(hashMap).get(crate::java_into_object({ let java_field = jcall!(initializerData).HISTORYPASSWORD.clone(); java_field }.clone()))).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() }).clone(); java_value };
            self.historySchema = { let java_value = ({ let java_object = (jcall!(hashMap).get(crate::java_into_object({ let java_field = jcall!(initializerData).HISTORYSCHEMA.clone(); java_field }.clone()))).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() }).clone(); java_value };
            self.historyServer = { let java_value = ({ let java_object = (jcall!(hashMap).get(crate::java_into_object({ let java_field = jcall!(initializerData).HISTORYSERVER.clone(); java_field }.clone()))).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() }).clone(); java_value };
            self.historyPort = { let java_value = ({ let java_object = (jcall!(hashMap).get(crate::java_into_object({ let java_field = jcall!(initializerData).HISTORYPORT.clone(); java_field }.clone()))).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() }).clone(); java_value };
            self.logJdbcDriver = { let java_value = ({ let java_object = (jcall!(hashMap).get(crate::java_into_object({ let java_field = jcall!(initializerData).LOGJDBCDRIVER.clone(); java_field }.clone()))).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() }).clone(); java_value };
            self.logName = { let java_value = ({ let java_object = (jcall!(hashMap).get(crate::java_into_object({ let java_field = jcall!(initializerData).LOGNAME.clone(); java_field }.clone()))).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() }).clone(); java_value };
            self.logUserName = { let java_value = ({ let java_object = (jcall!(hashMap).get(crate::java_into_object({ let java_field = jcall!(initializerData).LOGUSERNAME.clone(); java_field }.clone()))).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() }).clone(); java_value };
            self.logPassword = { let java_value = ({ let java_object = (jcall!(hashMap).get(crate::java_into_object({ let java_field = jcall!(initializerData).LOGPASSWORD.clone(); java_field }.clone()))).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() }).clone(); java_value };
            self.logSchema = { let java_value = ({ let java_object = (jcall!(hashMap).get(crate::java_into_object({ let java_field = jcall!(initializerData).LOGSCHEMA.clone(); java_field }.clone()))).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() }).clone(); java_value };
            self.logServer = { let java_value = ({ let java_object = (jcall!(hashMap).get(crate::java_into_object({ let java_field = jcall!(initializerData).LOGSERVER.clone(); java_field }.clone()))).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() }).clone(); java_value };
            self.logPort = { let java_value = ({ let java_object = (jcall!(hashMap).get(crate::java_into_object({ let java_field = jcall!(initializerData).LOGPORT.clone(); java_field }.clone()))).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() }).clone(); java_value };
            self.inventoryJdbcDriver = { let java_value = ({ let java_object = (jcall!(hashMap).get(crate::java_into_object({ let java_field = jcall!(initializerData).INVENTORYJDBCDRIVER.clone(); java_field }.clone()))).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() }).clone(); java_value };
            self.inventoryName = { let java_value = ({ let java_object = (jcall!(hashMap).get(crate::java_into_object({ let java_field = jcall!(initializerData).INVENTORYNAME.clone(); java_field }.clone()))).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() }).clone(); java_value };
            self.inventoryUserName = { let java_value = ({ let java_object = (jcall!(hashMap).get(crate::java_into_object({ let java_field = jcall!(initializerData).INVENTORYUSERNAME.clone(); java_field }.clone()))).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() }).clone(); java_value };
            self.inventoryPassword = { let java_value = ({ let java_object = (jcall!(hashMap).get(crate::java_into_object({ let java_field = jcall!(initializerData).INVENTORYPASSWORD.clone(); java_field }.clone()))).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() }).clone(); java_value };
            self.inventorySchema = { let java_value = ({ let java_object = (jcall!(hashMap).get(crate::java_into_object({ let java_field = jcall!(initializerData).INVENTORYSCHEMA.clone(); java_field }.clone()))).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() }).clone(); java_value };
            self.inventoryServer = { let java_value = ({ let java_object = (jcall!(hashMap).get(crate::java_into_object({ let java_field = jcall!(initializerData).INVENTORYSERVER.clone(); java_field }.clone()))).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() }).clone(); java_value };
            self.inventoryPort = { let java_value = ({ let java_object = (jcall!(hashMap).get(crate::java_into_object({ let java_field = jcall!(initializerData).INVENTORYPORT.clone(); java_field }.clone()))).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() }).clone(); java_value };
            self.staticPagesJdbcDriver = { let java_value = ({ let java_object = (jcall!(hashMap).get(crate::java_into_object({ let java_field = jcall!(initializerData).STATICPAGESJDBCDRIVER.clone(); java_field }.clone()))).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() }).clone(); java_value };
            self.staticPagesName = { let java_value = ({ let java_object = (jcall!(hashMap).get(crate::java_into_object({ let java_field = jcall!(initializerData).STATICPAGESNAME.clone(); java_field }.clone()))).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() }).clone(); java_value };
            self.staticPagesUserName = { let java_value = ({ let java_object = (jcall!(hashMap).get(crate::java_into_object({ let java_field = jcall!(initializerData).STATICPAGESUSERNAME.clone(); java_field }.clone()))).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() }).clone(); java_value };
            self.staticPagesPassword = { let java_value = ({ let java_object = (jcall!(hashMap).get(crate::java_into_object({ let java_field = jcall!(initializerData).STATICPAGESPASSWORD.clone(); java_field }.clone()))).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() }).clone(); java_value };
            self.staticPagesSchema = { let java_value = ({ let java_object = (jcall!(hashMap).get(crate::java_into_object({ let java_field = jcall!(initializerData).STATICPAGESSCHEMA.clone(); java_field }.clone()))).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() }).clone(); java_value };
            self.staticPagesServer = { let java_value = ({ let java_object = (jcall!(hashMap).get(crate::java_into_object({ let java_field = jcall!(initializerData).STATICPAGESSERVER.clone(); java_field }.clone()))).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() }).clone(); java_value };
            self.staticPagesPort = { let java_value = ({ let java_object = (jcall!(hashMap).get(crate::java_into_object({ let java_field = jcall!(initializerData).STATICPAGESPORT.clone(); java_field }.clone()))).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() }).clone(); java_value };
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
        if !{ let java_arguments = (self.userJdbcDriver.clone(),); self.isJdbcDriverValid(java_arguments.0) } {
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
        if !{ let java_arguments = (self.historyJdbcDriver.clone(),); self.isJdbcDriverValid(java_arguments.0) } {
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
        if !{ let java_arguments = (self.logJdbcDriver.clone(),); self.isJdbcDriverValid(java_arguments.0) } {
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
        if !{ let java_arguments = (self.inventoryJdbcDriver.clone(),); self.isJdbcDriverValid(java_arguments.0) } {
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
        if !{ let java_arguments = (self.staticPagesJdbcDriver.clone(),); self.isJdbcDriverValid(java_arguments.0) } {
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
        if !{ let java_arguments = (self.userJdbcDriver.clone(),); self.isJdbcDriverValid(java_arguments.0) } {
            isJdbcDriversValid = false;
            jcall!(stringBuffer).append({ let java_arguments = (self.userJdbcDriver.clone(),); self.getJdbcDriverValidationInfo(java_arguments.0) });
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
        if !{ let java_arguments = (self.historyJdbcDriver.clone(),); self.isJdbcDriverValid(java_arguments.0) } {
            isJdbcDriversValid = false;
            jcall!(stringBuffer).append({ let java_arguments = (self.historyJdbcDriver.clone(),); self.getJdbcDriverValidationInfo(java_arguments.0) });
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
        if !{ let java_arguments = (self.logJdbcDriver.clone(),); self.isJdbcDriverValid(java_arguments.0) } {
            isJdbcDriversValid = false;
            jcall!(stringBuffer).append({ let java_arguments = (self.logJdbcDriver.clone(),); self.getJdbcDriverValidationInfo(java_arguments.0) });
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
        if !{ let java_arguments = (self.inventoryJdbcDriver.clone(),); self.isJdbcDriverValid(java_arguments.0) } {
            isJdbcDriversValid = false;
            jcall!(stringBuffer).append({ let java_arguments = (self.inventoryJdbcDriver.clone(),); self.getJdbcDriverValidationInfo(java_arguments.0) });
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
        if !{ let java_arguments = (self.staticPagesJdbcDriver.clone(),); self.isJdbcDriverValid(java_arguments.0) } {
            isJdbcDriversValid = false;
            jcall!(stringBuffer).append({ let java_arguments = (self.staticPagesJdbcDriver.clone(),); self.getJdbcDriverValidationInfo(java_arguments.0) });
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

