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
// use crate::java::sql::ResultSet::ResultSet;
// use crate::java::sql::ResultSetMetaData::ResultSetMetaData;
use crate::java::util::HashMap::HashMap;
use crate::java::util::Set::Set;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
use crate::org::allbinary::business::init::db::DbConnectionInfo::DbConnectionInfo;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use crate::org::allbinary::logic::string::StringValidationUtil::StringValidationUtil;
use crate::org::allbinary::logic::string::regex::replace::Replace::Replace;
use crate::java::lang::Exception::*;
use super::AbSqlColumn::AbSqlColumn;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct AbSqlRow {
    pub base: AbSqlColumn,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub EQUAL_QUOTE: String,
    pub ESCAPE_QUOTES: String,
    pub GET_ROW: String,
    pub METHOD_GET_ROW: String,
    pub METHOD_GET_ROWS: String,
    pub METHOD_GET_ALL_ROWS: String,
    pub METHOD_UPDATE_WHERE: String,
    pub METHOD_GET_ROWS_WHERE_BETWEEN: String,
    pub METHOD_DELETE_WHERE: String,
    pub ROW_VALUE_LABEL: String,
    pub ROW_VALUES_LABEL: String,
    pub NO_RESULTS_IN_RESULT_SET: String,
    pub COLUMN_NAME_LABEL: String,
    pub INSERT_END: String,
}


impl AbSqlRow {
    

    pub fn new_struct(mut databaseConnectionInfoInterface: Option<Rc<RefCell<dyn crate::org::allbinary::business::init::db::DbConnectionInfo::DbConnectionInfoDyn>>>) -> Self {
        let mut this = Self {
            base: AbSqlColumn::new_struct(databaseConnectionInfoInterface.clone()),
            logUtil: None,
            EQUAL_QUOTE: String::new(),
            ESCAPE_QUOTES: String::new(),
            GET_ROW: String::new(),
            METHOD_GET_ROW: String::new(),
            METHOD_GET_ROWS: String::new(),
            METHOD_GET_ALL_ROWS: String::new(),
            METHOD_UPDATE_WHERE: String::new(),
            METHOD_GET_ROWS_WHERE_BETWEEN: String::new(),
            METHOD_DELETE_WHERE: String::new(),
            ROW_VALUE_LABEL: String::new(),
            ROW_VALUES_LABEL: String::new(),
            NO_RESULTS_IN_RESULT_SET: String::new(),
            COLUMN_NAME_LABEL: String::new(),
            INSERT_END: String::new(),
        };
        this.logUtil = LogUtil::getInstance();
        this.EQUAL_QUOTE = String::from("=\"");
        this.ESCAPE_QUOTES = String::from("\\\"");
        this.GET_ROW = String::from("Row Value: ");
        this.METHOD_GET_ROW = String::from("getRow()");
        this.METHOD_GET_ROWS = String::from("getRows()");
        this.METHOD_GET_ALL_ROWS = String::from("getAllRows()");
        this.METHOD_UPDATE_WHERE = String::from("updateWhere()");
        this.METHOD_GET_ROWS_WHERE_BETWEEN = String::from("getRowsWhereBetween()");
        this.METHOD_DELETE_WHERE = String::from("deleteWhere()");
        this.ROW_VALUE_LABEL = String::from("Row Value: ");
        this.ROW_VALUES_LABEL = String::from("Row Values: ");
        this.NO_RESULTS_IN_RESULT_SET = String::from("No Results in ResultSet");
        this.COLUMN_NAME_LABEL = String::from("columnName: ");
        this.INSERT_END = String::from("')");
        // super(...) is in the struct literal above
        this
    }

    pub fn new(mut databaseConnectionInfoInterface: Option<Rc<RefCell<dyn crate::org::allbinary::business::init::db::DbConnectionInfo::DbConnectionInfoDyn>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(databaseConnectionInfoInterface))))
    }
    

    // synchronized
    pub fn getValue(&mut self, mut value: String) -> String {
        if jcall!(StringValidationUtil::getInstance()).isEmpty(value.clone()) {
            return { let java_field = jcall!(self.stringUtil).EMPTY_STRING.clone(); java_field }.clone();
        } else {
            return value.clone();
        }
    }
    

    // synchronized
    pub fn updateWhere3(&mut self, mut key: String, mut value: String, mut updatedKeyValuePairs: Option<Rc<RefCell<HashMap<crate::JavaObject, crate::JavaObject>>>>) {
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        jcall!(stringBuffer).append({ let java_field = jcall!(self.sqlStrings).UPDATE.clone(); java_field }.clone());
        jcall!(stringBuffer).append(self.getTableName());
        jcall!(stringBuffer).append({ let java_field = jcall!(self.sqlStrings).SET.clone(); java_field }.clone());
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut keyArray: Vec<Option<Rc<RefCell<Box<dyn std::any::Any>>>>> = jcall!(jcall!(updatedKeyValuePairs).keySet()).toArray();
            let mut size: i32 = (keyArray.len() as i32).clone();
            for i in (0..size) {
                let mut columnName: String = { let java_object = (keyArray[(i) as usize]).clone(); let value = jcall!(java_object); value.as_ref().downcast_ref::<String>().cloned().unwrap_or_else(|| format!("{:?}", value.as_ref().type_id())) };
                jcall!(stringBuffer).append({ let java_field = jcall!(self.commonSeps).SPACE.clone(); java_field }.clone());
                jcall!(stringBuffer).append(columnName.clone());
                jcall!(stringBuffer).append(self.EQUAL_QUOTE.clone());
                let mut columnValue: String = { let java_object = (jcall!(updatedKeyValuePairs).get(crate::java_into_object(columnName.clone()))).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() };
                if false {
                    columnValue = { let java_value = ({ let java_field = jcall!(self.stringUtil).EMPTY_STRING.clone(); java_field }).clone(); java_value }.clone();
                } else {
                    columnValue = { let java_value = (jcall!(Replace::new2({ let java_field = jcall!(self.sqlStrings).CLOSE_QUOTE.clone(); java_field }.clone(), self.ESCAPE_QUOTES.clone())).all(columnValue.clone())).clone(); java_value };
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
                jcall!(self.logUtil).putF(format!("{}{}", self.SUCCESS_SQL_STATEMENT, sqlStatement), crate::java_into_object(String::from(std::any::type_name::<Self>())), self.METHOD_UPDATE_WHERE.clone());
            }
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            if jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypes.clone(); java_field }).LOGGING.clone(); java_field }).contains({ let java_field = jcall!(jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypeFactory.clone(); java_field }).getInstance()).SQLLOGGINGERROR.clone(); java_field }.clone()) {
                jcall!(self.logUtil).put(format!("{}{}", self.FAILED_SQL_STATEMENT, stringBuffer), crate::java_into_object(String::from(std::any::type_name::<Self>())), self.METHOD_UPDATE_WHERE.clone(), crate::java_into_object(e.clone()));
            }
            }
        }
    }
    

    // throws Exception
    // Java overloaded method: Rust needs a different name
    // synchronized
    pub fn updateWhere(&mut self, mut whereKeyValuePairs: Option<Rc<RefCell<HashMap<crate::JavaObject, crate::JavaObject>>>>, mut updatedKeyValuePairs: Option<Rc<RefCell<HashMap<crate::JavaObject, crate::JavaObject>>>>) {
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        jcall!(stringBuffer).append({ let java_field = jcall!(self.sqlStrings).UPDATE.clone(); java_field }.clone());
        jcall!(stringBuffer).append(self.getTableName());
        jcall!(stringBuffer).append({ let java_field = jcall!(self.sqlStrings).SET.clone(); java_field }.clone());
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut keyArray: Vec<Option<Rc<RefCell<Box<dyn std::any::Any>>>>> = jcall!(jcall!(updatedKeyValuePairs).keySet()).toArray();
            let mut size: i32 = (keyArray.len() as i32).clone();
            for i in (0..size) {
                let mut columnName: String = { let java_object = (keyArray[(i) as usize]).clone(); let value = jcall!(java_object); value.as_ref().downcast_ref::<String>().cloned().unwrap_or_else(|| format!("{:?}", value.as_ref().type_id())) };
                jcall!(stringBuffer).append({ let java_field = jcall!(self.commonSeps).SPACE.clone(); java_field }.clone());
                jcall!(stringBuffer).append(columnName.clone());
                jcall!(stringBuffer).append(self.EQUAL_QUOTE.clone());
                if jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypes.clone(); java_field }).LOGGING.clone(); java_field }).contains({ let java_field = jcall!(jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypeFactory.clone(); java_field }).getInstance()).SQLLOGGING.clone(); java_field }.clone()) {
                    jcall!(self.logUtil).putF(format!("{}{}", self.COLUMN_NAME_LABEL, columnName), crate::java_into_object(String::from(std::any::type_name::<Self>())), self.METHOD_UPDATE_WHERE.clone());
                }
                let mut columnValue: String = { let java_object = (jcall!(updatedKeyValuePairs).get(crate::java_into_object(columnName.clone()))).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() };
                if false {
                    columnValue = { let java_value = ({ let java_field = jcall!(self.stringUtil).EMPTY_STRING.clone(); java_field }).clone(); java_value }.clone();
                } else {
                    columnValue = { let java_value = (jcall!(Replace::new2({ let java_field = jcall!(self.sqlStrings).CLOSE_QUOTE.clone(); java_field }.clone(), self.ESCAPE_QUOTES.clone())).all(columnValue.clone())).clone(); java_value };
                }
                jcall!(stringBuffer).append(self.getValue(columnValue.clone()));
                jcall!(stringBuffer).append({ let java_field = jcall!(self.sqlStrings).CLOSE_QUOTE.clone(); java_field }.clone());
                if i < size - 1 {
                    jcall!(stringBuffer).append({ let java_field = jcall!(self.commonSeps).COMMA.clone(); java_field }.clone());
                }
            }
            jcall!(stringBuffer).append({ let java_field = jcall!(self.sqlStrings).WHERE.clone(); java_field }.clone());
            let mut set: Option<Rc<RefCell<dyn Set>>> = jcall!(whereKeyValuePairs).keySet();
            let mut keyArray2: Vec<Option<Rc<RefCell<Box<dyn std::any::Any>>>>> = jcall!(set).toArray();
            let mut size2: i32 = (keyArray2.len() as i32).clone();
            for i in (0..size2) {
                let mut key: String = { let java_object = (keyArray2[(i) as usize]).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() };
                let mut value: String = { let java_object = (jcall!(whereKeyValuePairs).get(crate::java_into_object(key.clone()))).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() };
                {
                    jcall!(stringBuffer).append(key.clone());
                    jcall!(stringBuffer).append({ let java_field = jcall!(self.sqlStrings).EQUAL_QUOTE.clone(); java_field }.clone());
                    jcall!(stringBuffer).append(self.getValue(value.clone()));
                    jcall!(stringBuffer).append({ let java_field = jcall!(self.sqlStrings).CLOSE_QUOTE.clone(); java_field }.clone());
                    if i < size2 - 1 {
                        jcall!(stringBuffer).append({ let java_field = jcall!(self.sqlStrings).AND.clone(); java_field }.clone());
                    }
                }
            }
            let mut sqlStatement: String = jcall!(stringBuffer).toString();
            self.executeSQLStatement(sqlStatement.clone());
            if jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypes.clone(); java_field }).LOGGING.clone(); java_field }).contains({ let java_field = jcall!(jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypeFactory.clone(); java_field }).getInstance()).SQLLOGGING.clone(); java_field }.clone()) {
                jcall!(self.logUtil).putF(format!("{}{}", self.SUCCESS_SQL_STATEMENT, sqlStatement), crate::java_into_object(String::from(std::any::type_name::<Self>())), self.METHOD_UPDATE_WHERE.clone());
            }
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            if jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypes.clone(); java_field }).LOGGING.clone(); java_field }).contains({ let java_field = jcall!(jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypeFactory.clone(); java_field }).getInstance()).SQLLOGGINGERROR.clone(); java_field }.clone()) {
                jcall!(self.logUtil).put(format!("{}{}", self.FAILED_SQL_STATEMENT, stringBuffer), crate::java_into_object(String::from(std::any::type_name::<Self>())), self.METHOD_UPDATE_WHERE.clone(), crate::java_into_object(e.clone()));
            }
            panic!("{:?}", e);
            }
        }
    }
    

    // synchronized
    pub fn deleteWhere2(&mut self, mut key: String, mut value: String) {
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        jcall!(stringBuffer).append({ let java_field = jcall!(self.sqlStrings).DELETE.clone(); java_field }.clone());
        jcall!(stringBuffer).append({ let java_field = jcall!(self.sqlStrings).FROM.clone(); java_field }.clone());
        jcall!(stringBuffer).append(self.getTableName());
        jcall!(stringBuffer).append({ let java_field = jcall!(self.sqlStrings).WHERE.clone(); java_field }.clone());
        jcall!(stringBuffer).append(key.clone());
        jcall!(stringBuffer).append({ let java_field = jcall!(self.sqlStrings).EQUAL_QUOTE.clone(); java_field }.clone());
        jcall!(stringBuffer).append(value.clone());
        jcall!(stringBuffer).append({ let java_field = jcall!(self.sqlStrings).CLOSE_QUOTE.clone(); java_field }.clone());
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut sqlStatement: String = jcall!(stringBuffer).toString();
            self.executeSQLStatement(sqlStatement.clone());
            if jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypes.clone(); java_field }).LOGGING.clone(); java_field }).contains({ let java_field = jcall!(jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypeFactory.clone(); java_field }).getInstance()).SQLLOGGING.clone(); java_field }.clone()) {
                jcall!(self.logUtil).putF(format!("{}{}", self.SUCCESS_SQL_STATEMENT, sqlStatement), crate::java_into_object(String::from(std::any::type_name::<Self>())), self.METHOD_DELETE_WHERE.clone());
            }
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            if jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypes.clone(); java_field }).LOGGING.clone(); java_field }).contains({ let java_field = jcall!(jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypeFactory.clone(); java_field }).getInstance()).SQLLOGGINGERROR.clone(); java_field }.clone()) {
                { let java_receiver = self.logUtil.clone(); let java_arguments = (format!("{}{}", self.FAILED_SQL_STATEMENT, jcall!(stringBuffer).toString()), crate::java_into_object(String::from(std::any::type_name::<Self>())), self.METHOD_DELETE_WHERE.clone(), crate::java_into_object(e.clone()),); jcall!(java_receiver).put(java_arguments.0, java_arguments.1, java_arguments.2, java_arguments.3); };
            }
            }
        }
    }
    

    // Java overloaded method: Rust needs a different name
    // synchronized
    pub fn deleteWhere(&mut self, mut keysAndValues: Option<Rc<RefCell<HashMap<crate::JavaObject, crate::JavaObject>>>>) {
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        jcall!(stringBuffer).append({ let java_field = jcall!(self.sqlStrings).DELETE.clone(); java_field }.clone());
        jcall!(stringBuffer).append({ let java_field = jcall!(self.sqlStrings).FROM.clone(); java_field }.clone());
        jcall!(stringBuffer).append(self.getTableName());
        jcall!(stringBuffer).append({ let java_field = jcall!(self.sqlStrings).WHERE.clone(); java_field }.clone());
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
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
            self.executeSQLStatement(sqlStatement.clone());
            if jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypes.clone(); java_field }).LOGGING.clone(); java_field }).contains({ let java_field = jcall!(jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypeFactory.clone(); java_field }).getInstance()).SQLLOGGING.clone(); java_field }.clone()) {
                jcall!(self.logUtil).putF(format!("{}{}", self.SUCCESS_SQL_STATEMENT, sqlStatement), crate::java_into_object(String::from(std::any::type_name::<Self>())), self.METHOD_DELETE_WHERE.clone());
            }
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            if jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypes.clone(); java_field }).LOGGING.clone(); java_field }).contains({ let java_field = jcall!(jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypeFactory.clone(); java_field }).getInstance()).SQLLOGGINGERROR.clone(); java_field }.clone()) {
                { let java_receiver = self.logUtil.clone(); let java_arguments = (format!("{}{}", self.FAILED_SQL_STATEMENT, jcall!(stringBuffer).toString()), crate::java_into_object(String::from(std::any::type_name::<Self>())), self.METHOD_DELETE_WHERE.clone(), crate::java_into_object(e.clone()),); jcall!(java_receiver).put(java_arguments.0, java_arguments.1, java_arguments.2, java_arguments.3); };
            }
            }
        }
    }
    

    pub fn insert(&mut self, mut values: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) {
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        jcall!(stringBuffer).append({ let java_field = jcall!(self.sqlStrings).INSERT_INTO.clone(); java_field }.clone());
        jcall!(stringBuffer).append(self.getTableName());
        jcall!(stringBuffer).append({ let java_field = jcall!(self.sqlStrings).VALUES.clone(); java_field }.clone());
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            for i in (0..{ let java_bound = jcall!(values).size() - 1; java_bound }) {
                let mut value: String = { let java_arguments = ({ let java_object = (jcall!(values).get(i.clone())).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() },); self.getValue(java_arguments.0) };
                value = { let java_value = (jcall!(Replace::new2({ let java_field = jcall!(self.sqlStrings).ESCAPE.clone(); java_field }.clone(), { let java_field = jcall!(self.sqlStrings).DOUBLE_ESCAPE.clone(); java_field }.clone())).all(value.clone())).clone(); java_value };
                jcall!(stringBuffer).append(value.clone());
                jcall!(stringBuffer).append({ let java_field = jcall!(self.sqlStrings).SINGLE_QUOTE_COMMA_SEP.clone(); java_field }.clone());
            }
            let mut value: String = { let java_arguments = ({ let java_object = (jcall!(values).get(jcall!(values).size() - 1)).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() },); self.getValue(java_arguments.0) };
            value = { let java_value = (jcall!(Replace::new2({ let java_field = jcall!(self.sqlStrings).ESCAPE.clone(); java_field }.clone(), { let java_field = jcall!(self.sqlStrings).DOUBLE_ESCAPE.clone(); java_field }.clone())).all(value.clone())).clone(); java_value };
            jcall!(stringBuffer).append(value.clone());
            jcall!(stringBuffer).append(self.INSERT_END.clone());
            let mut sqlStatement: String = jcall!(stringBuffer).toString();
            self.executeSQLStatement(sqlStatement.clone());
            if jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypes.clone(); java_field }).LOGGING.clone(); java_field }).contains({ let java_field = jcall!(jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypeFactory.clone(); java_field }).getInstance()).SQLLOGGING.clone(); java_field }.clone()) {
                jcall!(self.logUtil).putF(format!("{}{}", self.SUCCESS_SQL_STATEMENT, sqlStatement), crate::java_into_object(String::from(std::any::type_name::<Self>())), self.INSERT.clone());
            }
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            if jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypes.clone(); java_field }).LOGGING.clone(); java_field }).contains({ let java_field = jcall!(jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypeFactory.clone(); java_field }).getInstance()).SQLLOGGINGERROR.clone(); java_field }.clone()) {
                { let java_receiver = self.logUtil.clone(); let java_arguments = (format!("{}{}", self.FAILED_SQL_STATEMENT, jcall!(stringBuffer).toString()), crate::java_into_object(String::from(std::any::type_name::<Self>())), self.base.INSERT.clone(), crate::java_into_object(e.clone()),); jcall!(java_receiver).put(java_arguments.0, java_arguments.1, java_arguments.2, java_arguments.3); };
            }
            }
        }
    }
    

    pub fn getRow(&mut self, mut keysAndValues: Option<Rc<RefCell<HashMap<crate::JavaObject, crate::JavaObject>>>>) -> Option<Rc<RefCell<HashMap<crate::JavaObject, crate::JavaObject>>>> {
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        jcall!(stringBuffer).append({ let java_field = jcall!(self.sqlStrings).SELECT_ALL_FROM.clone(); java_field }.clone());
        jcall!(stringBuffer).append(self.getTableName());
        jcall!(stringBuffer).append({ let java_field = jcall!(self.sqlStrings).WHERE.clone(); java_field }.clone());
        let java_try_result = match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut result: Option<Rc<RefCell<HashMap<crate::JavaObject, crate::JavaObject>>>> = None;
            let mut set: Option<Rc<RefCell<dyn Set>>> = jcall!(keysAndValues).keySet();
            let mut keyArray: Vec<Option<Rc<RefCell<Box<dyn std::any::Any>>>>> = jcall!(set).toArray();
            let mut size: i32 = (keyArray.len() as i32).clone();
            for index in (0..size) {
                let mut key: String = { let java_object = (keyArray[(index) as usize]).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() };
                let mut value: String = { let java_object = (jcall!(keysAndValues).get(crate::java_into_object(key.clone()))).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() }.clone();
                jcall!(stringBuffer).append(key.clone());
                jcall!(stringBuffer).append({ let java_field = jcall!(self.sqlStrings).EQUAL_QUOTE.clone(); java_field }.clone());
                jcall!(stringBuffer).append(self.getValue(value.clone()));
                jcall!(stringBuffer).append({ let java_field = jcall!(self.sqlStrings).CLOSE_QUOTE.clone(); java_field }.clone());
                if index < size - 1 {
                    jcall!(stringBuffer).append({ let java_field = jcall!(self.sqlStrings).AND.clone(); java_field }.clone());
                }
            }
            let mut sqlStatement: String = jcall!(stringBuffer).toString();
            if jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypes.clone(); java_field }).LOGGING.clone(); java_field }).contains({ let java_field = jcall!(jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypeFactory.clone(); java_field }).getInstance()).SQLLOGGING.clone(); java_field }.clone()) {
                jcall!(self.logUtil).putF(format!("{}{}", { let java_field = jcall!(self.sqlStrings).SQL_STATEMENT_LABEL.clone(); java_field }, sqlStatement), crate::java_into_object(String::from(std::any::type_name::<Self>())), self.METHOD_GET_ROW.clone());
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
                    { let java_receiver = self.logUtil.clone(); let java_arguments = (format!("{}{}", self.ROW_VALUE_LABEL, jcall!(result).toString()), crate::java_into_object(String::from(std::any::type_name::<Self>())), self.METHOD_GET_ROW.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
                }
                return Some(result.clone());
            }
            if jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypes.clone(); java_field }).LOGGING.clone(); java_field }).contains({ let java_field = jcall!(jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypeFactory.clone(); java_field }).getInstance()).SQLLOGGINGERROR.clone(); java_field }.clone()) {
                jcall!(self.logUtil).putF(self.NO_RESULTS_IN_RESULT_SET.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), self.METHOD_GET_ROW.clone());
            }
            return Some(None);
        })) {
            Ok(java_return) => java_return,
            Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            if jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypes.clone(); java_field }).LOGGING.clone(); java_field }).contains({ let java_field = jcall!(jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypeFactory.clone(); java_field }).getInstance()).SQLLOGGINGERROR.clone(); java_field }.clone()) {
                jcall!(self.logUtil).put(format!("{}{}", self.FAILED_SQL_STATEMENT, stringBuffer), crate::java_into_object(String::from(std::any::type_name::<Self>())), self.METHOD_GET_ROW.clone(), crate::java_into_object(e.clone()));
            }
            return None;
            None
            }
        };if let Some(java_return) = java_try_result { return java_return; }unreachable!("try statement")
    }
    

    pub fn getRows(&mut self, mut keysAndValues: Option<Rc<RefCell<HashMap<crate::JavaObject, crate::JavaObject>>>>) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        return { let java_arguments = (keysAndValues.clone(), { let java_field = jcall!(StringUtil::getInstance()).EMPTY_STRING.clone(); java_field }.clone(),); self.getRows2(java_arguments.0, java_arguments.1) };
    }
    

    // Java overloaded method: Rust needs a different name
    pub fn getRows2(&mut self, mut keysAndValues: Option<Rc<RefCell<HashMap<crate::JavaObject, crate::JavaObject>>>>, mut more: String) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        jcall!(stringBuffer).append({ let java_field = jcall!(self.sqlStrings).SELECT_ALL_FROM.clone(); java_field }.clone());
        jcall!(stringBuffer).append(self.getTableName());
        jcall!(stringBuffer).append({ let java_field = jcall!(self.sqlStrings).WHERE.clone(); java_field }.clone());
        let java_try_result = match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut rows: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
            let mut set: Option<Rc<RefCell<dyn Set>>> = jcall!(keysAndValues).keySet();
            let mut keyArray: Vec<Option<Rc<RefCell<Box<dyn std::any::Any>>>>> = jcall!(set).toArray();
            let mut size: i32 = (keyArray.len() as i32).clone();
            for index in (0..size) {
                let mut key: String = { let java_object = (keyArray[(index) as usize]).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() };
                let mut value: String = { let java_object = (jcall!(keysAndValues).get(crate::java_into_object(key.clone()))).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() }.clone();
                jcall!(stringBuffer).append(key.clone());
                jcall!(stringBuffer).append({ let java_field = jcall!(self.sqlStrings).EQUAL_QUOTE.clone(); java_field }.clone());
                jcall!(stringBuffer).append(self.getValue(value.clone()));
                jcall!(stringBuffer).append({ let java_field = jcall!(self.sqlStrings).CLOSE_QUOTE.clone(); java_field }.clone());
                if index < size - 1 {
                    jcall!(stringBuffer).append({ let java_field = jcall!(self.sqlStrings).AND.clone(); java_field }.clone());
                }
            }
            if jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypes.clone(); java_field }).LOGGING.clone(); java_field }).contains({ let java_field = jcall!(jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypeFactory.clone(); java_field }).getInstance()).SQLLOGGING.clone(); java_field }.clone()) {
                jcall!(self.logUtil).putF(format!("{}{}", { let java_field = jcall!(self.sqlStrings).SQL_STATEMENT_LABEL.clone(); java_field }, stringBuffer), crate::java_into_object(String::from(std::any::type_name::<Self>())), self.METHOD_GET_ROWS.clone());
            }
            jcall!(stringBuffer).append(more.clone());
            let mut sqlStatement: String = jcall!(stringBuffer).toString();
            let mut rset: Option<Rc<RefCell<ResultSet>>> = self.executeSQLStatement(sqlStatement.clone());
            let mut resultSetMetaData: Option<Rc<RefCell<ResultSetMetaData>>> = jcall!(rset).getMetaData();
            while jcall!(rset).next() {
                let mut result: Option<Rc<RefCell<HashMap<crate::JavaObject, crate::JavaObject>>>> = jcall!(self.stdUtil).createHashMap();
                let mut columnCount: i32 = jcall!(resultSetMetaData).getColumnCount();
                for index in (1..=columnCount) {
                    let mut columnName: String = jcall!(resultSetMetaData).getColumnName(index.clone());
                    let mut field: String = jcall!(rset).getString(columnName.clone());
                    jcall!(result).put(crate::java_into_object(columnName.clone()), crate::java_into_object(field.clone()));
                }
                let mut resultAsString: String = jcall!(result).toString();
                if jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypes.clone(); java_field }).LOGGING.clone(); java_field }).contains({ let java_field = jcall!(jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypeFactory.clone(); java_field }).getInstance()).SQLLOGGING.clone(); java_field }.clone()) {
                    jcall!(self.logUtil).putF(format!("{}{}", self.GET_ROW, resultAsString), crate::java_into_object(String::from(std::any::type_name::<Self>())), self.METHOD_GET_ROWS.clone());
                }
                jcall!(rows).add(crate::java_into_object(result.clone()));
            }
            return Some(rows.clone());
        })) {
            Ok(java_return) => java_return,
            Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            if jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypes.clone(); java_field }).LOGGING.clone(); java_field }).contains({ let java_field = jcall!(jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypeFactory.clone(); java_field }).getInstance()).SQLLOGGINGERROR.clone(); java_field }.clone()) {
                jcall!(self.logUtil).put(format!("{}{}", self.FAILED_SQL_STATEMENT, stringBuffer), crate::java_into_object(String::from(std::any::type_name::<Self>())), self.METHOD_GET_ROWS.clone(), crate::java_into_object(e.clone()));
            }
            return None;
            None
            }
        };if let Some(java_return) = java_try_result { return java_return; }unreachable!("try statement")
    }
    

    pub fn getAllRows(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        jcall!(stringBuffer).append({ let java_field = jcall!(self.sqlStrings).SELECT_ALL_FROM.clone(); java_field }.clone());
        jcall!(stringBuffer).append(self.getTableName());
        let java_try_result = match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            if jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypes.clone(); java_field }).LOGGING.clone(); java_field }).contains({ let java_field = jcall!(jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypeFactory.clone(); java_field }).getInstance()).SQLLOGGING.clone(); java_field }.clone()) {
                jcall!(self.logUtil).putF(format!("{}{}", { let java_field = jcall!(self.sqlStrings).SQL_STATEMENT_LABEL.clone(); java_field }, stringBuffer), crate::java_into_object(String::from(std::any::type_name::<Self>())), self.METHOD_GET_ALL_ROWS.clone());
            }
            let mut rows: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
            let mut sqlStatement: String = jcall!(stringBuffer).toString();
            let mut rset: Option<Rc<RefCell<ResultSet>>> = self.executeSQLStatement(sqlStatement.clone());
            let mut resultSetMetaData: Option<Rc<RefCell<ResultSetMetaData>>> = jcall!(rset).getMetaData();
            while jcall!(rset).next() {
                let mut result: Option<Rc<RefCell<HashMap<crate::JavaObject, crate::JavaObject>>>> = jcall!(self.stdUtil).createHashMap();
                let mut columnCount: i32 = jcall!(resultSetMetaData).getColumnCount();
                for index in (1..=columnCount) {
                    let mut columnName: String = jcall!(resultSetMetaData).getColumnName(index.clone());
                    let mut field: String = jcall!(rset).getString(columnName.clone());
                    jcall!(result).put(crate::java_into_object(columnName.clone()), crate::java_into_object(field.clone()));
                }
                if jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypes.clone(); java_field }).LOGGING.clone(); java_field }).contains({ let java_field = jcall!(jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypeFactory.clone(); java_field }).getInstance()).SQLLOGGING.clone(); java_field }.clone()) {
                    { let java_receiver = self.logUtil.clone(); let java_arguments = (format!("{}{}", self.ROW_VALUES_LABEL, jcall!(result).toString()), crate::java_into_object(String::from(std::any::type_name::<Self>())), self.METHOD_GET_ALL_ROWS.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
                }
                jcall!(rows).add(crate::java_into_object(result.clone()));
            }
            return Some(rows.clone());
        })) {
            Ok(java_return) => java_return,
            Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            if jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypes.clone(); java_field }).LOGGING.clone(); java_field }).contains({ let java_field = jcall!(jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypeFactory.clone(); java_field }).getInstance()).SQLLOGGINGERROR.clone(); java_field }.clone()) {
                jcall!(self.logUtil).put(format!("{}{}", self.FAILED_SQL_STATEMENT, stringBuffer), crate::java_into_object(String::from(std::any::type_name::<Self>())), self.METHOD_GET_ALL_ROWS.clone(), crate::java_into_object(e.clone()));
            }
            return None;
            None
            }
        };if let Some(java_return) = java_try_result { return java_return; }unreachable!("try statement")
    }
    

    pub fn getRowsWhereBetween4(&mut self, mut whereKeyValuePairs: Option<Rc<RefCell<HashMap<crate::JavaObject, crate::JavaObject>>>>, mut betweenColumn: String, mut smallest: String, mut largest: String) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        jcall!(stringBuffer).append({ let java_field = jcall!(self.sqlStrings).SELECT_ALL_FROM.clone(); java_field }.clone());
        jcall!(stringBuffer).append(self.getTableName());
        let java_try_result = match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut rows: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
            jcall!(stringBuffer).append({ let java_field = jcall!(self.sqlStrings).WHERE.clone(); java_field }.clone());
            let mut set: Option<Rc<RefCell<dyn Set>>> = jcall!(whereKeyValuePairs).keySet();
            let mut key: String = String::new();
            let mut value: String = String::new();
            let mut keyArray: Vec<Option<Rc<RefCell<Box<dyn std::any::Any>>>>> = jcall!(set).toArray();
            let mut size: i32 = (keyArray.len() as i32).clone();
            for index in (0..size) {
                key = { let java_object = (keyArray[(index) as usize]).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() };
                value = { let java_value = ({ let java_object = (jcall!(whereKeyValuePairs).get(crate::java_into_object(key.clone()))).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() }).clone(); java_value };
                jcall!(stringBuffer).append(key.clone());
                jcall!(stringBuffer).append({ let java_field = jcall!(self.sqlStrings).EQUAL_QUOTE.clone(); java_field }.clone());
                jcall!(stringBuffer).append(self.getValue(value.clone()));
                jcall!(stringBuffer).append({ let java_field = jcall!(self.sqlStrings).CLOSE_QUOTE.clone(); java_field }.clone());
                jcall!(stringBuffer).append({ let java_field = jcall!(self.sqlStrings).AND.clone(); java_field }.clone());
            }
            jcall!(stringBuffer).append(betweenColumn.clone());
            jcall!(stringBuffer).append({ let java_field = jcall!(self.sqlStrings).MORE_THAN_QUOTE.clone(); java_field }.clone());
            jcall!(stringBuffer).append(smallest.clone());
            jcall!(stringBuffer).append({ let java_field = jcall!(self.sqlStrings).CLOSE_QUOTE.clone(); java_field }.clone());
            jcall!(stringBuffer).append({ let java_field = jcall!(self.sqlStrings).AND.clone(); java_field }.clone());
            jcall!(stringBuffer).append(betweenColumn.clone());
            jcall!(stringBuffer).append({ let java_field = jcall!(self.sqlStrings).LESS_THAN_QUOTE.clone(); java_field }.clone());
            jcall!(stringBuffer).append(largest.clone());
            jcall!(stringBuffer).append({ let java_field = jcall!(self.sqlStrings).CLOSE_QUOTE.clone(); java_field }.clone());
            let mut sqlStatement: String = jcall!(stringBuffer).toString();
            if jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypes.clone(); java_field }).LOGGING.clone(); java_field }).contains({ let java_field = jcall!(jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypeFactory.clone(); java_field }).getInstance()).SQLLOGGING.clone(); java_field }.clone()) {
                jcall!(self.logUtil).putF(format!("{}{}", { let java_field = jcall!(self.sqlStrings).SQL_STATEMENT_LABEL.clone(); java_field }, sqlStatement), crate::java_into_object(String::from(std::any::type_name::<Self>())), self.METHOD_GET_ROWS_WHERE_BETWEEN.clone());
            }
            let mut rset: Option<Rc<RefCell<ResultSet>>> = self.executeSQLStatement(sqlStatement.clone());
            let mut resultSetMetaData: Option<Rc<RefCell<ResultSetMetaData>>> = jcall!(rset).getMetaData();
            let mut result: Option<Rc<RefCell<HashMap<crate::JavaObject, crate::JavaObject>>>> = None;
            let mut columnNames: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = None;
            let mut columnCount: i32 = 0;
            let mut columnName: String = String::new();
            let mut field: String = String::new();
            while jcall!(rset).next() {
                result = { let java_value = (jcall!(self.stdUtil).createHashMap()).clone(); java_value };
                columnNames = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
                columnCount = { let java_value = (jcall!(resultSetMetaData).getColumnCount()).clone(); java_value };
                for index in (1..=columnCount) {
                    columnName = { let java_value = (jcall!(resultSetMetaData).getColumnName(index.clone())).clone(); java_value };
                    field = { let java_value = (jcall!(rset).getString(columnName.clone())).clone(); java_value };
                    jcall!(result).put(crate::java_into_object(columnName.clone()), crate::java_into_object(field.clone()));
                }
                if jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypes.clone(); java_field }).LOGGING.clone(); java_field }).contains({ let java_field = jcall!(jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypeFactory.clone(); java_field }).getInstance()).SQLLOGGING.clone(); java_field }.clone()) {
                    { let java_receiver = self.logUtil.clone(); let java_arguments = (format!("{}{}", self.ROW_VALUE_LABEL, jcall!(result).toString()), crate::java_into_object(String::from(std::any::type_name::<Self>())), self.METHOD_GET_ROWS_WHERE_BETWEEN.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
                }
                jcall!(rows).add(crate::java_into_object(result.clone()));
            }
            return Some(rows.clone());
        })) {
            Ok(java_return) => java_return,
            Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            if jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypes.clone(); java_field }).LOGGING.clone(); java_field }).contains({ let java_field = jcall!(jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypeFactory.clone(); java_field }).getInstance()).SQLLOGGINGERROR.clone(); java_field }.clone()) {
                jcall!(self.logUtil).put(format!("{}{}", self.FAILED_SQL_STATEMENT, stringBuffer), crate::java_into_object(String::from(std::any::type_name::<Self>())), self.METHOD_GET_ROWS_WHERE_BETWEEN.clone(), crate::java_into_object(e.clone()));
            }
            return None;
            None
            }
        };if let Some(java_return) = java_try_result { return java_return; }unreachable!("try statement")
    }
    

    // Java overloaded method: Rust needs a different name
    pub fn getRowsWhereBetween(&mut self, mut betweenColumn: String, mut smallest: String, mut largest: String) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        jcall!(stringBuffer).append({ let java_field = jcall!(self.sqlStrings).SELECT_ALL_FROM.clone(); java_field }.clone());
        jcall!(stringBuffer).append(self.getTableName());
        let java_try_result = match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut rows: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
            jcall!(stringBuffer).append({ let java_field = jcall!(self.sqlStrings).WHERE.clone(); java_field }.clone());
            jcall!(stringBuffer).append(betweenColumn.clone());
            jcall!(stringBuffer).append({ let java_field = jcall!(self.sqlStrings).MORE_THAN_QUOTE.clone(); java_field }.clone());
            jcall!(stringBuffer).append(smallest.clone());
            jcall!(stringBuffer).append({ let java_field = jcall!(self.sqlStrings).CLOSE_QUOTE.clone(); java_field }.clone());
            jcall!(stringBuffer).append({ let java_field = jcall!(self.sqlStrings).AND.clone(); java_field }.clone());
            jcall!(stringBuffer).append(betweenColumn.clone());
            jcall!(stringBuffer).append({ let java_field = jcall!(self.sqlStrings).LESS_THAN_QUOTE.clone(); java_field }.clone());
            jcall!(stringBuffer).append(largest.clone());
            jcall!(stringBuffer).append({ let java_field = jcall!(self.sqlStrings).CLOSE_QUOTE.clone(); java_field }.clone());
            let mut sqlStatement: String = jcall!(stringBuffer).toString();
            if jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypes.clone(); java_field }).LOGGING.clone(); java_field }).contains({ let java_field = jcall!(jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypeFactory.clone(); java_field }).getInstance()).SQLLOGGING.clone(); java_field }.clone()) {
                jcall!(self.logUtil).putF(format!("{}{}", { let java_field = jcall!(self.sqlStrings).SQL_STATEMENT_LABEL.clone(); java_field }, sqlStatement), crate::java_into_object(String::from(std::any::type_name::<Self>())), self.METHOD_GET_ROWS_WHERE_BETWEEN.clone());
            }
            let mut rset: Option<Rc<RefCell<ResultSet>>> = self.executeSQLStatement(sqlStatement.clone());
            let mut resultSetMetaData: Option<Rc<RefCell<ResultSetMetaData>>> = jcall!(rset).getMetaData();
            let mut columnName: String = String::new();
            let mut field: String = String::new();
            let mut result: Option<Rc<RefCell<HashMap<crate::JavaObject, crate::JavaObject>>>> = None;
            let mut columnNames: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = None;
            let mut columnCount: i32 = 0;
            while jcall!(rset).next() {
                result = { let java_value = (jcall!(self.stdUtil).createHashMap()).clone(); java_value };
                columnNames = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
                columnCount = { let java_value = (jcall!(resultSetMetaData).getColumnCount()).clone(); java_value };
                for index in (1..=columnCount) {
                    columnName = { let java_value = (jcall!(resultSetMetaData).getColumnName(index.clone())).clone(); java_value };
                    field = { let java_value = (jcall!(rset).getString(columnName.clone())).clone(); java_value };
                    jcall!(result).put(crate::java_into_object(columnName.clone()), crate::java_into_object(field.clone()));
                }
                if jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypes.clone(); java_field }).LOGGING.clone(); java_field }).contains({ let java_field = jcall!(jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypeFactory.clone(); java_field }).getInstance()).SQLLOGGING.clone(); java_field }.clone()) {
                    { let java_receiver = self.logUtil.clone(); let java_arguments = (format!("{}{}", self.ROW_VALUE_LABEL, jcall!(result).toString()), crate::java_into_object(String::from(std::any::type_name::<Self>())), self.METHOD_GET_ROWS_WHERE_BETWEEN.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
                }
                jcall!(rows).add(crate::java_into_object(result.clone()));
            }
            return Some(rows.clone());
        })) {
            Ok(java_return) => java_return,
            Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            if jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypes.clone(); java_field }).LOGGING.clone(); java_field }).contains({ let java_field = jcall!(jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypeFactory.clone(); java_field }).getInstance()).SQLLOGGINGERROR.clone(); java_field }.clone()) {
                jcall!(self.logUtil).put(format!("{}{}", self.FAILED_SQL_STATEMENT, stringBuffer), crate::java_into_object(String::from(std::any::type_name::<Self>())), self.METHOD_GET_ROWS_WHERE_BETWEEN.clone(), crate::java_into_object(e.clone()));
            }
            return None;
            None
            }
        };if let Some(java_return) = java_try_result { return java_return; }unreachable!("try statement")
    }
}
impl std::ops::Deref for AbSqlRow {
    type Target = AbSqlColumn;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for AbSqlRow {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

pub trait AbSqlRowDyn: crate::org::allbinary::logic::communication::sql::AbSqlColumn::AbSqlColumnDyn {
    fn as_AbSqlRow(&self) -> &AbSqlRow;
    fn as_AbSqlRow_mut(&mut self) -> &mut AbSqlRow;
    fn getValue(&mut self, mut value: String) -> String { AbSqlRow::getValue(self.as_AbSqlRow_mut(), value) }
    fn updateWhere3(&mut self, mut key: String, mut value: String, mut updatedKeyValuePairs: Option<Rc<RefCell<HashMap<crate::JavaObject, crate::JavaObject>>>>) { AbSqlRow::updateWhere3(self.as_AbSqlRow_mut(), key, value, updatedKeyValuePairs) }
    fn updateWhere(&mut self, mut whereKeyValuePairs: Option<Rc<RefCell<HashMap<crate::JavaObject, crate::JavaObject>>>>, mut updatedKeyValuePairs: Option<Rc<RefCell<HashMap<crate::JavaObject, crate::JavaObject>>>>) { AbSqlRow::updateWhere(self.as_AbSqlRow_mut(), whereKeyValuePairs, updatedKeyValuePairs) }
    fn deleteWhere2(&mut self, mut key: String, mut value: String) { AbSqlRow::deleteWhere2(self.as_AbSqlRow_mut(), key, value) }
    fn deleteWhere(&mut self, mut keysAndValues: Option<Rc<RefCell<HashMap<crate::JavaObject, crate::JavaObject>>>>) { AbSqlRow::deleteWhere(self.as_AbSqlRow_mut(), keysAndValues) }
    fn insert(&mut self, mut values: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) { AbSqlRow::insert(self.as_AbSqlRow_mut(), values) }
    fn getRow(&mut self, mut keysAndValues: Option<Rc<RefCell<HashMap<crate::JavaObject, crate::JavaObject>>>>) -> Option<Rc<RefCell<HashMap<crate::JavaObject, crate::JavaObject>>>> { AbSqlRow::getRow(self.as_AbSqlRow_mut(), keysAndValues) }
    fn getRows(&mut self, mut keysAndValues: Option<Rc<RefCell<HashMap<crate::JavaObject, crate::JavaObject>>>>) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> { AbSqlRow::getRows(self.as_AbSqlRow_mut(), keysAndValues) }
    fn getRows2(&mut self, mut keysAndValues: Option<Rc<RefCell<HashMap<crate::JavaObject, crate::JavaObject>>>>, mut more: String) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> { AbSqlRow::getRows2(self.as_AbSqlRow_mut(), keysAndValues, more) }
    fn getAllRows(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> { AbSqlRow::getAllRows(self.as_AbSqlRow_mut()) }
    fn getRowsWhereBetween4(&mut self, mut whereKeyValuePairs: Option<Rc<RefCell<HashMap<crate::JavaObject, crate::JavaObject>>>>, mut betweenColumn: String, mut smallest: String, mut largest: String) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> { AbSqlRow::getRowsWhereBetween4(self.as_AbSqlRow_mut(), whereKeyValuePairs, betweenColumn, smallest, largest) }
    fn getRowsWhereBetween(&mut self, mut betweenColumn: String, mut smallest: String, mut largest: String) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> { AbSqlRow::getRowsWhereBetween(self.as_AbSqlRow_mut(), betweenColumn, smallest, largest) }
}
impl AbSqlRowDyn for AbSqlRow {
    fn as_AbSqlRow(&self) -> &AbSqlRow { self }
    fn as_AbSqlRow_mut(&mut self) -> &mut AbSqlRow { self }
}

impl crate::org::allbinary::logic::communication::sql::AbSqlBasic::AbSqlBasicDyn for AbSqlRow {
    fn as_AbSqlBasic(&self) -> &crate::org::allbinary::logic::communication::sql::AbSqlBasic::AbSqlBasic { &self.base.base.base }
    fn as_AbSqlBasic_mut(&mut self) -> &mut crate::org::allbinary::logic::communication::sql::AbSqlBasic::AbSqlBasic { &mut self.base.base.base }
}
impl crate::org::allbinary::logic::communication::sql::AbSqlColumn::AbSqlColumnDyn for AbSqlRow {
    fn as_AbSqlColumn(&self) -> &crate::org::allbinary::logic::communication::sql::AbSqlColumn::AbSqlColumn { &self.base }
    fn as_AbSqlColumn_mut(&mut self) -> &mut crate::org::allbinary::logic::communication::sql::AbSqlColumn::AbSqlColumn { &mut self.base }
}
impl crate::org::allbinary::logic::communication::sql::AbSqlTable::AbSqlTableDyn for AbSqlRow {
    fn as_AbSqlTable(&self) -> &crate::org::allbinary::logic::communication::sql::AbSqlTable::AbSqlTable { &self.base.base }
    fn as_AbSqlTable_mut(&mut self) -> &mut crate::org::allbinary::logic::communication::sql::AbSqlTable::AbSqlTable { &mut self.base.base }
}


