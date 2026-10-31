// 
// To change this license header, choose License Headers in Project Properties. 
// To change this template file, choose Tools | Templates 
// and open the template in the editor.  
// Generated Code Do Not Modify
// package org.allbinary.logic.communication.sql
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::java::net::InetAddress::InetAddress;
use crate::org::allbinary::business::init::db::DatabaseConnectionInfoInterface::DatabaseConnectionInfoInterface;
use crate::org::allbinary::business::init::db::DbConnectionInfo::DbConnectionInfo;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use super::AbSqlBean::AbSqlBean;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct AbDatabaseManagement {
    pub base: AbSqlBean,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub commonStrings: Option<Rc<RefCell<CommonStrings>>>,
    pub sqlCommandLog: Option<Rc<RefCell<StringMaker>>>,
    pub GRANT_ALL: String,
    pub DOT_STAR: String,
    pub TO: String,
    pub AT_LOCALHOST: String,
    pub IDENTIFIED_BY: String,
    pub WITH_GRANT_OPTION: String,
    pub FAILED: String,
    pub GET_HOST_NAME: String,
    pub METHOD_ADD_USER: String,
    pub METHOD_ADD_USERS: String,
    pub METHOD_ADD_DB: String,
    pub METHOD_ADD_DATABASES: String,
    pub METHOD_ADD_TABLES: String,
    pub UNABLE_TO_CREATE_USER: String,
    pub UNABLE_TO_CREATE_DATABASE: String,
    pub UNABLE_TO_CREATE_TABLES: String,
    pub ADD_TABLES_RESULTS_LABEL: String,
}


impl AbDatabaseManagement {
    

    pub fn new_struct1(mut databaseConnectionInfoInterface: Option<Rc<RefCell<dyn DatabaseConnectionInfoInterface>>>) -> Self {
        let mut this = Self {
            base: AbSqlBean::new_struct((databaseConnectionInfoInterface).clone() /* as DbConnectionInfo */),
            logUtil: None,
            commonStrings: None,
            sqlCommandLog: None,
            GRANT_ALL: String::new(),
            DOT_STAR: String::new(),
            TO: String::new(),
            AT_LOCALHOST: String::new(),
            IDENTIFIED_BY: String::new(),
            WITH_GRANT_OPTION: String::new(),
            FAILED: String::new(),
            GET_HOST_NAME: String::new(),
            METHOD_ADD_USER: String::new(),
            METHOD_ADD_USERS: String::new(),
            METHOD_ADD_DB: String::new(),
            METHOD_ADD_DATABASES: String::new(),
            METHOD_ADD_TABLES: String::new(),
            UNABLE_TO_CREATE_USER: String::new(),
            UNABLE_TO_CREATE_DATABASE: String::new(),
            UNABLE_TO_CREATE_TABLES: String::new(),
            ADD_TABLES_RESULTS_LABEL: String::new(),
        };
        this.logUtil = LogUtil::getInstance();
        this.commonStrings = CommonStrings::getInstance();
        this.sqlCommandLog = StringMaker::new();
        this.GRANT_ALL = String::from("GRANT SELECT, INSERT, UPDATE, DELETE, CREATE, DROP, INDEX, ALTER ON ");
        this.DOT_STAR = String::from(".*");
        this.TO = String::from(" TO ");
        this.AT_LOCALHOST = String::from("@localhost");
        this.IDENTIFIED_BY = String::from(" IDENTIFIED BY '");
        this.WITH_GRANT_OPTION = String::from("' WITH GRANT OPTION");
        this.FAILED = String::from("Failed");
        this.GET_HOST_NAME = String::from("getHostName()");
        this.METHOD_ADD_USER = String::from("addUser()");
        this.METHOD_ADD_USERS = String::from("addUsers()");
        this.METHOD_ADD_DB = String::from("addDb()");
        this.METHOD_ADD_DATABASES = String::from("addDatabases()");
        this.METHOD_ADD_TABLES = String::from("addTables()");
        this.UNABLE_TO_CREATE_USER = String::from("Unable to Create User: ");
        this.UNABLE_TO_CREATE_DATABASE = String::from("Unable to Create Database: ");
        this.UNABLE_TO_CREATE_TABLES = String::from("Unable to Create Tables");
        this.ADD_TABLES_RESULTS_LABEL = String::from("Add Table Results: ");
        // super(...) is in the struct literal above
        this
    }

    pub fn new1(mut databaseConnectionInfoInterface: Option<Rc<RefCell<dyn DatabaseConnectionInfoInterface>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct1(databaseConnectionInfoInterface))))
    }
    

    pub fn new_struct1(mut connectionInfo: Option<Rc<RefCell<dyn crate::org::allbinary::business::init::db::DbConnectionInfo::DbConnectionInfoDyn>>>) -> Self {
        let mut this = Self {
            base: AbSqlBean::new_struct(connectionInfo.clone()),
            logUtil: None,
            commonStrings: None,
            sqlCommandLog: None,
            GRANT_ALL: String::new(),
            DOT_STAR: String::new(),
            TO: String::new(),
            AT_LOCALHOST: String::new(),
            IDENTIFIED_BY: String::new(),
            WITH_GRANT_OPTION: String::new(),
            FAILED: String::new(),
            GET_HOST_NAME: String::new(),
            METHOD_ADD_USER: String::new(),
            METHOD_ADD_USERS: String::new(),
            METHOD_ADD_DB: String::new(),
            METHOD_ADD_DATABASES: String::new(),
            METHOD_ADD_TABLES: String::new(),
            UNABLE_TO_CREATE_USER: String::new(),
            UNABLE_TO_CREATE_DATABASE: String::new(),
            UNABLE_TO_CREATE_TABLES: String::new(),
            ADD_TABLES_RESULTS_LABEL: String::new(),
        };
        this.logUtil = LogUtil::getInstance();
        this.commonStrings = CommonStrings::getInstance();
        this.sqlCommandLog = StringMaker::new();
        this.GRANT_ALL = String::from("GRANT SELECT, INSERT, UPDATE, DELETE, CREATE, DROP, INDEX, ALTER ON ");
        this.DOT_STAR = String::from(".*");
        this.TO = String::from(" TO ");
        this.AT_LOCALHOST = String::from("@localhost");
        this.IDENTIFIED_BY = String::from(" IDENTIFIED BY '");
        this.WITH_GRANT_OPTION = String::from("' WITH GRANT OPTION");
        this.FAILED = String::from("Failed");
        this.GET_HOST_NAME = String::from("getHostName()");
        this.METHOD_ADD_USER = String::from("addUser()");
        this.METHOD_ADD_USERS = String::from("addUsers()");
        this.METHOD_ADD_DB = String::from("addDb()");
        this.METHOD_ADD_DATABASES = String::from("addDatabases()");
        this.METHOD_ADD_TABLES = String::from("addTables()");
        this.UNABLE_TO_CREATE_USER = String::from("Unable to Create User: ");
        this.UNABLE_TO_CREATE_DATABASE = String::from("Unable to Create Database: ");
        this.UNABLE_TO_CREATE_TABLES = String::from("Unable to Create Tables");
        this.ADD_TABLES_RESULTS_LABEL = String::from("Add Table Results: ");
        // super(...) is in the struct literal above
        this
    }

    pub fn new1(mut connectionInfo: Option<Rc<RefCell<dyn crate::org::allbinary::business::init::db::DbConnectionInfo::DbConnectionInfoDyn>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct1(connectionInfo))))
    }
    

    pub fn getHostName(&mut self) -> String {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut addr: Option<Rc<RefCell<InetAddress>>> = InetAddress::getLocalHost();
            return jcall!(addr).getHostName();
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            jcall!(self.logUtil).put(self.FAILED.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), self.GET_HOST_NAME.clone(), crate::java_into_object(e.clone()));
            return None;
            }
        }
    }
    

    // throws Exception
    pub fn addDbUser4(&mut self, mut hostName: String, mut db: String, mut userName: String, mut password: String) -> bool {
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        jcall!(stringBuffer).append(self.GRANT_ALL.clone());
        jcall!(stringBuffer).append(db.clone());
        jcall!(stringBuffer).append(self.DOT_STAR.clone());
        jcall!(stringBuffer).append(self.TO.clone());
        jcall!(stringBuffer).append(userName.clone());
        jcall!(stringBuffer).append(hostName.clone());
        jcall!(stringBuffer).append(self.IDENTIFIED_BY.clone());
        jcall!(stringBuffer).append(password.clone());
        jcall!(stringBuffer).append(self.WITH_GRANT_OPTION.clone());
        let mut sqlStatement: String = jcall!(stringBuffer).toString();
        jcall!(self.sqlCommandLog).append(sqlStatement.clone());
        jcall!(self.sqlCommandLog).append(jcall!(self.commonSeps).NEW_LINE.clone());
        self.base.executeSQLStatement(sqlStatement.clone());
        return true;
    }
    

    // Java overloaded method: Rust needs a different name
    pub fn addDbUser(&mut self, mut db: String, mut userName: String, mut password: String) -> bool {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            self.addDbUser4(self.AT_LOCALHOST.clone(), db.clone(), userName.clone(), password.clone());
            let mut hostName: String = self.getHostName();
            if true {
                self.addDbUser4(hostName.clone(), db.clone(), userName.clone(), password.clone());
            }
            return true;
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            jcall!(self.logUtil).put(format!("{}{}", self.UNABLE_TO_CREATE_USER, userName), crate::java_into_object(String::from(std::any::type_name::<Self>())), self.METHOD_ADD_USER.clone(), crate::java_into_object(e.clone()));
            return false;
            }
        }
    }
    

    pub fn addDb(&mut self, mut db: String) -> Option<bool> {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut sqlStatement: String = format!("{}{}", jcall!(self.sqlStrings).CREATE_DATABASE, db);
            jcall!(self.sqlCommandLog).append(sqlStatement.clone());
            jcall!(self.sqlCommandLog).append(jcall!(self.commonSeps).NEW_LINE.clone());
            self.base.executeSQLStatement(sqlStatement.clone());
            return true;
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            jcall!(self.logUtil).put(format!("{}{}", self.UNABLE_TO_CREATE_DATABASE, db), crate::java_into_object(String::from(std::any::type_name::<Self>())), self.METHOD_ADD_DB.clone(), crate::java_into_object(e.clone()));
            return false;
            }
        }
    }
}
impl std::ops::Deref for AbDatabaseManagement {
    type Target = AbSqlBean;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for AbDatabaseManagement {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

pub trait AbDatabaseManagementDyn: crate::org::allbinary::logic::communication::sql::AbSqlBean::AbSqlBeanDyn {
    fn as_AbDatabaseManagement(&self) -> &AbDatabaseManagement;
    fn as_AbDatabaseManagement_mut(&mut self) -> &mut AbDatabaseManagement;
    fn getHostName(&mut self) -> String { AbDatabaseManagement::getHostName(self.as_AbDatabaseManagement_mut()) }
    fn addDbUser4(&mut self, mut hostName: String, mut db: String, mut userName: String, mut password: String) -> bool { AbDatabaseManagement::addDbUser4(self.as_AbDatabaseManagement_mut(), hostName, db, userName, password) }
    fn addDbUser(&mut self, mut db: String, mut userName: String, mut password: String) -> bool { AbDatabaseManagement::addDbUser(self.as_AbDatabaseManagement_mut(), db, userName, password) }
    fn addDb(&mut self, mut db: String) -> Option<bool> { AbDatabaseManagement::addDb(self.as_AbDatabaseManagement_mut(), db) }
}
impl AbDatabaseManagementDyn for AbDatabaseManagement {
    fn as_AbDatabaseManagement(&self) -> &AbDatabaseManagement { self }
    fn as_AbDatabaseManagement_mut(&mut self) -> &mut AbDatabaseManagement { self }
}

impl crate::org::allbinary::logic::communication::sql::AbSqlBasic::AbSqlBasicDyn for AbDatabaseManagement {
    fn as_AbSqlBasic(&self) -> &crate::org::allbinary::logic::communication::sql::AbSqlBasic::AbSqlBasic { &self.base.base.base.base.base }
    fn as_AbSqlBasic_mut(&mut self) -> &mut crate::org::allbinary::logic::communication::sql::AbSqlBasic::AbSqlBasic { &mut self.base.base.base.base.base }
}
impl crate::org::allbinary::logic::communication::sql::AbSqlBean::AbSqlBeanDyn for AbDatabaseManagement {
    fn as_AbSqlBean(&self) -> &crate::org::allbinary::logic::communication::sql::AbSqlBean::AbSqlBean { &self.base }
    fn as_AbSqlBean_mut(&mut self) -> &mut crate::org::allbinary::logic::communication::sql::AbSqlBean::AbSqlBean { &mut self.base }
}
impl crate::org::allbinary::logic::communication::sql::AbSqlColumn::AbSqlColumnDyn for AbDatabaseManagement {
    fn as_AbSqlColumn(&self) -> &crate::org::allbinary::logic::communication::sql::AbSqlColumn::AbSqlColumn { &self.base.base.base }
    fn as_AbSqlColumn_mut(&mut self) -> &mut crate::org::allbinary::logic::communication::sql::AbSqlColumn::AbSqlColumn { &mut self.base.base.base }
}
impl crate::org::allbinary::logic::communication::sql::AbSqlRow::AbSqlRowDyn for AbDatabaseManagement {
    fn as_AbSqlRow(&self) -> &crate::org::allbinary::logic::communication::sql::AbSqlRow::AbSqlRow { &self.base.base }
    fn as_AbSqlRow_mut(&mut self) -> &mut crate::org::allbinary::logic::communication::sql::AbSqlRow::AbSqlRow { &mut self.base.base }
}
impl crate::org::allbinary::logic::communication::sql::AbSqlTable::AbSqlTableDyn for AbDatabaseManagement {
    fn as_AbSqlTable(&self) -> &crate::org::allbinary::logic::communication::sql::AbSqlTable::AbSqlTable { &self.base.base.base.base }
    fn as_AbSqlTable_mut(&mut self) -> &mut crate::org::allbinary::logic::communication::sql::AbSqlTable::AbSqlTable { &mut self.base.base.base.base }
}


