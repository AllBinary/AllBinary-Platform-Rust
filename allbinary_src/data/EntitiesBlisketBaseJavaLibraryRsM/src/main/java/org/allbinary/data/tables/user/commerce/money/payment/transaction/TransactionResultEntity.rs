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
// package org.allbinary.data.tables.user.commerce.money.payment.transaction
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use crate::Calendar;
use std::collections::HashMap;
use super::*;
// use crate::java::util::Calendar::Calendar;
// use crate::java::util::HashMap::HashMap;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
use crate::org::allbinary::business::entry::EntryData::EntryData;
use crate::org::allbinary::business::init::db::UserDbInitInfo::UserDbInitInfo;
use crate::org::allbinary::business::user::UserData::UserData;
use crate::org::allbinary::business::user::commerce::inventory::order::OrderData::OrderData;
use crate::org::allbinary::business::user::commerce::money::payment::gateway::transaction::PaymentTransactionKeysFactory::PaymentTransactionKeysFactory;
use crate::org::allbinary::business::user::commerce::money::payment::gateway::transaction::TransactionResult::TransactionResult;
use crate::org::allbinary::business::user::commerce::money::payment::gateway::transaction::TransactionResultInterface::TransactionResultInterface;
use crate::org::allbinary::logic::StdUtil::StdUtil;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::communication::sql::AbSqlBean::AbSqlBean;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use super::TransactionResultEntityInterface::TransactionResultEntityInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct TransactionResultEntity {
    pub base: AbSqlBean,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub tableName: String,
}

impl TransactionResultEntityInterface for TransactionResultEntity {
    fn getTransactionResultInterface(&mut self, mut orderNumber: String) -> Option<Rc<RefCell<dyn TransactionResultInterface>>> { Self::getTransactionResultInterface(self, orderNumber) }
    fn remove(&mut self, mut userName: String, mut orderNumber: String) { Self::remove(self, userName, orderNumber) }
    fn add(&mut self, mut userName: String, mut orderNumber: String, mut transactionResultInterface: Option<Rc<RefCell<dyn TransactionResultInterface>>>) { Self::add(self, userName, orderNumber, transactionResultInterface) }
}
impl crate::org::allbinary::data::tables::BasicDataTableInterface::BasicDataTableInterface for TransactionResultEntity {
    fn createTable(&mut self) -> String { Self::createTable(self) }
    fn dropTable(&mut self) -> String { Self::dropTable(self) }
}

impl TransactionResultEntity {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: AbSqlBean::new_struct(UserDbInitInfo::new()),
            logUtil: None,
            tableName: String::new(),
        };
        this.logUtil = LogUtil::getInstance();
        this.tableName = String::from("vresults");
        // super(...) is in the struct literal above
        this.setTableName(this.tableName.clone());
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn remove(&mut self, mut userName: String, mut orderNumber: String) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut whereHashMap: Option<Rc<RefCell<HashMap>>> = jcall!(StdUtil::getInstance()).createHashMap();
            jcall!(whereHashMap).put(crate::org::allbinary::business::user::commerce::inventory::order::OrderData::OrderData_ID.with(Clone::clone).clone(), (orderNumber).clone() /* as String */);
            jcall!(whereHashMap).put(crate::org::allbinary::business::user::UserData::UserData_USERNAME.with(Clone::clone).clone(), userName.clone());
            self.base.deleteWhere(whereHashMap.clone());
            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).SQLLOGGING.clone()) {
                jcall!(self.logUtil).putF(jcall!(self.commonStrings).SUCCESS.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("remove"));
            }
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).SQLLOGGING.clone()) {
                jcall!(self.logUtil).put(jcall!(self.commonStrings).FAILURE.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("remove"), crate::java_into_object(e.clone()));
            }
            }
        }
    }
    

    pub fn add(&mut self, mut userName: String, mut orderNumber: String, mut transactionResultInterface: Option<Rc<RefCell<dyn TransactionResultInterface>>>) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut values: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
            jcall!(values).add(crate::java_into_object(orderNumber.clone()));
            jcall!(values).add(crate::java_into_object(userName.clone()));
            jcall!(values).addAllList(jcall!(transactionResultInterface).getValues());
            let mut calendar: Option<Rc<RefCell<Calendar>>> = Calendar::getInstance();
            let mut time: String = jcall!(Some((jcall!(calendar).getTimeInMillis()) as i64)).toString();
            jcall!(values).add(crate::java_into_object(time.clone()));
            jcall!(values).add(crate::java_into_object(time.clone()));
            self.base.insert(values.clone());
            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).SQLLOGGING.clone()) {
                jcall!(self.logUtil).putF(jcall!(self.commonStrings).SUCCESS.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("add"));
            }
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).SQLLOGGING.clone()) {
                jcall!(self.logUtil).put(jcall!(self.commonStrings).FAILURE.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("add"), crate::java_into_object(e.clone()));
            }
            }
        }
    }
    

    pub fn getTransactionResultInterface(&mut self, mut orderNumber: String) -> Option<Rc<RefCell<dyn TransactionResultInterface>>> {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut resultHashMap: Option<Rc<RefCell<HashMap>>> = jcall!(StdUtil::getInstance()).createHashMap();
            let mut updateKeyAndValue: Option<Rc<RefCell<HashMap>>> = jcall!(StdUtil::getInstance()).createHashMap();
            jcall!(updateKeyAndValue).put(crate::org::allbinary::business::user::commerce::inventory::order::OrderData::OrderData_ID.with(Clone::clone).clone(), orderNumber.clone());
            resultHashMap = self.base.getRow(updateKeyAndValue.clone());
            if resultHashMap.is_some() {
                let mut transactionResult: Option<Rc<RefCell<TransactionResult>>> = TransactionResult::new1(resultHashMap.clone());
                if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).SQLLOGGING.clone()) {
                    jcall!(self.logUtil).putF(jcall!(self.commonStrings).SUCCESS.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("getTransactionResultInterface"));
                }
                return (transactionResult).clone() /* as TransactionResultInterface */;
            } else {
                return None;
            }
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).SQLLOGGING.clone()) {
                jcall!(self.logUtil).put(jcall!(self.commonStrings).FAILURE.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("getTransactionResultInterface"), crate::java_into_object(e.clone()));
            }
            return None;
            }
        }
    }
    

    pub fn createTableStatement(&mut self) -> String {
        let mut paymentTransactionKeysFactory: Option<Rc<RefCell<PaymentTransactionKeysFactory>>> = PaymentTransactionKeysFactory::getInstance();
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        jcall!(stringBuffer).append(jcall!(self.sqlStrings).CREATE_TABLE.clone()).append(self.tableName.clone()).append(jcall!(self.sqlStrings).START.clone()).append(crate::org::allbinary::business::user::commerce::inventory::order::OrderData::OrderData_ID.with(Clone::clone).clone()).append(jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone()).append(crate::org::allbinary::business::user::UserData::UserData_USERNAME.with(Clone::clone).clone()).append(jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone());
        jcall!(stringBuffer).append(jcall!(jcall!(paymentTransactionKeysFactory).RESULT).toString()).append(jcall!(self.sqlTypeStrings).MAX_INT_NOT_NULL.clone()).append(jcall!(jcall!(paymentTransactionKeysFactory).PNREF).toString()).append(jcall!(self.sqlTypeStrings).TWELVE_CHAR_COLUMN_NOT_NULL.clone());
        jcall!(stringBuffer).append(jcall!(jcall!(paymentTransactionKeysFactory).RESPMSG).toString()).append(jcall!(self.sqlTypeStrings).BLOB_NOT_NULL.clone()).append(jcall!(jcall!(paymentTransactionKeysFactory).AUTHCODE).toString()).append(jcall!(self.sqlTypeStrings).SIX_CHAR_COLUMN_NOT_NULL.clone()).append(jcall!(jcall!(paymentTransactionKeysFactory).AVSADDR).toString()).append(jcall!(self.sqlTypeStrings).ONE_CHAR_COLUMN_NOT_NULL.clone());
        jcall!(stringBuffer).append(jcall!(jcall!(paymentTransactionKeysFactory).AVSZIP).toString()).append(jcall!(self.sqlTypeStrings).ONE_CHAR_COLUMN_NOT_NULL.clone()).append(jcall!(jcall!(paymentTransactionKeysFactory).ORIGRESULT).toString()).append(jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone()).append(jcall!(jcall!(paymentTransactionKeysFactory).STATUS).toString()).append(jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone());
        jcall!(stringBuffer).append(jcall!(jcall!(paymentTransactionKeysFactory).FRAUDCODE).toString()).append(jcall!(self.sqlTypeStrings).TWO_INT_NOT_NULL.clone());
        jcall!(stringBuffer).append(jcall!(jcall!(paymentTransactionKeysFactory).FRAUDMSG).toString()).append(jcall!(self.sqlTypeStrings).MAX_INT_NOT_NULL.clone());
        jcall!(stringBuffer).append(jcall!(jcall!(paymentTransactionKeysFactory).ERRCODE).toString()).append(jcall!(self.sqlTypeStrings).MAX_INT_NOT_NULL.clone());
        jcall!(stringBuffer).append(jcall!(jcall!(paymentTransactionKeysFactory).SCORE).toString()).append(jcall!(self.sqlTypeStrings).THREE_INT_NOT_NULL.clone());
        jcall!(stringBuffer).append(jcall!(jcall!(paymentTransactionKeysFactory).REASON1).toString()).append(jcall!(self.sqlTypeStrings).FOUR_INT_NOT_NULL.clone()).append(jcall!(jcall!(paymentTransactionKeysFactory).REASON2).toString()).append(jcall!(self.sqlTypeStrings).FOUR_INT_NOT_NULL.clone()).append(jcall!(jcall!(paymentTransactionKeysFactory).REASON3).toString());
        jcall!(stringBuffer).append(jcall!(self.sqlTypeStrings).FOUR_INT_NOT_NULL.clone()).append(jcall!(jcall!(paymentTransactionKeysFactory).EXCEPTION1).toString()).append(jcall!(self.sqlTypeStrings).FOUR_INT_NOT_NULL.clone()).append(jcall!(jcall!(paymentTransactionKeysFactory).EXCEPTION2).toString()).append(jcall!(self.sqlTypeStrings).FOUR_INT_NOT_NULL.clone()).append(jcall!(jcall!(paymentTransactionKeysFactory).EXCEPTION3).toString()).append(jcall!(self.sqlTypeStrings).FOUR_INT_NOT_NULL.clone()).append(jcall!(jcall!(paymentTransactionKeysFactory).EXCEPTION4).toString()).append(jcall!(self.sqlTypeStrings).FOUR_INT_NOT_NULL.clone()).append(jcall!(jcall!(paymentTransactionKeysFactory).EXCEPTION5).toString()).append(jcall!(self.sqlTypeStrings).FOUR_INT_NOT_NULL.clone()).append(jcall!(jcall!(paymentTransactionKeysFactory).EXCEPTION6).toString()).append(jcall!(self.sqlTypeStrings).FOUR_INT_NOT_NULL.clone()).append(jcall!(jcall!(paymentTransactionKeysFactory).EXCEPTION7).toString()).append(jcall!(self.sqlTypeStrings).FOUR_INT_NOT_NULL.clone()).append(jcall!(EntryData::getInstance()).LASTMODIFIED.clone()).append(jcall!(self.sqlTypeStrings).MAX_BIG_INT_UNSIGNED_NOT_NULL.clone()).append(jcall!(EntryData::getInstance()).TIMECREATED.clone()).append(jcall!(self.sqlTypeStrings).MAX_BIG_INT_UNSIGNED_NOT_NULL.clone()).append(jcall!(self.sqlStrings).PRIMARY_KEY.clone()).append(crate::org::allbinary::business::user::commerce::inventory::order::OrderData::OrderData_ID.with(Clone::clone).clone()).append(jcall!(self.sqlStrings).END.clone());
        return jcall!(stringBuffer).toString();
    }
    

    pub fn createTable(&mut self) -> String {
        return self.base.createTable(self.createTableStatement());
    }
    

    pub fn dropTable(&mut self) -> String {
        return self.base.dropTable();
    }
}
impl std::ops::Deref for TransactionResultEntity {
    type Target = AbSqlBean;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for TransactionResultEntity {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::logic::communication::sql::AbSqlBasic::AbSqlBasicDyn for TransactionResultEntity {
    fn as_AbSqlBasic(&self) -> &crate::org::allbinary::logic::communication::sql::AbSqlBasic::AbSqlBasic { &self.base.base.base.base.base }
    fn as_AbSqlBasic_mut(&mut self) -> &mut crate::org::allbinary::logic::communication::sql::AbSqlBasic::AbSqlBasic { &mut self.base.base.base.base.base }
}
impl crate::org::allbinary::logic::communication::sql::AbSqlBean::AbSqlBeanDyn for TransactionResultEntity {
    fn as_AbSqlBean(&self) -> &crate::org::allbinary::logic::communication::sql::AbSqlBean::AbSqlBean { &self.base }
    fn as_AbSqlBean_mut(&mut self) -> &mut crate::org::allbinary::logic::communication::sql::AbSqlBean::AbSqlBean { &mut self.base }
}
impl crate::org::allbinary::logic::communication::sql::AbSqlColumn::AbSqlColumnDyn for TransactionResultEntity {
    fn as_AbSqlColumn(&self) -> &crate::org::allbinary::logic::communication::sql::AbSqlColumn::AbSqlColumn { &self.base.base.base }
    fn as_AbSqlColumn_mut(&mut self) -> &mut crate::org::allbinary::logic::communication::sql::AbSqlColumn::AbSqlColumn { &mut self.base.base.base }
}
impl crate::org::allbinary::logic::communication::sql::AbSqlRow::AbSqlRowDyn for TransactionResultEntity {
    fn as_AbSqlRow(&self) -> &crate::org::allbinary::logic::communication::sql::AbSqlRow::AbSqlRow { &self.base.base }
    fn as_AbSqlRow_mut(&mut self) -> &mut crate::org::allbinary::logic::communication::sql::AbSqlRow::AbSqlRow { &mut self.base.base }
}
impl crate::org::allbinary::logic::communication::sql::AbSqlTable::AbSqlTableDyn for TransactionResultEntity {
    fn as_AbSqlTable(&self) -> &crate::org::allbinary::logic::communication::sql::AbSqlTable::AbSqlTable { &self.base.base.base.base }
    fn as_AbSqlTable_mut(&mut self) -> &mut crate::org::allbinary::logic::communication::sql::AbSqlTable::AbSqlTable { &mut self.base.base.base.base }
    fn dropTable(&mut self) -> String { Self::dropTable(self) }
}


