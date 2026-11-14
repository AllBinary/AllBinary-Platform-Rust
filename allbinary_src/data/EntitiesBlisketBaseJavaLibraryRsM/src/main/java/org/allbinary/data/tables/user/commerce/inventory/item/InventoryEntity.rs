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
// package org.allbinary.data.tables.user.commerce.inventory.item
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::java::util::HashMap::HashMap;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
use crate::org::allbinary::business::context::modules::storefront::StoreFrontInterface::StoreFrontInterface;
use crate::org::allbinary::business::entry::EntryData::EntryData;
use crate::org::allbinary::business::init::db::InventoryDbInitInfo::InventoryDbInitInfo;
use crate::org::allbinary::business::user::commerce::inventory::item::BasicItem::BasicItem;
use crate::org::allbinary::business::user::commerce::inventory::item::BasicItemData::BasicItemData;
use crate::org::allbinary::business::user::commerce::inventory::item::ItemInterface::ItemInterface;
use crate::org::allbinary::business::user::commerce::money::MoneyException::MoneyException;
use crate::org::allbinary::logic::StdUtil::StdUtil;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::communication::sql::AbSqlBean::AbSqlBean;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::string::StringValidationUtil::StringValidationUtil;
use crate::java::lang::Exception::*;
use super::InventoryEntityInterface::InventoryEntityInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct InventoryEntity {
    pub base: AbSqlBean,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub basicItemData: Option<Rc<RefCell<BasicItemData>>>,
    pub tableName: String,
}

// impl InventoryEntityInterface for InventoryEntity with the methods of the trait

impl InventoryEntity {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: AbSqlBean::new_struct(InventoryDbInitInfo::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::business::init::db::DbConnectionInfo::DbConnectionInfoDyn>>)),
            logUtil: None,
            basicItemData: None,
            tableName: String::new(),
        };
        this.logUtil = LogUtil::getInstance();
        this.basicItemData = BasicItemData::getInstance();
        this.tableName = String::from("basicinventory");
        // super(...) is in the struct literal above
        { let java_arguments = (this.tableName.clone(),); this.setTableName(java_arguments.0) };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn insert(&mut self, mut values: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            { let java_arguments = (values.clone(),); self.base.insert(java_arguments.0) };
            if jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypes.clone(); java_field }).LOGGING.clone(); java_field }).contains({ let java_field = jcall!(jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypeFactory.clone(); java_field }).getInstance()).SQLLOGGING.clone(); java_field }.clone()) {
                jcall!(self.logUtil).putF({ let java_field = jcall!(self.commonStrings).SUCCESS.clone(); java_field }.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), self.base.INSERT.clone());
            }
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            if jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypes.clone(); java_field }).LOGGING.clone(); java_field }).contains({ let java_field = jcall!(jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypeFactory.clone(); java_field }).getInstance()).SQLLOGGING.clone(); java_field }.clone()) {
                jcall!(self.logUtil).put({ let java_field = jcall!(self.commonStrings).FAILURE.clone(); java_field }.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), self.base.INSERT.clone(), crate::java_into_object(e.clone()));
            }
            }
        }
    }
    

    pub fn delete(&mut self, mut value: String) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            { let java_arguments = ({ let java_field = jcall!(self.basicItemData).ID.clone(); java_field }.clone(), value.clone(),); self.base.deleteWhere2(java_arguments.0, java_arguments.1) };
            if jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypes.clone(); java_field }).LOGGING.clone(); java_field }).contains({ let java_field = jcall!(jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypeFactory.clone(); java_field }).getInstance()).SQLLOGGING.clone(); java_field }.clone()) {
                jcall!(self.logUtil).putF({ let java_field = jcall!(self.commonStrings).SUCCESS.clone(); java_field }.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(self.commonStrings).delete.clone(); java_field }.clone());
            }
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            if jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypes.clone(); java_field }).LOGGING.clone(); java_field }).contains({ let java_field = jcall!(jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypeFactory.clone(); java_field }).getInstance()).SQLLOGGING.clone(); java_field }.clone()) {
                jcall!(self.logUtil).put({ let java_field = jcall!(self.commonStrings).FAILURE.clone(); java_field }.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(self.commonStrings).delete.clone(); java_field }.clone(), crate::java_into_object(e.clone()));
            }
            }
        }
    }
    

    // throws Exception
    pub fn getItems(&mut self, mut storeFrontInterface: Option<Rc<RefCell<dyn StoreFrontInterface>>>) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        if jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypes.clone(); java_field }).LOGGING.clone(); java_field }).contains({ let java_field = jcall!(jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypeFactory.clone(); java_field }).getInstance()).SQLLOGGING.clone(); java_field }.clone()) {
            { let java_receiver = self.logUtil.clone(); let java_arguments = (format!("{}{}", String::from("Getting Items For: "), jcall!(storeFrontInterface).getName()), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("getItems"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        }
        let mut itemVector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        let mut keysAndValues: Option<Rc<RefCell<HashMap<crate::JavaObject, crate::JavaObject>>>> = jcall!(StdUtil::getInstance()).createHashMap();
        let mut itemHashMapVector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = { let java_arguments = (keysAndValues.clone(),); self.base.getRows(java_arguments.0) };
        let mut size: i32 = jcall!(itemHashMapVector).size();
        for i in (0..size) {
            let mut itemHashMap: Option<Rc<RefCell<HashMap<crate::JavaObject, crate::JavaObject>>>> = { let java_object = (jcall!(itemHashMapVector).get(i.clone())).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<HashMap<crate::JavaObject, crate::JavaObject>>>>>().expect("ClassCastException").clone()) };
            if itemHashMap.is_some() {
                let mut category: String = { let java_object = (jcall!(itemHashMap).get(crate::java_into_object({ let java_field = jcall!(self.basicItemData).CATEGORY.clone(); java_field }.clone()))).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() };
                if !jcall!(StringValidationUtil::getInstance()).isEmpty(category.clone()) && category.starts_with(&jcall!(storeFrontInterface).getCategoryPath()) {
                    jcall!(itemVector).add(crate::java_into_object(BasicItem::new1(itemHashMap.clone()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::business::user::commerce::inventory::item::BasicItem::BasicItemDyn>>)));
                }
            }
        }
        return itemVector.clone();
    }
    

    // throws MoneyException
    pub fn getItem(&mut self, mut id: String) -> Option<Rc<RefCell<dyn ItemInterface>>> {
        let mut keysAndValues: Option<Rc<RefCell<HashMap<crate::JavaObject, crate::JavaObject>>>> = jcall!(StdUtil::getInstance()).createHashMap();
        jcall!(keysAndValues).put(crate::java_into_object({ let java_field = jcall!(self.basicItemData).ID.clone(); java_field }.clone()), crate::java_into_object(id.clone()));
        let mut itemHashMap: Option<Rc<RefCell<HashMap<crate::JavaObject, crate::JavaObject>>>> = { let java_arguments = (keysAndValues.clone(),); self.base.getRow(java_arguments.0) };
        if itemHashMap.is_some() {
            return BasicItem::new1(itemHashMap.clone()).map(|reference| reference as Rc<RefCell<dyn ItemInterface>>).map(|_java_reference| -> Rc<RefCell<dyn ItemInterface>> { panic!("ClassCastException: the Rust trait of BasicItem does not have ItemInterface as a base trait") });
        } else {
            return None;
        }
    }
    

    pub fn getWeight(&mut self, mut id: String) -> String {
        return { let java_arguments = ({ let java_field = jcall!(self.basicItemData).ID.clone(); java_field }.clone(), id.clone(), { let java_field = jcall!(self.basicItemData).WEIGHT.clone(); java_field }.clone(),); self.base.getField3(java_arguments.0, java_arguments.1, java_arguments.2) };
    }
    

    pub fn createTableStatement(&mut self) -> String {
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        jcall!(stringBuffer).append({ let java_field = jcall!(self.sqlStrings).CREATE_TABLE.clone(); java_field }.clone());
        jcall!(stringBuffer).append(self.tableName.clone());
        jcall!(stringBuffer).append({ let java_field = jcall!(self.sqlStrings).START.clone(); java_field }.clone());
        jcall!(stringBuffer).append({ let java_field = jcall!(self.basicItemData).ID.clone(); java_field }.clone());
        jcall!(stringBuffer).append({ let java_field = jcall!(self.sqlTypeStrings).MAX_BIG_INT_UNSIGNED_NOT_NULL.clone(); java_field }.clone());
        jcall!(stringBuffer).append({ let java_field = jcall!(self.basicItemData).NUMBER.clone(); java_field }.clone());
        jcall!(stringBuffer).append({ let java_field = jcall!(self.sqlTypeStrings).MAX_BIG_INT_UNSIGNED_NOT_NULL.clone(); java_field }.clone());
        jcall!(stringBuffer).append({ let java_field = jcall!(self.basicItemData).INBASKETS.clone(); java_field }.clone());
        jcall!(stringBuffer).append({ let java_field = jcall!(self.sqlTypeStrings).MAX_BIG_INT_UNSIGNED_NOT_NULL.clone(); java_field }.clone());
        jcall!(stringBuffer).append({ let java_field = jcall!(self.basicItemData).WEIGHT.clone(); java_field }.clone());
        jcall!(stringBuffer).append(String::from(" VARCHAR(20) NOT NULL,"));
        jcall!(stringBuffer).append({ let java_field = jcall!(EntryData::getInstance()).ENABLE.clone(); java_field }.clone());
        jcall!(stringBuffer).append({ let java_field = jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone(); java_field }.clone());
        jcall!(stringBuffer).append({ let java_field = jcall!(self.basicItemData).NEWORUSED.clone(); java_field }.clone());
        jcall!(stringBuffer).append({ let java_field = jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone(); java_field }.clone());
        jcall!(stringBuffer).append({ let java_field = jcall!(self.basicItemData).SUMMARY.clone(); java_field }.clone());
        jcall!(stringBuffer).append({ let java_field = jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone(); java_field }.clone());
        jcall!(stringBuffer).append({ let java_field = jcall!(self.basicItemData).DISTRIBUTOR.clone(); java_field }.clone());
        jcall!(stringBuffer).append({ let java_field = jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone(); java_field }.clone());
        jcall!(stringBuffer).append({ let java_field = jcall!(self.basicItemData).IDUSEDBYDISTRIBUTOR.clone(); java_field }.clone());
        jcall!(stringBuffer).append({ let java_field = jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone(); java_field }.clone());
        jcall!(stringBuffer).append({ let java_field = jcall!(self.basicItemData).PRODUCEDBY.clone(); java_field }.clone());
        jcall!(stringBuffer).append({ let java_field = jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone(); java_field }.clone());
        jcall!(stringBuffer).append({ let java_field = jcall!(self.basicItemData).PRODUCTIONDATE.clone(); java_field }.clone());
        jcall!(stringBuffer).append({ let java_field = jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone(); java_field }.clone());
        jcall!(stringBuffer).append({ let java_field = jcall!(self.basicItemData).STARTPRODUCTIONDATE.clone(); java_field }.clone());
        jcall!(stringBuffer).append({ let java_field = jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone(); java_field }.clone());
        jcall!(stringBuffer).append({ let java_field = jcall!(self.basicItemData).DESCRIPTION.clone(); java_field }.clone());
        jcall!(stringBuffer).append({ let java_field = jcall!(self.sqlTypeStrings).BLOB_NOT_NULL.clone(); java_field }.clone());
        jcall!(stringBuffer).append({ let java_field = jcall!(self.basicItemData).KEYWORDS.clone(); java_field }.clone());
        jcall!(stringBuffer).append({ let java_field = jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone(); java_field }.clone());
        jcall!(stringBuffer).append({ let java_field = jcall!(self.basicItemData).CATEGORY.clone(); java_field }.clone());
        jcall!(stringBuffer).append({ let java_field = jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone(); java_field }.clone());
        jcall!(stringBuffer).append({ let java_field = jcall!(self.basicItemData).TYPE.clone(); java_field }.clone());
        jcall!(stringBuffer).append({ let java_field = jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone(); java_field }.clone());
        jcall!(stringBuffer).append({ let java_field = jcall!(self.basicItemData).SMALLIMAGE.clone(); java_field }.clone());
        jcall!(stringBuffer).append({ let java_field = jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone(); java_field }.clone());
        jcall!(stringBuffer).append({ let java_field = jcall!(self.basicItemData).MEDIUMIMAGE.clone(); java_field }.clone());
        jcall!(stringBuffer).append({ let java_field = jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone(); java_field }.clone());
        jcall!(stringBuffer).append({ let java_field = jcall!(self.basicItemData).LARGEIMAGE.clone(); java_field }.clone());
        jcall!(stringBuffer).append({ let java_field = jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone(); java_field }.clone());
        jcall!(stringBuffer).append({ let java_field = jcall!(EntryData::getInstance()).LASTMODIFIED.clone(); java_field }.clone());
        jcall!(stringBuffer).append({ let java_field = jcall!(self.sqlTypeStrings).MAX_BIG_INT_UNSIGNED_NOT_NULL.clone(); java_field }.clone());
        jcall!(stringBuffer).append({ let java_field = jcall!(EntryData::getInstance()).TIMECREATED.clone(); java_field }.clone());
        jcall!(stringBuffer).append({ let java_field = jcall!(self.sqlTypeStrings).MAX_BIG_INT_UNSIGNED_NOT_NULL.clone(); java_field }.clone());
        jcall!(stringBuffer).append({ let java_field = jcall!(self.basicItemData).PRICE.clone(); java_field }.clone());
        jcall!(stringBuffer).append(String::from(" VARCHAR(20) NOT NULL,"));
        jcall!(stringBuffer).append({ let java_field = jcall!(self.basicItemData).COMMENT.clone(); java_field }.clone());
        jcall!(stringBuffer).append({ let java_field = jcall!(self.sqlTypeStrings).BLOB_NOT_NULL.clone(); java_field }.clone());
        jcall!(stringBuffer).append({ let java_field = jcall!(self.basicItemData).CUSTOMS.clone(); java_field }.clone());
        jcall!(stringBuffer).append({ let java_field = jcall!(self.sqlTypeStrings).MAX_BIG_INT_UNSIGNED_NOT_NULL.clone(); java_field }.clone());
        jcall!(stringBuffer).append({ let java_field = jcall!(self.basicItemData).DOWNLOADS.clone(); java_field }.clone());
        jcall!(stringBuffer).append({ let java_field = jcall!(self.sqlTypeStrings).MAX_BIG_INT_UNSIGNED_NOT_NULL.clone(); java_field }.clone());
        jcall!(stringBuffer).append({ let java_field = jcall!(self.basicItemData).GROUPS.clone(); java_field }.clone());
        jcall!(stringBuffer).append({ let java_field = jcall!(self.sqlTypeStrings).MAX_BIG_INT_UNSIGNED_NOT_NULL.clone(); java_field }.clone());
        jcall!(stringBuffer).append({ let java_field = jcall!(self.basicItemData).OPTIONS.clone(); java_field }.clone());
        jcall!(stringBuffer).append({ let java_field = jcall!(self.sqlTypeStrings).MAX_BIG_INT_UNSIGNED_NOT_NULL.clone(); java_field }.clone());
        jcall!(stringBuffer).append({ let java_field = jcall!(self.basicItemData).PERMISSIONS.clone(); java_field }.clone());
        jcall!(stringBuffer).append({ let java_field = jcall!(self.sqlTypeStrings).MAX_BIG_INT_UNSIGNED_NOT_NULL.clone(); java_field }.clone());
        jcall!(stringBuffer).append({ let java_field = jcall!(self.basicItemData).SPECIALS.clone(); java_field }.clone());
        jcall!(stringBuffer).append({ let java_field = jcall!(self.sqlTypeStrings).MAX_BIG_INT_UNSIGNED_NOT_NULL.clone(); java_field }.clone());
        jcall!(stringBuffer).append({ let java_field = jcall!(self.sqlStrings).PRIMARY_KEY.clone(); java_field }.clone());
        jcall!(stringBuffer).append({ let java_field = jcall!(self.basicItemData).ID.clone(); java_field }.clone());
        jcall!(stringBuffer).append({ let java_field = jcall!(self.sqlStrings).END.clone(); java_field }.clone());
        return jcall!(stringBuffer).toString();
    }
    

    pub fn createTable(&mut self) -> String {
        return { let java_arguments = (self.createTableStatement(),); self.base.createTable(java_arguments.0) };
    }
    

    pub fn update(&mut self, mut updatedValues: Option<Rc<RefCell<HashMap<crate::JavaObject, crate::JavaObject>>>>) {
        { let java_arguments = ({ let java_field = jcall!(self.basicItemData).ID.clone(); java_field }.clone(), { let java_object = (jcall!(updatedValues).get(crate::java_into_object({ let java_field = jcall!(self.basicItemData).ID.clone(); java_field }.clone()))).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() }, updatedValues.clone(),); self.base.updateWhere3(java_arguments.0, java_arguments.1, java_arguments.2) };
    }
    

    pub fn dropTable(&mut self) -> String {
        return self.base.dropTable();
    }
}
impl std::ops::Deref for InventoryEntity {
    type Target = AbSqlBean;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for InventoryEntity {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::logic::communication::sql::AbSqlBasic::AbSqlBasicDyn for InventoryEntity {
    fn as_AbSqlBasic(&self) -> &crate::org::allbinary::logic::communication::sql::AbSqlBasic::AbSqlBasic { &self.base.base.base.base.base }
    fn as_AbSqlBasic_mut(&mut self) -> &mut crate::org::allbinary::logic::communication::sql::AbSqlBasic::AbSqlBasic { &mut self.base.base.base.base.base }
}
impl crate::org::allbinary::logic::communication::sql::AbSqlBean::AbSqlBeanDyn for InventoryEntity {
    fn as_AbSqlBean(&self) -> &crate::org::allbinary::logic::communication::sql::AbSqlBean::AbSqlBean { &self.base }
    fn as_AbSqlBean_mut(&mut self) -> &mut crate::org::allbinary::logic::communication::sql::AbSqlBean::AbSqlBean { &mut self.base }
}
impl crate::org::allbinary::logic::communication::sql::AbSqlColumn::AbSqlColumnDyn for InventoryEntity {
    fn as_AbSqlColumn(&self) -> &crate::org::allbinary::logic::communication::sql::AbSqlColumn::AbSqlColumn { &self.base.base.base }
    fn as_AbSqlColumn_mut(&mut self) -> &mut crate::org::allbinary::logic::communication::sql::AbSqlColumn::AbSqlColumn { &mut self.base.base.base }
}
impl crate::org::allbinary::logic::communication::sql::AbSqlRow::AbSqlRowDyn for InventoryEntity {
    fn as_AbSqlRow(&self) -> &crate::org::allbinary::logic::communication::sql::AbSqlRow::AbSqlRow { &self.base.base }
    fn as_AbSqlRow_mut(&mut self) -> &mut crate::org::allbinary::logic::communication::sql::AbSqlRow::AbSqlRow { &mut self.base.base }
    fn insert(&mut self, mut values: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) { Self::insert(self, values) }
}
impl crate::org::allbinary::logic::communication::sql::AbSqlTable::AbSqlTableDyn for InventoryEntity {
    fn as_AbSqlTable(&self) -> &crate::org::allbinary::logic::communication::sql::AbSqlTable::AbSqlTable { &self.base.base.base.base }
    fn as_AbSqlTable_mut(&mut self) -> &mut crate::org::allbinary::logic::communication::sql::AbSqlTable::AbSqlTable { &mut self.base.base.base.base }
    fn dropTable(&mut self) -> String { Self::dropTable(self) }
}


