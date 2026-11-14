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
use super::*;
use crate::java::util::Calendar::Calendar;
use crate::java::util::HashMap::HashMap;
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
use crate::java::lang::Exception::*;
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
            base: AbSqlBean::new_struct(UserDbInitInfo::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::business::init::db::DbConnectionInfo::DbConnectionInfoDyn>>)),
            logUtil: None,
            tableName: String::new(),
        };
        this.logUtil = LogUtil::getInstance();
        this.tableName = String::from("vtrans");
        // super(...) is in the struct literal above
        { let java_arguments = (this.tableName.clone(),); this.setTableName(java_arguments.0) };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn remove(&mut self, mut userName: String, mut orderNumber: String) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut whereHashMap: Option<Rc<RefCell<HashMap<crate::JavaObject, crate::JavaObject>>>> = jcall!(StdUtil::getInstance()).createHashMap();
            jcall!(whereHashMap).put(crate::java_into_object(crate::org::allbinary::business::user::commerce::inventory::order::OrderData::OrderData_ID.with(Clone::clone).clone()), crate::java_into_object((orderNumber).clone() /* as String */));
            jcall!(whereHashMap).put(crate::java_into_object(crate::org::allbinary::business::user::UserData::UserData_USERNAME.with(Clone::clone).clone()), crate::java_into_object(userName.clone()));
            { let java_arguments = (whereHashMap.clone(),); self.base.deleteWhere(java_arguments.0) };
            if jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypes.clone(); java_field }).LOGGING.clone(); java_field }).contains({ let java_field = jcall!(jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypeFactory.clone(); java_field }).getInstance()).SQLLOGGING.clone(); java_field }.clone()) {
                jcall!(self.logUtil).putF({ let java_field = jcall!(self.commonStrings).SUCCESS.clone(); java_field }.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("remove"));
            }
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            if jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypes.clone(); java_field }).LOGGING.clone(); java_field }).contains({ let java_field = jcall!(jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypeFactory.clone(); java_field }).getInstance()).SQLLOGGING.clone(); java_field }.clone()) {
                jcall!(self.logUtil).put({ let java_field = jcall!(self.commonStrings).FAILURE.clone(); java_field }.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("remove"), crate::java_into_object(e.clone()));
            }
            }
        }
    }
    

    pub fn add(&mut self, mut userName: String, mut orderNumber: String, mut paymentTransactionInterface: Option<Rc<RefCell<dyn PaymentTransactionInterface>>>) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut values: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
            jcall!(values).add(crate::java_into_object(orderNumber.clone()));
            jcall!(values).add(crate::java_into_object(userName.clone()));
            jcall!(values).addAllList(jcall!(paymentTransactionInterface).toVector());
            let mut calendar: Option<Rc<RefCell<Calendar>>> = Calendar::getInstance();
            let mut time: String = (Some((jcall!(calendar).getTimeInMillis()) as i64)).expect("NullPointerException").to_string().clone();
            jcall!(values).add(crate::java_into_object(time.clone()));
            jcall!(values).add(crate::java_into_object(time.clone()));
            { let java_arguments = (values.clone(),); self.base.insert(java_arguments.0) };
            if jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypes.clone(); java_field }).LOGGING.clone(); java_field }).contains({ let java_field = jcall!(jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypeFactory.clone(); java_field }).getInstance()).SQLLOGGING.clone(); java_field }.clone()) {
                jcall!(self.logUtil).putF({ let java_field = jcall!(self.commonStrings).SUCCESS.clone(); java_field }.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("add"));
            }
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            if jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypes.clone(); java_field }).LOGGING.clone(); java_field }).contains({ let java_field = jcall!(jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypeFactory.clone(); java_field }).getInstance()).SQLLOGGING.clone(); java_field }.clone()) {
                jcall!(self.logUtil).put({ let java_field = jcall!(self.commonStrings).FAILURE.clone(); java_field }.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("add"), crate::java_into_object(e.clone()));
            }
            }
        }
    }
    

    pub fn createTableStatement(&mut self) -> String {
        let mut paymentTransactionKeysFactory: Option<Rc<RefCell<PaymentTransactionKeysFactory>>> = PaymentTransactionKeysFactory::getInstance();
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        jcall!(stringBuffer).append({ let java_field = jcall!(self.sqlStrings).CREATE_TABLE.clone(); java_field }.clone());
        jcall!(stringBuffer).append(self.tableName.clone());
        jcall!(stringBuffer).append({ let java_field = jcall!(self.sqlStrings).START.clone(); java_field }.clone());
        jcall!(stringBuffer).append(crate::org::allbinary::business::user::commerce::inventory::order::OrderData::OrderData_ID.with(Clone::clone).clone()).append({ let java_field = jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone(); java_field }.clone()).append(crate::org::allbinary::business::user::UserData::UserData_USERNAME.with(Clone::clone).clone()).append({ let java_field = jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone(); java_field }.clone()).append(jcall!({ let java_field = jcall!(paymentTransactionKeysFactory).TRXTYPE.clone(); java_field }).toString()).append({ let java_field = jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone(); java_field }.clone()).append(jcall!({ let java_field = jcall!(paymentTransactionKeysFactory).TENDER.clone(); java_field }).toString()).append({ let java_field = jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone(); java_field }.clone()).append(jcall!({ let java_field = jcall!(paymentTransactionKeysFactory).ACCT.clone(); java_field }).toString()).append({ let java_field = jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone(); java_field }.clone()).append(jcall!({ let java_field = jcall!(paymentTransactionKeysFactory).EXPDATE.clone(); java_field }).toString()).append({ let java_field = jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone(); java_field }.clone()).append(jcall!({ let java_field = jcall!(paymentTransactionKeysFactory).AMT.clone(); java_field }).toString()).append({ let java_field = jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone(); java_field }.clone()).append(jcall!({ let java_field = jcall!(paymentTransactionKeysFactory).AUTHCODE.clone(); java_field }).toString()).append({ let java_field = jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone(); java_field }.clone()).append(jcall!({ let java_field = jcall!(paymentTransactionKeysFactory).MICR.clone(); java_field }).toString()).append({ let java_field = jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone(); java_field }.clone()).append(jcall!({ let java_field = jcall!(paymentTransactionKeysFactory).CHECKNUM.clone(); java_field }).toString()).append({ let java_field = jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone(); java_field }.clone()).append(jcall!({ let java_field = jcall!(paymentTransactionKeysFactory).NAME.clone(); java_field }).toString()).append({ let java_field = jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone(); java_field }.clone()).append(jcall!({ let java_field = jcall!(paymentTransactionKeysFactory).STREET.clone(); java_field }).toString()).append({ let java_field = jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone(); java_field }.clone()).append(jcall!({ let java_field = jcall!(paymentTransactionKeysFactory).CITY.clone(); java_field }).toString()).append({ let java_field = jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone(); java_field }.clone()).append(jcall!({ let java_field = jcall!(paymentTransactionKeysFactory).STATE.clone(); java_field }).toString()).append({ let java_field = jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone(); java_field }.clone()).append(jcall!({ let java_field = jcall!(paymentTransactionKeysFactory).ZIP.clone(); java_field }).toString()).append({ let java_field = jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone(); java_field }.clone()).append(jcall!({ let java_field = jcall!(paymentTransactionKeysFactory).DL.clone(); java_field }).toString()).append({ let java_field = jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone(); java_field }.clone()).append(jcall!({ let java_field = jcall!(paymentTransactionKeysFactory).EMAIL.clone(); java_field }).toString()).append({ let java_field = jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone(); java_field }.clone()).append(jcall!({ let java_field = jcall!(paymentTransactionKeysFactory).COMMENT1.clone(); java_field }).toString()).append({ let java_field = jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone(); java_field }.clone()).append(jcall!({ let java_field = jcall!(paymentTransactionKeysFactory).COMMENT2.clone(); java_field }).toString()).append({ let java_field = jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone(); java_field }.clone()).append(jcall!({ let java_field = jcall!(paymentTransactionKeysFactory).ORIGID.clone(); java_field }).toString()).append({ let java_field = jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone(); java_field }.clone()).append(jcall!({ let java_field = jcall!(paymentTransactionKeysFactory).PONUM.clone(); java_field }).toString()).append({ let java_field = jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone(); java_field }.clone()).append(jcall!({ let java_field = jcall!(paymentTransactionKeysFactory).DESC.clone(); java_field }).toString()).append({ let java_field = jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone(); java_field }.clone()).append(jcall!({ let java_field = jcall!(paymentTransactionKeysFactory).DESC1TO4.clone(); java_field }).toString()).append({ let java_field = jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone(); java_field }.clone()).append(jcall!({ let java_field = jcall!(paymentTransactionKeysFactory).INVNUM.clone(); java_field }).toString()).append({ let java_field = jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone(); java_field }.clone()).append(jcall!({ let java_field = jcall!(paymentTransactionKeysFactory).SHIPTOZIP.clone(); java_field }).toString()).append({ let java_field = jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone(); java_field }.clone()).append(jcall!({ let java_field = jcall!(paymentTransactionKeysFactory).SWIPE.clone(); java_field }).toString()).append({ let java_field = jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone(); java_field }.clone()).append(jcall!({ let java_field = jcall!(paymentTransactionKeysFactory).TAXAMT.clone(); java_field }).toString()).append({ let java_field = jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone(); java_field }.clone()).append(jcall!({ let java_field = jcall!(paymentTransactionKeysFactory).COMMCARD.clone(); java_field }).toString()).append({ let java_field = jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone(); java_field }.clone()).append(jcall!({ let java_field = jcall!(paymentTransactionKeysFactory).DUTYAMT.clone(); java_field }).toString()).append({ let java_field = jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone(); java_field }.clone()).append(jcall!({ let java_field = jcall!(paymentTransactionKeysFactory).FREIGHTAMT.clone(); java_field }).toString()).append({ let java_field = jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone(); java_field }.clone()).append(jcall!({ let java_field = jcall!(paymentTransactionKeysFactory).ORDERDATE.clone(); java_field }).toString()).append({ let java_field = jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone(); java_field }.clone()).append(jcall!({ let java_field = jcall!(paymentTransactionKeysFactory).TAXEXEMPT.clone(); java_field }).toString()).append({ let java_field = jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone(); java_field }.clone()).append(jcall!({ let java_field = jcall!(paymentTransactionKeysFactory).COUNTRYCODE.clone(); java_field }).toString()).append({ let java_field = jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone(); java_field }.clone()).append(jcall!({ let java_field = jcall!(paymentTransactionKeysFactory).CUSTCODE.clone(); java_field }).toString()).append({ let java_field = jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone(); java_field }.clone()).append(jcall!({ let java_field = jcall!(paymentTransactionKeysFactory).CVV2.clone(); java_field }).toString()).append({ let java_field = jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone(); java_field }.clone()).append(jcall!({ let java_field = jcall!(paymentTransactionKeysFactory).ABA.clone(); java_field }).toString()).append({ let java_field = jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone(); java_field }.clone()).append(jcall!({ let java_field = jcall!(paymentTransactionKeysFactory).ACCTTYPE.clone(); java_field }).toString()).append({ let java_field = jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone(); java_field }.clone()).append(jcall!({ let java_field = jcall!(paymentTransactionKeysFactory).DISCOUNT.clone(); java_field }).toString()).append({ let java_field = jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone(); java_field }.clone()).append(jcall!({ let java_field = jcall!(paymentTransactionKeysFactory).FIRSTNAME.clone(); java_field }).toString()).append({ let java_field = jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone(); java_field }.clone()).append(jcall!({ let java_field = jcall!(paymentTransactionKeysFactory).LASTNAME.clone(); java_field }).toString()).append({ let java_field = jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone(); java_field }.clone()).append(jcall!({ let java_field = jcall!(paymentTransactionKeysFactory).SHIPFROMZIP.clone(); java_field }).toString()).append({ let java_field = jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone(); java_field }.clone()).append(jcall!({ let java_field = jcall!(paymentTransactionKeysFactory).PRENOTE.clone(); java_field }).toString()).append({ let java_field = jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone(); java_field }.clone()).append(jcall!({ let java_field = jcall!(paymentTransactionKeysFactory).CHKTYPE.clone(); java_field }).toString()).append({ let java_field = jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone(); java_field }.clone()).append(jcall!({ let java_field = jcall!(paymentTransactionKeysFactory).DOB.clone(); java_field }).toString()).append({ let java_field = jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone(); java_field }.clone()).append(jcall!({ let java_field = jcall!(paymentTransactionKeysFactory).PHONENUM.clone(); java_field }).toString()).append({ let java_field = jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone(); java_field }.clone()).append(jcall!({ let java_field = jcall!(paymentTransactionKeysFactory).SS.clone(); java_field }).toString()).append(String::from(" VARCHAR(30) NOT NULL,")).append(jcall!({ let java_field = jcall!(paymentTransactionKeysFactory).COMPANYNAME.clone(); java_field }).toString()).append({ let java_field = jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone(); java_field }.clone()).append(jcall!({ let java_field = jcall!(paymentTransactionKeysFactory).COUNTRY.clone(); java_field }).toString()).append({ let java_field = jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone(); java_field }.clone()).append(jcall!({ let java_field = jcall!(paymentTransactionKeysFactory).SHIPTOCITY.clone(); java_field }).toString()).append({ let java_field = jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone(); java_field }.clone()).append(jcall!({ let java_field = jcall!(paymentTransactionKeysFactory).SHIPTOFIRSTNAME.clone(); java_field }).toString()).append({ let java_field = jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone(); java_field }.clone()).append(jcall!({ let java_field = jcall!(paymentTransactionKeysFactory).SHIPTOLASTNAME.clone(); java_field }).toString()).append({ let java_field = jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone(); java_field }.clone()).append(jcall!({ let java_field = jcall!(paymentTransactionKeysFactory).SHIPTOSTATE.clone(); java_field }).toString()).append({ let java_field = jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone(); java_field }.clone()).append(jcall!({ let java_field = jcall!(paymentTransactionKeysFactory).SHIPTOSTREET.clone(); java_field }).toString()).append({ let java_field = jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone(); java_field }.clone()).append(jcall!({ let java_field = jcall!(paymentTransactionKeysFactory).SPECIAL1.clone(); java_field }).toString()).append({ let java_field = jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone(); java_field }.clone()).append(jcall!({ let java_field = jcall!(paymentTransactionKeysFactory).SPECIAL2.clone(); java_field }).toString()).append({ let java_field = jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone(); java_field }.clone()).append(jcall!({ let java_field = jcall!(paymentTransactionKeysFactory).SPECIAL3.clone(); java_field }).toString()).append({ let java_field = jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone(); java_field }.clone()).append({ let java_field = jcall!(EntryData::getInstance()).LASTMODIFIED.clone(); java_field }.clone()).append({ let java_field = jcall!(self.sqlTypeStrings).MAX_BIG_INT_UNSIGNED_NOT_NULL.clone(); java_field }.clone()).append({ let java_field = jcall!(EntryData::getInstance()).TIMECREATED.clone(); java_field }.clone()).append({ let java_field = jcall!(self.sqlTypeStrings).MAX_BIG_INT_UNSIGNED_NOT_NULL.clone(); java_field }.clone()).append({ let java_field = jcall!(self.sqlStrings).PRIMARY_KEY.clone(); java_field }.clone()).append(crate::org::allbinary::business::user::commerce::inventory::order::OrderData::OrderData_ID.with(Clone::clone).clone()).append({ let java_field = jcall!(self.sqlStrings).END.clone(); java_field }.clone());
        return jcall!(stringBuffer).toString();
    }
    

    pub fn createTable(&mut self) -> String {
        return { let java_arguments = (self.createTableStatement(),); self.base.createTable(java_arguments.0) };
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


