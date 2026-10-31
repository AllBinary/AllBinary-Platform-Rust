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
// package org.allbinary.logic.communication.sql.connection.pool
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;
use super::*;
// use crate::java::sql::Connection::Connection;
// use crate::java::sql::DriverManager::DriverManager;
// use crate::java::sql::SQLException::SQLException;
// use crate::java::util::HashMap::HashMap;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
use crate::org::allbinary::logic::StdUtil::StdUtil;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct SqlConnectionPool {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub stdUtil: Option<Rc<RefCell<StdUtil>>>,
    pub connectionHashMap: Option<Rc<RefCell<HashMap>>>,
    pub FIRST_NEW_CONNECTION_CREATED: String,
    pub FIRST: String,
    pub NEW_CONNECTION_FOR: String,
    pub CREATED: String,
    pub NUMBER_OF_SQL_CONNECTIONS_FOR: String,
    pub NUMBER_OF_SQL_CONNECTION_VECTORS: String,
    pub IS: String,
    pub CONNECTION_ALLREADY_CLOSED: String,
    pub METHOD_GET: String,
    pub METHOD_ADD: String,
}

thread_local! { pub static SqlConnectionPool_instance: Option<Rc<RefCell<SqlConnectionPool>>> = SqlConnectionPool::new(); }

impl SqlConnectionPool {
    

    pub fn getInstance() -> Option<Rc<RefCell<SqlConnectionPool>>> {
        return SqlConnectionPool_instance.with(Clone::clone).clone();
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            logUtil: None,
            stdUtil: None,
            connectionHashMap: None,
            FIRST_NEW_CONNECTION_CREATED: String::new(),
            FIRST: String::new(),
            NEW_CONNECTION_FOR: String::new(),
            CREATED: String::new(),
            NUMBER_OF_SQL_CONNECTIONS_FOR: String::new(),
            NUMBER_OF_SQL_CONNECTION_VECTORS: String::new(),
            IS: String::new(),
            CONNECTION_ALLREADY_CLOSED: String::new(),
            METHOD_GET: String::new(),
            METHOD_ADD: String::new(),
        };
        this.logUtil = LogUtil::getInstance();
        this.stdUtil = StdUtil::getInstance();
        this.connectionHashMap = None;
        this.FIRST_NEW_CONNECTION_CREATED = String::from("First New Connection Created: ");
        this.FIRST = String::from("First ");
        this.NEW_CONNECTION_FOR = String::from("New Connection For ");
        this.CREATED = String::from(" Created");
        this.NUMBER_OF_SQL_CONNECTIONS_FOR = String::from("Number Of Sql Connections for: ");
        this.NUMBER_OF_SQL_CONNECTION_VECTORS = String::from("Number Of Sql Connection Vectors: ");
        this.IS = String::from(" is ");
        this.CONNECTION_ALLREADY_CLOSED = String::from("Connection AllReady Closed");
        this.METHOD_GET = String::from("get()");
        this.METHOD_ADD = String::from("add()");
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    // throws SQLException
    // synchronized
    pub fn get(&mut self, mut url: String) -> Option<Rc<RefCell<Connection>>> {
        if self.connectionHashMap.is_none() {
            self.connectionHashMap = jcall!(self.stdUtil).createHashMap();
            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).SQLLOGGINGPOOL.clone()) {
                jcall!(self.logUtil).putF(format!("{}{}", self.FIRST_NEW_CONNECTION_CREATED, url), crate::java_into_object(String::from(std::any::type_name::<Self>())), self.METHOD_GET.clone());
            }
            return DriverManager::getConnection(url.clone());
        } else {
            let mut connectionVector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = (jcall!(self.connectionHashMap).get(url.clone())).clone() /* as BasicArrayList */;
            if connectionVector.is_none() {
                connectionVector = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
                if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).SQLLOGGINGPOOL.clone()) {
                    let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
                    jcall!(stringBuffer).append(self.FIRST.clone());
                    jcall!(stringBuffer).append(self.NEW_CONNECTION_FOR.clone());
                    jcall!(stringBuffer).append(url.clone());
                    jcall!(stringBuffer).append(self.CREATED.clone());
                    { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(stringBuffer).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), self.METHOD_GET.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
                }
                return DriverManager::getConnection(url.clone());
            } else if jcall!(connectionVector).size() == 0 {
                if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).SQLLOGGINGPOOL.clone()) {
                    { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(StringMaker::new()).append(self.NEW_CONNECTION_FOR.clone()).append(url.clone()).append(self.CREATED.clone()).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), self.METHOD_GET.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
                }
                return DriverManager::getConnection(url.clone());
            } else {
                let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
                let mut size: i32 = jcall!(connectionVector).size();
                for i in (0..size) {
                    let mut sqlConnection: Option<Rc<RefCell<Connection>>> = (jcall!(connectionVector).get(i.clone())).clone() /* as Connection */;
                    if !jcall!(sqlConnection).isClosed() {
                        jcall!(connectionVector).remove(crate::java_into_object(sqlConnection.clone()));
                        jcall!(self.connectionHashMap).put(url.clone(), connectionVector.clone());
                        if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).SQLLOGGINGPOOL.clone()) {
                            jcall!(stringBuffer).delete(0, jcall!(stringBuffer).length());
                            jcall!(stringBuffer).append(self.NUMBER_OF_SQL_CONNECTIONS_FOR.clone());
                            jcall!(stringBuffer).append(url.clone());
                            jcall!(stringBuffer).append(self.IS.clone());
                            jcall!(stringBuffer).appendint(jcall!(connectionVector).size());
                            { let java_receiver = self.logUtil.clone(); let java_arguments = (format!("{}{}", self.NUMBER_OF_SQL_CONNECTION_VECTORS, jcall!(self.connectionHashMap).size()), crate::java_into_object(String::from(std::any::type_name::<Self>())), self.METHOD_GET.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
                            { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(stringBuffer).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), self.METHOD_GET.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
                        }
                        return (sqlConnection).clone() /* as Connection */;
                    }
                }
            }
        }
        if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).SQLLOGGINGPOOL.clone()) {
            let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
            jcall!(stringBuffer).append(self.NEW_CONNECTION_FOR.clone());
            jcall!(stringBuffer).append(url.clone());
            jcall!(stringBuffer).append(self.CREATED.clone());
            { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(stringBuffer).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), self.METHOD_GET.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        }
        return DriverManager::getConnection(url.clone());
    }
    

    // throws SQLException
    // synchronized
    pub fn add(&mut self, mut url: String, mut sqlConnection: Option<Rc<RefCell<Connection>>>) {
        if !jcall!(sqlConnection).isClosed() {
            let mut connectionVector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = None;
            if self.connectionHashMap.is_none() {
                self.connectionHashMap = jcall!(self.stdUtil).createHashMap();
                connectionVector = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
                jcall!(connectionVector).add(crate::java_into_object(sqlConnection.clone()));
            } else {
                connectionVector = (jcall!(self.connectionHashMap).get(url.clone())).clone() /* as BasicArrayList */;
                if connectionVector.is_none() {
                    connectionVector = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
                }
                jcall!(connectionVector).add(crate::java_into_object(sqlConnection.clone()));
                jcall!(self.connectionHashMap).put(url.clone(), connectionVector.clone());
            }
            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).SQLLOGGINGPOOL.clone()) {
                let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
                jcall!(stringBuffer).append(self.NUMBER_OF_SQL_CONNECTIONS_FOR.clone());
                jcall!(stringBuffer).append(url.clone());
                jcall!(stringBuffer).append(self.IS.clone());
                jcall!(stringBuffer).appendint(jcall!(connectionVector).size());
                { let java_receiver = self.logUtil.clone(); let java_arguments = (format!("{}{}", self.NUMBER_OF_SQL_CONNECTION_VECTORS, jcall!(self.connectionHashMap).size()), crate::java_into_object(String::from(std::any::type_name::<Self>())), self.METHOD_ADD.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
                { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(stringBuffer).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), self.METHOD_ADD.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
            }
        }
        if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).SQLLOGGINGPOOL.clone()) {
            jcall!(self.logUtil).putF(self.CONNECTION_ALLREADY_CLOSED.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), self.METHOD_ADD.clone());
        }
    }
}

