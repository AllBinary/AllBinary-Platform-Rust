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
use std::collections::HashMap;
use super::*;
// use crate::java::sql::ResultSet::ResultSet;
// use crate::java::util::HashMap::HashMap;
// use crate::java::util::Set::Set;
use crate::org::allbinary::business::init::db::DbConnectionInfo::DbConnectionInfo;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use super::AbSqlRow::AbSqlRow;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct AbSqlBean {
    pub base: AbSqlRow,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub METHOD_GET_FIELD: String,
    pub METHOD_IS_SUBSET_OF_ENTRY: String,
}


impl AbSqlBean {
    

    pub fn new_struct(mut databaseConnectionInfoInterface: Option<Rc<RefCell<dyn crate::org::allbinary::business::init::db::DbConnectionInfo::DbConnectionInfoDyn>>>) -> Self {
        let mut this = Self {
            base: AbSqlRow::new_struct(databaseConnectionInfoInterface.clone()),
            logUtil: None,
            METHOD_GET_FIELD: String::new(),
            METHOD_IS_SUBSET_OF_ENTRY: String::new(),
        };
        this.logUtil = LogUtil::getInstance();
        this.METHOD_GET_FIELD = String::from("getField");
        this.METHOD_IS_SUBSET_OF_ENTRY = String::from("isSubsetOfEntry");
        // super(...) is in the struct literal above
        this
    }

    pub fn new(mut databaseConnectionInfoInterface: Option<Rc<RefCell<dyn crate::org::allbinary::business::init::db::DbConnectionInfo::DbConnectionInfoDyn>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(databaseConnectionInfoInterface))))
    }
    

    pub fn getField3(&mut self, mut key: String, mut value: String, mut requestedField: String) -> String {
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        jcall!(stringBuffer).append(jcall!(self.sqlStrings).SELECT.clone());
        jcall!(stringBuffer).append(requestedField.clone());
        jcall!(stringBuffer).append(jcall!(self.sqlStrings).FROM.clone());
        jcall!(stringBuffer).append(self.getTableName());
        jcall!(stringBuffer).append(jcall!(self.sqlStrings).WHERE.clone());
        jcall!(stringBuffer).append(key.clone());
        jcall!(stringBuffer).append(jcall!(self.sqlStrings).EQUAL_QUOTE.clone());
        jcall!(stringBuffer).append(value.clone());
        jcall!(stringBuffer).append(jcall!(self.sqlStrings).CLOSE_QUOTE.clone());
        let mut sqlStatement: String = jcall!(stringBuffer).toString();
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut field: String = jcall!(self.stringUtil).EMPTY_STRING.clone();
            let mut rset: Option<Rc<RefCell<ResultSet>>> = self.executeSQLStatement(sqlStatement.clone());
            while jcall!(rset).next() {
                field = jcall!(rset).getString(requestedField.clone());
                if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).SQLLOGGING.clone()) {
                    jcall!(stringBuffer).delete(0, jcall!(stringBuffer).length());
                    jcall!(stringBuffer).append(jcall!(self.sqlStrings).SQL_STATEMENT_LABEL.clone());
                    jcall!(stringBuffer).append(sqlStatement.clone());
                    jcall!(stringBuffer).append(jcall!(self.sqlStrings).FIELD_VALUE.clone());
                    jcall!(stringBuffer).append(field.clone());
                    { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(stringBuffer).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), self.METHOD_GET_FIELD.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
                }
                return field.clone();
            }
            return None;
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).SQLLOGGINGERROR.clone()) {
                jcall!(self.logUtil).put(format!("{}{}", self.FAILED_SQL_STATEMENT, sqlStatement), crate::java_into_object(String::from(std::any::type_name::<Self>())), self.METHOD_GET_FIELD.clone(), crate::java_into_object(e.clone()));
            }
            return format!("{}{}", self.FAILED_SQL_STATEMENT, sqlStatement);
            }
        }
    }
    

    // Java overloaded method: Rust needs a different name
    pub fn getField(&mut self, mut keysAndValues: Option<Rc<RefCell<HashMap>>>, mut requestedField: String) -> String {
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        jcall!(stringBuffer).append(jcall!(self.sqlStrings).SELECT.clone());
        jcall!(stringBuffer).append(requestedField.clone());
        jcall!(stringBuffer).append(jcall!(self.sqlStrings).FROM.clone());
        jcall!(stringBuffer).append(self.getTableName());
        jcall!(stringBuffer).append(jcall!(self.sqlStrings).WHERE.clone());
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut field: String = jcall!(self.stringUtil).EMPTY_STRING.clone();
            let mut set: Option<Rc<RefCell<dyn Set>>> = jcall!(keysAndValues).keySet();
            let mut keyArray: Vec<Option<Rc<RefCell<Box<dyn std::any::Any>>>>> = jcall!(set).toArray();
            let mut size: i32 = (keyArray.len() as i32).clone();
            let mut key: String = String::new();
            let mut value: String = String::new();
            for i in (0..size) {
                key = (keyArray[(i) as usize]).clone() /* as String */;
                value = (jcall!(keysAndValues).get(key.clone())).clone() /* as String */.clone();
                jcall!(stringBuffer).append(key.clone());
                jcall!(stringBuffer).append(jcall!(self.sqlStrings).EQUAL_QUOTE.clone());
                jcall!(stringBuffer).append(value.clone());
                jcall!(stringBuffer).append(jcall!(self.sqlStrings).CLOSE_QUOTE.clone());
                if i < size - 1 {
                    jcall!(stringBuffer).append(jcall!(self.sqlStrings).AND.clone());
                }
            }
            let mut sqlStatement: String = jcall!(stringBuffer).toString();
            let mut rset: Option<Rc<RefCell<ResultSet>>> = self.executeSQLStatement(sqlStatement.clone());
            while jcall!(rset).next() {
                field = jcall!(rset).getString(requestedField.clone());
                if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).SQLLOGGING.clone()) {
                    jcall!(stringBuffer).delete(0, jcall!(stringBuffer).length());
                    jcall!(stringBuffer).append(jcall!(self.sqlStrings).SQL_STATEMENT_LABEL.clone());
                    jcall!(stringBuffer).append(sqlStatement.clone());
                    jcall!(stringBuffer).append(jcall!(self.sqlStrings).FIELD_VALUE.clone());
                    jcall!(stringBuffer).append(field.clone());
                    { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(stringBuffer).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), self.METHOD_GET_FIELD.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
                }
                return field.clone();
            }
            return None;
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            let mut sqlStatement: String = jcall!(stringBuffer).toString();
            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).SQLLOGGINGERROR.clone()) {
                jcall!(self.logUtil).put(format!("{}{}", self.FAILED_SQL_STATEMENT, sqlStatement), crate::java_into_object(String::from(std::any::type_name::<Self>())), self.METHOD_GET_FIELD.clone(), crate::java_into_object(e.clone()));
            }
            return format!("{}{}", self.FAILED_SQL_STATEMENT, sqlStatement);
            }
        }
    }
    

    pub fn isSubsetOfEntry(&mut self, mut key: String, mut value: String, mut columnsAndValues: Option<Rc<RefCell<HashMap>>>) -> String {
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        jcall!(stringBuffer).append(jcall!(self.sqlStrings).SELECT.clone());
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut keyArray: Vec<Option<Rc<RefCell<Box<dyn std::any::Any>>>>> = jcall!(jcall!(columnsAndValues).keySet()).toArray();
            let mut size: i32 = (keyArray.len() as i32).clone();
            jcall!(stringBuffer).append(jcall!(self.commonSeps).SPACE.clone());
            jcall!(stringBuffer).append(key.clone());
            for i in (0..size) {
                jcall!(stringBuffer).append(jcall!(self.commonSeps).SPACE.clone());
                jcall!(stringBuffer).append({ let value = jcall!(keyArray[(i) as usize]); value.as_ref().downcast_ref::<String>().cloned().unwrap_or_else(|| format!("{:?}", value.as_ref().type_id())) });
            }
            jcall!(stringBuffer).append(jcall!(self.sqlStrings).FROM.clone());
            jcall!(stringBuffer).append(self.getTableName());
            jcall!(stringBuffer).append(jcall!(self.sqlStrings).WHERE.clone());
            jcall!(stringBuffer).append(key.clone());
            jcall!(stringBuffer).append(jcall!(self.sqlStrings).EQUAL_QUOTE.clone());
            jcall!(stringBuffer).append(value.clone());
            jcall!(stringBuffer).append(jcall!(self.sqlStrings).CLOSE_QUOTE.clone());
            let mut sqlStatement: String = jcall!(stringBuffer).toString();
            let mut rset: Option<Rc<RefCell<ResultSet>>> = self.executeSQLStatement(sqlStatement.clone());
            let mut columnName: String = String::new();
            let mut field: String = String::new();
            while jcall!(rset).next() {
                let mut keyArray2: Vec<Option<Rc<RefCell<Box<dyn std::any::Any>>>>> = jcall!(jcall!(columnsAndValues).keySet()).toArray();
                let mut size2: i32 = (keyArray2.len() as i32).clone();
                for i in (0..size2) {
                    columnName = { let value = jcall!(keyArray2[(i) as usize]); value.as_ref().downcast_ref::<String>().cloned().unwrap_or_else(|| format!("{:?}", value.as_ref().type_id())) };
                    field = jcall!(jcall!(rset).getObject(columnName.clone())).toString();
                    if (field.cmp(&(jcall!(columnsAndValues).get(columnName.clone())).clone() /* as String */) as i32) != 0 {
                        return jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).globals).GLOBALS2).NOTASUBSET.clone();
                    }
                }
            }
            if jcall!(rset).getRow() != 1 {
                return jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).globals).GLOBALS2).NOTASUBSET.clone();
            } else {
                return jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).globals).GLOBALS2).ISASUBSET.clone();
            }
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            let mut sqlStatement: String = jcall!(stringBuffer).toString();
            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).SQLLOGGINGERROR.clone()) {
                jcall!(self.logUtil).put(format!("{}{}", self.FAILED_SQL_STATEMENT, sqlStatement), crate::java_into_object(String::from(std::any::type_name::<Self>())), self.METHOD_IS_SUBSET_OF_ENTRY.clone(), crate::java_into_object(e.clone()));
            }
            return format!("{}{}", self.FAILED_SQL_STATEMENT, sqlStatement);
            }
        }
    }
}
impl std::ops::Deref for AbSqlBean {
    type Target = AbSqlRow;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for AbSqlBean {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

pub trait AbSqlBeanDyn: crate::org::allbinary::logic::communication::sql::AbSqlRow::AbSqlRowDyn {
    fn as_AbSqlBean(&self) -> &AbSqlBean;
    fn as_AbSqlBean_mut(&mut self) -> &mut AbSqlBean;
    fn getField3(&mut self, mut key: String, mut value: String, mut requestedField: String) -> String { AbSqlBean::getField3(self.as_AbSqlBean_mut(), key, value, requestedField) }
    fn getField(&mut self, mut keysAndValues: Option<Rc<RefCell<HashMap>>>, mut requestedField: String) -> String { AbSqlBean::getField(self.as_AbSqlBean_mut(), keysAndValues, requestedField) }
    fn isSubsetOfEntry(&mut self, mut key: String, mut value: String, mut columnsAndValues: Option<Rc<RefCell<HashMap>>>) -> String { AbSqlBean::isSubsetOfEntry(self.as_AbSqlBean_mut(), key, value, columnsAndValues) }
}
impl AbSqlBeanDyn for AbSqlBean {
    fn as_AbSqlBean(&self) -> &AbSqlBean { self }
    fn as_AbSqlBean_mut(&mut self) -> &mut AbSqlBean { self }
}

impl crate::org::allbinary::logic::communication::sql::AbSqlBasic::AbSqlBasicDyn for AbSqlBean {
    fn as_AbSqlBasic(&self) -> &crate::org::allbinary::logic::communication::sql::AbSqlBasic::AbSqlBasic { &self.base.base.base.base }
    fn as_AbSqlBasic_mut(&mut self) -> &mut crate::org::allbinary::logic::communication::sql::AbSqlBasic::AbSqlBasic { &mut self.base.base.base.base }
}
impl crate::org::allbinary::logic::communication::sql::AbSqlColumn::AbSqlColumnDyn for AbSqlBean {
    fn as_AbSqlColumn(&self) -> &crate::org::allbinary::logic::communication::sql::AbSqlColumn::AbSqlColumn { &self.base.base }
    fn as_AbSqlColumn_mut(&mut self) -> &mut crate::org::allbinary::logic::communication::sql::AbSqlColumn::AbSqlColumn { &mut self.base.base }
}
impl crate::org::allbinary::logic::communication::sql::AbSqlRow::AbSqlRowDyn for AbSqlBean {
    fn as_AbSqlRow(&self) -> &crate::org::allbinary::logic::communication::sql::AbSqlRow::AbSqlRow { &self.base }
    fn as_AbSqlRow_mut(&mut self) -> &mut crate::org::allbinary::logic::communication::sql::AbSqlRow::AbSqlRow { &mut self.base }
}
impl crate::org::allbinary::logic::communication::sql::AbSqlTable::AbSqlTableDyn for AbSqlBean {
    fn as_AbSqlTable(&self) -> &crate::org::allbinary::logic::communication::sql::AbSqlTable::AbSqlTable { &self.base.base.base }
    fn as_AbSqlTable_mut(&mut self) -> &mut crate::org::allbinary::logic::communication::sql::AbSqlTable::AbSqlTable { &mut self.base.base.base }
}


