// 
// AllBinary Open License Version 1 
// Copyright (c) 2002 AllBinary 
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
// package org.allbinary.data.tables.generator
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
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::communication::sql::AbSqlBean::AbSqlBean;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use super::IdGeneratorEntityInterface::IdGeneratorEntityInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct IdGeneratorEntity {
    pub base: AbSqlBean,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub tableName: String,
    pub tableData: String,
    pub NAME: String,
    pub VALUE: String,
}

// impl IdGeneratorEntityInterface for IdGeneratorEntity with the methods of the trait

impl IdGeneratorEntity {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: AbSqlBean::new_struct(UserDbInitInfo::new()),
            logUtil: None,
            tableName: String::new(),
            tableData: String::new(),
            NAME: String::new(),
            VALUE: String::new(),
        };
        this.logUtil = LogUtil::getInstance();
        this.tableName = String::from("idgenerator");
        this.NAME = String::from("NAME");
        this.VALUE = String::from("VALUE");
        // super(...) is in the struct literal above
        this.tableData = jcall!(StringMaker::new()).append(jcall!(this.sqlStrings).CREATE_TABLE.clone()).append(this.tableName.clone()).append(jcall!(this.sqlStrings).START.clone()).append(this.NAME.clone()).append(jcall!(this.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone()).append(this.VALUE.clone()).append(jcall!(this.sqlTypeStrings).MAX_BIG_INT_UNSIGNED_NOT_NULL.clone()).append(jcall!(this.sqlStrings).PRIMARY_KEY.clone()).append(this.NAME.clone()).append(jcall!(this.sqlStrings).END.clone()).toString();
        this.base.setTableName(this.tableName.clone());
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    // throws Exception
    pub fn get(&mut self, mut name: String) -> Option<i64> {
        let mut keysAndValues: Option<Rc<RefCell<HashMap>>> = jcall!(StdUtil::getInstance()).createHashMap();
        jcall!(keysAndValues).put(self.NAME.clone(), name.clone());
        let mut hashMap: Option<Rc<RefCell<HashMap>>> = self.base.getRow(keysAndValues.clone());
        if jcall!(((jcall!(hashMap).get(self.NAME.clone())).clone() /* as String */)).compareTo(name.clone()) != 0 {
            panic!("{:?}", format!("{}: {:?}", "Exception", (String::from("results do not match"))));
        }
        let mut value: String = (jcall!(hashMap).get(self.VALUE.clone())).clone() /* as String */;
        return value.clone().parse::<i64>().unwrap();
    }
    

    pub fn insert(&mut self, mut values: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            self.base.insert(values.clone());
            jcall!(self.logUtil).putF(jcall!(self.commonStrings).SUCCESS.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), INSERT.clone());
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            jcall!(self.logUtil).put(jcall!(self.commonStrings).FAILURE.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), INSERT.clone(), crate::java_into_object(e.clone()));
            }
        }
    }
    

    pub fn delete(&mut self, mut value: String) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            self.base.deleteWhere2(self.NAME.clone(), value.clone());
            jcall!(self.logUtil).putF(jcall!(self.commonStrings).SUCCESS.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).delete.clone());
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            jcall!(self.logUtil).put(jcall!(self.commonStrings).FAILURE.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).delete.clone(), crate::java_into_object(e.clone()));
            }
        }
    }
    

    pub fn update2(&mut self, mut name: String, mut value: Option<i64>) {
        let mut map: Option<Rc<RefCell<HashMap>>> = jcall!(StdUtil::getInstance()).createHashMap();
        jcall!(map).put(self.NAME.clone(), name.clone());
        jcall!(map).put(self.VALUE.clone(), value.clone());
        self.update(map.clone());
    }
    

    // Java overloaded method: Rust needs a different name
    pub fn update(&mut self, mut hashMap: Option<Rc<RefCell<HashMap>>>) {
        self.base.updateWhere3(self.NAME.clone(), (jcall!(hashMap).get(self.NAME.clone())).clone() /* as String */, hashMap.clone());
    }
    

    pub fn createTable(&mut self) -> String {
        let mut returnStr: String = self.base.createTable(self.tableData.clone());
        return returnStr.clone();
    }
    

    pub fn dropTable(&mut self) -> String {
        return self.base.dropTable();
    }
}
impl std::ops::Deref for IdGeneratorEntity {
    type Target = AbSqlBean;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for IdGeneratorEntity {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::logic::communication::sql::AbSqlBasic::AbSqlBasicDyn for IdGeneratorEntity {
    fn as_AbSqlBasic(&self) -> &crate::org::allbinary::logic::communication::sql::AbSqlBasic::AbSqlBasic { &self.base.base.base.base.base }
    fn as_AbSqlBasic_mut(&mut self) -> &mut crate::org::allbinary::logic::communication::sql::AbSqlBasic::AbSqlBasic { &mut self.base.base.base.base.base }
}
impl crate::org::allbinary::logic::communication::sql::AbSqlBean::AbSqlBeanDyn for IdGeneratorEntity {
    fn as_AbSqlBean(&self) -> &crate::org::allbinary::logic::communication::sql::AbSqlBean::AbSqlBean { &self.base }
    fn as_AbSqlBean_mut(&mut self) -> &mut crate::org::allbinary::logic::communication::sql::AbSqlBean::AbSqlBean { &mut self.base }
}
impl crate::org::allbinary::logic::communication::sql::AbSqlColumn::AbSqlColumnDyn for IdGeneratorEntity {
    fn as_AbSqlColumn(&self) -> &crate::org::allbinary::logic::communication::sql::AbSqlColumn::AbSqlColumn { &self.base.base.base }
    fn as_AbSqlColumn_mut(&mut self) -> &mut crate::org::allbinary::logic::communication::sql::AbSqlColumn::AbSqlColumn { &mut self.base.base.base }
}
impl crate::org::allbinary::logic::communication::sql::AbSqlRow::AbSqlRowDyn for IdGeneratorEntity {
    fn as_AbSqlRow(&self) -> &crate::org::allbinary::logic::communication::sql::AbSqlRow::AbSqlRow { &self.base.base }
    fn as_AbSqlRow_mut(&mut self) -> &mut crate::org::allbinary::logic::communication::sql::AbSqlRow::AbSqlRow { &mut self.base.base }
    fn insert(&mut self, mut values: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) { Self::insert(self, values) }
}
impl crate::org::allbinary::logic::communication::sql::AbSqlTable::AbSqlTableDyn for IdGeneratorEntity {
    fn as_AbSqlTable(&self) -> &crate::org::allbinary::logic::communication::sql::AbSqlTable::AbSqlTable { &self.base.base.base.base }
    fn as_AbSqlTable_mut(&mut self) -> &mut crate::org::allbinary::logic::communication::sql::AbSqlTable::AbSqlTable { &mut self.base.base.base.base }
    fn dropTable(&mut self) -> String { Self::dropTable(self) }
}


