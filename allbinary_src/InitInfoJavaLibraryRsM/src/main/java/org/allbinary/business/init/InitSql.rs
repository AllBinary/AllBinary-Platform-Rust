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
// package org.allbinary.business.init
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;
use super::*;
// use crate::java::sql::Connection::Connection;
// use crate::java::sql::DriverManager::DriverManager;
// use crate::java::sql::ResultSet::ResultSet;
// use crate::java::sql::ResultSetMetaData::ResultSetMetaData;
// use crate::java::sql::SQLException::SQLException;
// use crate::java::sql::Statement::Statement;
// use crate::java::util::HashMap::HashMap;
// use crate::java::util::Set::Set;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
use crate::org::allbinary::business::init::db::DatabaseConnectionInfoInterface::DatabaseConnectionInfoInterface;
use crate::org::allbinary::business::init::db::DbConnectionInfo::DbConnectionInfo;
use crate::org::allbinary::logic::StdUtil::StdUtil;
use crate::org::allbinary::logic::communication::log::PreLogUtil::PreLogUtil;
use crate::org::allbinary::logic::communication::log::config::r#type::LogConfigTypeFactory::LogConfigTypeFactory;
use crate::org::allbinary::logic::communication::log::config::r#type::LogConfigTypes::LogConfigTypes;
use crate::org::allbinary::logic::communication::sql::SqlStrings::SqlStrings;
use crate::org::allbinary::logic::communication::sql::SqlTypeStrings::SqlTypeStrings;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use crate::org::allbinary::logic::string::StringValidationUtil::StringValidationUtil;
use crate::org::allbinary::string::CommonSeps::CommonSeps;
use crate::org::allbinary::string::CommonStrings::CommonStrings;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct InitSql {
    pub stringUtil: Option<Rc<RefCell<StringUtil>>>,
    pub commonSeps: Option<Rc<RefCell<CommonSeps>>>,
    pub sqlTypeStrings: Option<Rc<RefCell<SqlTypeStrings>>>,
    pub sqlStrings: Option<Rc<RefCell<SqlStrings>>>,
    pub stdUtil: Option<Rc<RefCell<StdUtil>>>,
    pub INSERT: String,
    pub UPDATE: String,
    pub databaseConnectionInfoInterface: Option<Rc<RefCell<dyn crate::org::allbinary::business::init::db::DbConnectionInfo::DbConnectionInfoDyn>>>,
    pub userid: String,
    pub password: String,
    pub tableName: String,
    pub conn: Option<Rc<RefCell<Connection>>>,
    pub useridAndPassword: bool,
    pub SUCCESS_SQL_STATEMENT: String,
    pub FAILED_SQL_STATEMENT: String,
    pub INIT_SQL: String,
    pub commonStrings: Option<Rc<RefCell<CommonStrings>>>,
    pub EQUAL_QUOTE: String,
    pub METHOD_GET_ROW: String,
    pub METHOD_UPDATE_WHERE: String,
    pub ROW_VALUE_LABEL: String,
    pub NO_RESULTS_IN_RESULT_SET: String,
    pub INSERT_END: String,
}


impl InitSql {
    

    pub fn new_struct(mut databaseConnectionInfoInterface: Option<Rc<RefCell<dyn crate::org::allbinary::business::init::db::DbConnectionInfo::DbConnectionInfoDyn>>>) -> Self {
        let mut this = Self {
            stringUtil: None,
            commonSeps: None,
            sqlTypeStrings: None,
            sqlStrings: None,
            stdUtil: None,
            INSERT: String::new(),
            UPDATE: String::new(),
            databaseConnectionInfoInterface: None,
            userid: String::new(),
            password: String::new(),
            tableName: String::new(),
            conn: None,
            useridAndPassword: false,
            SUCCESS_SQL_STATEMENT: String::new(),
            FAILED_SQL_STATEMENT: String::new(),
            INIT_SQL: String::new(),
            commonStrings: None,
            EQUAL_QUOTE: String::new(),
            METHOD_GET_ROW: String::new(),
            METHOD_UPDATE_WHERE: String::new(),
            ROW_VALUE_LABEL: String::new(),
            NO_RESULTS_IN_RESULT_SET: String::new(),
            INSERT_END: String::new(),
        };
        this.stringUtil = StringUtil::getInstance();
        this.commonSeps = CommonSeps::getInstance();
        this.sqlTypeStrings = SqlTypeStrings::getInstance();
        this.sqlStrings = SqlStrings::getInstance();
        this.stdUtil = StdUtil::getInstance();
        this.INSERT = String::from("insert");
        this.UPDATE = String::from("update");
        this.SUCCESS_SQL_STATEMENT = String::from("Success\nSQL Statement: ");
        this.FAILED_SQL_STATEMENT = String::from("Failed\nSQL Statement: ");
        this.INIT_SQL = String::from("InitSql");
        this.commonStrings = CommonStrings::getInstance();
        this.EQUAL_QUOTE = String::from("=\"");
        this.METHOD_GET_ROW = String::from("getRow()");
        this.METHOD_UPDATE_WHERE = String::from("updateWhere()");
        this.ROW_VALUE_LABEL = String::from("Row Value: ");
        this.NO_RESULTS_IN_RESULT_SET = String::from("No Results in ResultSet");
        this.INSERT_END = String::from("')");
        this.setDatabaseConnectionInfoInterface(databaseConnectionInfoInterface.clone());
        this
    }

    pub fn new(mut databaseConnectionInfoInterface: Option<Rc<RefCell<dyn crate::org::allbinary::business::init::db::DbConnectionInfo::DbConnectionInfoDyn>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(databaseConnectionInfoInterface))))
    }
    

    pub fn setTable(&mut self, mut tableName: String) {
        self.tableName = tableName.clone();
    }
    

    pub fn setDatabaseConnectionInfoInterface(&mut self, mut databaseConnectionInfoInterface: Option<Rc<RefCell<dyn crate::org::allbinary::business::init::db::DbConnectionInfo::DbConnectionInfoDyn>>>) {
        self.databaseConnectionInfoInterface = databaseConnectionInfoInterface.clone();
    }
    

    pub fn createTable(&mut self, mut tableData: String) -> bool {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
            jcall!(stringBuffer).append(String::from("Creating Table: "));
            jcall!(stringBuffer).append(tableData.clone());
            PreLogUtil::put(jcall!(stringBuffer).toString(), self.INIT_SQL.clone(), String::from("createTable()"));
            self.executeSQLStatement(tableData.clone());
            return true;
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            if jcall!(unsafe { crate::org::allbinary::logic::communication::log::config::r#type::LogConfigTypes::LogConfigTypes_LOGGING.clone() }).contains(crate::java_into_object(jcall!(LogConfigTypeFactory::getInstance()).SQLLOGGINGERROR.clone())) {
                PreLogUtil::putOE(jcall!(self.commonStrings).EXCEPTION.clone(), self.INIT_SQL.clone(), String::from("createTable()"), e.clone());
            }
            return false;
            }
        }
    }
    

    pub fn dropTable(&mut self) -> bool {
        let mut sqlStatement: String = format!("{}{}", jcall!(self.sqlStrings).DROP_TABLE, self.tableName);
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            self.executeSQLStatement(sqlStatement.clone());
            return true;
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            if jcall!(unsafe { crate::org::allbinary::logic::communication::log::config::r#type::LogConfigTypes::LogConfigTypes_LOGGING.clone() }).contains(crate::java_into_object(jcall!(LogConfigTypeFactory::getInstance()).SQLLOGGINGERROR.clone())) {
                PreLogUtil::putOE(jcall!(self.commonStrings).EXCEPTION.clone(), self.INIT_SQL.clone(), String::from("dropTables()"), e.clone());
            }
            return false;
            }
        }
    }
    

    pub fn getRow(&mut self, mut keysAndValues: Option<Rc<RefCell<HashMap>>>) -> Option<Rc<RefCell<HashMap>>> {
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        jcall!(stringBuffer).append(jcall!(self.sqlStrings).SELECT_ALL_FROM.clone());
        jcall!(stringBuffer).append(self.tableName.clone());
        jcall!(stringBuffer).append(jcall!(self.sqlStrings).WHERE.clone());
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut result: Option<Rc<RefCell<HashMap>>> = None;
            let mut set: Option<Rc<RefCell<dyn Set>>> = jcall!(keysAndValues).keySet();
            let mut keyArray: Vec<Option<Rc<RefCell<Box<dyn std::any::Any>>>>> = jcall!(set).toArray();
            let mut size: i32 = (keyArray.len() as i32).clone();
            for i in (0..size) {
                let mut key: String = (keyArray[(i) as usize]).clone() /* as String */;
                let mut value: String = (jcall!(keysAndValues).get(key.clone())).clone() /* as String */.clone();
                jcall!(stringBuffer).append(key.clone());
                jcall!(stringBuffer).append(jcall!(self.sqlStrings).EQUAL_QUOTE.clone());
                jcall!(stringBuffer).append(self.getValue(value.clone()));
                jcall!(stringBuffer).append(jcall!(self.sqlStrings).CLOSE_QUOTE.clone());
                if i < size - 1 {
                    jcall!(stringBuffer).append(jcall!(self.sqlStrings).AND.clone());
                }
            }
            let mut sqlStatement: String = jcall!(stringBuffer).toString();
            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).SQLLOGGING.clone()) {
                PreLogUtil::put(format!("{}{}", jcall!(self.sqlStrings).SQL_STATEMENT_LABEL, sqlStatement), self.INIT_SQL.clone(), self.METHOD_GET_ROW.clone());
            }
            let mut rset: Option<Rc<RefCell<ResultSet>>> = self.executeSQLStatement(sqlStatement.clone());
            let mut resultSetMetaData: Option<Rc<RefCell<ResultSetMetaData>>> = jcall!(rset).getMetaData();
            while jcall!(rset).next() {
                result = jcall!(self.stdUtil).createHashMap();
                let mut columnCount: i32 = jcall!(resultSetMetaData).getColumnCount();
                for index in (1..=columnCount) {
                    let mut columnName: String = jcall!(resultSetMetaData).getColumnName(index.clone());
                    let mut field: String = jcall!(rset).getString(columnName.clone());
                    jcall!(result).put(columnName.clone(), field.clone());
                }
                if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).SQLLOGGING.clone()) {
                    PreLogUtil::put(format!("{}{}", self.ROW_VALUE_LABEL, jcall!(result).toString()), self.INIT_SQL.clone(), self.METHOD_GET_ROW.clone());
                }
                return result.clone();
            }
            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).SQLLOGGINGERROR.clone()) {
                PreLogUtil::put(self.NO_RESULTS_IN_RESULT_SET.clone(), self.INIT_SQL.clone(), self.METHOD_GET_ROW.clone());
            }
            return None;
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).SQLLOGGINGERROR.clone()) {
                PreLogUtil::putOE(format!("{}{}", self.FAILED_SQL_STATEMENT, stringBuffer), self.INIT_SQL.clone(), self.METHOD_GET_ROW.clone(), e.clone());
            }
            return None;
            }
        }
    }
    

    // synchronized
    pub fn updateWhere(&mut self, mut key: String, mut value: String, mut updatedKeyValuePairs: Option<Rc<RefCell<HashMap>>>) {
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        jcall!(stringBuffer).append(jcall!(self.sqlStrings).UPDATE.clone());
        jcall!(stringBuffer).append(self.tableName.clone());
        jcall!(stringBuffer).append(jcall!(self.sqlStrings).SET.clone());
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut columnArray: Vec<Option<Rc<RefCell<Box<dyn std::any::Any>>>>> = jcall!(jcall!(updatedKeyValuePairs).keySet()).toArray();
            let mut size: i32 = (columnArray.len() as i32).clone();
            for i in (0..size) {
                let mut columnName: String = { let value = jcall!(columnArray[(i) as usize]); value.as_ref().downcast_ref::<String>().cloned().unwrap_or_else(|| format!("{:?}", value.as_ref().type_id())) };
                jcall!(stringBuffer).append(jcall!(self.commonSeps).SPACE.clone());
                jcall!(stringBuffer).append(columnName.clone());
                jcall!(stringBuffer).append(self.EQUAL_QUOTE.clone());
                let mut columnValue: String = (jcall!(updatedKeyValuePairs).get(columnName.clone())).clone() /* as String */;
                if false {
                    columnValue = jcall!(self.stringUtil).EMPTY_STRING.clone();
                } else {
                }
                jcall!(stringBuffer).append(self.getValue(columnValue.clone()));
                jcall!(stringBuffer).append(jcall!(self.sqlStrings).CLOSE_QUOTE.clone());
                if i < size - 1 {
                    jcall!(stringBuffer).append(jcall!(self.commonSeps).COMMA.clone());
                }
            }
            jcall!(stringBuffer).append(jcall!(self.sqlStrings).WHERE.clone());
            jcall!(stringBuffer).append(key.clone());
            jcall!(stringBuffer).append(jcall!(self.sqlStrings).EQUAL_QUOTE.clone());
            jcall!(stringBuffer).append(self.getValue(value.clone()));
            jcall!(stringBuffer).append(jcall!(self.sqlStrings).CLOSE_QUOTE.clone());
            let mut sqlStatement: String = jcall!(stringBuffer).toString();
            self.executeSQLStatement(sqlStatement.clone());
            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).SQLLOGGING.clone()) {
                PreLogUtil::put(format!("{}{}", self.SUCCESS_SQL_STATEMENT, sqlStatement), self.INIT_SQL.clone(), self.METHOD_UPDATE_WHERE.clone());
            }
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).SQLLOGGINGERROR.clone()) {
                PreLogUtil::putOE(format!("{}{}", self.FAILED_SQL_STATEMENT, stringBuffer), self.INIT_SQL.clone(), self.METHOD_UPDATE_WHERE.clone(), e.clone());
            }
            }
        }
    }
    

    pub fn insert(&mut self, mut values: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) {
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        jcall!(stringBuffer).append(jcall!(self.sqlStrings).INSERT_INTO.clone());
        jcall!(stringBuffer).append(self.tableName.clone());
        jcall!(stringBuffer).append(jcall!(self.sqlStrings).VALUES.clone());
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut size: i32 = jcall!(values).size();
            for i in (0..size - 1) {
                let mut value: String = { let java_arguments = ((jcall!(values).get(i.clone())).clone() /* as String */,); self.getValue(java_arguments.0) };
                jcall!(stringBuffer).append(value.clone());
                jcall!(stringBuffer).append(jcall!(self.sqlStrings).SINGLE_QUOTE_COMMA_SEP.clone());
            }
            let mut value: String = { let java_arguments = ((jcall!(values).get(jcall!(values).size() - 1)).clone() /* as String */,); self.getValue(java_arguments.0) };
            jcall!(stringBuffer).append(value.clone());
            jcall!(stringBuffer).append(self.INSERT_END.clone());
            let mut sqlStatement: String = jcall!(stringBuffer).toString();
            self.executeSQLStatement(sqlStatement.clone());
            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).SQLLOGGING.clone()) {
                PreLogUtil::put(format!("{}{}", self.SUCCESS_SQL_STATEMENT, sqlStatement), self.INIT_SQL.clone(), self.INSERT.clone());
            }
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).SQLLOGGINGERROR.clone()) {
                PreLogUtil::putOE(format!("{}{}", self.FAILED_SQL_STATEMENT, jcall!(stringBuffer).toString()), self.INIT_SQL.clone(), self.INSERT.clone(), e.clone());
            }
            }
        }
    }
    

    // synchronized
    pub fn getValue(&mut self, mut value: String) -> String {
        if jcall!(StringValidationUtil::getInstance()).isEmpty(value.clone()) {
            return jcall!(self.stringUtil).EMPTY_STRING.clone();
        } else {
            return value.clone();
        }
    }
    

    // throws Exception, SQLException
    pub fn executeSQLStatement(&mut self, mut statement: String) -> Option<Rc<RefCell<ResultSet>>> {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            if self.conn.is_none() {
                self.initialize();
            }
            let mut stmt: Option<Rc<RefCell<Statement>>> = jcall!(self.conn).createStatement();
            jcall!(stmt).execute(statement.clone());
            let mut rset: Option<Rc<RefCell<ResultSet>>> = jcall!(stmt).getResultSet();
            jcall!(stmt).close();
            return rset.clone();
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            if jcall!(unsafe { crate::org::allbinary::logic::communication::log::config::r#type::LogConfigTypes::LogConfigTypes_LOGGING.clone() }).contains(crate::java_into_object(jcall!(LogConfigTypeFactory::getInstance()).SQLLOGGINGERROR.clone())) {
                PreLogUtil::putOE(jcall!(self.commonStrings).EXCEPTION.clone(), self.INIT_SQL.clone(), String::from("executeSQLStatement()"), e.clone());
            }
            panic!("{:?}", e);
            }
        }
    }
    

    // throws SQLException
    pub fn createConnection(&mut self) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            if self.useridAndPassword == true {
                self.conn = DriverManager::getConnection(jcall!(self.databaseConnectionInfoInterface).getUrl(), self.userid.clone(), self.password.clone());
            } else {
                self.conn = DriverManager::getConnection(jcall!(self.databaseConnectionInfoInterface).getUrl());
            }
        })) {
            Ok(_) => {}
            Err(se) => {
            let se: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(se as Box<dyn std::any::Any>)));

            PreLogUtil::putOE(jcall!(self.commonStrings).EXCEPTION.clone(), self.INIT_SQL.clone(), String::from("createConnection()"), se.clone());
            panic!("{:?}", se);
            }
        }
    }
    

    // throws Exception
    pub fn initialize(&mut self) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                let mut jdbcDriver: String = jcall!(self.databaseConnectionInfoInterface).getJdbcDriver();
                PreLogUtil::put(jcall!(StringMaker::new()).append(String::from("Loading DbConnnectionInfo: ")).append(std::any::type_name_of_val(&self.databaseConnectionInfoInterface).to_string()).append(String::from(" Driver: ")).append(jdbcDriver.clone()).toString(), self.INIT_SQL.clone(), String::from("initialize()"));
                jcall!(Class::Class::forName(jdbcDriver.clone())).newInstance();
            })) {
                Ok(_) => {}
                Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

                if jcall!(unsafe { crate::org::allbinary::logic::communication::log::config::r#type::LogConfigTypes::LogConfigTypes_LOGGING.clone() }).contains(crate::java_into_object(jcall!(LogConfigTypeFactory::getInstance()).SQLLOGGINGERROR.clone())) {
                    PreLogUtil::putOE(format!("{}{}", String::from("LoadDriver Failed: "), jcall!(self.databaseConnectionInfoInterface).getJdbcDriver()), self.INIT_SQL.clone(), String::from("initialize()"), e.clone());
                }
                panic!("{:?}", e);
                }
            }
            if false && false {
                self.useridAndPassword = true;
            }
            self.createConnection();
        })) {
            Ok(_) => {}
            Err(se) => {
            let se: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(se as Box<dyn std::any::Any>)));

            if jcall!(unsafe { crate::org::allbinary::logic::communication::log::config::r#type::LogConfigTypes::LogConfigTypes_LOGGING.clone() }).contains(crate::java_into_object(jcall!(LogConfigTypeFactory::getInstance()).SQLLOGGINGERROR.clone())) {
                PreLogUtil::putOE(jcall!(self.commonStrings).EXCEPTION.clone(), self.INIT_SQL.clone(), String::from("initialize()"), se.clone());
            }
            panic!("{:?}", se);
            }
        }
    }
    

    pub fn getDatabaseConnectionInfoInterface(&mut self) -> Option<Rc<RefCell<dyn DatabaseConnectionInfoInterface>>> {
        return self.databaseConnectionInfoInterface.clone().map(|reference| reference as Rc<RefCell<dyn DatabaseConnectionInfoInterface>>);
    }
}
pub trait InitSqlDyn {
    fn as_InitSql(&self) -> &InitSql;
    fn as_InitSql_mut(&mut self) -> &mut InitSql;
    fn setTable(&mut self, mut tableName: String) { InitSql::setTable(self.as_InitSql_mut(), tableName) }
    fn setDatabaseConnectionInfoInterface(&mut self, mut databaseConnectionInfoInterface: Option<Rc<RefCell<dyn crate::org::allbinary::business::init::db::DbConnectionInfo::DbConnectionInfoDyn>>>) { InitSql::setDatabaseConnectionInfoInterface(self.as_InitSql_mut(), databaseConnectionInfoInterface) }
    fn createTable(&mut self, mut tableData: String) -> bool { InitSql::createTable(self.as_InitSql_mut(), tableData) }
    fn dropTable(&mut self) -> bool { InitSql::dropTable(self.as_InitSql_mut()) }
    fn getRow(&mut self, mut keysAndValues: Option<Rc<RefCell<HashMap>>>) -> Option<Rc<RefCell<HashMap>>> { InitSql::getRow(self.as_InitSql_mut(), keysAndValues) }
    fn updateWhere(&mut self, mut key: String, mut value: String, mut updatedKeyValuePairs: Option<Rc<RefCell<HashMap>>>) { InitSql::updateWhere(self.as_InitSql_mut(), key, value, updatedKeyValuePairs) }
    fn insert(&mut self, mut values: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) { InitSql::insert(self.as_InitSql_mut(), values) }
    fn getValue(&mut self, mut value: String) -> String { InitSql::getValue(self.as_InitSql_mut(), value) }
    fn executeSQLStatement(&mut self, mut statement: String) -> Option<Rc<RefCell<ResultSet>>> { InitSql::executeSQLStatement(self.as_InitSql_mut(), statement) }
    fn createConnection(&mut self) { InitSql::createConnection(self.as_InitSql_mut()) }
    fn initialize(&mut self) { InitSql::initialize(self.as_InitSql_mut()) }
    fn getDatabaseConnectionInfoInterface(&mut self) -> Option<Rc<RefCell<dyn DatabaseConnectionInfoInterface>>> { InitSql::getDatabaseConnectionInfoInterface(self.as_InitSql_mut()) }
}
impl InitSqlDyn for InitSql {
    fn as_InitSql(&self) -> &InitSql { self }
    fn as_InitSql_mut(&mut self) -> &mut InitSql { self }
}


