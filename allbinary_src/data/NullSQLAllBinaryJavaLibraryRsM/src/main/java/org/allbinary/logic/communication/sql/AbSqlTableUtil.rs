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
use crate::org::allbinary::business::installer::Portion::Portion;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use crate::org::allbinary::string::CommonSeps::CommonSeps;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use super::SqlStrings::SqlStrings;
use super::AbSqlTable::AbSqlTable;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct AbSqlTableUtil {
    pub TABLE_LABEL: String,
    pub stringUtil: Option<Rc<RefCell<StringUtil>>>,
    pub commonStrings: Option<Rc<RefCell<CommonStrings>>>,
    pub commonSeps: Option<Rc<RefCell<CommonSeps>>>,
    pub sqlStrings: Option<Rc<RefCell<SqlStrings>>>,
}

thread_local! { pub static AbSqlTableUtil_instance: Option<Rc<RefCell<AbSqlTableUtil>>> = AbSqlTableUtil::new(); }

impl AbSqlTableUtil {

    pub fn new_struct() -> Self {
        let mut this = Self {
            TABLE_LABEL: String::new(),
            stringUtil: None,
            commonStrings: None,
            commonSeps: None,
            sqlStrings: None,
        };
        this.TABLE_LABEL = String::from("Table: ");
        this.stringUtil = StringUtil::getInstance();
        this.commonStrings = CommonStrings::getInstance();
        this.commonSeps = CommonSeps::getInstance();
        this.sqlStrings = SqlStrings::getInstance();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<AbSqlTableUtil>>> {
        return AbSqlTableUtil_instance.with(Clone::clone).clone();
    }
    

    // synchronized
    pub fn backupTable(&mut self, mut abSqlTable: Option<Rc<RefCell<dyn crate::org::allbinary::logic::communication::sql::AbSqlTable::AbSqlTableDyn>>>) -> String {
        let mut tableName: String = jcall!(abSqlTable).getTableName();
        return format!("{}{}", format!("{}{}", self.TABLE_LABEL, tableName), String::from(" Backup Failed"));
    }
    

    // synchronized
    pub fn restoreTable(&mut self, mut abSqlTable: Option<Rc<RefCell<dyn crate::org::allbinary::logic::communication::sql::AbSqlTable::AbSqlTableDyn>>>, mut portion: Option<Rc<RefCell<Portion>>>) -> String {
        let mut tableName: String = jcall!(abSqlTable).getTableName();
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        jcall!(stringBuffer).append(self.TABLE_LABEL.clone());
        jcall!(stringBuffer).append(tableName.clone());
        jcall!(stringBuffer).append(String::from(" Restoration Failed"));
        return jcall!(stringBuffer).toString();
    }
}

