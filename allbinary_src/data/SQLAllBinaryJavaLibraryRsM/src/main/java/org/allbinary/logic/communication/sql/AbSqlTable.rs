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
// package org.allbinary.logic.communication.sql
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::business::init::db::DbConnectionInfo::DbConnectionInfo;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use super::AbSqlBasic::AbSqlBasic;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct AbSqlTable {
    pub base: AbSqlBasic,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub tableName: String,
    pub METHOD_CREATED_TABLE: String,
    pub METHOD_DROP_TABLE: String,
    pub TABLE_CREATION_SUCCESS: String,
    pub DROPPED_SUCCESS: String,
}


impl AbSqlTable {
    

    pub fn new_struct(mut databaseConnectionInfoInterface: Option<Rc<RefCell<dyn crate::org::allbinary::business::init::db::DbConnectionInfo::DbConnectionInfoDyn>>>) -> Self {
        let mut this = Self {
            base: AbSqlBasic::new_struct(databaseConnectionInfoInterface.clone()),
            logUtil: None,
            tableName: String::new(),
            METHOD_CREATED_TABLE: String::new(),
            METHOD_DROP_TABLE: String::new(),
            TABLE_CREATION_SUCCESS: String::new(),
            DROPPED_SUCCESS: String::new(),
        };
        this.logUtil = LogUtil::getInstance();
        this.METHOD_CREATED_TABLE = String::from("createTable()");
        this.METHOD_DROP_TABLE = String::from("dropTable()");
        this.TABLE_CREATION_SUCCESS = String::from("Table Creation Successful: ");
        this.DROPPED_SUCCESS = String::from(" Dropped Successfully");
        // super(...) is in the struct literal above
        this
    }

    pub fn new(mut databaseConnectionInfoInterface: Option<Rc<RefCell<dyn crate::org::allbinary::business::init::db::DbConnectionInfo::DbConnectionInfoDyn>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(databaseConnectionInfoInterface))))
    }
    

    pub fn setTableName(&mut self, mut tableName: String) {
        self.tableName = tableName.clone();
    }
    

    pub fn getTableName(&mut self) -> String {
        return self.tableName.clone();
    }
    

    // synchronized
    pub fn createTable(&mut self, mut data: String) -> String {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            self.executeSQLStatement(data.clone());
            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).SQLLOGGING.clone()) {
                jcall!(self.logUtil).putF(format!("{}{}", format!("{}{}", format!("{}{}", self.TABLE_CREATION_SUCCESS, self.tableName), String::from(" with statement: ")), data), crate::java_into_object(String::from(std::any::type_name::<Self>())), self.METHOD_CREATED_TABLE.clone());
            }
            return format!("{}{}", self.tableName, jcall!(self.sqlStrings).CREATE_RETURN);
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).SQLLOGGINGERROR.clone()) {
                jcall!(self.logUtil).put(format!("{}{}", format!("{}{}", format!("{}{}", String::from("Table Creation Failed: "), self.tableName), String::from(" with statement: ")), data), crate::java_into_object(String::from(std::any::type_name::<Self>())), self.METHOD_CREATED_TABLE.clone(), crate::java_into_object(e.clone()));
            }
            return format!("{}{}", format!("{}{}", String::from("Failed to Create "), self.tableName), String::from(" table."));
            }
        }
    }
    

    // synchronized
    pub fn dropTable(&mut self) -> String {
        let mut sqlStatement: String = format!("{}{}", jcall!(self.sqlStrings).DROP_TABLE, self.tableName);
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            self.executeSQLStatement(sqlStatement.clone());
            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).SQLLOGGING.clone()) {
                jcall!(self.logUtil).putF(format!("{}{}", self.SUCCESS_SQL_STATEMENT, sqlStatement), crate::java_into_object(String::from(std::any::type_name::<Self>())), self.METHOD_DROP_TABLE.clone());
            }
            return format!("{}{}", self.tableName, self.DROPPED_SUCCESS);
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).SQLLOGGINGERROR.clone()) {
                jcall!(self.logUtil).put(format!("{}{}", self.FAILED_SQL_STATEMENT, sqlStatement), crate::java_into_object(String::from(std::any::type_name::<Self>())), self.METHOD_DROP_TABLE.clone(), crate::java_into_object(e.clone()));
            }
            return format!("{}{}", format!("{}{}", String::from("Failed to Drop "), self.tableName), String::from(" table."));
            }
        }
    }
}
impl std::ops::Deref for AbSqlTable {
    type Target = AbSqlBasic;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for AbSqlTable {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

pub trait AbSqlTableDyn: crate::org::allbinary::logic::communication::sql::AbSqlBasic::AbSqlBasicDyn {
    fn as_AbSqlTable(&self) -> &AbSqlTable;
    fn as_AbSqlTable_mut(&mut self) -> &mut AbSqlTable;
    fn setTableName(&mut self, mut tableName: String) { AbSqlTable::setTableName(self.as_AbSqlTable_mut(), tableName) }
    fn getTableName(&mut self) -> String { AbSqlTable::getTableName(self.as_AbSqlTable_mut()) }
    fn createTable(&mut self, mut data: String) -> String { AbSqlTable::createTable(self.as_AbSqlTable_mut(), data) }
    fn dropTable(&mut self) -> String { AbSqlTable::dropTable(self.as_AbSqlTable_mut()) }
}
impl AbSqlTableDyn for AbSqlTable {
    fn as_AbSqlTable(&self) -> &AbSqlTable { self }
    fn as_AbSqlTable_mut(&mut self) -> &mut AbSqlTable { self }
}

impl crate::org::allbinary::logic::communication::sql::AbSqlBasic::AbSqlBasicDyn for AbSqlTable {
    fn as_AbSqlBasic(&self) -> &crate::org::allbinary::logic::communication::sql::AbSqlBasic::AbSqlBasic { &self.base }
    fn as_AbSqlBasic_mut(&mut self) -> &mut crate::org::allbinary::logic::communication::sql::AbSqlBasic::AbSqlBasic { &mut self.base }
}


