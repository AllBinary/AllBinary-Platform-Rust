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
// package org.allbinary.business.init
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::java::util::HashMap::HashMap;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
use crate::org::allbinary::business::init::db::UserDbInitInfo::UserDbInitInfo;
use crate::org::allbinary::logic::StdUtil::StdUtil;
use crate::org::allbinary::logic::communication::log::PreLogUtil::PreLogUtil;
use crate::org::allbinary::logic::communication::log::config::r#type::LogConfigTypeFactory::LogConfigTypeFactory;
use crate::org::allbinary::logic::communication::log::config::r#type::LogConfigTypes::LogConfigTypes;
use crate::org::allbinary::logic::io::path::AbPathData::AbPathData;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::java::lang::Exception::*;
use super::InitSql::InitSql;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct InitInfoEntity {
    pub base: InitSql,
    pub NAME: String,
    pub NOTHING: String,
    pub tableName: String,
    pub tableData: String,
    pub NOT_IN_DB: String,
    pub METHOD_GET: String,
    pub METHOD_IS: String,
    pub METHOD_ADD: String,
    pub CREATED_SUCCESS: String,
    pub FAILED_TO_CREATE: String,
}


impl InitInfoEntity {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: InitSql::new_struct(UserDbInitInfo::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::business::init::db::DbConnectionInfo::DbConnectionInfoDyn>>)),
            NAME: String::new(),
            NOTHING: String::new(),
            tableName: String::new(),
            tableData: String::new(),
            NOT_IN_DB: String::new(),
            METHOD_GET: String::new(),
            METHOD_IS: String::new(),
            METHOD_ADD: String::new(),
            CREATED_SUCCESS: String::new(),
            FAILED_TO_CREATE: String::new(),
        };
        this.NAME = String::from("InitInfoEntity");
        this.NOTHING = String::from("NOTHING");
        this.tableName = String::from("initdata");
        this.NOT_IN_DB = String::from("Not In DB");
        this.METHOD_GET = String::from("get()");
        this.METHOD_IS = String::from("is()");
        this.METHOD_ADD = String::from("add()");
        this.CREATED_SUCCESS = String::from(" Created Successfully");
        this.FAILED_TO_CREATE = String::from("Failed to create ");
        // super(...) is in the struct literal above
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        jcall!(stringBuffer).append({ let java_field = jcall!(this.sqlStrings).CREATE_TABLE.clone(); java_field }.clone());
        jcall!(stringBuffer).append(this.tableName.clone());
        jcall!(stringBuffer).append({ let java_field = jcall!(this.sqlStrings).START.clone(); java_field }.clone());
        jcall!(stringBuffer).append(this.NOTHING.clone());
        jcall!(stringBuffer).append({ let java_field = jcall!(this.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone(); java_field }.clone());
        jcall!(stringBuffer).append({ let java_field = jcall!(InitInfo::InitInfo::getInstance()).TESTING.clone(); java_field }.clone());
        jcall!(stringBuffer).append({ let java_field = jcall!(this.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone(); java_field }.clone());
        jcall!(stringBuffer).append({ let java_field = jcall!(InitInfo::InitInfo::getInstance()).TESTHTMLPATH.clone(); java_field }.clone());
        jcall!(stringBuffer).append({ let java_field = jcall!(this.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone(); java_field }.clone());
        jcall!(stringBuffer).append({ let java_field = jcall!(InitInfo::InitInfo::getInstance()).MAINPATH.clone(); java_field }.clone());
        jcall!(stringBuffer).append({ let java_field = jcall!(this.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone(); java_field }.clone());
        jcall!(stringBuffer).append({ let java_field = jcall!(this.sqlStrings).PRIMARY_KEY.clone(); java_field }.clone());
        jcall!(stringBuffer).append(this.NOTHING.clone());
        jcall!(stringBuffer).append({ let java_field = jcall!(this.sqlStrings).END.clone(); java_field }.clone());
        this.tableData = { let java_value = (jcall!(stringBuffer).toString()).clone(); java_value };
        { let java_arguments = (this.tableName.clone(),); this.setTable(java_arguments.0) };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn get(&mut self) -> bool {
        let java_try_result = match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut keyAndValue: Option<Rc<RefCell<HashMap<crate::JavaObject, crate::JavaObject>>>> = jcall!(StdUtil::getInstance()).createHashMap();
            jcall!(keyAndValue).put(crate::java_into_object(self.NOTHING.clone()), crate::java_into_object(self.NOTHING.clone()));
            let mut hashMap: Option<Rc<RefCell<HashMap<crate::JavaObject, crate::JavaObject>>>> = self.getRow(keyAndValue.clone());
            if hashMap.is_some() {
                jcall!(InitInfo::InitInfo::getInstance()).set1(hashMap.clone());
                return Some(true);
            } else {
                if jcall!(unsafe { crate::org::allbinary::logic::communication::log::config::r#type::LogConfigTypes::LogConfigTypes_LOGGING.clone() }).contains(crate::java_into_object({ let java_field = jcall!(LogConfigTypeFactory::getInstance()).PRELOADER.clone(); java_field }.clone())) {
                    PreLogUtil::put(self.NOT_IN_DB.clone(), crate::java_into_object(self.NAME.clone()), self.METHOD_GET.clone());
                }
                return Some(false);
            }None
        })) {
            Ok(java_return) => java_return,
            Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            if jcall!(unsafe { crate::org::allbinary::logic::communication::log::config::r#type::LogConfigTypes::LogConfigTypes_LOGGING.clone() }).contains(crate::java_into_object({ let java_field = jcall!(LogConfigTypeFactory::getInstance()).PRELOADERERROR.clone(); java_field }.clone())) {
                PreLogUtil::putOE({ let java_field = jcall!(self.commonStrings).EXCEPTION.clone(); java_field }.clone(), crate::java_into_object(self.NAME.clone()), self.METHOD_GET.clone(), crate::java_into_object(e.clone()));
            }
            return false;
            None
            }
        };if let Some(java_return) = java_try_result { return java_return; }unreachable!("try statement")
    }
    

    pub fn is(&mut self) -> bool {
        let java_try_result = match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut keyAndValue: Option<Rc<RefCell<HashMap<crate::JavaObject, crate::JavaObject>>>> = jcall!(StdUtil::getInstance()).createHashMap();
            jcall!(keyAndValue).put(crate::java_into_object(self.NOTHING.clone()), crate::java_into_object(self.NOTHING.clone()));
            let mut hashMap: Option<Rc<RefCell<HashMap<crate::JavaObject, crate::JavaObject>>>> = self.getRow(keyAndValue.clone());
            if hashMap.is_some() {
                return Some(true);
            } else {
                if jcall!(unsafe { crate::org::allbinary::logic::communication::log::config::r#type::LogConfigTypes::LogConfigTypes_LOGGING.clone() }).contains(crate::java_into_object({ let java_field = jcall!(LogConfigTypeFactory::getInstance()).PRELOADER.clone(); java_field }.clone())) {
                    PreLogUtil::put(self.NOT_IN_DB.clone(), crate::java_into_object(self.NAME.clone()), self.METHOD_IS.clone());
                }
                return Some(false);
            }None
        })) {
            Ok(java_return) => java_return,
            Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            if jcall!(unsafe { crate::org::allbinary::logic::communication::log::config::r#type::LogConfigTypes::LogConfigTypes_LOGGING.clone() }).contains(crate::java_into_object({ let java_field = jcall!(LogConfigTypeFactory::getInstance()).PRELOADERERROR.clone(); java_field }.clone())) {
                PreLogUtil::putOE({ let java_field = jcall!(self.commonStrings).EXCEPTION.clone(); java_field }.clone(), crate::java_into_object(self.NAME.clone()), self.METHOD_IS.clone(), crate::java_into_object(e.clone()));
            }
            return false;
            None
            }
        };if let Some(java_return) = java_try_result { return java_return; }unreachable!("try statement")
    }
    

    pub fn update(&mut self) {
        { let java_arguments = (self.NOTHING.clone(), self.NOTHING.clone(), jcall!(InitInfo::InitInfo::getInstance()).toHashMap(),); self.updateWhere(java_arguments.0, java_arguments.1, java_arguments.2) };
    }
    

    pub fn add(&mut self) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut values: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
            jcall!(values).add(crate::java_into_object(self.NOTHING.clone()));
            jcall!(values).add(crate::java_into_object(jcall!(InitInfo::InitInfo::getInstance()).getTesting()));
            jcall!(values).add(crate::java_into_object(jcall!(InitInfo::InitInfo::getInstance()).getTestHtmlPath()));
            jcall!(values).add(crate::java_into_object(jcall!(InitInfo::InitInfo::getInstance()).getMainPath()));
            self.insert(values.clone());
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            if jcall!(unsafe { crate::org::allbinary::logic::communication::log::config::r#type::LogConfigTypes::LogConfigTypes_LOGGING.clone() }).contains(crate::java_into_object({ let java_field = jcall!(LogConfigTypeFactory::getInstance()).PRELOADERERROR.clone(); java_field }.clone())) {
                PreLogUtil::putOE({ let java_field = jcall!(self.commonStrings).EXCEPTION.clone(); java_field }.clone(), crate::java_into_object(self.NAME.clone()), self.METHOD_ADD.clone(), crate::java_into_object(e.clone()));
            }
            }
        }
    }
    

    pub fn createTable(&mut self) -> String {
        if { let java_arguments = (self.tableData.clone(),); self.base.createTable(java_arguments.0) } {
            return format!("{}{}", self.tableName, self.CREATED_SUCCESS);
        } else {
            let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
            jcall!(stringBuffer).append(self.FAILED_TO_CREATE.clone());
            jcall!(stringBuffer).append(self.tableData.clone());
            jcall!(stringBuffer).append({ let java_field = jcall!(AbPathData::getInstance()).EXTENSION_SEP.clone(); java_field }.clone());
            return jcall!(stringBuffer).toString();
        }
    }
}
impl std::ops::Deref for InitInfoEntity {
    type Target = InitSql;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for InitInfoEntity {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::business::init::InitSql::InitSqlDyn for InitInfoEntity {
    fn as_InitSql(&self) -> &crate::org::allbinary::business::init::InitSql::InitSql { &self.base }
    fn as_InitSql_mut(&mut self) -> &mut crate::org::allbinary::business::init::InitSql::InitSql { &mut self.base }
}


