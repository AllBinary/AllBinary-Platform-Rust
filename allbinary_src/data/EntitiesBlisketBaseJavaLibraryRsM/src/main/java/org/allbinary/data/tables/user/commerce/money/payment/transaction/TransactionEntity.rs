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
use crate::org::allbinary::business::user::commerce::money::payment::gateway::transaction::PaymentTransactionInterface::PaymentTransactionInterface;
use crate::org::allbinary::business::user::commerce::money::payment::gateway::transaction::PaymentTransactionKeysFactory::PaymentTransactionKeysFactory;
use crate::org::allbinary::logic::StdUtil::StdUtil;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::communication::sql::AbSqlBean::AbSqlBean;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use super::TransactionEntityInterface::TransactionEntityInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct TransactionEntity {
    pub base: AbSqlBean,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub tableName: String,
}

impl TransactionEntityInterface for TransactionEntity {
    fn remove(&mut self, mut userName: String, mut orderNumber: String) { Self::remove(self, userName, orderNumber) }
    fn add(&mut self, mut userName: String, mut orderNumber: String, mut paymentTransactionInterface: Option<Rc<RefCell<dyn PaymentTransactionInterface>>>) { Self::add(self, userName, orderNumber, paymentTransactionInterface) }
}
impl crate::org::allbinary::data::tables::BasicDataTableInterface::BasicDataTableInterface for TransactionEntity {
    fn createTable(&mut self) -> String { Self::createTable(self) }
    fn dropTable(&mut self) -> String { Self::dropTable(self) }
}

impl TransactionEntity {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: AbSqlBean::new_struct(UserDbInitInfo::new()),
            logUtil: None,
            tableName: String::new(),
        };
        this.logUtil = LogUtil::getInstance();
        this.tableName = String::from("vtrans");
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
    

    pub fn add(&mut self, mut userName: String, mut orderNumber: String, mut paymentTransactionInterface: Option<Rc<RefCell<dyn PaymentTransactionInterface>>>) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut values: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
            jcall!(values).add(crate::java_into_object(orderNumber.clone()));
            jcall!(values).add(crate::java_into_object(userName.clone()));
            jcall!(values).addAllList(jcall!(paymentTransactionInterface).toVector());
            let mut calendar: Option<Rc<RefCell<Calendar>>> = Calendar::getInstance();
            let mut time: String = jcall!(Some((jcall!(calendar).getTimeInMillis()) as i64)).toString().clone();
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
    

    pub fn createTableStatement(&mut self) -> String {
        let mut paymentTransactionKeysFactory: Option<Rc<RefCell<PaymentTransactionKeysFactory>>> = PaymentTransactionKeysFactory::getInstance();
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        jcall!(stringBuffer).append(jcall!(self.sqlStrings).CREATE_TABLE.clone());
        jcall!(stringBuffer).append(self.tableName.clone());
        jcall!(stringBuffer).append(jcall!(self.sqlStrings).START.clone());
        jcall!(stringBuffer).append(crate::org::allbinary::business::user::commerce::inventory::order::OrderData::OrderData_ID.with(Clone::clone).clone()).append(jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone()).append(crate::org::allbinary::business::user::UserData::UserData_USERNAME.with(Clone::clone).clone()).append(jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone()).append(jcall!(jcall!(paymentTransactionKeysFactory).TRXTYPE).toString()).append(jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone()).append(jcall!(jcall!(paymentTransactionKeysFactory).TENDER).toString()).append(jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone()).append(jcall!(jcall!(paymentTransactionKeysFactory).ACCT).toString()).append(jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone()).append(jcall!(jcall!(paymentTransactionKeysFactory).EXPDATE).toString()).append(jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone()).append(jcall!(jcall!(paymentTransactionKeysFactory).AMT).toString()).append(jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone()).append(jcall!(jcall!(paymentTransactionKeysFactory).AUTHCODE).toString()).append(jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone()).append(jcall!(jcall!(paymentTransactionKeysFactory).MICR).toString()).append(jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone()).append(jcall!(jcall!(paymentTransactionKeysFactory).CHECKNUM).toString()).append(jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone()).append(jcall!(jcall!(paymentTransactionKeysFactory).NAME).toString()).append(jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone()).append(jcall!(jcall!(paymentTransactionKeysFactory).STREET).toString()).append(jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone()).append(jcall!(jcall!(paymentTransactionKeysFactory).CITY).toString()).append(jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone()).append(jcall!(jcall!(paymentTransactionKeysFactory).STATE).toString()).append(jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone()).append(jcall!(jcall!(paymentTransactionKeysFactory).ZIP).toString()).append(jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone()).append(jcall!(jcall!(paymentTransactionKeysFactory).DL).toString()).append(jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone()).append(jcall!(jcall!(paymentTransactionKeysFactory).EMAIL).toString()).append(jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone()).append(jcall!(jcall!(paymentTransactionKeysFactory).COMMENT1).toString()).append(jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone()).append(jcall!(jcall!(paymentTransactionKeysFactory).COMMENT2).toString()).append(jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone()).append(jcall!(jcall!(paymentTransactionKeysFactory).ORIGID).toString()).append(jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone()).append(jcall!(jcall!(paymentTransactionKeysFactory).PONUM).toString()).append(jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone()).append(jcall!(jcall!(paymentTransactionKeysFactory).DESC).toString()).append(jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone()).append(jcall!(jcall!(paymentTransactionKeysFactory).DESC1TO4).toString()).append(jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone()).append(jcall!(jcall!(paymentTransactionKeysFactory).INVNUM).toString()).append(jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone()).append(jcall!(jcall!(paymentTransactionKeysFactory).SHIPTOZIP).toString()).append(jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone()).append(jcall!(jcall!(paymentTransactionKeysFactory).SWIPE).toString()).append(jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone()).append(jcall!(jcall!(paymentTransactionKeysFactory).TAXAMT).toString()).append(jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone()).append(jcall!(jcall!(paymentTransactionKeysFactory).COMMCARD).toString()).append(jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone()).append(jcall!(jcall!(paymentTransactionKeysFactory).DUTYAMT).toString()).append(jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone()).append(jcall!(jcall!(paymentTransactionKeysFactory).FREIGHTAMT).toString()).append(jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone()).append(jcall!(jcall!(paymentTransactionKeysFactory).ORDERDATE).toString()).append(jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone()).append(jcall!(jcall!(paymentTransactionKeysFactory).TAXEXEMPT).toString()).append(jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone()).append(jcall!(jcall!(paymentTransactionKeysFactory).COUNTRYCODE).toString()).append(jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone()).append(jcall!(jcall!(paymentTransactionKeysFactory).CUSTCODE).toString()).append(jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone()).append(jcall!(jcall!(paymentTransactionKeysFactory).CVV2).toString()).append(jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone()).append(jcall!(jcall!(paymentTransactionKeysFactory).ABA).toString()).append(jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone()).append(jcall!(jcall!(paymentTransactionKeysFactory).ACCTTYPE).toString()).append(jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone()).append(jcall!(jcall!(paymentTransactionKeysFactory).DISCOUNT).toString()).append(jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone()).append(jcall!(jcall!(paymentTransactionKeysFactory).FIRSTNAME).toString()).append(jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone()).append(jcall!(jcall!(paymentTransactionKeysFactory).LASTNAME).toString()).append(jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone()).append(jcall!(jcall!(paymentTransactionKeysFactory).SHIPFROMZIP).toString()).append(jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone()).append(jcall!(jcall!(paymentTransactionKeysFactory).PRENOTE).toString()).append(jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone()).append(jcall!(jcall!(paymentTransactionKeysFactory).CHKTYPE).toString()).append(jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone()).append(jcall!(jcall!(paymentTransactionKeysFactory).DOB).toString()).append(jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone()).append(jcall!(jcall!(paymentTransactionKeysFactory).PHONENUM).toString()).append(jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone()).append(jcall!(jcall!(paymentTransactionKeysFactory).SS).toString()).append(String::from(" VARCHAR(30) NOT NULL,")).append(jcall!(jcall!(paymentTransactionKeysFactory).COMPANYNAME).toString()).append(jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone()).append(jcall!(jcall!(paymentTransactionKeysFactory).COUNTRY).toString()).append(jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone()).append(jcall!(jcall!(paymentTransactionKeysFactory).SHIPTOCITY).toString()).append(jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone()).append(jcall!(jcall!(paymentTransactionKeysFactory).SHIPTOFIRSTNAME).toString()).append(jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone()).append(jcall!(jcall!(paymentTransactionKeysFactory).SHIPTOLASTNAME).toString()).append(jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone()).append(jcall!(jcall!(paymentTransactionKeysFactory).SHIPTOSTATE).toString()).append(jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone()).append(jcall!(jcall!(paymentTransactionKeysFactory).SHIPTOSTREET).toString()).append(jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone()).append(jcall!(jcall!(paymentTransactionKeysFactory).SPECIAL1).toString()).append(jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone()).append(jcall!(jcall!(paymentTransactionKeysFactory).SPECIAL2).toString()).append(jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone()).append(jcall!(jcall!(paymentTransactionKeysFactory).SPECIAL3).toString()).append(jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone()).append(jcall!(EntryData::getInstance()).LASTMODIFIED.clone()).append(jcall!(self.sqlTypeStrings).MAX_BIG_INT_UNSIGNED_NOT_NULL.clone()).append(jcall!(EntryData::getInstance()).TIMECREATED.clone()).append(jcall!(self.sqlTypeStrings).MAX_BIG_INT_UNSIGNED_NOT_NULL.clone()).append(jcall!(self.sqlStrings).PRIMARY_KEY.clone()).append(crate::org::allbinary::business::user::commerce::inventory::order::OrderData::OrderData_ID.with(Clone::clone).clone()).append(jcall!(self.sqlStrings).END.clone());
        return jcall!(stringBuffer).toString();
    }
    

    pub fn createTable(&mut self) -> String {
        return self.base.createTable(self.createTableStatement());
    }
    

    pub fn dropTable(&mut self) -> String {
        return self.base.dropTable();
    }
}
impl std::ops::Deref for TransactionEntity {
    type Target = AbSqlBean;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for TransactionEntity {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::logic::communication::sql::AbSqlBasic::AbSqlBasicDyn for TransactionEntity {
    fn as_AbSqlBasic(&self) -> &crate::org::allbinary::logic::communication::sql::AbSqlBasic::AbSqlBasic { &self.base.base.base.base.base }
    fn as_AbSqlBasic_mut(&mut self) -> &mut crate::org::allbinary::logic::communication::sql::AbSqlBasic::AbSqlBasic { &mut self.base.base.base.base.base }
}
impl crate::org::allbinary::logic::communication::sql::AbSqlBean::AbSqlBeanDyn for TransactionEntity {
    fn as_AbSqlBean(&self) -> &crate::org::allbinary::logic::communication::sql::AbSqlBean::AbSqlBean { &self.base }
    fn as_AbSqlBean_mut(&mut self) -> &mut crate::org::allbinary::logic::communication::sql::AbSqlBean::AbSqlBean { &mut self.base }
}
impl crate::org::allbinary::logic::communication::sql::AbSqlColumn::AbSqlColumnDyn for TransactionEntity {
    fn as_AbSqlColumn(&self) -> &crate::org::allbinary::logic::communication::sql::AbSqlColumn::AbSqlColumn { &self.base.base.base }
    fn as_AbSqlColumn_mut(&mut self) -> &mut crate::org::allbinary::logic::communication::sql::AbSqlColumn::AbSqlColumn { &mut self.base.base.base }
}
impl crate::org::allbinary::logic::communication::sql::AbSqlRow::AbSqlRowDyn for TransactionEntity {
    fn as_AbSqlRow(&self) -> &crate::org::allbinary::logic::communication::sql::AbSqlRow::AbSqlRow { &self.base.base }
    fn as_AbSqlRow_mut(&mut self) -> &mut crate::org::allbinary::logic::communication::sql::AbSqlRow::AbSqlRow { &mut self.base.base }
}
impl crate::org::allbinary::logic::communication::sql::AbSqlTable::AbSqlTableDyn for TransactionEntity {
    fn as_AbSqlTable(&self) -> &crate::org::allbinary::logic::communication::sql::AbSqlTable::AbSqlTable { &self.base.base.base.base }
    fn as_AbSqlTable_mut(&mut self) -> &mut crate::org::allbinary::logic::communication::sql::AbSqlTable::AbSqlTable { &mut self.base.base.base.base }
    fn dropTable(&mut self) -> String { Self::dropTable(self) }
}


