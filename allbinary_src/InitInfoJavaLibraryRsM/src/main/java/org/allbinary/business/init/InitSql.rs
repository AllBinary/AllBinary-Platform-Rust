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
use super::*;
// use crate::java::sql::Connection::Connection;
// use crate::java::sql::DriverManager::DriverManager;
// use crate::java::sql::ResultSet::ResultSet;
// use crate::java::sql::ResultSetMetaData::ResultSetMetaData;
// use crate::java::sql::SQLException::SQLException;
// use crate::java::sql::Statement::Statement;
use crate::java::util::HashMap::HashMap;
use crate::java::util::Set::Set;
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
use crate::java::lang::Exception::*;

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

    pub fn toString(&mut self) -> String {
        String::from(std::any::type_name::<Self>())
    }
    

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
        let java_try_result = match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
            jcall!(stringBuffer).append(String::from("Creating Table: "));
            jcall!(stringBuffer).append(tableData.clone());
            PreLogUtil::put(jcall!(stringBuffer).toString(), crate::java_into_object(self.INIT_SQL.clone()), String::from("createTable()"));
            self.executeSQLStatement(tableData.clone());
            return Some(true);
        })) {
            Ok(java_return) => java_return,
            Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            if jcall!(unsafe { crate::org::allbinary::logic::communication::log::config::r#type::LogConfigTypes::LogConfigTypes_LOGGING.clone() }).contains(crate::java_into_object({ let java_field = jcall!(LogConfigTypeFactory::getInstance()).SQLLOGGINGERROR.clone(); java_field }.clone())) {
                PreLogUtil::putOE({ let java_field = jcall!(self.commonStrings).EXCEPTION.clone(); java_field }.clone(), crate::java_into_object(self.INIT_SQL.clone()), String::from("createTable()"), crate::java_into_object(e.clone()));
            }
            return false;
            None
            }
        };if let Some(java_return) = java_try_result { return java_return; }unreachable!("try statement")
    }
    

    pub fn dropTable(&mut self) -> bool {
        let mut sqlStatement: String = format!("{}{}", { let java_field = jcall!(self.sqlStrings).DROP_TABLE.clone(); java_field }, self.tableName);
        let java_try_result = match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            self.executeSQLStatement(sqlStatement.clone());
            return Some(true);
        })) {
            Ok(java_return) => java_return,
            Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            if jcall!(unsafe { crate::org::allbinary::logic::communication::log::config::r#type::LogConfigTypes::LogConfigTypes_LOGGING.clone() }).contains(crate::java_into_object({ let java_field = jcall!(LogConfigTypeFactory::getInstance()).SQLLOGGINGERROR.clone(); java_field }.clone())) {
                PreLogUtil::putOE({ let java_field = jcall!(self.commonStrings).EXCEPTION.clone(); java_field }.clone(), crate::java_into_object(self.INIT_SQL.clone()), String::from("dropTables()"), crate::java_into_object(e.clone()));
            }
            return false;
            None
            }
        };if let Some(java_return) = java_try_result { return java_return; }unreachable!("try statement")
    }
    

    pub fn getRow(&mut self, mut keysAndValues: Option<Rc<RefCell<HashMap<crate::JavaObject, crate::JavaObject>>>>) -> Option<Rc<RefCell<HashMap<crate::JavaObject, crate::JavaObject>>>> {
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        jcall!(stringBuffer).append({ let java_field = jcall!(self.sqlStrings).SELECT_ALL_FROM.clone(); java_field }.clone());
        jcall!(stringBuffer).append(self.tableName.clone());
        jcall!(stringBuffer).append({ let java_field = jcall!(self.sqlStrings).WHERE.clone(); java_field }.clone());
        let java_try_result = match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut result: Option<Rc<RefCell<HashMap<crate::JavaObject, crate::JavaObject>>>> = None;
            let mut set: Option<Rc<RefCell<dyn Set>>> = jcall!(keysAndValues).keySet();
            let mut keyArray: Vec<Option<Rc<RefCell<Box<dyn std::any::Any>>>>> = jcall!(set).toArray();
            let mut size: i32 = (keyArray.len() as i32).clone();
            for i in (0..size) {
                let mut key: String = { let java_object = (keyArray[(i) as usize]).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() };
                let mut value: String = { let java_object = (jcall!(keysAndValues).get(crate::java_into_object(key.clone()))).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() }.clone();
                jcall!(stringBuffer).append(key.clone());
                jcall!(stringBuffer).append({ let java_field = jcall!(self.sqlStrings).EQUAL_QUOTE.clone(); java_field }.clone());
                jcall!(stringBuffer).append(self.getValue(value.clone()));
                jcall!(stringBuffer).append({ let java_field = jcall!(self.sqlStrings).CLOSE_QUOTE.clone(); java_field }.clone());
                if i < size - 1 {
                    jcall!(stringBuffer).append({ let java_field = jcall!(self.sqlStrings).AND.clone(); java_field }.clone());
                }
            }
            let mut sqlStatement: String = jcall!(stringBuffer).toString();
            if jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypes.clone(); java_field }).LOGGING.clone(); java_field }).contains({ let java_field = jcall!(jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypeFactory.clone(); java_field }).getInstance()).SQLLOGGING.clone(); java_field }.clone()) {
                PreLogUtil::put(format!("{}{}", { let java_field = jcall!(self.sqlStrings).SQL_STATEMENT_LABEL.clone(); java_field }, sqlStatement), crate::java_into_object(self.INIT_SQL.clone()), self.METHOD_GET_ROW.clone());
            }
            let mut rset: Option<Rc<RefCell<ResultSet>>> = self.executeSQLStatement(sqlStatement.clone());
            let mut resultSetMetaData: Option<Rc<RefCell<ResultSetMetaData>>> = jcall!(rset).getMetaData();
            while jcall!(rset).next() {
                result = { let java_value = (jcall!(self.stdUtil).createHashMap()).clone(); java_value };
                let mut columnCount: i32 = jcall!(resultSetMetaData).getColumnCount();
                for index in (1..=columnCount) {
                    let mut columnName: String = jcall!(resultSetMetaData).getColumnName(index.clone());
                    let mut field: String = jcall!(rset).getString(columnName.clone());
                    jcall!(result).put(crate::java_into_object(columnName.clone()), crate::java_into_object(field.clone()));
                }
                if jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypes.clone(); java_field }).LOGGING.clone(); java_field }).contains({ let java_field = jcall!(jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypeFactory.clone(); java_field }).getInstance()).SQLLOGGING.clone(); java_field }.clone()) {
                    PreLogUtil::put(format!("{}{}", self.ROW_VALUE_LABEL, jcall!(result).toString()), crate::java_into_object(self.INIT_SQL.clone()), self.METHOD_GET_ROW.clone());
                }
                return Some(result.clone());
            }
            if jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypes.clone(); java_field }).LOGGING.clone(); java_field }).contains({ let java_field = jcall!(jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypeFactory.clone(); java_field }).getInstance()).SQLLOGGINGERROR.clone(); java_field }.clone()) {
                PreLogUtil::put(self.NO_RESULTS_IN_RESULT_SET.clone(), crate::java_into_object(self.INIT_SQL.clone()), self.METHOD_GET_ROW.clone());
            }
            return Some(None);
        })) {
            Ok(java_return) => java_return,
            Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            if jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypes.clone(); java_field }).LOGGING.clone(); java_field }).contains({ let java_field = jcall!(jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypeFactory.clone(); java_field }).getInstance()).SQLLOGGINGERROR.clone(); java_field }.clone()) {
                PreLogUtil::putOE(format!("{}{}", self.FAILED_SQL_STATEMENT, stringBuffer), crate::java_into_object(self.INIT_SQL.clone()), self.METHOD_GET_ROW.clone(), crate::java_into_object(e.clone()));
            }
            return None;
            None
            }
        };if let Some(java_return) = java_try_result { return java_return; }unreachable!("try statement")
    }
    

    // synchronized
    pub fn updateWhere(&mut self, mut key: String, mut value: String, mut updatedKeyValuePairs: Option<Rc<RefCell<HashMap<crate::JavaObject, crate::JavaObject>>>>) {
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        jcall!(stringBuffer).append({ let java_field = jcall!(self.sqlStrings).UPDATE.clone(); java_field }.clone());
        jcall!(stringBuffer).append(self.tableName.clone());
        jcall!(stringBuffer).append({ let java_field = jcall!(self.sqlStrings).SET.clone(); java_field }.clone());
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut columnArray: Vec<Option<Rc<RefCell<Box<dyn std::any::Any>>>>> = jcall!(jcall!(updatedKeyValuePairs).keySet()).toArray();
            let mut size: i32 = (columnArray.len() as i32).clone();
            for i in (0..size) {
                let mut columnName: String = { let java_object = (columnArray[(i) as usize]).clone(); let value = jcall!(java_object); value.as_ref().downcast_ref::<String>().cloned().unwrap_or_else(|| format!("{:?}", value.as_ref().type_id())) };
                jcall!(stringBuffer).append({ let java_field = jcall!(self.commonSeps).SPACE.clone(); java_field }.clone());
                jcall!(stringBuffer).append(columnName.clone());
                jcall!(stringBuffer).append(self.EQUAL_QUOTE.clone());
                let mut columnValue: String = { let java_object = (jcall!(updatedKeyValuePairs).get(crate::java_into_object(columnName.clone()))).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() };
                if false {
                    columnValue = { let java_value = ({ let java_field = jcall!(self.stringUtil).EMPTY_STRING.clone(); java_field }).clone(); java_value }.clone();
                } else {
                }
                jcall!(stringBuffer).append(self.getValue(columnValue.clone()));
                jcall!(stringBuffer).append({ let java_field = jcall!(self.sqlStrings).CLOSE_QUOTE.clone(); java_field }.clone());
                if i < size - 1 {
                    jcall!(stringBuffer).append({ let java_field = jcall!(self.commonSeps).COMMA.clone(); java_field }.clone());
                }
            }
            jcall!(stringBuffer).append({ let java_field = jcall!(self.sqlStrings).WHERE.clone(); java_field }.clone());
            jcall!(stringBuffer).append(key.clone());
            jcall!(stringBuffer).append({ let java_field = jcall!(self.sqlStrings).EQUAL_QUOTE.clone(); java_field }.clone());
            jcall!(stringBuffer).append(self.getValue(value.clone()));
            jcall!(stringBuffer).append({ let java_field = jcall!(self.sqlStrings).CLOSE_QUOTE.clone(); java_field }.clone());
            let mut sqlStatement: String = jcall!(stringBuffer).toString();
            self.executeSQLStatement(sqlStatement.clone());
            if jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypes.clone(); java_field }).LOGGING.clone(); java_field }).contains({ let java_field = jcall!(jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypeFactory.clone(); java_field }).getInstance()).SQLLOGGING.clone(); java_field }.clone()) {
                PreLogUtil::put(format!("{}{}", self.SUCCESS_SQL_STATEMENT, sqlStatement), crate::java_into_object(self.INIT_SQL.clone()), self.METHOD_UPDATE_WHERE.clone());
            }
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            if jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypes.clone(); java_field }).LOGGING.clone(); java_field }).contains({ let java_field = jcall!(jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypeFactory.clone(); java_field }).getInstance()).SQLLOGGINGERROR.clone(); java_field }.clone()) {
                PreLogUtil::putOE(format!("{}{}", self.FAILED_SQL_STATEMENT, stringBuffer), crate::java_into_object(self.INIT_SQL.clone()), self.METHOD_UPDATE_WHERE.clone(), crate::java_into_object(e.clone()));
            }
            }
        }
    }
    

    pub fn insert(&mut self, mut values: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) {
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        jcall!(stringBuffer).append({ let java_field = jcall!(self.sqlStrings).INSERT_INTO.clone(); java_field }.clone());
        jcall!(stringBuffer).append(self.tableName.clone());
        jcall!(stringBuffer).append({ let java_field = jcall!(self.sqlStrings).VALUES.clone(); java_field }.clone());
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut size: i32 = jcall!(values).size();
            for i in (0..size - 1) {
                let mut value: String = { let java_arguments = ({ let java_object = (jcall!(values).get(i.clone())).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() },); self.getValue(java_arguments.0) };
                jcall!(stringBuffer).append(value.clone());
                jcall!(stringBuffer).append({ let java_field = jcall!(self.sqlStrings).SINGLE_QUOTE_COMMA_SEP.clone(); java_field }.clone());
            }
            let mut value: String = { let java_arguments = ({ let java_object = (jcall!(values).get(jcall!(values).size() - 1)).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() },); self.getValue(java_arguments.0) };
            jcall!(stringBuffer).append(value.clone());
            jcall!(stringBuffer).append(self.INSERT_END.clone());
            let mut sqlStatement: String = jcall!(stringBuffer).toString();
            self.executeSQLStatement(sqlStatement.clone());
            if jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypes.clone(); java_field }).LOGGING.clone(); java_field }).contains({ let java_field = jcall!(jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypeFactory.clone(); java_field }).getInstance()).SQLLOGGING.clone(); java_field }.clone()) {
                PreLogUtil::put(format!("{}{}", self.SUCCESS_SQL_STATEMENT, sqlStatement), crate::java_into_object(self.INIT_SQL.clone()), self.INSERT.clone());
            }
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            if jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypes.clone(); java_field }).LOGGING.clone(); java_field }).contains({ let java_field = jcall!(jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypeFactory.clone(); java_field }).getInstance()).SQLLOGGINGERROR.clone(); java_field }.clone()) {
                PreLogUtil::putOE(format!("{}{}", self.FAILED_SQL_STATEMENT, jcall!(stringBuffer).toString()), crate::java_into_object(self.INIT_SQL.clone()), self.INSERT.clone(), crate::java_into_object(e.clone()));
            }
            }
        }
    }
    

    // synchronized
    pub fn getValue(&mut self, mut value: String) -> String {
        if jcall!(StringValidationUtil::getInstance()).isEmpty(value.clone()) {
            return { let java_field = jcall!(self.stringUtil).EMPTY_STRING.clone(); java_field }.clone();
        } else {
            return value.clone();
        }
    }
    

    // throws Exception, SQLException
    pub fn executeSQLStatement(&mut self, mut statement: String) -> Option<Rc<RefCell<ResultSet>>> {
        let java_try_result = match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            if self.conn.is_none() {
                self.initialize();
            }
            let mut stmt: Option<Rc<RefCell<Statement>>> = jcall!(self.conn).createStatement();
            jcall!(stmt).execute(statement.clone());
            let mut rset: Option<Rc<RefCell<ResultSet>>> = jcall!(stmt).getResultSet();
            jcall!(stmt).close();
            return Some(rset.clone());
        })) {
            Ok(java_return) => java_return,
            Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            if jcall!(unsafe { crate::org::allbinary::logic::communication::log::config::r#type::LogConfigTypes::LogConfigTypes_LOGGING.clone() }).contains(crate::java_into_object({ let java_field = jcall!(LogConfigTypeFactory::getInstance()).SQLLOGGINGERROR.clone(); java_field }.clone())) {
                PreLogUtil::putOE({ let java_field = jcall!(self.commonStrings).EXCEPTION.clone(); java_field }.clone(), crate::java_into_object(self.INIT_SQL.clone()), String::from("executeSQLStatement()"), crate::java_into_object(e.clone()));
            }
            panic!("{:?}", e);
            None
            }
        };if let Some(java_return) = java_try_result { return java_return; }unreachable!("try statement")
    }
    

    // throws SQLException
    pub fn createConnection(&mut self) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            if self.useridAndPassword == true {
                self.conn = { let java_value = (DriverManager::getConnection(jcall!(self.databaseConnectionInfoInterface).as_DbConnectionInfo_mut().getUrl(), self.userid.clone(), self.password.clone())).clone(); java_value };
            } else {
                self.conn = { let java_value = (DriverManager::getConnection(jcall!(self.databaseConnectionInfoInterface).as_DbConnectionInfo_mut().getUrl())).clone(); java_value };
            }
        })) {
            Ok(_) => {}
            Err(se) => {
            let se: String = crate::java::lang::Exception::java_panic_message(&se);

            PreLogUtil::putOE({ let java_field = jcall!(self.commonStrings).EXCEPTION.clone(); java_field }.clone(), crate::java_into_object(self.INIT_SQL.clone()), String::from("createConnection()"), crate::java_into_object(se.clone()));
            panic!("{:?}", se);
            }
        }
    }
    

    // throws Exception
    pub fn initialize(&mut self) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                let mut jdbcDriver: String = jcall!(self.databaseConnectionInfoInterface).as_DbConnectionInfo_mut().getJdbcDriver();
                PreLogUtil::put(jcall!(StringMaker::new()).append(String::from("Loading DbConnnectionInfo: ")).append(std::any::type_name_of_val(&self.databaseConnectionInfoInterface).to_string()).append(String::from(" Driver: ")).append(jdbcDriver.clone()).toString(), crate::java_into_object(self.INIT_SQL.clone()), String::from("initialize()"));
                jcall!(Class::Class::forName(jdbcDriver.clone())).newInstance();
            })) {
                Ok(_) => {}
                Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

                if jcall!(unsafe { crate::org::allbinary::logic::communication::log::config::r#type::LogConfigTypes::LogConfigTypes_LOGGING.clone() }).contains(crate::java_into_object({ let java_field = jcall!(LogConfigTypeFactory::getInstance()).SQLLOGGINGERROR.clone(); java_field }.clone())) {
                    PreLogUtil::putOE(format!("{}{}", String::from("LoadDriver Failed: "), jcall!(self.databaseConnectionInfoInterface).as_DbConnectionInfo_mut().getJdbcDriver()), crate::java_into_object(self.INIT_SQL.clone()), String::from("initialize()"), crate::java_into_object(e.clone()));
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
            let se: String = crate::java::lang::Exception::java_panic_message(&se);

            if jcall!(unsafe { crate::org::allbinary::logic::communication::log::config::r#type::LogConfigTypes::LogConfigTypes_LOGGING.clone() }).contains(crate::java_into_object({ let java_field = jcall!(LogConfigTypeFactory::getInstance()).SQLLOGGINGERROR.clone(); java_field }.clone())) {
                PreLogUtil::putOE({ let java_field = jcall!(self.commonStrings).EXCEPTION.clone(); java_field }.clone(), crate::java_into_object(self.INIT_SQL.clone()), String::from("initialize()"), crate::java_into_object(se.clone()));
            }
            panic!("{:?}", se);
            }
        }
    }
    

    pub fn getDatabaseConnectionInfoInterface(&mut self) -> Option<Rc<RefCell<dyn DatabaseConnectionInfoInterface>>> {
        return self.databaseConnectionInfoInterface.clone().map(|reference| reference as Rc<RefCell<dyn DatabaseConnectionInfoInterface>>);
    }
}
pub trait InitSqlDyn: std::any::Any {
    fn as_InitSql(&self) -> &InitSql;
    fn as_InitSql_mut(&mut self) -> &mut InitSql;
    fn setTable(&mut self, mut tableName: String) { InitSql::setTable(self.as_InitSql_mut(), tableName) }
    fn setDatabaseConnectionInfoInterface(&mut self, mut databaseConnectionInfoInterface: Option<Rc<RefCell<dyn crate::org::allbinary::business::init::db::DbConnectionInfo::DbConnectionInfoDyn>>>) { InitSql::setDatabaseConnectionInfoInterface(self.as_InitSql_mut(), databaseConnectionInfoInterface) }
    fn createTable(&mut self, mut tableData: String) -> bool { InitSql::createTable(self.as_InitSql_mut(), tableData) }
    fn dropTable(&mut self) -> bool { InitSql::dropTable(self.as_InitSql_mut()) }
    fn getRow(&mut self, mut keysAndValues: Option<Rc<RefCell<HashMap<crate::JavaObject, crate::JavaObject>>>>) -> Option<Rc<RefCell<HashMap<crate::JavaObject, crate::JavaObject>>>> { InitSql::getRow(self.as_InitSql_mut(), keysAndValues) }
    fn updateWhere(&mut self, mut key: String, mut value: String, mut updatedKeyValuePairs: Option<Rc<RefCell<HashMap<crate::JavaObject, crate::JavaObject>>>>) { InitSql::updateWhere(self.as_InitSql_mut(), key, value, updatedKeyValuePairs) }
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


