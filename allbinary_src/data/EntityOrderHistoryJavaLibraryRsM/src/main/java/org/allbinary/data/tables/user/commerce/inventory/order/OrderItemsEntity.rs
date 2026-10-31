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
// package org.allbinary.data.tables.user.commerce.inventory.order
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;
use super::*;
// use crate::java::util::HashMap::HashMap;
// use crate::java::util::ListIterator::ListIterator;
// use crate::java::util::Set::Set;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
use crate::org::allbinary::business::context::modules::storefront::StoreFrontData::StoreFrontData;
use crate::org::allbinary::business::entry::EntryData::EntryData;
use crate::org::allbinary::business::init::db::HistoryDbInitInfo::HistoryDbInitInfo;
use crate::org::allbinary::logic::StdUtil::StdUtil;
use crate::org::allbinary::business::user::UserData::UserData;
use crate::org::allbinary::business::user::address::ShippingAddressData::ShippingAddressData;
use crate::org::allbinary::business::user::commerce::inventory::basket::BasketInterface::BasketInterface;
use crate::org::allbinary::business::user::commerce::inventory::basket::BasketReview::BasketReview;
use crate::org::allbinary::business::user::commerce::inventory::item::BasicItemData::BasicItemData;
use crate::org::allbinary::business::user::commerce::inventory::item::Item::Item;
use crate::org::allbinary::business::user::commerce::inventory::item::ItemInterface::ItemInterface;
use crate::org::allbinary::business::user::commerce::inventory::order::OrderData::OrderData;
use crate::org::allbinary::business::user::commerce::inventory::order::OrderHistoryData::OrderHistoryData;
use crate::org::allbinary::business::user::commerce::inventory::order::OrderInterface::OrderInterface;
use crate::org::allbinary::business::user::commerce::shipping::ShippingMethodData::ShippingMethodData;
use crate::org::allbinary::data::generator::OrderItemIdGenerator::OrderItemIdGenerator;
use crate::org::allbinary::data::tables::TableDataFactory::TableDataFactory;
use crate::org::allbinary::data::tables::user::commerce::inventory::item::InventoryEntity::InventoryEntity;
use crate::org::allbinary::data::tables::user::commerce::inventory::item::InventoryEntityFactory::InventoryEntityFactory;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::communication::sql::AbSqlBean::AbSqlBean;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use super::OrderItemsEntityInterface::OrderItemsEntityInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct OrderItemsEntity {
    pub base: AbSqlBean,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub basicItemData: Option<Rc<RefCell<BasicItemData>>>,
    pub tableName: String,
}

// impl OrderItemsEntityInterface for OrderItemsEntity with the methods of the trait

impl OrderItemsEntity {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: AbSqlBean::new_struct(HistoryDbInitInfo::new()),
            logUtil: None,
            basicItemData: None,
            tableName: String::new(),
        };
        this.logUtil = LogUtil::getInstance();
        this.basicItemData = BasicItemData::getInstance();
        this.tableName = String::from("orderitems");
        // super(...) is in the struct literal above
        this.setTableName(this.tableName.clone());
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn insert(&mut self, mut userName: String, mut order: Option<Rc<RefCell<dyn OrderInterface>>>) {
        let mut vector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut basket: Option<Rc<RefCell<dyn BasketInterface>>> = jcall!(order).getBasket();
            let mut items: Option<Rc<RefCell<dyn Set>>> = jcall!(basket).getIds();
            let mut ZERO_STRING: String = jcall!(TableDataFactory::getInstance()).ZERO_STRING.clone();
            let mut EMPTY_STRING: String = jcall!(StringUtil::getInstance()).EMPTY_STRING.clone();
            let mut inventoryEntity: Option<Rc<RefCell<InventoryEntity>>> = jcall!(InventoryEntityFactory::getInstance()).getInventoryEntityInstance();
            let mut itemsArray: Vec<Option<Rc<RefCell<Box<dyn std::any::Any>>>>> = jcall!(items).toArray();
            let mut itemsSize: i32 = (itemsArray.len() as i32).clone();
            for i in (0..itemsSize) {
                let mut item: String = (itemsArray[(i) as usize]).clone() /* as String */;
                let mut itemInterface: Option<Rc<RefCell<dyn ItemInterface>>> = jcall!(inventoryEntity).getItem(item.clone());
                vector = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
                jcall!(vector).add(crate::java_into_object(jcall!(OrderItemIdGenerator::new()).getNext()));
                jcall!(vector).add(crate::java_into_object(jcall!(order).getId()));
                jcall!(vector).add(crate::java_into_object(jcall!(TableDataFactory::getInstance()).INTEGER_MAX_VALUE_STRING.clone()));
                jcall!(vector).add(crate::java_into_object(userName.clone()));
                jcall!(vector).add(crate::java_into_object(jcall!(order).getStoreName()));
                jcall!(vector).add(crate::java_into_object(jcall!(itemInterface).getId()));
                jcall!(vector).add(crate::java_into_object(jcall!(jcall!(basket).getNumberOf(item.clone())).toString()));
                jcall!(vector).add(crate::java_into_object(jcall!(itemInterface).getNewOrUsed()));
                jcall!(vector).add(crate::java_into_object(jcall!(itemInterface).getSummary()));
                jcall!(vector).add(crate::java_into_object(jcall!(itemInterface).getDistributor()));
                jcall!(vector).add(crate::java_into_object(jcall!(itemInterface).getIdUsedByDistributor()));
                jcall!(vector).add(crate::java_into_object(jcall!(itemInterface).getProducedBy()));
                jcall!(vector).add(crate::java_into_object(jcall!(itemInterface).getProductionDate()));
                jcall!(vector).add(crate::java_into_object(jcall!(itemInterface).getStartProductionDate()));
                jcall!(vector).add(crate::java_into_object(jcall!(itemInterface).getDescription()));
                jcall!(vector).add(crate::java_into_object(jcall!(itemInterface).getKeywords()));
                jcall!(vector).add(crate::java_into_object(jcall!(itemInterface).getCategory()));
                jcall!(vector).add(crate::java_into_object(jcall!(itemInterface).getType()));
                jcall!(vector).add(crate::java_into_object(jcall!(itemInterface).getSmallImage()));
                jcall!(vector).add(crate::java_into_object(jcall!(itemInterface).getMediumImage()));
                jcall!(vector).add(crate::java_into_object(jcall!(itemInterface).getLargeImage()));
                jcall!(vector).add(crate::java_into_object(jcall!(itemInterface).getWeight()));
                jcall!(vector).add(crate::java_into_object(jcall!(jcall!(itemInterface).getPrice()).toString()));
                jcall!(vector).add(crate::java_into_object(jcall!(itemInterface).getComment()));
                jcall!(vector).add(crate::java_into_object(jcall!(itemInterface).getCustoms()));
                jcall!(vector).add(crate::java_into_object(jcall!(itemInterface).getDownloads()));
                jcall!(vector).add(crate::java_into_object(EMPTY_STRING.clone()));
                jcall!(vector).add(crate::java_into_object(EMPTY_STRING.clone()));
                jcall!(vector).add(crate::java_into_object(EMPTY_STRING.clone()));
                jcall!(vector).add(crate::java_into_object(EMPTY_STRING.clone()));
                jcall!(vector).add(crate::java_into_object(EMPTY_STRING.clone()));
                jcall!(vector).add(crate::java_into_object(EMPTY_STRING.clone()));
                jcall!(vector).add(crate::java_into_object(EMPTY_STRING.clone()));
                jcall!(vector).add(crate::java_into_object(EMPTY_STRING.clone()));
                jcall!(vector).add(crate::java_into_object(EMPTY_STRING.clone()));
                jcall!(vector).add(crate::java_into_object(EMPTY_STRING.clone()));
                jcall!(vector).add(crate::java_into_object(EMPTY_STRING.clone()));
                jcall!(vector).add(crate::java_into_object(EMPTY_STRING.clone()));
                jcall!(vector).add(crate::java_into_object(EMPTY_STRING.clone()));
                jcall!(vector).add(crate::java_into_object(ZERO_STRING.clone()));
                jcall!(vector).add(crate::java_into_object(ZERO_STRING.clone()));
                jcall!(vector).add(crate::java_into_object(ZERO_STRING.clone()));
                jcall!(vector).add(crate::java_into_object(ZERO_STRING.clone()));
                self.insert(vector.clone());
            }
            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).SQLLOGGING.clone()) {
                { let java_receiver = self.logUtil.clone(); let java_arguments = (format!("{}{}", String::from("Command Success Added: "), jcall!(items).size()), crate::java_into_object(String::from(std::any::type_name::<Self>())), INSERT.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
            }
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).SQLLOGGING.clone()) {
                { let java_receiver = self.logUtil.clone(); let java_arguments = (format!("{}{}", String::from("Command Failed: "), jcall!(vector).toString()), crate::java_into_object(String::from(std::any::type_name::<Self>())), INSERT.clone(), crate::java_into_object(e.clone()),); jcall!(java_receiver).put(java_arguments.0, java_arguments.1, java_arguments.2, java_arguments.3); };
            }
            }
        }
    }
    

    pub fn setStatus(&mut self, mut orderId: String, mut groupId: String, mut status: String) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut whereHashMap: Option<Rc<RefCell<HashMap>>> = jcall!(StdUtil::getInstance()).createHashMap();
            let mut updateHashMap: Option<Rc<RefCell<HashMap>>> = jcall!(StdUtil::getInstance()).createHashMap();
            jcall!(whereHashMap).put(crate::org::allbinary::business::user::commerce::inventory::order::OrderData::OrderData_ID.with(Clone::clone).clone(), orderId.clone());
            jcall!(whereHashMap).put(crate::org::allbinary::business::user::commerce::shipping::ShippingMethodData::ShippingMethodData_GROUP.with(Clone::clone).clone(), groupId.clone());
            jcall!(updateHashMap).put(crate::org::allbinary::business::user::commerce::inventory::order::OrderHistoryData::OrderHistoryData_STATUS.with(Clone::clone).clone(), status.clone());
            self.base.updateWhere(whereHashMap.clone(), updateHashMap.clone());
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).SQLLOGGING.clone()) {
                jcall!(self.logUtil).put(jcall!(self.commonStrings).FAILURE.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("setStatus"), crate::java_into_object(e.clone()));
            }
            }
        }
    }
    

    pub fn isEverythingShipped(&mut self, mut orderId: String) -> bool {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut itemStatusVector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = self.base.getColumnWhere(crate::org::allbinary::business::user::commerce::inventory::order::OrderHistoryData::OrderHistoryData_STATUS.with(Clone::clone).clone(), crate::org::allbinary::business::user::commerce::inventory::order::OrderData::OrderData_ID.with(Clone::clone).clone(), orderId.clone());
            let mut size: i32 = jcall!(itemStatusVector).size();
            for index in (0..size) {
                let mut status: String = (jcall!(itemStatusVector).get(index.clone())).clone() /* as String */;
                if (status.cmp(&crate::org::allbinary::business::user::commerce::inventory::order::OrderHistoryData::OrderHistoryData_SHIPPED.with(Clone::clone).clone()) as i32) != 0 {
                    return false;
                }
            }
            return true;
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).SQLLOGGING.clone()) {
                jcall!(self.logUtil).put(jcall!(self.commonStrings).FAILURE.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("isEverythingShipped"), crate::java_into_object(e.clone()));
            }
            return false;
            }
        }
    }
    

    pub fn getBasketReview(&mut self, mut orderId: String) -> Option<Rc<RefCell<BasketReview>>> {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut basketReview: Option<Rc<RefCell<BasketReview>>> = BasketReview::new();
            let mut keyValues: Option<Rc<RefCell<HashMap>>> = jcall!(StdUtil::getInstance()).createHashMap();
            jcall!(keyValues).put(crate::org::allbinary::business::user::commerce::inventory::order::OrderData::OrderData_ID.with(Clone::clone).clone(), orderId.clone());
            let mut items: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = self.base.getRows(keyValues.clone());
            let mut entryData: Option<Rc<RefCell<EntryData>>> = EntryData::getInstance();
            if items.is_some() && jcall!(items).size() > 0 {
                let mut EMPTY_STRING: String = jcall!(StringUtil::getInstance()).EMPTY_STRING.clone();
                let mut itemsArray: Vec<Option<Rc<RefCell<Box<dyn std::any::Any>>>>> = jcall!(items).toArray();
                let mut itemsSize: i32 = (itemsArray.len() as i32).clone();
                for i in (0..itemsSize) {
                    let mut itemHashMap: Option<Rc<RefCell<HashMap>>> = (itemsArray[(i) as usize]).clone() /* as HashMap */;
                    jcall!(itemHashMap).put(jcall!(self.basicItemData).INBASKETS.clone(), EMPTY_STRING.clone());
                    jcall!(itemHashMap).put(jcall!(entryData).ENABLE.clone(), EMPTY_STRING.clone());
                    jcall!(itemHashMap).put(jcall!(entryData).TIMECREATED.clone(), EMPTY_STRING.clone());
                    jcall!(itemHashMap).put(jcall!(entryData).LASTMODIFIED.clone(), EMPTY_STRING.clone());
                    jcall!(itemHashMap).put(jcall!(self.basicItemData).GROUPS.clone(), EMPTY_STRING.clone());
                    jcall!(itemHashMap).put(jcall!(self.basicItemData).OPTIONS.clone(), EMPTY_STRING.clone());
                    jcall!(itemHashMap).put(jcall!(self.basicItemData).PERMISSIONS.clone(), EMPTY_STRING.clone());
                    jcall!(itemHashMap).put(jcall!(self.basicItemData).SPECIALS.clone(), EMPTY_STRING.clone());
                    let mut item: Option<Rc<RefCell<dyn crate::org::allbinary::business::user::commerce::inventory::item::Item::ItemDyn>>> = Item::new1(itemHashMap.clone()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::business::user::commerce::inventory::item::Item::ItemDyn>>);
                    jcall!(basketReview).addItem(item.clone());
                }
                return basketReview.clone();
            } else {
                return None;
            }
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).SQLLOGGING.clone()) {
                jcall!(self.logUtil).put(jcall!(self.commonStrings).FAILURE.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("getBasketReview"), crate::java_into_object(e.clone()));
            }
            return None;
            }
        }
    }
    

    pub fn createTableStatement(&mut self) -> String {
        let mut entryData: Option<Rc<RefCell<EntryData>>> = EntryData::getInstance();
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        jcall!(stringBuffer).append(String::from("CREATE TABLE "));
        jcall!(stringBuffer).append(self.tableName.clone());
        jcall!(stringBuffer).append(String::from(" ("));
        jcall!(stringBuffer).append(jcall!(entryData).ID.clone());
        jcall!(stringBuffer).append(jcall!(self.sqlTypeStrings).MAX_BIG_INT_UNSIGNED_NOT_NULL.clone());
        jcall!(stringBuffer).append(crate::org::allbinary::business::user::commerce::inventory::order::OrderData::OrderData_ID.with(Clone::clone).clone());
        jcall!(stringBuffer).append(jcall!(self.sqlTypeStrings).MAX_BIG_INT_UNSIGNED_NOT_NULL.clone());
        jcall!(stringBuffer).append(crate::org::allbinary::business::user::commerce::shipping::ShippingMethodData::ShippingMethodData_GROUP.with(Clone::clone).clone());
        jcall!(stringBuffer).append(jcall!(self.sqlTypeStrings).MAX_BIG_INT_UNSIGNED_NOT_NULL.clone());
        jcall!(stringBuffer).append(crate::org::allbinary::business::user::UserData::UserData_USERNAME.with(Clone::clone).clone());
        jcall!(stringBuffer).append(jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone());
        jcall!(stringBuffer).append(jcall!(StoreFrontData::getInstance()).NAME.clone());
        jcall!(stringBuffer).append(jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone());
        jcall!(stringBuffer).append(jcall!(self.basicItemData).ID.clone());
        jcall!(stringBuffer).append(jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone());
        jcall!(stringBuffer).append(jcall!(self.basicItemData).NUMBER.clone());
        jcall!(stringBuffer).append(jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone());
        jcall!(stringBuffer).append(jcall!(self.basicItemData).NEWORUSED.clone());
        jcall!(stringBuffer).append(jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone());
        jcall!(stringBuffer).append(jcall!(self.basicItemData).SUMMARY.clone());
        jcall!(stringBuffer).append(jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone());
        jcall!(stringBuffer).append(jcall!(self.basicItemData).DISTRIBUTOR.clone());
        jcall!(stringBuffer).append(jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone());
        jcall!(stringBuffer).append(jcall!(self.basicItemData).IDUSEDBYDISTRIBUTOR.clone());
        jcall!(stringBuffer).append(jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone());
        jcall!(stringBuffer).append(jcall!(self.basicItemData).PRODUCEDBY.clone());
        jcall!(stringBuffer).append(jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone());
        jcall!(stringBuffer).append(jcall!(self.basicItemData).PRODUCTIONDATE.clone());
        jcall!(stringBuffer).append(jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone());
        jcall!(stringBuffer).append(jcall!(self.basicItemData).STARTPRODUCTIONDATE.clone());
        jcall!(stringBuffer).append(jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone());
        jcall!(stringBuffer).append(jcall!(self.basicItemData).DESCRIPTION.clone());
        jcall!(stringBuffer).append(jcall!(self.sqlTypeStrings).BLOB_NOT_NULL.clone());
        jcall!(stringBuffer).append(jcall!(self.basicItemData).KEYWORDS.clone());
        jcall!(stringBuffer).append(jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone());
        jcall!(stringBuffer).append(jcall!(self.basicItemData).CATEGORY.clone());
        jcall!(stringBuffer).append(jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone());
        jcall!(stringBuffer).append(jcall!(self.basicItemData).TYPE.clone());
        jcall!(stringBuffer).append(jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone());
        jcall!(stringBuffer).append(jcall!(self.basicItemData).SMALLIMAGE.clone());
        jcall!(stringBuffer).append(jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone());
        jcall!(stringBuffer).append(jcall!(self.basicItemData).MEDIUMIMAGE.clone());
        jcall!(stringBuffer).append(jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone());
        jcall!(stringBuffer).append(jcall!(self.basicItemData).LARGEIMAGE.clone());
        jcall!(stringBuffer).append(jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone());
        jcall!(stringBuffer).append(jcall!(self.basicItemData).WEIGHT.clone());
        jcall!(stringBuffer).append(jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone());
        jcall!(stringBuffer).append(jcall!(self.basicItemData).PRICE.clone());
        jcall!(stringBuffer).append(String::from(" VARCHAR(20) NOT NULL,"));
        jcall!(stringBuffer).append(jcall!(self.basicItemData).COMMENT.clone());
        jcall!(stringBuffer).append(jcall!(self.sqlTypeStrings).BLOB_NOT_NULL.clone());
        jcall!(stringBuffer).append(jcall!(self.basicItemData).CUSTOMS.clone());
        jcall!(stringBuffer).append(jcall!(self.sqlTypeStrings).MAX_BIG_INT_UNSIGNED_NOT_NULL.clone());
        jcall!(stringBuffer).append(jcall!(self.basicItemData).DOWNLOADS.clone());
        jcall!(stringBuffer).append(jcall!(self.sqlTypeStrings).MAX_BIG_INT_UNSIGNED_NOT_NULL.clone());
        jcall!(stringBuffer).append(crate::org::allbinary::business::user::commerce::shipping::ShippingMethodData::ShippingMethodData_COST.with(Clone::clone).clone());
        jcall!(stringBuffer).append(jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone());
        jcall!(stringBuffer).append(crate::org::allbinary::business::user::commerce::inventory::order::OrderHistoryData::OrderHistoryData_TAX.with(Clone::clone).clone());
        jcall!(stringBuffer).append(jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone());
        jcall!(stringBuffer).append(crate::org::allbinary::business::user::address::ShippingAddressData::ShippingAddressData_NAME.with(Clone::clone).clone());
        jcall!(stringBuffer).append(jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone());
        jcall!(stringBuffer).append(crate::org::allbinary::business::user::address::ShippingAddressData::ShippingAddressData_STREET.with(Clone::clone).clone());
        jcall!(stringBuffer).append(jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone());
        jcall!(stringBuffer).append(crate::org::allbinary::business::user::address::ShippingAddressData::ShippingAddressData_CITY.with(Clone::clone).clone());
        jcall!(stringBuffer).append(jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone());
        jcall!(stringBuffer).append(crate::org::allbinary::business::user::address::ShippingAddressData::ShippingAddressData_STATE.with(Clone::clone).clone());
        jcall!(stringBuffer).append(jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone());
        jcall!(stringBuffer).append(crate::org::allbinary::business::user::address::ShippingAddressData::ShippingAddressData_CODE.with(Clone::clone).clone());
        jcall!(stringBuffer).append(jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone());
        jcall!(stringBuffer).append(crate::org::allbinary::business::user::address::ShippingAddressData::ShippingAddressData_COUNTRY.with(Clone::clone).clone());
        jcall!(stringBuffer).append(jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone());
        jcall!(stringBuffer).append(crate::org::allbinary::business::user::commerce::shipping::ShippingMethodData::ShippingMethodData_NAME.with(Clone::clone).clone());
        jcall!(stringBuffer).append(jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone());
        jcall!(stringBuffer).append(jcall!(entryData).SPECIAL.clone());
        jcall!(stringBuffer).append(String::from(" VARCHAR(255) ,"));
        jcall!(stringBuffer).append(crate::org::allbinary::business::user::commerce::inventory::order::OrderHistoryData::OrderHistoryData_STATUS.with(Clone::clone).clone());
        jcall!(stringBuffer).append(jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone());
        jcall!(stringBuffer).append(crate::org::allbinary::business::user::commerce::inventory::order::OrderHistoryData::OrderHistoryData_CANCELINFO.with(Clone::clone).clone());
        jcall!(stringBuffer).append(jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone());
        jcall!(stringBuffer).append(crate::org::allbinary::business::user::commerce::inventory::order::OrderHistoryData::OrderHistoryData_CANCELTYPE.with(Clone::clone).clone());
        jcall!(stringBuffer).append(jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone());
        jcall!(stringBuffer).append(crate::org::allbinary::business::user::commerce::inventory::order::OrderHistoryData::OrderHistoryData_SHIPPEDDATE.with(Clone::clone).clone());
        jcall!(stringBuffer).append(String::from(" BIGINT(19) UNSIGNED ,"));
        jcall!(stringBuffer).append(crate::org::allbinary::business::user::commerce::inventory::order::OrderHistoryData::OrderHistoryData_ORDERDATE.with(Clone::clone).clone());
        jcall!(stringBuffer).append(jcall!(self.sqlTypeStrings).MAX_BIG_INT_UNSIGNED_NOT_NULL.clone());
        jcall!(stringBuffer).append(crate::org::allbinary::business::user::commerce::inventory::order::OrderHistoryData::OrderHistoryData_TRANSDATE.with(Clone::clone).clone());
        jcall!(stringBuffer).append(String::from(" BIGINT(19) UNSIGNED ,"));
        jcall!(stringBuffer).append(crate::org::allbinary::business::user::commerce::inventory::order::OrderHistoryData::OrderHistoryData_CANCELDATE.with(Clone::clone).clone());
        jcall!(stringBuffer).append(String::from(" BIGINT(19) UNSIGNED ,"));
        jcall!(stringBuffer).append(String::from("PRIMARY KEY("));
        jcall!(stringBuffer).append(jcall!(entryData).ID.clone());
        jcall!(stringBuffer).append(String::from(") )"));
        return jcall!(stringBuffer).toString();
    }
    

    pub fn createTable(&mut self) -> String {
        return self.base.createTable(self.createTableStatement());
    }
    

    pub fn dropTable(&mut self) -> String {
        return self.base.dropTable();
    }
}
impl std::ops::Deref for OrderItemsEntity {
    type Target = AbSqlBean;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for OrderItemsEntity {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::logic::communication::sql::AbSqlBasic::AbSqlBasicDyn for OrderItemsEntity {
    fn as_AbSqlBasic(&self) -> &crate::org::allbinary::logic::communication::sql::AbSqlBasic::AbSqlBasic { &self.base.base.base.base.base }
    fn as_AbSqlBasic_mut(&mut self) -> &mut crate::org::allbinary::logic::communication::sql::AbSqlBasic::AbSqlBasic { &mut self.base.base.base.base.base }
}
impl crate::org::allbinary::logic::communication::sql::AbSqlBean::AbSqlBeanDyn for OrderItemsEntity {
    fn as_AbSqlBean(&self) -> &crate::org::allbinary::logic::communication::sql::AbSqlBean::AbSqlBean { &self.base }
    fn as_AbSqlBean_mut(&mut self) -> &mut crate::org::allbinary::logic::communication::sql::AbSqlBean::AbSqlBean { &mut self.base }
}
impl crate::org::allbinary::logic::communication::sql::AbSqlColumn::AbSqlColumnDyn for OrderItemsEntity {
    fn as_AbSqlColumn(&self) -> &crate::org::allbinary::logic::communication::sql::AbSqlColumn::AbSqlColumn { &self.base.base.base }
    fn as_AbSqlColumn_mut(&mut self) -> &mut crate::org::allbinary::logic::communication::sql::AbSqlColumn::AbSqlColumn { &mut self.base.base.base }
}
impl crate::org::allbinary::logic::communication::sql::AbSqlRow::AbSqlRowDyn for OrderItemsEntity {
    fn as_AbSqlRow(&self) -> &crate::org::allbinary::logic::communication::sql::AbSqlRow::AbSqlRow { &self.base.base }
    fn as_AbSqlRow_mut(&mut self) -> &mut crate::org::allbinary::logic::communication::sql::AbSqlRow::AbSqlRow { &mut self.base.base }
}
impl crate::org::allbinary::logic::communication::sql::AbSqlTable::AbSqlTableDyn for OrderItemsEntity {
    fn as_AbSqlTable(&self) -> &crate::org::allbinary::logic::communication::sql::AbSqlTable::AbSqlTable { &self.base.base.base.base }
    fn as_AbSqlTable_mut(&mut self) -> &mut crate::org::allbinary::logic::communication::sql::AbSqlTable::AbSqlTable { &mut self.base.base.base.base }
    fn dropTable(&mut self) -> String { Self::dropTable(self) }
}


