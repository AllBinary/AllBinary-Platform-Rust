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
// package org.allbinary.data.tables.context.module.storefronts
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;
use super::*;
// use crate::java::util::HashMap::HashMap;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
use crate::org::allbinary::business::context::modules::storefront::StoreFront::StoreFront;
use crate::org::allbinary::business::context::modules::storefront::StoreFrontData::StoreFrontData;
use crate::org::allbinary::business::entry::EntryData::EntryData;
use crate::org::allbinary::business::init::db::UserDbInitInfo::UserDbInitInfo;
use crate::org::allbinary::logic::StdUtil::StdUtil;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::communication::sql::AbSqlBean::AbSqlBean;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use super::StoreFrontsEntityInterface::StoreFrontsEntityInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct StoreFrontsEntity {
    pub base: AbSqlBean,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub tableName: String,
}

// impl StoreFrontsEntityInterface for StoreFrontsEntity with the methods of the trait

impl StoreFrontsEntity {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: AbSqlBean::new_struct(UserDbInitInfo::new()),
            logUtil: None,
            tableName: String::new(),
        };
        this.logUtil = LogUtil::getInstance();
        this.tableName = String::from("storefronts");
        // super(...) is in the struct literal above
        this.setTableName(this.tableName.clone());
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
    

    pub fn delete(&mut self, mut value: String) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            self.base.deleteWhere2(jcall!(StoreFrontData::getInstance()).NAME.clone(), value.clone());
            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).SQLLOGGING.clone()) {
                jcall!(self.logUtil).putF(jcall!(self.commonStrings).SUCCESS.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).delete.clone());
            }
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).SQLLOGGING.clone()) {
                jcall!(self.logUtil).put(jcall!(self.commonStrings).FAILURE.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).delete.clone(), crate::java_into_object(e.clone()));
            }
            }
        }
    }
    

    // throws Exception
    pub fn getStoreFrontInterface(&mut self, mut name: String) -> Option<Rc<RefCell<StoreFront>>> {
        let mut keysAndValues: Option<Rc<RefCell<HashMap>>> = jcall!(StdUtil::getInstance()).createHashMap();
        jcall!(keysAndValues).put(jcall!(StoreFrontData::getInstance()).NAME.clone(), name.clone());
        let mut storeHashMap: Option<Rc<RefCell<HashMap>>> = self.base.getRow(keysAndValues.clone());
        if storeHashMap.is_some() {
            return StoreFront::new1(storeHashMap.clone());
        } else {
            return StoreFront::new();
        }
    }
    

    pub fn getStoreFrontNames(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        let mut storeFrontNames: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = self.base.getColumn(jcall!(StoreFrontData::getInstance()).NAME.clone());
        return storeFrontNames.clone();
    }
    

    pub fn update(&mut self, mut updatedValues: Option<Rc<RefCell<HashMap>>>) {
        self.base.updateWhere3(jcall!(StoreFrontData::getInstance()).NAME.clone(), (jcall!(updatedValues).get(jcall!(StoreFrontData::getInstance()).NAME.clone())).clone() /* as String */, updatedValues.clone());
    }
    

    pub fn createTableStatement(&mut self) -> String {
        let mut storeFrontData: Option<Rc<RefCell<StoreFrontData>>> = StoreFrontData::getInstance();
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        jcall!(stringBuffer).append(jcall!(self.sqlStrings).CREATE_TABLE.clone());
        jcall!(stringBuffer).append(self.tableName.clone());
        jcall!(stringBuffer).append(jcall!(self.sqlStrings).START.clone());
        jcall!(stringBuffer).append(jcall!(storeFrontData).NAME.clone());
        jcall!(stringBuffer).append(jcall!(self.sqlTypeStrings).SIXTY_CHAR_COLUMN_NOT_NULL.clone());
        jcall!(stringBuffer).append(jcall!(storeFrontData).HOMEHOSTNAME.clone());
        jcall!(stringBuffer).append(jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone());
        jcall!(stringBuffer).append(jcall!(storeFrontData).HOMEHOSTNAMEPATH.clone());
        jcall!(stringBuffer).append(jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone());
        jcall!(stringBuffer).append(jcall!(storeFrontData).HOSTNAME.clone());
        jcall!(stringBuffer).append(jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone());
        jcall!(stringBuffer).append(jcall!(storeFrontData).HOSTNAMEPATH.clone());
        jcall!(stringBuffer).append(jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone());
        jcall!(stringBuffer).append(jcall!(storeFrontData).TESTHOMEHOSTNAME.clone());
        jcall!(stringBuffer).append(jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone());
        jcall!(stringBuffer).append(jcall!(storeFrontData).TESTHOMEHOSTNAMEPATH.clone());
        jcall!(stringBuffer).append(jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone());
        jcall!(stringBuffer).append(jcall!(storeFrontData).TESTHOSTNAME.clone());
        jcall!(stringBuffer).append(jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone());
        jcall!(stringBuffer).append(jcall!(storeFrontData).TESTHOSTNAMEPATH.clone());
        jcall!(stringBuffer).append(jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone());
        jcall!(stringBuffer).append(jcall!(storeFrontData).IMAGEPATH.clone());
        jcall!(stringBuffer).append(jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone());
        jcall!(stringBuffer).append(jcall!(storeFrontData).STATICPATH.clone());
        jcall!(stringBuffer).append(jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone());
        jcall!(stringBuffer).append(jcall!(storeFrontData).CATEGORYPATH.clone());
        jcall!(stringBuffer).append(jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone());
        jcall!(stringBuffer).append(jcall!(storeFrontData).INVENTORYCONTROL.clone());
        jcall!(stringBuffer).append(jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone());
        jcall!(stringBuffer).append(jcall!(storeFrontData).CONFIGURATION.clone());
        jcall!(stringBuffer).append(jcall!(self.sqlTypeStrings).BLOB_NOT_NULL.clone());
        jcall!(stringBuffer).append(jcall!(storeFrontData).SUBSTORES.clone());
        jcall!(stringBuffer).append(jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone());
        jcall!(stringBuffer).append(jcall!(storeFrontData).TAGLOCATION.clone());
        jcall!(stringBuffer).append(jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone());
        jcall!(stringBuffer).append(jcall!(storeFrontData).PACKAGELOCATION.clone());
        jcall!(stringBuffer).append(jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone());
        jcall!(stringBuffer).append(jcall!(storeFrontData).FTP.clone());
        jcall!(stringBuffer).append(jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone());
        jcall!(stringBuffer).append(jcall!(storeFrontData).FTPPATH.clone());
        jcall!(stringBuffer).append(jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone());
        jcall!(stringBuffer).append(jcall!(storeFrontData).FTPUSERNAME.clone());
        jcall!(stringBuffer).append(jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone());
        jcall!(stringBuffer).append(jcall!(storeFrontData).FTPPASSWORD.clone());
        jcall!(stringBuffer).append(jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone());
        jcall!(stringBuffer).append(jcall!(storeFrontData).TESTFTP.clone());
        jcall!(stringBuffer).append(jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone());
        jcall!(stringBuffer).append(jcall!(storeFrontData).TESTFTPPATH.clone());
        jcall!(stringBuffer).append(jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone());
        jcall!(stringBuffer).append(jcall!(storeFrontData).TESTFTPUSERNAME.clone());
        jcall!(stringBuffer).append(jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone());
        jcall!(stringBuffer).append(jcall!(storeFrontData).TESTFTPPASSWORD.clone());
        jcall!(stringBuffer).append(jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone());
        jcall!(stringBuffer).append(jcall!(EntryData::getInstance()).TIMECREATED.clone());
        jcall!(stringBuffer).append(jcall!(self.sqlTypeStrings).MAX_BIG_INT_UNSIGNED_NOT_NULL.clone());
        jcall!(stringBuffer).append(jcall!(jcall!(EntryData::getInstance()).getInstance()).LASTMODIFIED.clone());
        jcall!(stringBuffer).append(jcall!(self.sqlTypeStrings).MAX_BIG_INT_UNSIGNED_NOT_NULL.clone());
        jcall!(stringBuffer).append(jcall!(self.sqlStrings).PRIMARY_KEY.clone());
        jcall!(stringBuffer).append(jcall!(storeFrontData).NAME.clone());
        jcall!(stringBuffer).append(jcall!(self.sqlStrings).END.clone());
        return jcall!(stringBuffer).toString();
    }
    

    pub fn createTable(&mut self) -> String {
        return self.base.createTable(self.createTableStatement());
    }
    

    pub fn dropTable(&mut self) -> String {
        return self.base.dropTable();
    }
}
impl std::ops::Deref for StoreFrontsEntity {
    type Target = AbSqlBean;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for StoreFrontsEntity {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::logic::communication::sql::AbSqlBasic::AbSqlBasicDyn for StoreFrontsEntity {
    fn as_AbSqlBasic(&self) -> &crate::org::allbinary::logic::communication::sql::AbSqlBasic::AbSqlBasic { &self.base.base.base.base.base }
    fn as_AbSqlBasic_mut(&mut self) -> &mut crate::org::allbinary::logic::communication::sql::AbSqlBasic::AbSqlBasic { &mut self.base.base.base.base.base }
}
impl crate::org::allbinary::logic::communication::sql::AbSqlBean::AbSqlBeanDyn for StoreFrontsEntity {
    fn as_AbSqlBean(&self) -> &crate::org::allbinary::logic::communication::sql::AbSqlBean::AbSqlBean { &self.base }
    fn as_AbSqlBean_mut(&mut self) -> &mut crate::org::allbinary::logic::communication::sql::AbSqlBean::AbSqlBean { &mut self.base }
}
impl crate::org::allbinary::logic::communication::sql::AbSqlColumn::AbSqlColumnDyn for StoreFrontsEntity {
    fn as_AbSqlColumn(&self) -> &crate::org::allbinary::logic::communication::sql::AbSqlColumn::AbSqlColumn { &self.base.base.base }
    fn as_AbSqlColumn_mut(&mut self) -> &mut crate::org::allbinary::logic::communication::sql::AbSqlColumn::AbSqlColumn { &mut self.base.base.base }
}
impl crate::org::allbinary::logic::communication::sql::AbSqlRow::AbSqlRowDyn for StoreFrontsEntity {
    fn as_AbSqlRow(&self) -> &crate::org::allbinary::logic::communication::sql::AbSqlRow::AbSqlRow { &self.base.base }
    fn as_AbSqlRow_mut(&mut self) -> &mut crate::org::allbinary::logic::communication::sql::AbSqlRow::AbSqlRow { &mut self.base.base }
    fn insert(&mut self, mut values: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) { Self::insert(self, values) }
}
impl crate::org::allbinary::logic::communication::sql::AbSqlTable::AbSqlTableDyn for StoreFrontsEntity {
    fn as_AbSqlTable(&self) -> &crate::org::allbinary::logic::communication::sql::AbSqlTable::AbSqlTable { &self.base.base.base.base }
    fn as_AbSqlTable_mut(&mut self) -> &mut crate::org::allbinary::logic::communication::sql::AbSqlTable::AbSqlTable { &mut self.base.base.base.base }
    fn dropTable(&mut self) -> String { Self::dropTable(self) }
}


