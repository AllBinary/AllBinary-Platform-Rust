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
// use crate::java::sql::Connection::Connection;
// use crate::java::sql::ResultSet::ResultSet;
// use crate::java::sql::SQLException::SQLException;
// use crate::java::sql::Statement::Statement;
use crate::org::allbinary::business::init::db::DatabaseConnectionInfoInterface::DatabaseConnectionInfoInterface;
use crate::org::allbinary::business::init::db::DbConnectionInfo::DbConnectionInfo;
use crate::org::allbinary::logic::StdUtil::StdUtil;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::communication::sql::connection::pool::SqlConnectionPool::SqlConnectionPool;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use crate::org::allbinary::string::CommonSeps::CommonSeps;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::java::lang::Thread::*;
use crate::java::lang::Exception::*;
use crate::java::lang::ClassLoader::*;
use super::SqlTypeStrings::SqlTypeStrings;
use super::SqlStrings::SqlStrings;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct AbSqlBasic {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub sqlConnectionPool: Option<Rc<RefCell<SqlConnectionPool>>>,
    pub databaseConnectionInfoInterface: Option<Rc<RefCell<dyn crate::org::allbinary::business::init::db::DbConnectionInfo::DbConnectionInfoDyn>>>,
    pub connectAttemptCounter: i32,
    pub conn: Option<Rc<RefCell<Connection>>>,
    pub stringUtil: Option<Rc<RefCell<StringUtil>>>,
    pub commonStrings: Option<Rc<RefCell<CommonStrings>>>,
    pub commonSeps: Option<Rc<RefCell<CommonSeps>>>,
    pub stdUtil: Option<Rc<RefCell<StdUtil>>>,
    pub sqlTypeStrings: Option<Rc<RefCell<SqlTypeStrings>>>,
    pub sqlStrings: Option<Rc<RefCell<SqlStrings>>>,
    pub INSERT: String,
    pub UPDATE: String,
    pub FAILED_SQL_STATEMENT: String,
    pub SUCCESS_SQL_STATEMENT: String,
    pub METHOD_EXECUTED_SQL_STATEMENT: String,
    pub METHOD_CREATE: String,
    pub DATABASE_CREATED_LABEL: String,
    pub DATABASE_CREATION_FAILED_LABEL: String,
    pub SQL_CONNECTION_FAILED: String,
    pub SQL_CONNECTION_RETRYING: String,
    pub SQL_CONNECTION_RETRY: String,
    pub METHOD_CREATE_CONNECTION: String,
    pub METHOD_INITIALIZE: String,
    pub LOAD_JDBC_DRIVER_FAILED_LABEL: String,
}


impl AbSqlBasic {

    pub fn toString(&mut self) -> String {
        String::from(std::any::type_name::<Self>())
    }
    

    pub fn new_struct(mut databaseConnectionInfoInterface: Option<Rc<RefCell<dyn crate::org::allbinary::business::init::db::DbConnectionInfo::DbConnectionInfoDyn>>>) -> Self {
        let mut this = Self {
            logUtil: None,
            sqlConnectionPool: None,
            databaseConnectionInfoInterface: None,
            connectAttemptCounter: 0,
            conn: None,
            stringUtil: None,
            commonStrings: None,
            commonSeps: None,
            stdUtil: None,
            sqlTypeStrings: None,
            sqlStrings: None,
            INSERT: String::new(),
            UPDATE: String::new(),
            FAILED_SQL_STATEMENT: String::new(),
            SUCCESS_SQL_STATEMENT: String::new(),
            METHOD_EXECUTED_SQL_STATEMENT: String::new(),
            METHOD_CREATE: String::new(),
            DATABASE_CREATED_LABEL: String::new(),
            DATABASE_CREATION_FAILED_LABEL: String::new(),
            SQL_CONNECTION_FAILED: String::new(),
            SQL_CONNECTION_RETRYING: String::new(),
            SQL_CONNECTION_RETRY: String::new(),
            METHOD_CREATE_CONNECTION: String::new(),
            METHOD_INITIALIZE: String::new(),
            LOAD_JDBC_DRIVER_FAILED_LABEL: String::new(),
        };
        this.logUtil = LogUtil::getInstance();
        this.sqlConnectionPool = SqlConnectionPool::getInstance();
        this.stringUtil = StringUtil::getInstance();
        this.commonStrings = CommonStrings::getInstance();
        this.commonSeps = CommonSeps::getInstance();
        this.stdUtil = StdUtil::getInstance();
        this.sqlTypeStrings = SqlTypeStrings::getInstance();
        this.sqlStrings = SqlStrings::getInstance();
        this.INSERT = String::from("insert");
        this.UPDATE = String::from("update");
        this.FAILED_SQL_STATEMENT = String::from("Failed\nSQL Statement: ");
        this.SUCCESS_SQL_STATEMENT = String::from("Success\nSQL Statement: ");
        this.METHOD_EXECUTED_SQL_STATEMENT = String::from("executedSQLStatement");
        this.METHOD_CREATE = String::from("create");
        this.DATABASE_CREATED_LABEL = String::from("Database Created: ");
        this.DATABASE_CREATION_FAILED_LABEL = String::from("Database Creation Failed: ");
        this.SQL_CONNECTION_FAILED = String::from("SQL Connection Failed");
        this.SQL_CONNECTION_RETRYING = String::from("SQL Connection Retrying");
        this.SQL_CONNECTION_RETRY = String::from("SQL Connection Retry");
        this.METHOD_CREATE_CONNECTION = String::from("createConnection()");
        this.METHOD_INITIALIZE = String::from("initialize");
        this.LOAD_JDBC_DRIVER_FAILED_LABEL = String::from("Load JDBC Driver Failed: ");
        this.databaseConnectionInfoInterface = databaseConnectionInfoInterface.clone();
        this.connectAttemptCounter = 0;
        this.conn = None;
        this
    }

    pub fn new(mut databaseConnectionInfoInterface: Option<Rc<RefCell<dyn crate::org::allbinary::business::init::db::DbConnectionInfo::DbConnectionInfoDyn>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(databaseConnectionInfoInterface))))
    }
    

    // throws SQLException
    pub fn executeSQLStatement(&mut self, mut statement: String) -> Option<Rc<RefCell<ResultSet>>> {
        let java_try_result = match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            self.initialize();
            let mut stmt: Option<Rc<RefCell<Statement>>> = jcall!(self.conn).createStatement();
            jcall!(stmt).execute(statement.clone());
            let mut rset: Option<Rc<RefCell<ResultSet>>> = jcall!(stmt).getResultSet();
            jcall!(stmt).close();
            let mut tempConnection: Option<Rc<RefCell<Connection>>> = self.conn.clone();
            self.conn = None;
            { let java_receiver = self.sqlConnectionPool.clone(); let java_arguments = ({ let java_argument = jcall!(self.getDatabaseConnectionInfoInterface()).getUrl(); java_argument }, tempConnection.clone(),); jcall!(java_receiver).add(java_arguments.0, java_arguments.1); };
            return Some(rset.clone());
        })) {
            Ok(java_return) => java_return,
            Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            if jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypes.clone(); java_field }).LOGGING.clone(); java_field }).contains({ let java_field = jcall!(jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypeFactory.clone(); java_field }).getInstance()).SQLLOGGINGERROR.clone(); java_field }.clone()) {
                jcall!(self.logUtil).put(format!("{}{}", self.FAILED_SQL_STATEMENT, statement), crate::java_into_object(String::from(std::any::type_name::<Self>())), self.METHOD_EXECUTED_SQL_STATEMENT.clone(), crate::java_into_object(e.clone()));
            }
            jcall!(self.conn).close();
            self.conn = None;
            panic!("{:?}", e);
            None
            }
        };if let Some(java_return) = java_try_result { return java_return; }unreachable!("try statement")
    }
    

    pub fn create(&mut self, mut databaseName: String) -> String {
        let java_try_result = match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            { let java_arguments = (format!("{}{}", { let java_field = jcall!(self.sqlStrings).CREATE_DATABASE.clone(); java_field }, databaseName),); self.executeSQLStatement(java_arguments.0) };
            if jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypes.clone(); java_field }).LOGGING.clone(); java_field }).contains({ let java_field = jcall!(jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypeFactory.clone(); java_field }).getInstance()).SQLLOGGING.clone(); java_field }.clone()) {
                jcall!(self.logUtil).putF(format!("{}{}", self.DATABASE_CREATED_LABEL, databaseName), crate::java_into_object(String::from(std::any::type_name::<Self>())), self.METHOD_CREATE.clone());
            }
            return Some(format!("{}{}", databaseName, { let java_field = jcall!(self.sqlStrings).CREATE_RETURN.clone(); java_field }));
        })) {
            Ok(java_return) => java_return,
            Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            if jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypes.clone(); java_field }).LOGGING.clone(); java_field }).contains({ let java_field = jcall!(jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypeFactory.clone(); java_field }).getInstance()).SQLLOGGINGERROR.clone(); java_field }.clone()) {
                jcall!(self.logUtil).put(format!("{}{}", self.DATABASE_CREATION_FAILED_LABEL, databaseName), crate::java_into_object(String::from(std::any::type_name::<Self>())), self.METHOD_CREATE.clone(), crate::java_into_object(e.clone()));
            }
            return format!("{}{}", self.DATABASE_CREATION_FAILED_LABEL, databaseName);
            None
            }
        };if let Some(java_return) = java_try_result { return java_return; }unreachable!("try statement")
    }
    

    pub fn createConnection(&mut self) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            self.conn = { let java_value = ({ let java_receiver = self.sqlConnectionPool.clone(); let java_arguments = ({ let java_argument = jcall!(self.getDatabaseConnectionInfoInterface()).getUrl(); java_argument },); let java_result = jcall!(java_receiver).get(java_arguments.0); java_result }).clone(); java_value };
        })) {
            Ok(_) => {}
            Err(se) => {
            let se: String = crate::java::lang::Exception::java_panic_message(&se);

            if jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypes.clone(); java_field }).LOGGING.clone(); java_field }).contains({ let java_field = jcall!(jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypeFactory.clone(); java_field }).getInstance()).SQLLOGGINGERROR.clone(); java_field }.clone()) {
                jcall!(self.logUtil).put(self.SQL_CONNECTION_RETRYING.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), self.METHOD_CREATE_CONNECTION.clone(), crate::java_into_object(se.clone()));
            }
            match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                jcall!(Thread::currentThread()).sleep(2000);
                if self.connectAttemptCounter < 10 {
                    self.connectAttemptCounter += 1;
                    self.initialize();
                }
            })) {
                Ok(_) => {}
                Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

                if jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypes.clone(); java_field }).LOGGING.clone(); java_field }).contains({ let java_field = jcall!(jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypeFactory.clone(); java_field }).getInstance()).SQLLOGGINGERROR.clone(); java_field }.clone()) {
                    { let java_receiver = self.logUtil.clone(); let java_arguments = (format!("{}{}", self.SQL_CONNECTION_RETRY, se.to_string()), crate::java_into_object(String::from(std::any::type_name::<Self>())), self.METHOD_CREATE_CONNECTION.clone(), crate::java_into_object(e.clone()),); jcall!(java_receiver).put(java_arguments.0, java_arguments.1, java_arguments.2, java_arguments.3); };
                }
                }
            }
            }
        }
    }
    

    pub fn initialize(&mut self) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            {
                match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    let mut loader: Option<Rc<RefCell<ClassLoader>>> = jcall!(Thread::currentThread()).getContextClassLoader();
                    let mut jdbcDriverClass: &'static str = jcall!(loader).loadClass({ let java_argument = jcall!(self.getDatabaseConnectionInfoInterface()).getJdbcDriver(); java_argument });
                    jcall!(jdbcDriverClass).newInstance();
                })) {
                    Ok(_) => {}
                    Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

                    if jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypes.clone(); java_field }).LOGGING.clone(); java_field }).contains({ let java_field = jcall!(jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypeFactory.clone(); java_field }).getInstance()).SQLLOGGINGERROR.clone(); java_field }.clone()) {
                        { let java_receiver = self.logUtil.clone(); let java_arguments = (format!("{}{}", self.LOAD_JDBC_DRIVER_FAILED_LABEL, { let java_value = jcall!(self.getDatabaseConnectionInfoInterface()).getJdbcDriver(); java_value }), crate::java_into_object(String::from(std::any::type_name::<Self>())), self.METHOD_INITIALIZE.clone(), crate::java_into_object(e.clone()),); jcall!(java_receiver).put(java_arguments.0, java_arguments.1, java_arguments.2, java_arguments.3); };
                    }
                    }
                }
            }
            self.createConnection();
        })) {
            Ok(_) => {}
            Err(se) => {
            let se: String = crate::java::lang::Exception::java_panic_message(&se);

            if jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypes.clone(); java_field }).LOGGING.clone(); java_field }).contains({ let java_field = jcall!(jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypeFactory.clone(); java_field }).getInstance()).SQLLOGGINGERROR.clone(); java_field }.clone()) {
                jcall!(self.logUtil).put(self.SQL_CONNECTION_FAILED.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), self.METHOD_INITIALIZE.clone(), crate::java_into_object(se.clone()));
            }
            }
        }
    }
    

    pub fn getDatabaseConnectionInfoInterface(&mut self) -> Option<Rc<RefCell<dyn DatabaseConnectionInfoInterface>>> {
        return self.databaseConnectionInfoInterface.clone().map(|reference| reference as Rc<RefCell<dyn DatabaseConnectionInfoInterface>>);
    }
    

    pub fn setDatabaseConnectionInfoInterface(&mut self, mut databaseConnectionInfoInterface: Option<Rc<RefCell<dyn crate::org::allbinary::business::init::db::DbConnectionInfo::DbConnectionInfoDyn>>>) {
        self.databaseConnectionInfoInterface = databaseConnectionInfoInterface.clone();
    }
}
pub trait AbSqlBasicDyn: std::any::Any {
    fn as_AbSqlBasic(&self) -> &AbSqlBasic;
    fn as_AbSqlBasic_mut(&mut self) -> &mut AbSqlBasic;
    fn executeSQLStatement(&mut self, mut statement: String) -> Option<Rc<RefCell<ResultSet>>> { AbSqlBasic::executeSQLStatement(self.as_AbSqlBasic_mut(), statement) }
    fn create(&mut self, mut databaseName: String) -> String { AbSqlBasic::create(self.as_AbSqlBasic_mut(), databaseName) }
    fn createConnection(&mut self) { AbSqlBasic::createConnection(self.as_AbSqlBasic_mut()) }
    fn initialize(&mut self) { AbSqlBasic::initialize(self.as_AbSqlBasic_mut()) }
    fn getDatabaseConnectionInfoInterface(&mut self) -> Option<Rc<RefCell<dyn DatabaseConnectionInfoInterface>>> { AbSqlBasic::getDatabaseConnectionInfoInterface(self.as_AbSqlBasic_mut()) }
    fn setDatabaseConnectionInfoInterface(&mut self, mut databaseConnectionInfoInterface: Option<Rc<RefCell<dyn crate::org::allbinary::business::init::db::DbConnectionInfo::DbConnectionInfoDyn>>>) { AbSqlBasic::setDatabaseConnectionInfoInterface(self.as_AbSqlBasic_mut(), databaseConnectionInfoInterface) }
}
impl AbSqlBasicDyn for AbSqlBasic {
    fn as_AbSqlBasic(&self) -> &AbSqlBasic { self }
    fn as_AbSqlBasic_mut(&mut self) -> &mut AbSqlBasic { self }
}


