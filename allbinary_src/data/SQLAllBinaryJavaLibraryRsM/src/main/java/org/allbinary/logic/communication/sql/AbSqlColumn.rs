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
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
use crate::org::allbinary::business::init::db::DbConnectionInfo::DbConnectionInfo;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use super::AbSqlTable::AbSqlTable;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct AbSqlColumn {
    pub base: AbSqlTable,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub LARGEST_INT_VALUE_IN_COLUMN: String,
    pub METHOD_GET_LARGETS_INTEGER_IN_COLUMN: String,
    pub METHOD_GET_COLUMN: String,
    pub METHOD_GET_COLUMN_WHERE: String,
    pub INVALID_RESULT: String,
}


impl AbSqlColumn {
    

    pub fn new_struct(mut databaseConnectionInfoInterface: Option<Rc<RefCell<dyn crate::org::allbinary::business::init::db::DbConnectionInfo::DbConnectionInfoDyn>>>) -> Self {
        let mut this = Self {
            base: AbSqlTable::new_struct(databaseConnectionInfoInterface.clone()),
            logUtil: None,
            LARGEST_INT_VALUE_IN_COLUMN: String::new(),
            METHOD_GET_LARGETS_INTEGER_IN_COLUMN: String::new(),
            METHOD_GET_COLUMN: String::new(),
            METHOD_GET_COLUMN_WHERE: String::new(),
            INVALID_RESULT: String::new(),
        };
        this.logUtil = LogUtil::getInstance();
        this.LARGEST_INT_VALUE_IN_COLUMN = String::from("\nLargest Int Value in column: ");
        this.METHOD_GET_LARGETS_INTEGER_IN_COLUMN = String::from("getLargestIntegerInColumn");
        this.METHOD_GET_COLUMN = String::from("getColumn");
        this.METHOD_GET_COLUMN_WHERE = String::from("getColumnWhere");
        this.INVALID_RESULT = String::from("-1");
        // super(...) is in the struct literal above
        this
    }

    pub fn new(mut databaseConnectionInfoInterface: Option<Rc<RefCell<dyn crate::org::allbinary::business::init::db::DbConnectionInfo::DbConnectionInfoDyn>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(databaseConnectionInfoInterface))))
    }
    

    pub fn getLargestIntegerInColumnWhere(&mut self, mut columnName: String, mut key: String, mut value: String) -> String {
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        jcall!(stringBuffer).append(jcall!(self.sqlStrings).SELECT.clone());
        jcall!(stringBuffer).append(columnName.clone());
        jcall!(stringBuffer).append(jcall!(self.sqlStrings).FROM.clone());
        jcall!(stringBuffer).append(self.getTableName());
        jcall!(stringBuffer).append(jcall!(self.sqlStrings).WHERE.clone());
        jcall!(stringBuffer).append(key.clone());
        jcall!(stringBuffer).append(jcall!(self.sqlStrings).EQUAL_QUOTE.clone());
        jcall!(stringBuffer).append(value.clone());
        jcall!(stringBuffer).append(jcall!(self.sqlStrings).CLOSE_QUOTE.clone());
        let mut sqlStatement: String = jcall!(stringBuffer).toString();
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut largest: i32 = 0;
            let mut rset: Option<Rc<RefCell<ResultSet>>> = self.executeSQLStatement(sqlStatement.clone());
            while jcall!(rset).next() {
                let mut intValue: i32 = jcall!(rset).getInt(columnName.clone());
                if intValue > largest {
                    largest = intValue.clone();
                }
            }
            let mut largestAsString: String = jcall!(Some((largest.clone()) as i32)).toString();
            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).SQLLOGGING.clone()) {
                jcall!(stringBuffer).delete(0, jcall!(stringBuffer).length());
                jcall!(stringBuffer).append(jcall!(self.sqlStrings).SQL_STATEMENT_LABEL.clone());
                jcall!(stringBuffer).append(sqlStatement.clone());
                jcall!(stringBuffer).append(self.LARGEST_INT_VALUE_IN_COLUMN.clone());
                jcall!(stringBuffer).append(largestAsString.clone());
                { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(stringBuffer).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), self.METHOD_GET_LARGETS_INTEGER_IN_COLUMN.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
            }
            return largestAsString.clone();
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).SQLLOGGINGERROR.clone()) {
                jcall!(self.logUtil).put(format!("{}{}", self.FAILED_SQL_STATEMENT, sqlStatement), crate::java_into_object(String::from(std::any::type_name::<Self>())), self.METHOD_GET_LARGETS_INTEGER_IN_COLUMN.clone(), crate::java_into_object(e.clone()));
            }
            return self.INVALID_RESULT.clone();
            }
        }
    }
    

    pub fn getColumn(&mut self, mut columnName: String) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        let mut column: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        jcall!(stringBuffer).append(jcall!(self.sqlStrings).SELECT.clone());
        jcall!(stringBuffer).append(columnName.clone());
        jcall!(stringBuffer).append(jcall!(self.sqlStrings).FROM.clone());
        jcall!(stringBuffer).append(self.getTableName());
        let mut sqlStatement: String = jcall!(stringBuffer).toString();
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut rset: Option<Rc<RefCell<ResultSet>>> = self.executeSQLStatement(sqlStatement.clone());
            while jcall!(rset).next() {
                let mut field: String = jcall!(jcall!(rset).getObject(columnName.clone())).toString();
                jcall!(column).add(crate::java_into_object(field.clone()));
            }
            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).SQLLOGGING.clone()) {
                jcall!(stringBuffer).delete(0, jcall!(stringBuffer).length());
                jcall!(stringBuffer).append(jcall!(self.sqlStrings).SQL_STATEMENT_LABEL.clone());
                jcall!(stringBuffer).append(sqlStatement.clone());
                jcall!(stringBuffer).append(jcall!(self.sqlStrings).COLUMN_VALUE.clone());
                jcall!(stringBuffer).append(jcall!(column).toString());
                { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(stringBuffer).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), self.METHOD_GET_COLUMN.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
            }
            return column.clone();
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).SQLLOGGINGERROR.clone()) {
                jcall!(self.logUtil).put(format!("{}{}", self.FAILED_SQL_STATEMENT, sqlStatement), crate::java_into_object(String::from(std::any::type_name::<Self>())), self.METHOD_GET_COLUMN.clone(), crate::java_into_object(e.clone()));
            }
            return column.clone();
            }
        }
    }
    

    pub fn getColumnWhere(&mut self, mut columnName: String, mut key: String, mut value: String) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        jcall!(stringBuffer).append(jcall!(self.sqlStrings).SELECT.clone());
        jcall!(stringBuffer).append(columnName.clone());
        jcall!(stringBuffer).append(jcall!(self.sqlStrings).FROM.clone());
        jcall!(stringBuffer).append(self.getTableName());
        jcall!(stringBuffer).append(jcall!(self.sqlStrings).WHERE.clone());
        jcall!(stringBuffer).append(key.clone());
        jcall!(stringBuffer).append(jcall!(self.sqlStrings).EQUAL_QUOTE.clone());
        jcall!(stringBuffer).append(value.clone());
        jcall!(stringBuffer).append(jcall!(self.sqlStrings).CLOSE_QUOTE.clone());
        let mut sqlStatement: String = jcall!(stringBuffer).toString();
        let mut column: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        let mut rset: Option<Rc<RefCell<ResultSet>>> = None;
        let mut field: String = String::new();
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            rset = self.executeSQLStatement(sqlStatement.clone());
            while jcall!(rset).next() {
                field = jcall!(jcall!(rset).getObject(columnName.clone())).toString();
                jcall!(column).add(crate::java_into_object(field.clone()));
            }
            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).SQLLOGGING.clone()) {
                jcall!(stringBuffer).delete(0, jcall!(stringBuffer).length());
                jcall!(stringBuffer).append(jcall!(self.sqlStrings).SQL_STATEMENT_LABEL.clone());
                jcall!(stringBuffer).append(sqlStatement.clone());
                jcall!(stringBuffer).append(jcall!(self.sqlStrings).COLUMN_VALUE.clone());
                jcall!(stringBuffer).append(jcall!(column).toString());
                { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(stringBuffer).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), self.METHOD_GET_COLUMN_WHERE.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
            }
            return column.clone();
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).SQLLOGGINGERROR.clone()) {
                jcall!(self.logUtil).put(format!("{}{}", self.FAILED_SQL_STATEMENT, sqlStatement), crate::java_into_object(String::from(std::any::type_name::<Self>())), self.METHOD_GET_COLUMN_WHERE.clone(), crate::java_into_object(e.clone()));
            }
            return column.clone();
            }
        }
    }
}
impl std::ops::Deref for AbSqlColumn {
    type Target = AbSqlTable;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for AbSqlColumn {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

pub trait AbSqlColumnDyn: crate::org::allbinary::logic::communication::sql::AbSqlTable::AbSqlTableDyn {
    fn as_AbSqlColumn(&self) -> &AbSqlColumn;
    fn as_AbSqlColumn_mut(&mut self) -> &mut AbSqlColumn;
    fn getLargestIntegerInColumnWhere(&mut self, mut columnName: String, mut key: String, mut value: String) -> String { AbSqlColumn::getLargestIntegerInColumnWhere(self.as_AbSqlColumn_mut(), columnName, key, value) }
    fn getColumn(&mut self, mut columnName: String) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> { AbSqlColumn::getColumn(self.as_AbSqlColumn_mut(), columnName) }
    fn getColumnWhere(&mut self, mut columnName: String, mut key: String, mut value: String) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> { AbSqlColumn::getColumnWhere(self.as_AbSqlColumn_mut(), columnName, key, value) }
}
impl AbSqlColumnDyn for AbSqlColumn {
    fn as_AbSqlColumn(&self) -> &AbSqlColumn { self }
    fn as_AbSqlColumn_mut(&mut self) -> &mut AbSqlColumn { self }
}

impl crate::org::allbinary::logic::communication::sql::AbSqlBasic::AbSqlBasicDyn for AbSqlColumn {
    fn as_AbSqlBasic(&self) -> &crate::org::allbinary::logic::communication::sql::AbSqlBasic::AbSqlBasic { &self.base.base }
    fn as_AbSqlBasic_mut(&mut self) -> &mut crate::org::allbinary::logic::communication::sql::AbSqlBasic::AbSqlBasic { &mut self.base.base }
}
impl crate::org::allbinary::logic::communication::sql::AbSqlTable::AbSqlTableDyn for AbSqlColumn {
    fn as_AbSqlTable(&self) -> &crate::org::allbinary::logic::communication::sql::AbSqlTable::AbSqlTable { &self.base }
    fn as_AbSqlTable_mut(&mut self) -> &mut crate::org::allbinary::logic::communication::sql::AbSqlTable::AbSqlTable { &mut self.base }
}


