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
use std::collections::HashMap;
use super::*;
// use crate::java::util::HashMap::HashMap;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
use crate::org::allbinary::business::init::db::UserDbInitInfo::UserDbInitInfo;
use crate::org::allbinary::logic::StdUtil::StdUtil;
use crate::org::allbinary::logic::communication::log::PreLogUtil::PreLogUtil;
use crate::org::allbinary::logic::communication::log::config::r#type::LogConfigTypeFactory::LogConfigTypeFactory;
use crate::org::allbinary::logic::communication::log::config::r#type::LogConfigTypes::LogConfigTypes;
use crate::org::allbinary::logic::io::path::AbPathData::AbPathData;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
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
            base: InitSql::new_struct(UserDbInitInfo::new()),
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
        jcall!(stringBuffer).append(jcall!(this.sqlStrings).CREATE_TABLE.clone());
        jcall!(stringBuffer).append(this.tableName.clone());
        jcall!(stringBuffer).append(jcall!(this.sqlStrings).START.clone());
        jcall!(stringBuffer).append(this.NOTHING.clone());
        jcall!(stringBuffer).append(jcall!(this.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone());
        jcall!(stringBuffer).append(jcall!(InitInfo::InitInfo::getInstance()).TESTING.clone());
        jcall!(stringBuffer).append(jcall!(this.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone());
        jcall!(stringBuffer).append(jcall!(InitInfo::InitInfo::getInstance()).TESTHTMLPATH.clone());
        jcall!(stringBuffer).append(jcall!(this.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone());
        jcall!(stringBuffer).append(jcall!(InitInfo::InitInfo::getInstance()).MAINPATH.clone());
        jcall!(stringBuffer).append(jcall!(this.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone());
        jcall!(stringBuffer).append(jcall!(this.sqlStrings).PRIMARY_KEY.clone());
        jcall!(stringBuffer).append(this.NOTHING.clone());
        jcall!(stringBuffer).append(jcall!(this.sqlStrings).END.clone());
        this.tableData = jcall!(stringBuffer).toString();
        this.setTable(this.tableName.clone());
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn get(&mut self) -> bool {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut keyAndValue: Option<Rc<RefCell<HashMap>>> = jcall!(StdUtil::getInstance()).createHashMap();
            jcall!(keyAndValue).put(self.NOTHING.clone(), self.NOTHING.clone());
            let mut hashMap: Option<Rc<RefCell<HashMap>>> = self.getRow(keyAndValue.clone());
            if hashMap.is_some() {
                jcall!(InitInfo::InitInfo::getInstance()).set1(hashMap.clone());
                return true;
            } else {
                if jcall!(unsafe { crate::org::allbinary::logic::communication::log::config::r#type::LogConfigTypes::LogConfigTypes_LOGGING.clone() }).contains(crate::java_into_object(jcall!(LogConfigTypeFactory::getInstance()).PRELOADER.clone())) {
                    PreLogUtil::put(self.NOT_IN_DB.clone(), self.NAME.clone(), self.METHOD_GET.clone());
                }
                return false;
            }
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            if jcall!(unsafe { crate::org::allbinary::logic::communication::log::config::r#type::LogConfigTypes::LogConfigTypes_LOGGING.clone() }).contains(crate::java_into_object(jcall!(LogConfigTypeFactory::getInstance()).PRELOADERERROR.clone())) {
                PreLogUtil::putOE(jcall!(self.commonStrings).EXCEPTION.clone(), self.NAME.clone(), self.METHOD_GET.clone(), e.clone());
            }
            return false;
            }
        }
    }
    

    pub fn is(&mut self) -> bool {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut keyAndValue: Option<Rc<RefCell<HashMap>>> = jcall!(StdUtil::getInstance()).createHashMap();
            jcall!(keyAndValue).put(self.NOTHING.clone(), self.NOTHING.clone());
            let mut hashMap: Option<Rc<RefCell<HashMap>>> = self.getRow(keyAndValue.clone());
            if hashMap.is_some() {
                return true;
            } else {
                if jcall!(unsafe { crate::org::allbinary::logic::communication::log::config::r#type::LogConfigTypes::LogConfigTypes_LOGGING.clone() }).contains(crate::java_into_object(jcall!(LogConfigTypeFactory::getInstance()).PRELOADER.clone())) {
                    PreLogUtil::put(self.NOT_IN_DB.clone(), self.NAME.clone(), self.METHOD_IS.clone());
                }
                return false;
            }
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            if jcall!(unsafe { crate::org::allbinary::logic::communication::log::config::r#type::LogConfigTypes::LogConfigTypes_LOGGING.clone() }).contains(crate::java_into_object(jcall!(LogConfigTypeFactory::getInstance()).PRELOADERERROR.clone())) {
                PreLogUtil::putOE(jcall!(self.commonStrings).EXCEPTION.clone(), self.NAME.clone(), self.METHOD_IS.clone(), e.clone());
            }
            return false;
            }
        }
    }
    

    pub fn update(&mut self) {
        { let java_arguments = (self.NOTHING.clone(), self.NOTHING.clone(), jcall!(InitInfo::InitInfo::getInstance()).toHashMap(),); self.updateWhere(java_arguments.0, java_arguments.1, java_arguments.2) };
    }
    

    pub fn add(&mut self) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut values: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
            jcall!(values).add(crate::java_into_object(self.NOTHING.clone()));
            jcall!(values).add(crate::java_into_object(jcall!(InitInfo::InitInfo::getInstance()).getTesting()));
            jcall!(values).add(crate::java_into_object(jcall!(InitInfo::InitInfo::getInstance()).getTestHtmlPath()));
            jcall!(values).add(crate::java_into_object(jcall!(InitInfo::InitInfo::getInstance()).getMainPath()));
            self.insert(values.clone());
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            if jcall!(unsafe { crate::org::allbinary::logic::communication::log::config::r#type::LogConfigTypes::LogConfigTypes_LOGGING.clone() }).contains(crate::java_into_object(jcall!(LogConfigTypeFactory::getInstance()).PRELOADERERROR.clone())) {
                PreLogUtil::putOE(jcall!(self.commonStrings).EXCEPTION.clone(), self.NAME.clone(), self.METHOD_ADD.clone(), e.clone());
            }
            }
        }
    }
    

    pub fn createTable(&mut self) -> String {
        if self.base.createTable(self.tableData.clone()) {
            return format!("{}{}", self.tableName, self.CREATED_SUCCESS);
        } else {
            let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
            jcall!(stringBuffer).append(self.FAILED_TO_CREATE.clone());
            jcall!(stringBuffer).append(self.tableData.clone());
            jcall!(stringBuffer).append(jcall!(AbPathData::getInstance()).EXTENSION_SEP.clone());
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


