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
// package org.allbinary.data.tables.user.quoterequest
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;
use super::*;
// use crate::java::util::HashMap::HashMap;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
use crate::org::allbinary::business::entry::EntryData::EntryData;
use crate::org::allbinary::business::init::db::UserDbInitInfo::UserDbInitInfo;
use crate::org::allbinary::business::quoterequest::QuoteRequestData::QuoteRequestData;
use crate::org::allbinary::business::user::UserData::UserData;
use crate::org::allbinary::business::user::quoterequest::QuoteRequest::QuoteRequest;
use crate::org::allbinary::logic::StdUtil::StdUtil;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::communication::sql::AbSqlBean::AbSqlBean;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use super::QuoteRequestEntityInterface::QuoteRequestEntityInterface;
use super::quoterequest::quoterequest;
use super::user::user;
use super::business::business;
use super::allbinary::allbinary;
use super::org::org;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct QuoteRequestEntity {
    pub base: AbSqlBean,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub tableName: String,
}

impl QuoteRequestEntityInterface for QuoteRequestEntity {
    fn get(&mut self, mut userName: String, mut id: i32) -> Option<Rc<RefCell<QuoteRequest>>> { Self::get(self, userName, id) }
    fn getIds(&mut self, mut userName: String) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> { Self::getIds(self, userName) }
    fn deleteWhere(&mut self, mut key: String, mut value: String) { Self::deleteWhere2(self, key, value) }
    fn insert(&mut self, mut values: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) { Self::insert(self, values) }
    fn update(&mut self, mut userName: String, mut updatedValues: Option<Rc<RefCell<HashMap>>>) { Self::update(self, userName, updatedValues) }
}
impl crate::org::allbinary::data::tables::BasicDataTableInterface::BasicDataTableInterface for QuoteRequestEntity {
    fn createTable(&mut self) -> String { Self::createTable(self) }
    fn dropTable(&mut self) -> String { Self::dropTable(self) }
}

impl QuoteRequestEntity {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: AbSqlBean::new_struct(UserDbInitInfo::new()),
            logUtil: None,
            tableName: String::new(),
        };
        this.logUtil = LogUtil::getInstance();
        this.tableName = String::from("quoterequest");
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
    pub fn get(&mut self, mut userName: String, mut id: i32) -> Option<Rc<RefCell<QuoteRequest>>> {
        let mut row: Option<Rc<RefCell<HashMap>>> = jcall!(StdUtil::getInstance()).createHashMap();
        jcall!(row).put(crate::org::allbinary::business::user::UserData::UserData_USERNAME.with(Clone::clone).clone(), userName.clone());
        jcall!(row).put(jcall!(QuoteRequestData::getInstance()).ID.clone(), (id.clone()).to_string());
        let mut quoteRequestHashMap: Option<Rc<RefCell<HashMap>>> = self.base.getRow(row.clone());
        if quoteRequestHashMap.is_some() {
            return QuoteRequest::new1(quoteRequestHashMap.clone());
        } else {
            return None;
        }
    }
    

    pub fn getIds(&mut self, mut userName: String) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        return self.base.getColumnWhere(jcall!(QuoteRequestData::getInstance()).ID.clone(), crate::org::allbinary::business::user::UserData::UserData_USERNAME.with(Clone::clone).clone(), userName.clone());
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
    

    pub fn createTableStatement(&mut self) -> String {
        let mut quoteRequestData: Option<Rc<RefCell<QuoteRequestData>>> = QuoteRequestData::getInstance();
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        jcall!(stringBuffer).append(jcall!(self.sqlStrings).CREATE_TABLE.clone()).append(self.tableName.clone()).append(jcall!(self.sqlStrings).START.clone());
        jcall!(stringBuffer).append(jcall!(quoteRequestData).ID.clone()).append(jcall!(self.sqlTypeStrings).MAX_BIG_INT_UNSIGNED_NOT_NULL.clone());
        jcall!(stringBuffer).append(crate::org::allbinary::business::user::UserData::UserData_USERNAME.with(Clone::clone).clone());
        jcall!(stringBuffer).append(jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone());
        jcall!(stringBuffer).append(jcall!(quoteRequestData).PROJECT_INFO.clone());
        jcall!(stringBuffer).append(jcall!(self.sqlTypeStrings).BLOB_NOT_NULL.clone());
        jcall!(stringBuffer).append(jcall!(quoteRequestData).CUSTOMER_COMMENTS.clone());
        jcall!(stringBuffer).append(jcall!(self.sqlTypeStrings).BLOB_NOT_NULL.clone());
        jcall!(stringBuffer).append(jcall!(quoteRequestData).BUDGET.clone());
        jcall!(stringBuffer).append(jcall!(self.sqlTypeStrings).BLOB_NOT_NULL.clone());
        jcall!(stringBuffer).append(jcall!(quoteRequestData).TIMEFRAME.clone());
        jcall!(stringBuffer).append(jcall!(self.sqlTypeStrings).BLOB_NOT_NULL.clone());
        jcall!(stringBuffer).append(jcall!(quoteRequestData).COMMENTS.clone());
        jcall!(stringBuffer).append(jcall!(self.sqlTypeStrings).BLOB_NOT_NULL.clone());
        jcall!(stringBuffer).append(jcall!(EntryData::getInstance()).TIMECREATED.clone());
        jcall!(stringBuffer).append(jcall!(self.sqlTypeStrings).MAX_BIG_INT_UNSIGNED_NOT_NULL.clone());
        jcall!(stringBuffer).append(jcall!(EntryData::getInstance()).LASTMODIFIED.clone());
        jcall!(stringBuffer).append(jcall!(self.sqlTypeStrings).MAX_BIG_INT_UNSIGNED_NOT_NULL.clone());
        jcall!(stringBuffer).append(jcall!(self.sqlStrings).PRIMARY_KEY.clone());
        jcall!(stringBuffer).append(jcall!(quoteRequestData).ID.clone());
        jcall!(stringBuffer).append(jcall!(self.sqlStrings).END.clone());
        return jcall!(stringBuffer).toString();
    }
    

    pub fn createTable(&mut self) -> String {
        return self.base.createTable(self.createTableStatement());
    }
    

    pub fn dropTable(&mut self) -> String {
        let mut result: String = self.base.dropTable().clone();
        return result.clone();
    }
    

    pub fn update(&mut self, mut userName: String, mut updatedValues: Option<Rc<RefCell<HashMap>>>) {
        self.base.updateWhere3(crate::org::allbinary::business::user::UserData::UserData_USERNAME.with(Clone::clone).clone(), userName.clone(), updatedValues.clone());
    }
}
impl std::ops::Deref for QuoteRequestEntity {
    type Target = AbSqlBean;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for QuoteRequestEntity {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::logic::communication::sql::AbSqlBasic::AbSqlBasicDyn for QuoteRequestEntity {
    fn as_AbSqlBasic(&self) -> &crate::org::allbinary::logic::communication::sql::AbSqlBasic::AbSqlBasic { &self.base.base.base.base.base }
    fn as_AbSqlBasic_mut(&mut self) -> &mut crate::org::allbinary::logic::communication::sql::AbSqlBasic::AbSqlBasic { &mut self.base.base.base.base.base }
}
impl crate::org::allbinary::logic::communication::sql::AbSqlBean::AbSqlBeanDyn for QuoteRequestEntity {
    fn as_AbSqlBean(&self) -> &crate::org::allbinary::logic::communication::sql::AbSqlBean::AbSqlBean { &self.base }
    fn as_AbSqlBean_mut(&mut self) -> &mut crate::org::allbinary::logic::communication::sql::AbSqlBean::AbSqlBean { &mut self.base }
}
impl crate::org::allbinary::logic::communication::sql::AbSqlColumn::AbSqlColumnDyn for QuoteRequestEntity {
    fn as_AbSqlColumn(&self) -> &crate::org::allbinary::logic::communication::sql::AbSqlColumn::AbSqlColumn { &self.base.base.base }
    fn as_AbSqlColumn_mut(&mut self) -> &mut crate::org::allbinary::logic::communication::sql::AbSqlColumn::AbSqlColumn { &mut self.base.base.base }
}
impl crate::org::allbinary::logic::communication::sql::AbSqlRow::AbSqlRowDyn for QuoteRequestEntity {
    fn as_AbSqlRow(&self) -> &crate::org::allbinary::logic::communication::sql::AbSqlRow::AbSqlRow { &self.base.base }
    fn as_AbSqlRow_mut(&mut self) -> &mut crate::org::allbinary::logic::communication::sql::AbSqlRow::AbSqlRow { &mut self.base.base }
    fn deleteWhere2(&mut self, mut key: String, mut value: String) { Self::deleteWhere2(self, key, value) }
    fn insert(&mut self, mut values: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) { Self::insert(self, values) }
}
impl crate::org::allbinary::logic::communication::sql::AbSqlTable::AbSqlTableDyn for QuoteRequestEntity {
    fn as_AbSqlTable(&self) -> &crate::org::allbinary::logic::communication::sql::AbSqlTable::AbSqlTable { &self.base.base.base.base }
    fn as_AbSqlTable_mut(&mut self) -> &mut crate::org::allbinary::logic::communication::sql::AbSqlTable::AbSqlTable { &mut self.base.base.base.base }
    fn dropTable(&mut self) -> String { Self::dropTable(self) }
}


