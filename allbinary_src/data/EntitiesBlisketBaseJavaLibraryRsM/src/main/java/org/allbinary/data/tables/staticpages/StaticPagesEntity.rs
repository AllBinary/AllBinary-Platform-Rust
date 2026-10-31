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
// package org.allbinary.data.tables.staticpages
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;
use super::*;
// use crate::java::util::HashMap::HashMap;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
use crate::org::allbinary::business::context::modules::storefront::StoreFrontData::StoreFrontData;
use crate::org::allbinary::business::init::db::StaticPagesDbInitInfo::StaticPagesDbInitInfo;
use crate::org::allbinary::business::user::commerce::inventory::item::BasicItemData::BasicItemData;
use crate::org::allbinary::logic::StdUtil::StdUtil;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::communication::sql::AbSqlBean::AbSqlBean;
use crate::org::allbinary::logic::control::search::SearchData::SearchData;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::string::CommonSeps::CommonSeps;
use super::StaticPagesEntityInterface::StaticPagesEntityInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct StaticPagesEntity {
    pub base: AbSqlBean,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub basicItemData: Option<Rc<RefCell<BasicItemData>>>,
    pub TABLENAME: String,
}

impl StaticPagesEntityInterface for StaticPagesEntity {
    fn getFile(&mut self, mut store: String, mut keywords: String) -> String { Self::getFile(self, store, keywords) }
    fn delete(&mut self, mut keywords: String) { Self::delete(self, keywords) }
    fn insert(&mut self, mut values: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) { Self::insert(self, values) }
    fn update(&mut self, mut updatedValues: Option<Rc<RefCell<HashMap>>>) { Self::update(self, updatedValues) }
}
impl crate::org::allbinary::data::tables::BasicDataTableInterface::BasicDataTableInterface for StaticPagesEntity {
    fn createTable(&mut self) -> String { Self::createTable(self) }
    fn dropTable(&mut self) -> String { Self::dropTable(self) }
}

impl StaticPagesEntity {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: AbSqlBean::new_struct(StaticPagesDbInitInfo::new()),
            logUtil: None,
            basicItemData: None,
            TABLENAME: String::new(),
        };
        this.logUtil = LogUtil::getInstance();
        this.basicItemData = BasicItemData::getInstance();
        this.TABLENAME = String::from("staticpages");
        // super(...) is in the struct literal above
        this.setTableName(this.TABLENAME.clone());
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn insert(&mut self, mut values: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            self.base.insert(values.clone());
            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).SQLLOGGING.clone()) {
                jcall!(self.logUtil).putF(jcall!(self.commonStrings).SUCCESS.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), INSERT.clone());
            }
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).SQLLOGGING.clone()) {
                jcall!(self.logUtil).put(jcall!(self.commonStrings).FAILURE.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), INSERT.clone(), crate::java_into_object(e.clone()));
            }
            }
        }
    }
    

    pub fn getFile(&mut self, mut store: String, mut keywords: String) -> String {
        let mut whereHashMap: Option<Rc<RefCell<HashMap>>> = jcall!(StdUtil::getInstance()).createHashMap();
        jcall!(whereHashMap).put(jcall!(StoreFrontData::getInstance()).NAME.clone(), store.clone());
        jcall!(whereHashMap).put(jcall!(self.basicItemData).KEYWORDS.clone(), keywords.clone());
        let mut file: String = self.base.getField(whereHashMap.clone(), crate::org::allbinary::logic::control::search::SearchData::SearchData_PAGE.with(Clone::clone).clone());
        return file.clone();
    }
    

    pub fn delete(&mut self, mut keywords: String) {
        self.base.deleteWhere2(jcall!(self.basicItemData).KEYWORDS.clone(), keywords.clone());
    }
    

    pub fn dropTable(&mut self) -> String {
        let mut returnStr: String = self.base.dropTable();
        return returnStr.clone();
    }
    

    pub fn update(&mut self, mut updatedValues: Option<Rc<RefCell<HashMap>>>) {
        self.base.updateWhere3(jcall!(self.basicItemData).KEYWORDS.clone(), (jcall!(updatedValues).get(jcall!(self.basicItemData).KEYWORDS.clone())).clone() /* as String */, updatedValues.clone());
    }
    

    pub fn createTableStatement(&mut self) -> String {
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        jcall!(stringBuffer).append(jcall!(self.sqlStrings).CREATE_TABLE.clone()).append(self.TABLENAME.clone()).append(jcall!(self.sqlStrings).START.clone()).append(jcall!(StoreFrontData::getInstance()).NAME.clone()).append(jcall!(self.sqlTypeStrings).SIXTY_CHAR_COLUMN_NOT_NULL.clone()).append(jcall!(self.basicItemData).KEYWORDS.clone()).append(jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone()).append(crate::org::allbinary::logic::control::search::SearchData::SearchData_PAGE.with(Clone::clone).clone()).append(jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone()).append(jcall!(self.sqlStrings).PRIMARY_KEY.clone()).append(jcall!(StoreFrontData::getInstance()).NAME.clone()).append(jcall!(CommonSeps::getInstance()).COMMA_SEP.clone()).append(jcall!(self.basicItemData).KEYWORDS.clone()).append(jcall!(self.sqlStrings).END.clone());
        return jcall!(stringBuffer).toString();
    }
    

    pub fn createTable(&mut self) -> String {
        return self.base.createTable(self.createTableStatement());
    }
}
impl std::ops::Deref for StaticPagesEntity {
    type Target = AbSqlBean;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for StaticPagesEntity {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::logic::communication::sql::AbSqlBasic::AbSqlBasicDyn for StaticPagesEntity {
    fn as_AbSqlBasic(&self) -> &crate::org::allbinary::logic::communication::sql::AbSqlBasic::AbSqlBasic { &self.base.base.base.base.base }
    fn as_AbSqlBasic_mut(&mut self) -> &mut crate::org::allbinary::logic::communication::sql::AbSqlBasic::AbSqlBasic { &mut self.base.base.base.base.base }
}
impl crate::org::allbinary::logic::communication::sql::AbSqlBean::AbSqlBeanDyn for StaticPagesEntity {
    fn as_AbSqlBean(&self) -> &crate::org::allbinary::logic::communication::sql::AbSqlBean::AbSqlBean { &self.base }
    fn as_AbSqlBean_mut(&mut self) -> &mut crate::org::allbinary::logic::communication::sql::AbSqlBean::AbSqlBean { &mut self.base }
}
impl crate::org::allbinary::logic::communication::sql::AbSqlColumn::AbSqlColumnDyn for StaticPagesEntity {
    fn as_AbSqlColumn(&self) -> &crate::org::allbinary::logic::communication::sql::AbSqlColumn::AbSqlColumn { &self.base.base.base }
    fn as_AbSqlColumn_mut(&mut self) -> &mut crate::org::allbinary::logic::communication::sql::AbSqlColumn::AbSqlColumn { &mut self.base.base.base }
}
impl crate::org::allbinary::logic::communication::sql::AbSqlRow::AbSqlRowDyn for StaticPagesEntity {
    fn as_AbSqlRow(&self) -> &crate::org::allbinary::logic::communication::sql::AbSqlRow::AbSqlRow { &self.base.base }
    fn as_AbSqlRow_mut(&mut self) -> &mut crate::org::allbinary::logic::communication::sql::AbSqlRow::AbSqlRow { &mut self.base.base }
    fn insert(&mut self, mut values: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) { Self::insert(self, values) }
}
impl crate::org::allbinary::logic::communication::sql::AbSqlTable::AbSqlTableDyn for StaticPagesEntity {
    fn as_AbSqlTable(&self) -> &crate::org::allbinary::logic::communication::sql::AbSqlTable::AbSqlTable { &self.base.base.base.base }
    fn as_AbSqlTable_mut(&mut self) -> &mut crate::org::allbinary::logic::communication::sql::AbSqlTable::AbSqlTable { &mut self.base.base.base.base }
    fn dropTable(&mut self) -> String { Self::dropTable(self) }
}


