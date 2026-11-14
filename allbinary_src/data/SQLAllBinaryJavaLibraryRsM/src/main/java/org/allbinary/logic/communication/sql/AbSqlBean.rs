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
use crate::java::util::HashMap::HashMap;
use crate::java::util::Set::Set;
use crate::org::allbinary::business::init::db::DbConnectionInfo::DbConnectionInfo;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::java::lang::Exception::*;
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
        jcall!(stringBuffer).append({ let java_field = jcall!(self.sqlStrings).SELECT.clone(); java_field }.clone());
        jcall!(stringBuffer).append(requestedField.clone());
        jcall!(stringBuffer).append({ let java_field = jcall!(self.sqlStrings).FROM.clone(); java_field }.clone());
        jcall!(stringBuffer).append(self.getTableName());
        jcall!(stringBuffer).append({ let java_field = jcall!(self.sqlStrings).WHERE.clone(); java_field }.clone());
        jcall!(stringBuffer).append(key.clone());
        jcall!(stringBuffer).append({ let java_field = jcall!(self.sqlStrings).EQUAL_QUOTE.clone(); java_field }.clone());
        jcall!(stringBuffer).append(value.clone());
        jcall!(stringBuffer).append({ let java_field = jcall!(self.sqlStrings).CLOSE_QUOTE.clone(); java_field }.clone());
        let mut sqlStatement: String = jcall!(stringBuffer).toString();
        let java_try_result = match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut field: String = { let java_field = jcall!(self.stringUtil).EMPTY_STRING.clone(); java_field }.clone();
            let mut rset: Option<Rc<RefCell<ResultSet>>> = self.executeSQLStatement(sqlStatement.clone());
            while jcall!(rset).next() {
                field = { let java_value = (jcall!(rset).getString(requestedField.clone())).clone(); java_value };
                if jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypes.clone(); java_field }).LOGGING.clone(); java_field }).contains({ let java_field = jcall!(jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypeFactory.clone(); java_field }).getInstance()).SQLLOGGING.clone(); java_field }.clone()) {
                    jcall!(stringBuffer).delete(0, jcall!(stringBuffer).length());
                    jcall!(stringBuffer).append({ let java_field = jcall!(self.sqlStrings).SQL_STATEMENT_LABEL.clone(); java_field }.clone());
                    jcall!(stringBuffer).append(sqlStatement.clone());
                    jcall!(stringBuffer).append({ let java_field = jcall!(self.sqlStrings).FIELD_VALUE.clone(); java_field }.clone());
                    jcall!(stringBuffer).append(field.clone());
                    { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(stringBuffer).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), self.METHOD_GET_FIELD.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
                }
                return Some(field.clone());
            }
            return Some(None);
        })) {
            Ok(java_return) => java_return,
            Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            if jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypes.clone(); java_field }).LOGGING.clone(); java_field }).contains({ let java_field = jcall!(jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypeFactory.clone(); java_field }).getInstance()).SQLLOGGINGERROR.clone(); java_field }.clone()) {
                jcall!(self.logUtil).put(format!("{}{}", self.FAILED_SQL_STATEMENT, sqlStatement), crate::java_into_object(String::from(std::any::type_name::<Self>())), self.METHOD_GET_FIELD.clone(), crate::java_into_object(e.clone()));
            }
            return format!("{}{}", self.FAILED_SQL_STATEMENT, sqlStatement);
            None
            }
        };if let Some(java_return) = java_try_result { return java_return; }unreachable!("try statement")
    }
    

    // Java overloaded method: Rust needs a different name
    pub fn getField(&mut self, mut keysAndValues: Option<Rc<RefCell<HashMap<crate::JavaObject, crate::JavaObject>>>>, mut requestedField: String) -> String {
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        jcall!(stringBuffer).append({ let java_field = jcall!(self.sqlStrings).SELECT.clone(); java_field }.clone());
        jcall!(stringBuffer).append(requestedField.clone());
        jcall!(stringBuffer).append({ let java_field = jcall!(self.sqlStrings).FROM.clone(); java_field }.clone());
        jcall!(stringBuffer).append(self.getTableName());
        jcall!(stringBuffer).append({ let java_field = jcall!(self.sqlStrings).WHERE.clone(); java_field }.clone());
        let java_try_result = match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut field: String = { let java_field = jcall!(self.stringUtil).EMPTY_STRING.clone(); java_field }.clone();
            let mut set: Option<Rc<RefCell<dyn Set>>> = jcall!(keysAndValues).keySet();
            let mut keyArray: Vec<Option<Rc<RefCell<Box<dyn std::any::Any>>>>> = jcall!(set).toArray();
            let mut size: i32 = (keyArray.len() as i32).clone();
            let mut key: String = String::new();
            let mut value: String = String::new();
            for i in (0..size) {
                key = { let java_object = (keyArray[(i) as usize]).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() };
                value = { let java_value = ({ let java_object = (jcall!(keysAndValues).get(crate::java_into_object(key.clone()))).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() }.clone()).clone(); java_value };
                jcall!(stringBuffer).append(key.clone());
                jcall!(stringBuffer).append({ let java_field = jcall!(self.sqlStrings).EQUAL_QUOTE.clone(); java_field }.clone());
                jcall!(stringBuffer).append(value.clone());
                jcall!(stringBuffer).append({ let java_field = jcall!(self.sqlStrings).CLOSE_QUOTE.clone(); java_field }.clone());
                if i < size - 1 {
                    jcall!(stringBuffer).append({ let java_field = jcall!(self.sqlStrings).AND.clone(); java_field }.clone());
                }
            }
            let mut sqlStatement: String = jcall!(stringBuffer).toString();
            let mut rset: Option<Rc<RefCell<ResultSet>>> = self.executeSQLStatement(sqlStatement.clone());
            while jcall!(rset).next() {
                field = { let java_value = (jcall!(rset).getString(requestedField.clone())).clone(); java_value };
                if jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypes.clone(); java_field }).LOGGING.clone(); java_field }).contains({ let java_field = jcall!(jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypeFactory.clone(); java_field }).getInstance()).SQLLOGGING.clone(); java_field }.clone()) {
                    jcall!(stringBuffer).delete(0, jcall!(stringBuffer).length());
                    jcall!(stringBuffer).append({ let java_field = jcall!(self.sqlStrings).SQL_STATEMENT_LABEL.clone(); java_field }.clone());
                    jcall!(stringBuffer).append(sqlStatement.clone());
                    jcall!(stringBuffer).append({ let java_field = jcall!(self.sqlStrings).FIELD_VALUE.clone(); java_field }.clone());
                    jcall!(stringBuffer).append(field.clone());
                    { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(stringBuffer).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), self.METHOD_GET_FIELD.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
                }
                return Some(field.clone());
            }
            return Some(None);
        })) {
            Ok(java_return) => java_return,
            Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            let mut sqlStatement: String = jcall!(stringBuffer).toString();
            if jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypes.clone(); java_field }).LOGGING.clone(); java_field }).contains({ let java_field = jcall!(jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypeFactory.clone(); java_field }).getInstance()).SQLLOGGINGERROR.clone(); java_field }.clone()) {
                jcall!(self.logUtil).put(format!("{}{}", self.FAILED_SQL_STATEMENT, sqlStatement), crate::java_into_object(String::from(std::any::type_name::<Self>())), self.METHOD_GET_FIELD.clone(), crate::java_into_object(e.clone()));
            }
            return format!("{}{}", self.FAILED_SQL_STATEMENT, sqlStatement);
            None
            }
        };if let Some(java_return) = java_try_result { return java_return; }unreachable!("try statement")
    }
    

    pub fn isSubsetOfEntry(&mut self, mut key: String, mut value: String, mut columnsAndValues: Option<Rc<RefCell<HashMap<crate::JavaObject, crate::JavaObject>>>>) -> String {
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        jcall!(stringBuffer).append({ let java_field = jcall!(self.sqlStrings).SELECT.clone(); java_field }.clone());
        let java_try_result = match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut keyArray: Vec<Option<Rc<RefCell<Box<dyn std::any::Any>>>>> = jcall!(jcall!(columnsAndValues).keySet()).toArray();
            let mut size: i32 = (keyArray.len() as i32).clone();
            jcall!(stringBuffer).append({ let java_field = jcall!(self.commonSeps).SPACE.clone(); java_field }.clone());
            jcall!(stringBuffer).append(key.clone());
            for i in (0..size) {
                jcall!(stringBuffer).append({ let java_field = jcall!(self.commonSeps).SPACE.clone(); java_field }.clone());
                jcall!(stringBuffer).append({ let java_object = (keyArray[(i) as usize]).clone(); let value = jcall!(java_object); value.as_ref().downcast_ref::<String>().cloned().unwrap_or_else(|| format!("{:?}", value.as_ref().type_id())) });
            }
            jcall!(stringBuffer).append({ let java_field = jcall!(self.sqlStrings).FROM.clone(); java_field }.clone());
            jcall!(stringBuffer).append(self.getTableName());
            jcall!(stringBuffer).append({ let java_field = jcall!(self.sqlStrings).WHERE.clone(); java_field }.clone());
            jcall!(stringBuffer).append(key.clone());
            jcall!(stringBuffer).append({ let java_field = jcall!(self.sqlStrings).EQUAL_QUOTE.clone(); java_field }.clone());
            jcall!(stringBuffer).append(value.clone());
            jcall!(stringBuffer).append({ let java_field = jcall!(self.sqlStrings).CLOSE_QUOTE.clone(); java_field }.clone());
            let mut sqlStatement: String = jcall!(stringBuffer).toString();
            let mut rset: Option<Rc<RefCell<ResultSet>>> = self.executeSQLStatement(sqlStatement.clone());
            let mut columnName: String = String::new();
            let mut field: String = String::new();
            while jcall!(rset).next() {
                let mut keyArray2: Vec<Option<Rc<RefCell<Box<dyn std::any::Any>>>>> = jcall!(jcall!(columnsAndValues).keySet()).toArray();
                let mut size2: i32 = (keyArray2.len() as i32).clone();
                for i in (0..size2) {
                    columnName = { let java_value = ({ let java_object = (keyArray2[(i) as usize]).clone(); let value = jcall!(java_object); value.as_ref().downcast_ref::<String>().cloned().unwrap_or_else(|| format!("{:?}", value.as_ref().type_id())) }).clone(); java_value };
                    field = { let java_value = (jcall!(jcall!(rset).getObject(columnName.clone())).toString()).clone(); java_value };
                    if (field.cmp(&{ let java_object = (jcall!(columnsAndValues).get(crate::java_into_object(columnName.clone()))).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() }) as i32) != 0 {
                        return Some({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).globals.clone(); java_field }).GLOBALS2.clone(); java_field }).NOTASUBSET.clone(); java_field }.clone());
                    }
                }
            }
            if jcall!(rset).getRow() != 1 {
                return Some({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).globals.clone(); java_field }).GLOBALS2.clone(); java_field }).NOTASUBSET.clone(); java_field }.clone());
            } else {
                return Some({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).globals.clone(); java_field }).GLOBALS2.clone(); java_field }).ISASUBSET.clone(); java_field }.clone());
            }None
        })) {
            Ok(java_return) => java_return,
            Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            let mut sqlStatement: String = jcall!(stringBuffer).toString();
            if jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypes.clone(); java_field }).LOGGING.clone(); java_field }).contains({ let java_field = jcall!(jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypeFactory.clone(); java_field }).getInstance()).SQLLOGGINGERROR.clone(); java_field }.clone()) {
                jcall!(self.logUtil).put(format!("{}{}", self.FAILED_SQL_STATEMENT, sqlStatement), crate::java_into_object(String::from(std::any::type_name::<Self>())), self.METHOD_IS_SUBSET_OF_ENTRY.clone(), crate::java_into_object(e.clone()));
            }
            return format!("{}{}", self.FAILED_SQL_STATEMENT, sqlStatement);
            None
            }
        };if let Some(java_return) = java_try_result { return java_return; }unreachable!("try statement")
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
    fn getField(&mut self, mut keysAndValues: Option<Rc<RefCell<HashMap<crate::JavaObject, crate::JavaObject>>>>, mut requestedField: String) -> String { AbSqlBean::getField(self.as_AbSqlBean_mut(), keysAndValues, requestedField) }
    fn isSubsetOfEntry(&mut self, mut key: String, mut value: String, mut columnsAndValues: Option<Rc<RefCell<HashMap<crate::JavaObject, crate::JavaObject>>>>) -> String { AbSqlBean::isSubsetOfEntry(self.as_AbSqlBean_mut(), key, value, columnsAndValues) }
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


