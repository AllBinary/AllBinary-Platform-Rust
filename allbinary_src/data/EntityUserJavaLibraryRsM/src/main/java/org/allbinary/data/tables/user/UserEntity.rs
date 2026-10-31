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
// package org.allbinary.data.tables.user
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;
use super::*;
// use crate::java::util::HashMap::HashMap;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
use crate::org::allbinary::business::context::modules::storefront::StoreFrontData::StoreFrontData;
use crate::org::allbinary::business::context::modules::storefront::StoreFrontInterface::StoreFrontInterface;
use crate::org::allbinary::business::entry::EntryData::EntryData;
use crate::org::allbinary::business::init::db::UserDbInitInfo::UserDbInitInfo;
use crate::org::allbinary::business::user::CreateUserFactory::CreateUserFactory;
use crate::org::allbinary::business::user::UserData::UserData;
use crate::org::allbinary::business::user::UserInterface::UserInterface;
use crate::org::allbinary::business::user::modules::User::User;
use crate::org::allbinary::business::user::role::UserRole::UserRole;
use crate::org::allbinary::business::user::role::UserRoleData::UserRoleData;
use crate::org::allbinary::business::user::role::UserRoleFactory::UserRoleFactory;
use crate::org::allbinary::globals::GLOBALS2::GLOBALS2;
use crate::org::allbinary::logic::StdUtil::StdUtil;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::communication::sql::AbSqlBean::AbSqlBean;
use crate::org::allbinary::logic::control::crypt::SuperCrypt::SuperCrypt;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::string::CommonSeps::CommonSeps;
use super::UserEntityInterface::UserEntityInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct UserEntity {
    pub base: AbSqlBean,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub tableName: String,
    pub COMMAND_SUCCESS_FOR_USER: String,
    pub PASSWORD_LABEL: String,
    pub EQUALS: String,
    pub COMMAND_SUCCESS_BUT_LOGIN_FAILED: String,
    pub INVALID_PASSWORD_LABEL: String,
    pub NOT_EQUAL: String,
    pub END_QUOTES: String,
}

// impl UserEntityInterface for UserEntity with the methods of the trait

impl UserEntity {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: AbSqlBean::new_struct(UserDbInitInfo::new()),
            logUtil: None,
            tableName: String::new(),
            COMMAND_SUCCESS_FOR_USER: String::new(),
            PASSWORD_LABEL: String::new(),
            EQUALS: String::new(),
            COMMAND_SUCCESS_BUT_LOGIN_FAILED: String::new(),
            INVALID_PASSWORD_LABEL: String::new(),
            NOT_EQUAL: String::new(),
            END_QUOTES: String::new(),
        };
        this.logUtil = LogUtil::getInstance();
        this.tableName = String::from("user");
        this.COMMAND_SUCCESS_FOR_USER = String::from("Command Success for user: ");
        this.PASSWORD_LABEL = String::from(" Password: ");
        this.EQUALS = String::from("==");
        this.COMMAND_SUCCESS_BUT_LOGIN_FAILED = String::from("Command Success but login failed for user: ");
        this.INVALID_PASSWORD_LABEL = String::from(" Password: \n\"");
        this.NOT_EQUAL = String::from("\"!=\"");
        this.END_QUOTES = jcall!(CommonSeps::getInstance()).QUOTE.clone();
        // super(...) is in the struct literal above
        this.setTableName(this.tableName.clone());
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn insert(&mut self, mut values: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            self.base.insert(values.clone());
            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).SQLLOGGING.clone()) {
                jcall!(self.logUtil).putF(jcall!(self.commonStrings).SUCCESS.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), INSERT.clone());
            }
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).SQLLOGGING.clone()) {
                jcall!(self.logUtil).put(jcall!(self.commonStrings).FAILURE.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), INSERT.clone(), crate::java_into_object(e.clone()));
            }
            }
        }
    }
    

    // throws Exception
    pub fn getAdministrators(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        return { let java_arguments = (jcall!(UserRoleFactory::getInstance()).ADMINISTRATOR.clone(),); self.getUsersWithRole(java_arguments.0) };
    }
    

    // throws Exception
    pub fn getStoreManagers(&mut self, mut storeFrontInterface: Option<Rc<RefCell<dyn StoreFrontInterface>>>) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        let mut keysAndValues: Option<Rc<RefCell<HashMap>>> = jcall!(StdUtil::getInstance()).createHashMap();
        let mut usersVector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        jcall!(keysAndValues).put(crate::org::allbinary::business::user::role::UserRoleData::UserRoleData_NAME.with(Clone::clone).clone(), jcall!(jcall!(UserRoleFactory::getInstance()).STOREMANAGER).toString());
        jcall!(keysAndValues).put(crate::org::allbinary::business::user::UserData::UserData_PERMISSIONS.with(Clone::clone).clone(), jcall!(storeFrontInterface).getName());
        let mut usersHashMapVector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = self.base.getRows(keysAndValues.clone());
        let mut size: i32 = jcall!(usersHashMapVector).size();
        for i in (0..size) {
            let mut userHashMap: Option<Rc<RefCell<HashMap>>> = (jcall!(usersHashMapVector).get(i.clone())).clone() /* as HashMap */;
            if userHashMap.is_some() {
                jcall!(usersVector).add(crate::java_into_object(User::new1(userHashMap.clone()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::business::user::modules::User::UserDyn>>)));
            }
        }
        return usersVector.clone();
    }
    

    // throws Exception
    pub fn getCustomers(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        return { let java_arguments = (jcall!(UserRoleFactory::getInstance()).CUSTOMER.clone(),); self.getUsersWithRole(java_arguments.0) };
    }
    

    // throws Exception
    pub fn getUsersWithRole(&mut self, mut userRole: Option<Rc<RefCell<dyn crate::org::allbinary::business::user::role::UserRole::UserRoleDyn>>>) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        let mut keysAndValues: Option<Rc<RefCell<HashMap>>> = jcall!(StdUtil::getInstance()).createHashMap();
        let mut usersVector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        jcall!(keysAndValues).put(crate::org::allbinary::business::user::role::UserRoleData::UserRoleData_NAME.with(Clone::clone).clone(), jcall!(userRole).toString());
        let mut usersHashMapVector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = self.base.getRows(keysAndValues.clone());
        let mut size: i32 = jcall!(usersHashMapVector).size();
        for index in (0..size) {
            let mut userHashMap: Option<Rc<RefCell<HashMap>>> = (jcall!(usersHashMapVector).get(index.clone())).clone() /* as HashMap */;
            if userHashMap.is_some() {
                jcall!(usersVector).add(crate::java_into_object(User::new1(userHashMap.clone()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::business::user::modules::User::UserDyn>>)));
            }
        }
        return usersVector.clone();
    }
    

    // throws Exception
    pub fn getUsers(&mut self, mut storeFrontInterface: Option<Rc<RefCell<dyn StoreFrontInterface>>>) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        let mut keysAndValues: Option<Rc<RefCell<HashMap>>> = jcall!(StdUtil::getInstance()).createHashMap();
        let mut usersVector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        jcall!(keysAndValues).put(jcall!(StoreFrontData::getInstance()).NAME.clone(), jcall!(storeFrontInterface).getName());
        let mut usersHashMapVector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = self.base.getRows(keysAndValues.clone());
        let mut size: i32 = jcall!(usersHashMapVector).size();
        for index in (0..size) {
            let mut userHashMap: Option<Rc<RefCell<HashMap>>> = (jcall!(usersHashMapVector).get(index.clone())).clone() /* as HashMap */;
            if userHashMap.is_some() {
                jcall!(usersVector).add(crate::java_into_object(User::new1(userHashMap.clone()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::business::user::modules::User::UserDyn>>)));
            }
        }
        return usersVector.clone();
    }
    

    // throws Exception
    pub fn getUser(&mut self, mut userName: String) -> Option<Rc<RefCell<dyn UserInterface>>> {
        let mut row: Option<Rc<RefCell<HashMap>>> = jcall!(StdUtil::getInstance()).createHashMap();
        jcall!(row).put(crate::org::allbinary::business::user::UserData::UserData_USERNAME.with(Clone::clone).clone(), userName.clone());
        let mut userHashMap: Option<Rc<RefCell<HashMap>>> = self.base.getRow(row.clone());
        if userHashMap.is_some() {
            return CreateUserFactory::getInstance(userHashMap.clone());
        } else {
            return None;
        }
    }
    

    pub fn deleteWhere2(&mut self, mut key: String, mut value: String) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            self.base.deleteWhere2(key.clone(), value.clone());
            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).SQLLOGGING.clone()) {
                jcall!(self.logUtil).putF(jcall!(self.commonStrings).SUCCESS.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("deleteWhere"));
            }
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).SQLLOGGING.clone()) {
                jcall!(self.logUtil).put(jcall!(self.commonStrings).FAILURE.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("deleteWhere"), crate::java_into_object(e.clone()));
            }
            }
        }
    }
    

    pub fn login(&mut self, mut userName: String, mut password: String) -> String {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut isUserNameAndPasswordCorrect: i32 = 0;
            let mut result: String = self.base.getField3(crate::org::allbinary::business::user::UserData::UserData_USERNAME.with(Clone::clone).clone(), userName.clone(), crate::org::allbinary::business::user::UserData::UserData_PASSWORD.with(Clone::clone).clone());
            let mut encryption: String = self.base.getField3(crate::org::allbinary::business::user::UserData::UserData_USERNAME.with(Clone::clone).clone(), userName.clone(), jcall!(EntryData::getInstance()).ENCRYPTION.clone());
            if true && (encryption.cmp(&jcall!(self.stringUtil).EMPTY_STRING.clone()) as i32) != 0 {
                isUserNameAndPasswordCorrect = (result.cmp(&jcall!(SuperCrypt::new(((Some((encryption.clone()) as i32)).unwrap() as i32))).encrypt(password.clone())) as i32);
            } else {
                if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).SQLLOGGING.clone()) {
                    jcall!(self.logUtil).putF(format!("{}{}", format!("{}{}", String::from("Command Success but login failed for user: "), userName), String::from(" because user did not exist")), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("login"));
                }
                return crate::org::allbinary::globals::GLOBALS2::GLOBALS2_LOGINFAILED.with(Clone::clone).clone();
            }
            if isUserNameAndPasswordCorrect == 0 {
                if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).SQLLOGGING.clone()) {
                    let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
                    jcall!(stringBuffer).append(self.COMMAND_SUCCESS_FOR_USER.clone());
                    jcall!(stringBuffer).append(userName.clone());
                    jcall!(stringBuffer).append(self.PASSWORD_LABEL.clone());
                    jcall!(stringBuffer).append(password.clone());
                    jcall!(stringBuffer).append(self.EQUALS.clone());
                    jcall!(stringBuffer).append(result.clone());
                    { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(stringBuffer).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("login"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
                }
                return crate::org::allbinary::globals::GLOBALS2::GLOBALS2_LOGINSUCCESS.with(Clone::clone).clone();
            } else {
                if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).SQLLOGGING.clone()) {
                    let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
                    jcall!(stringBuffer).append(self.COMMAND_SUCCESS_BUT_LOGIN_FAILED.clone());
                    jcall!(stringBuffer).append(userName.clone());
                    jcall!(stringBuffer).append(self.INVALID_PASSWORD_LABEL.clone());
                    jcall!(stringBuffer).append(jcall!(SuperCrypt::new(((Some((encryption.clone()) as i32)).unwrap() as i32))).encrypt(password.clone()));
                    jcall!(stringBuffer).append(self.NOT_EQUAL.clone());
                    jcall!(stringBuffer).append(result.clone());
                    jcall!(stringBuffer).append(self.END_QUOTES.clone());
                    { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(stringBuffer).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("login"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
                }
                return crate::org::allbinary::globals::GLOBALS2::GLOBALS2_LOGINFAILED.with(Clone::clone).clone();
            }
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).SQLLOGGING.clone()) {
                jcall!(self.logUtil).put(jcall!(self.commonStrings).FAILURE.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("login"), crate::java_into_object(e.clone()));
            }
            return String::from("Error");
            }
        }
    }
    

    pub fn update(&mut self, mut userName: String, mut updatedValues: Option<Rc<RefCell<HashMap>>>) {
        self.base.updateWhere3(crate::org::allbinary::business::user::UserData::UserData_USERNAME.with(Clone::clone).clone(), userName.clone(), updatedValues.clone());
    }
    

    pub fn dropTables(&mut self) -> String {
        return self.base.dropTable();
    }
    

    pub fn createTableStatement(&mut self) -> String {
        let mut entryData: Option<Rc<RefCell<EntryData>>> = EntryData::getInstance();
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        jcall!(stringBuffer).append(jcall!(self.sqlStrings).CREATE_TABLE.clone()).append(self.tableName.clone()).append(jcall!(self.sqlStrings).START.clone()).append(crate::org::allbinary::business::user::UserData::UserData_USERNAME.with(Clone::clone).clone()).append(jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone()).append(crate::org::allbinary::business::user::UserData::UserData_PREFIXNAME.with(Clone::clone).clone()).append(jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN.clone()).append(crate::org::allbinary::business::user::UserData::UserData_FIRSTNAME.with(Clone::clone).clone()).append(jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN.clone()).append(crate::org::allbinary::business::user::UserData::UserData_LASTNAME.with(Clone::clone).clone()).append(jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN.clone()).append(crate::org::allbinary::business::user::UserData::UserData_MIDDLENAME.with(Clone::clone).clone()).append(jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN.clone()).append(crate::org::allbinary::business::user::UserData::UserData_SUFFIXNAME.with(Clone::clone).clone()).append(jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN.clone()).append(crate::org::allbinary::business::user::UserData::UserData_COMPANY.with(Clone::clone).clone()).append(jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN.clone()).append(crate::org::allbinary::business::user::UserData::UserData_POSITIONATCOMPANY.with(Clone::clone).clone()).append(jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN.clone()).append(crate::org::allbinary::business::user::UserData::UserData_MAINEMAIL.with(Clone::clone).clone()).append(jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone()).append(crate::org::allbinary::business::user::UserData::UserData_SECONDARYEMAIL.with(Clone::clone).clone()).append(jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN.clone()).append(crate::org::allbinary::business::user::UserData::UserData_HOMEPHONE.with(Clone::clone).clone()).append(jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN.clone()).append(crate::org::allbinary::business::user::UserData::UserData_CELLPHONE.with(Clone::clone).clone()).append(jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN.clone()).append(crate::org::allbinary::business::user::UserData::UserData_WORKPHONE.with(Clone::clone).clone()).append(jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN.clone()).append(crate::org::allbinary::business::user::UserData::UserData_OTHERCONTACT.with(Clone::clone).clone()).append(jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN.clone()).append(crate::org::allbinary::business::user::UserData::UserData_ELECTRONICDEVICE.with(Clone::clone).clone()).append(jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN.clone()).append(crate::org::allbinary::business::user::UserData::UserData_FAX.with(Clone::clone).clone()).append(jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN.clone()).append(crate::org::allbinary::business::user::role::UserRoleData::UserRoleData_NAME.with(Clone::clone).clone()).append(jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone()).append(crate::org::allbinary::business::user::UserData::UserData_CONFIGURATION.with(Clone::clone).clone()).append(jcall!(self.sqlTypeStrings).BLOB_NOT_NULL.clone()).append(crate::org::allbinary::business::user::UserData::UserData_PERMISSIONS.with(Clone::clone).clone()).append(jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN.clone()).append(jcall!(entryData).ENCRYPTION.clone()).append(jcall!(self.sqlTypeStrings).MAX_BIG_INT_UNSIGNED_NOT_NULL.clone()).append(crate::org::allbinary::business::user::UserData::UserData_SECRET.with(Clone::clone).clone()).append(jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN.clone()).append(crate::org::allbinary::business::user::UserData::UserData_PASSWORD.with(Clone::clone).clone()).append(jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone()).append(jcall!(entryData).ENABLE.clone()).append(jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN.clone()).append(jcall!(entryData).TIMECREATED.clone()).append(jcall!(self.sqlTypeStrings).MAX_BIG_INT_UNSIGNED_NOT_NULL.clone()).append(jcall!(entryData).LASTMODIFIED.clone()).append(jcall!(self.sqlTypeStrings).MAX_BIG_INT_UNSIGNED_NOT_NULL.clone()).append(jcall!(self.sqlStrings).PRIMARY_KEY.clone()).append(crate::org::allbinary::business::user::UserData::UserData_USERNAME.with(Clone::clone).clone()).append(jcall!(self.sqlStrings).END.clone());
        return jcall!(stringBuffer).toString();
    }
    

    pub fn createTable(&mut self) -> String {
        return self.base.createTable(self.createTableStatement());
    }
}
impl std::ops::Deref for UserEntity {
    type Target = AbSqlBean;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for UserEntity {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::logic::communication::sql::AbSqlBasic::AbSqlBasicDyn for UserEntity {
    fn as_AbSqlBasic(&self) -> &crate::org::allbinary::logic::communication::sql::AbSqlBasic::AbSqlBasic { &self.base.base.base.base.base }
    fn as_AbSqlBasic_mut(&mut self) -> &mut crate::org::allbinary::logic::communication::sql::AbSqlBasic::AbSqlBasic { &mut self.base.base.base.base.base }
}
impl crate::org::allbinary::logic::communication::sql::AbSqlBean::AbSqlBeanDyn for UserEntity {
    fn as_AbSqlBean(&self) -> &crate::org::allbinary::logic::communication::sql::AbSqlBean::AbSqlBean { &self.base }
    fn as_AbSqlBean_mut(&mut self) -> &mut crate::org::allbinary::logic::communication::sql::AbSqlBean::AbSqlBean { &mut self.base }
}
impl crate::org::allbinary::logic::communication::sql::AbSqlColumn::AbSqlColumnDyn for UserEntity {
    fn as_AbSqlColumn(&self) -> &crate::org::allbinary::logic::communication::sql::AbSqlColumn::AbSqlColumn { &self.base.base.base }
    fn as_AbSqlColumn_mut(&mut self) -> &mut crate::org::allbinary::logic::communication::sql::AbSqlColumn::AbSqlColumn { &mut self.base.base.base }
}
impl crate::org::allbinary::logic::communication::sql::AbSqlRow::AbSqlRowDyn for UserEntity {
    fn as_AbSqlRow(&self) -> &crate::org::allbinary::logic::communication::sql::AbSqlRow::AbSqlRow { &self.base.base }
    fn as_AbSqlRow_mut(&mut self) -> &mut crate::org::allbinary::logic::communication::sql::AbSqlRow::AbSqlRow { &mut self.base.base }
    fn deleteWhere2(&mut self, mut key: String, mut value: String) { Self::deleteWhere2(self, key, value) }
    fn insert(&mut self, mut values: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) { Self::insert(self, values) }
}
impl crate::org::allbinary::logic::communication::sql::AbSqlTable::AbSqlTableDyn for UserEntity {
    fn as_AbSqlTable(&self) -> &crate::org::allbinary::logic::communication::sql::AbSqlTable::AbSqlTable { &self.base.base.base.base }
    fn as_AbSqlTable_mut(&mut self) -> &mut crate::org::allbinary::logic::communication::sql::AbSqlTable::AbSqlTable { &mut self.base.base.base.base }
}


