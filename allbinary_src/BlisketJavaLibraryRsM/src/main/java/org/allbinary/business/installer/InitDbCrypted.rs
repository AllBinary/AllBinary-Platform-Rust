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
// package org.allbinary.business.installer
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::business::init::InitInfoEntity::InitInfoEntity;
use crate::org::allbinary::business::init::db::DatabaseConnectionInfoInterface::DatabaseConnectionInfoInterface;
use crate::org::allbinary::business::init::db::DbConnectionInfo::DbConnectionInfo;
use crate::org::allbinary::business::init::db::HistoryDbInitInfo::HistoryDbInitInfo;
use crate::org::allbinary::business::init::db::InventoryDbInitInfo::InventoryDbInitInfo;
use crate::org::allbinary::business::init::db::LogDbInitInfo::LogDbInitInfo;
use crate::org::allbinary::business::init::db::StaticPagesDbInitInfo::StaticPagesDbInitInfo;
use crate::org::allbinary::business::init::db::UserDbInitInfo::UserDbInitInfo;
use crate::org::allbinary::data::tables::context::module::storefronts::StoreFrontsEntityFactory::StoreFrontsEntityFactory;
use crate::org::allbinary::data::tables::log::LogTableEntityFactory::LogTableEntityFactory;
use crate::org::allbinary::data::tables::staticpages::StaticPagesEntityFactory::StaticPagesEntityFactory;
use crate::org::allbinary::data::tables::transform::info::TransformInfoEntityBuilder::TransformInfoEntityBuilder;
use crate::org::allbinary::data::tables::user::UserEntityFactory::UserEntityFactory;
use crate::org::allbinary::data::tables::user::address::billing::BillingAddressesEntityFactory::BillingAddressesEntityFactory;
use crate::org::allbinary::data::tables::user::address::shipping::ShippingAddressesEntityFactory::ShippingAddressesEntityFactory;
use crate::org::allbinary::data::tables::user::commerce::inventory::item::InventoryEntityFactory::InventoryEntityFactory;
use crate::org::allbinary::data::tables::user::commerce::inventory::order::OrderHistoryEntityFactory::OrderHistoryEntityFactory;
use crate::org::allbinary::data::tables::user::commerce::inventory::order::OrderItemsEntityFactory::OrderItemsEntityFactory;
use crate::org::allbinary::data::tables::user::commerce::money::payment::PaymentEntityFactory::PaymentEntityFactory;
use crate::org::allbinary::data::tables::user::commerce::money::payment::gateway::PaymentGatewayEntityFactory::PaymentGatewayEntityFactory;
use crate::org::allbinary::data::tables::user::commerce::money::payment::transaction::TransactionEntityFactory::TransactionEntityFactory;
use crate::org::allbinary::data::tables::user::commerce::money::payment::transaction::TransactionResultEntityFactory::TransactionResultEntityFactory;
use crate::org::allbinary::data::tables::user::quoterequest::QuoteRequestEntityFactory::QuoteRequestEntityFactory;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::communication::sql::AbDatabaseManagement::AbDatabaseManagement;
use crate::org::allbinary::logic::io::AbDataOutputStream::AbDataOutputStream;
use crate::org::allbinary::logic::io::DataOutputStreamFactory::DataOutputStreamFactory;
use crate::org::allbinary::logic::io::file::AbFile::AbFile;
use crate::org::allbinary::logic::io::file::FileFactory::FileFactory;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

//Warning you must have sql root access
pub struct InitDbCrypted {
    pub base: AbDatabaseManagement,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub userDbInitInfo: Option<Rc<RefCell<UserDbInitInfo>>>,
    pub inventoryDbInitInfo: Option<Rc<RefCell<InventoryDbInitInfo>>>,
    pub historyDbInitInfo: Option<Rc<RefCell<HistoryDbInitInfo>>>,
    pub staticpagesDbInitInfo: Option<Rc<RefCell<StaticPagesDbInitInfo>>>,
    pub logDbInitInfo: Option<Rc<RefCell<LogDbInitInfo>>>,
    pub SQL_FILE: String,
}


impl InitDbCrypted {
    

    pub fn new_struct(mut databaseConnectionInfoInterface: Option<Rc<RefCell<dyn DatabaseConnectionInfoInterface>>>) -> Self {
        let mut this = Self {
            base: AbDatabaseManagement::new_struct1((databaseConnectionInfoInterface).clone() /* as DbConnectionInfo */),
            logUtil: None,
            userDbInitInfo: None,
            inventoryDbInitInfo: None,
            historyDbInitInfo: None,
            staticpagesDbInitInfo: None,
            logDbInitInfo: None,
            SQL_FILE: String::new(),
        };
        this.logUtil = LogUtil::getInstance();
        this.SQL_FILE = String::from("initWeblisket.sql");
        // super(...) is in the struct literal above
        this
    }

    pub fn new(mut databaseConnectionInfoInterface: Option<Rc<RefCell<dyn DatabaseConnectionInfoInterface>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(databaseConnectionInfoInterface))))
    }
    

    pub fn addUsers(&mut self) -> Option<bool> {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            jcall!(self.logUtil).putF(jcall!(self.commonStrings).START.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), self.METHOD_ADD_USERS.clone());
            self.userDbInitInfo = UserDbInitInfo::new();
            self.inventoryDbInitInfo = InventoryDbInitInfo::new();
            self.historyDbInitInfo = HistoryDbInitInfo::new();
            self.staticpagesDbInitInfo = StaticPagesDbInitInfo::new();
            self.logDbInitInfo = LogDbInitInfo::new();
            let mut CUSTOMERDB: String = jcall!(self.userDbInitInfo).getName();
            let mut CUSTOMERDBUSER: String = jcall!(self.userDbInitInfo).getUserName();
            let mut CUSTOMERDBPASSWORD: String = jcall!(self.userDbInitInfo).getPassword();
            if !self.addDbUser(CUSTOMERDB.clone(), CUSTOMERDBUSER.clone(), CUSTOMERDBPASSWORD.clone()) {
                jcall!(self.logUtil).putF(format!("{}{}", self.UNABLE_TO_CREATE_USER, CUSTOMERDBUSER), crate::java_into_object(String::from(std::any::type_name::<Self>())), self.METHOD_ADD_USERS.clone());
                return false;
            }
            let mut INVENTORYDB: String = jcall!(self.inventoryDbInitInfo).getName();
            let mut INVENTORYDBUSER: String = jcall!(self.inventoryDbInitInfo).getUserName();
            let mut INVENTORYDBPASSWORD: String = jcall!(self.inventoryDbInitInfo).getPassword();
            if !self.addDbUser(INVENTORYDB.clone(), INVENTORYDBUSER.clone(), INVENTORYDBPASSWORD.clone()) {
                jcall!(self.logUtil).putF(format!("{}{}", self.UNABLE_TO_CREATE_USER, INVENTORYDBUSER), crate::java_into_object(String::from(std::any::type_name::<Self>())), self.METHOD_ADD_USERS.clone());
                return false;
            }
            let mut HISTORYDB: String = jcall!(self.historyDbInitInfo).getName();
            let mut HISTORYDBUSER: String = jcall!(self.historyDbInitInfo).getUserName();
            let mut HISTORYDBPASSWORD: String = jcall!(self.historyDbInitInfo).getPassword();
            if !self.addDbUser(HISTORYDB.clone(), HISTORYDBUSER.clone(), HISTORYDBPASSWORD.clone()) {
                jcall!(self.logUtil).putF(format!("{}{}", self.UNABLE_TO_CREATE_USER, HISTORYDBUSER), crate::java_into_object(String::from(std::any::type_name::<Self>())), self.METHOD_ADD_USERS.clone());
                return false;
            }
            let mut STATICPAGESDB: String = jcall!(self.staticpagesDbInitInfo).getName();
            let mut STATICPAGESDBUSER: String = jcall!(self.staticpagesDbInitInfo).getUserName();
            let mut STATICPAGESDBPASSWORD: String = jcall!(self.staticpagesDbInitInfo).getPassword();
            if !self.addDbUser(STATICPAGESDB.clone(), STATICPAGESDBUSER.clone(), STATICPAGESDBPASSWORD.clone()) {
                jcall!(self.logUtil).putF(format!("{}{}", self.UNABLE_TO_CREATE_USER, STATICPAGESDBUSER), crate::java_into_object(String::from(std::any::type_name::<Self>())), self.METHOD_ADD_USERS.clone());
                return false;
            }
            let mut LOGDB: String = jcall!(self.logDbInitInfo).getName();
            let mut LOGDBUSER: String = jcall!(self.logDbInitInfo).getUserName();
            let mut LOGDBPASSWORD: String = jcall!(self.logDbInitInfo).getPassword();
            if !self.addDbUser(LOGDB.clone(), LOGDBUSER.clone(), LOGDBPASSWORD.clone()) {
                jcall!(self.logUtil).putF(format!("{}{}", self.UNABLE_TO_CREATE_USER, LOGDBUSER), crate::java_into_object(String::from(std::any::type_name::<Self>())), self.METHOD_ADD_USERS.clone());
                return false;
            }
            jcall!(self.logUtil).putF(jcall!(self.commonStrings).END.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), self.METHOD_ADD_USERS.clone());
            return true;
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), self.METHOD_ADD_USERS.clone(), crate::java_into_object(e.clone()));
            return true;
            }
        }
    }
    

    pub fn addDatabases(&mut self) -> Option<bool> {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            jcall!(self.logUtil).putF(jcall!(self.commonStrings).START.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), self.METHOD_ADD_DATABASES.clone());
            self.userDbInitInfo = UserDbInitInfo::new();
            self.inventoryDbInitInfo = InventoryDbInitInfo::new();
            self.historyDbInitInfo = HistoryDbInitInfo::new();
            self.staticpagesDbInitInfo = StaticPagesDbInitInfo::new();
            self.logDbInitInfo = LogDbInitInfo::new();
            let mut CUSTOMERDB: String = jcall!(self.userDbInitInfo).getName();
            if !self.addDb(CUSTOMERDB.clone()) {
                jcall!(self.logUtil).putF(format!("{}{}", self.UNABLE_TO_CREATE_DATABASE, CUSTOMERDB), crate::java_into_object(String::from(std::any::type_name::<Self>())), self.METHOD_ADD_DATABASES.clone());
                return false;
            }
            let mut INVENTORYDB: String = jcall!(self.inventoryDbInitInfo).getName();
            if !self.addDb(INVENTORYDB.clone()) {
                jcall!(self.logUtil).putF(format!("{}{}", self.UNABLE_TO_CREATE_DATABASE, INVENTORYDB), crate::java_into_object(String::from(std::any::type_name::<Self>())), self.METHOD_ADD_DATABASES.clone());
                return false;
            }
            let mut HISTORYDB: String = jcall!(self.historyDbInitInfo).getName();
            if !self.addDb(HISTORYDB.clone()) {
                jcall!(self.logUtil).putF(format!("{}{}", self.UNABLE_TO_CREATE_DATABASE, HISTORYDB), crate::java_into_object(String::from(std::any::type_name::<Self>())), self.METHOD_ADD_DATABASES.clone());
                return false;
            }
            let mut STATICPAGESDB: String = jcall!(self.staticpagesDbInitInfo).getName();
            if !self.addDb(STATICPAGESDB.clone()) {
                jcall!(self.logUtil).putF(format!("{}{}", self.UNABLE_TO_CREATE_DATABASE, STATICPAGESDB), crate::java_into_object(String::from(std::any::type_name::<Self>())), self.METHOD_ADD_DATABASES.clone());
                return false;
            }
            let mut LOGDB: String = jcall!(self.logDbInitInfo).getName();
            if !self.addDb(LOGDB.clone()) {
                jcall!(self.logUtil).putF(format!("{}{}", self.UNABLE_TO_CREATE_DATABASE, LOGDB), crate::java_into_object(String::from(std::any::type_name::<Self>())), self.METHOD_ADD_DATABASES.clone());
                return false;
            }
            let mut file: Option<Rc<RefCell<AbFile>>> = jcall!(FileFactory::getInstance()).getInstance1(self.SQL_FILE.clone());
            jcall!(file).createNewFile();
            let mut idOutData: Option<Rc<RefCell<AbDataOutputStream>>> = jcall!(DataOutputStreamFactory::getInstance()).getInstanceForAbFile(file.clone());
            jcall!(idOutData).writeBytes(jcall!(self.sqlCommandLog).toString());
            return true;
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), self.METHOD_ADD_DATABASES.clone(), crate::java_into_object(e.clone()));
            return true;
            }
        }
    }
    

    pub fn addTables(&mut self) -> Option<bool> {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
            jcall!(self.logUtil).putF(jcall!(self.commonStrings).START.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), self.METHOD_ADD_TABLES.clone());
            jcall!(stringBuffer).append(jcall!(UserEntityFactory::getInstance()).createTable());
            jcall!(stringBuffer).append(jcall!(jcall!(StoreFrontsEntityFactory::getInstance()).getStoreFrontsEntityInstance()).createTable());
            jcall!(stringBuffer).append(jcall!(jcall!(BillingAddressesEntityFactory::getInstance()).getInstance1(jcall!(self.stringUtil).EMPTY_STRING.clone())).createTable());
            jcall!(stringBuffer).append(jcall!(jcall!(ShippingAddressesEntityFactory::getInstance()).getInstance1(jcall!(self.stringUtil).EMPTY_STRING.clone())).createTable());
            jcall!(stringBuffer).append(jcall!(jcall!(PaymentEntityFactory::getInstance()).getPaymentEntityInstance()).createTable());
            jcall!(stringBuffer).append(jcall!(jcall!(QuoteRequestEntityFactory::getInstance()).getQuoteRequestEntityInstance()).createTable());
            jcall!(stringBuffer).append(jcall!(OrderHistoryEntityFactory::getInstance()).createTable());
            jcall!(stringBuffer).append(jcall!(OrderItemsEntityFactory::getInstance()).createTable());
            jcall!(stringBuffer).append(jcall!(jcall!(TransactionEntityFactory::getInstance()).getTransactionEntityInstance()).createTable());
            jcall!(stringBuffer).append(jcall!(jcall!(TransactionResultEntityFactory::getInstance()).getTransactionResultEntityInstance()).createTable());
            jcall!(stringBuffer).append(jcall!(PaymentGatewayEntityFactory::getInstance()).createTable());
            jcall!(stringBuffer).append(jcall!(jcall!(InventoryEntityFactory::getInstance()).getInventoryEntityInstance()).createTable());
            jcall!(stringBuffer).append(jcall!(InitInfoEntity::new()).createTable());
            jcall!(stringBuffer).append(jcall!(jcall!(LogTableEntityFactory::getInstance()).getLogTableEntityInstance()).createTable());
            jcall!(stringBuffer).append(jcall!(jcall!(StaticPagesEntityFactory::getInstance()).getStaticPagesEntityInstance()).createTable());
            jcall!(stringBuffer).append(jcall!(TransformInfoEntityBuilder::getInstance()).createTable());
            { let java_receiver = self.logUtil.clone(); let java_arguments = (format!("{}{}", ADD_TABLES_RESULTS_LABEL, jcall!(stringBuffer).toString()), crate::java_into_object(String::from(std::any::type_name::<Self>())), self.METHOD_ADD_TABLES.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
            return true;
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), self.METHOD_ADD_TABLES.clone(), crate::java_into_object(e.clone()));
            return false;
            }
        }
    }
}
impl std::ops::Deref for InitDbCrypted {
    type Target = AbDatabaseManagement;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for InitDbCrypted {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::logic::communication::sql::AbDatabaseManagement::AbDatabaseManagementDyn for InitDbCrypted {
    fn as_AbDatabaseManagement(&self) -> &crate::org::allbinary::logic::communication::sql::AbDatabaseManagement::AbDatabaseManagement { &self.base }
    fn as_AbDatabaseManagement_mut(&mut self) -> &mut crate::org::allbinary::logic::communication::sql::AbDatabaseManagement::AbDatabaseManagement { &mut self.base }
}
impl crate::org::allbinary::logic::communication::sql::AbSqlBasic::AbSqlBasicDyn for InitDbCrypted {
    fn as_AbSqlBasic(&self) -> &crate::org::allbinary::logic::communication::sql::AbSqlBasic::AbSqlBasic { &self.base.base.base.base.base.base }
    fn as_AbSqlBasic_mut(&mut self) -> &mut crate::org::allbinary::logic::communication::sql::AbSqlBasic::AbSqlBasic { &mut self.base.base.base.base.base.base }
}
impl crate::org::allbinary::logic::communication::sql::AbSqlBean::AbSqlBeanDyn for InitDbCrypted {
    fn as_AbSqlBean(&self) -> &crate::org::allbinary::logic::communication::sql::AbSqlBean::AbSqlBean { &self.base.base }
    fn as_AbSqlBean_mut(&mut self) -> &mut crate::org::allbinary::logic::communication::sql::AbSqlBean::AbSqlBean { &mut self.base.base }
}
impl crate::org::allbinary::logic::communication::sql::AbSqlColumn::AbSqlColumnDyn for InitDbCrypted {
    fn as_AbSqlColumn(&self) -> &crate::org::allbinary::logic::communication::sql::AbSqlColumn::AbSqlColumn { &self.base.base.base.base }
    fn as_AbSqlColumn_mut(&mut self) -> &mut crate::org::allbinary::logic::communication::sql::AbSqlColumn::AbSqlColumn { &mut self.base.base.base.base }
}
impl crate::org::allbinary::logic::communication::sql::AbSqlRow::AbSqlRowDyn for InitDbCrypted {
    fn as_AbSqlRow(&self) -> &crate::org::allbinary::logic::communication::sql::AbSqlRow::AbSqlRow { &self.base.base.base }
    fn as_AbSqlRow_mut(&mut self) -> &mut crate::org::allbinary::logic::communication::sql::AbSqlRow::AbSqlRow { &mut self.base.base.base }
}
impl crate::org::allbinary::logic::communication::sql::AbSqlTable::AbSqlTableDyn for InitDbCrypted {
    fn as_AbSqlTable(&self) -> &crate::org::allbinary::logic::communication::sql::AbSqlTable::AbSqlTable { &self.base.base.base.base.base }
    fn as_AbSqlTable_mut(&mut self) -> &mut crate::org::allbinary::logic::communication::sql::AbSqlTable::AbSqlTable { &mut self.base.base.base.base.base }
}


